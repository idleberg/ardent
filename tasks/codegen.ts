/**
 * Generates Ardent's lookup tables from the Dent Style Specification.
 *
 * The spec owns the data; this script only emits the Rust form of it. Run it after upgrading
 * the spec and commit the result. Pass `--check` to verify the committed files are in step
 * with the installed spec (used in CI).
 *
 * The spec is located via `DENT_SPEC_DIR`, or `node_modules/@nsis/dent-spec`.
 */

import { existsSync } from 'node:fs';
import { mkdir, mkdtemp, readFile, rm, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { dirname, join } from 'node:path';

type CanonicalTable = { values: string[] };
type ParametersTable = {
	global: string[];
	globalPrefixes: string[];
	instruction: Record<string, string[]>;
};
type VariablesTable = { variables: string[]; defines: string[]; langStrings: string[] };
type BlocksTable = Record<'open' | 'case' | 'close' | 'mid' | 'closeAfter', string[]>;

const check = process.argv.includes('--check');
const srcDir = join(import.meta.dir, '..', 'src');

function resolveSpecDir(): string {
	const candidates = [
		process.env.DENT_SPEC_DIR,
		join(import.meta.dir, '..', 'node_modules', '@nsis', 'dent-spec'),
	].filter((path): path is string => Boolean(path));

	for (const candidate of candidates) {
		if (existsSync(join(candidate, 'tables', 'casing.json'))) return candidate;
	}

	console.error(
		'Cannot find @nsis/dent-spec.\n' +
			'Run `bun install`, or point DENT_SPEC_DIR at a checkout of the spec package.',
	);
	process.exit(1);
}

const specDir = resolveSpecDir();

async function table<T>(name: string): Promise<T> {
	return JSON.parse(await readFile(join(specDir, 'tables', `${name}.json`), 'utf-8')) as T;
}

const specVersion = JSON.parse(await readFile(join(specDir, 'package.json'), 'utf-8')).version;

const casing = await table<CanonicalTable>('casing');
const includes = await table<CanonicalTable>('includes');
const parameters = await table<ParametersTable>('parameters');
const variables = await table<VariablesTable>('variables');
const blocks = await table<BlocksTable>('blocks');

/** Rust string literal — the tables contain backslashes (`$INSTDIR\\…`) and quotes. */
function quote(value: string): string {
	return `"${value.replaceAll('\\', '\\\\').replaceAll('"', '\\"')}"`;
}

function banner(description: string): string {
	return `//! ${description}\n//!\n//! Generated from the Dent Style Specification ${specVersion} by \`mise run spec:codegen\`.\n//! Do not edit — change the spec instead.\n`;
}

/** `HashMap<&'static str, &'static str>` keyed by the lowercased spelling. */
function mapTable(name: string, doc: string, values: string[]): string {
	const entries = values
		.map((value) => `\t\t(${quote(value.toLowerCase())}, ${quote(value)}),`)
		.join('\n');

	return `/// ${doc}\npub static ${name}: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {\n\tHashMap::from([\n${entries}\n\t])\n});\n`;
}

function setTable(name: string, doc: string, values: string[]): string {
	const entries = values.map((value) => `\t\t${quote(value)},`).join('\n');

	return `/// ${doc}\npub static ${name}: LazyLock<HashSet<String>> = LazyLock::new(|| {\n\tlower_set(&[\n${entries}\n\t])\n});\n`;
}

const files: Record<string, string> = {
	'canonical_casing.rs': `${banner('Canonical casing lookup table for NSIS instructions and compiler commands.')}
use std::collections::HashMap;
use std::sync::LazyLock;

${mapTable(
	'CANONICAL_CASING',
	'Maps lowercased NSIS instruction names to their canonical casing.',
	casing.values,
)}`,

	'canonical_includes.rs': `${banner('Canonical casing lookup table for the NSIS bundled include libraries.')}
use std::collections::HashMap;
use std::sync::LazyLock;

${mapTable(
	'CANONICAL_INCLUDES',
	'Maps lowercased macro keywords (including `${...}` delimiters) to their canonical casing.',
	includes.values,
)}`,

	'canonical_variables.rs': `${banner('Canonical casing lookup tables for NSIS built-in names.')}
use std::collections::HashMap;
use std::sync::LazyLock;

${mapTable(
	'BUILTIN_VARIABLES',
	'Maps lowercased built-in variable names (without the leading `$`) to their canonical casing.',
	variables.variables,
)}
${mapTable(
	'BUILTIN_DEFINES',
	'Maps lowercased built-in define names (without the surrounding `${}`) to their canonical casing.',
	variables.defines,
)}
${mapTable(
	'BUILTIN_LANGSTRINGS',
	'Maps lowercased built-in language string names (including the leading `^`) to their canonical casing.',
	variables.langStrings,
)}`,

	'canonical_parameters.rs': `${banner('Canonical casing lookup tables for NSIS instruction parameters.')}
use std::collections::HashMap;
use std::sync::LazyLock;

${mapTable(
	'GLOBAL_PARAMETERS',
	'Maps lowercased global parameters (e.g. `/silent`) to their canonical casing.',
	parameters.global,
)}
${mapTable(
	'GLOBAL_PARAMETER_PREFIXES',
	'Maps lowercased parameter prefixes (e.g. `/timeout=`) to their canonical casing.',
	parameters.globalPrefixes,
)}
/// Maps lowercased instruction names to the canonical casing of their parameters.
pub static INSTRUCTION_PARAMETERS: LazyLock<
	HashMap<&'static str, HashMap<&'static str, &'static str>>,
> = LazyLock::new(|| {
	HashMap::from([
${Object.entries(parameters.instruction)
	.map(
		([instruction, values]) =>
			`\t\t(\n\t\t\t${quote(instruction)},\n\t\t\tHashMap::from([\n${values
				.map((value) => `\t\t\t\t(${quote(value.toLowerCase())}, ${quote(value)}),`)
				.join('\n')}\n\t\t\t]),\n\t\t),`,
	)
	.join('\n')}
	])
});
`,

	'rules.rs': `${banner('Block-structure rules: which keywords open, close or continue a block.')}
use std::collections::HashSet;
use std::sync::LazyLock;

fn lower_set(keywords: &[&str]) -> HashSet<String> {
	keywords.iter().map(|k| k.to_lowercase()).collect()
}

${setTable('OPEN', 'Keywords that open a new indentation block (e.g. `Section`, `Function`, `!if`).', blocks.open)}
${setTable(
	'CASE',
	'Keywords that open a case arm within a switch/select block.\n/// These print one level inside their parent and indent their body one further level,\n/// without pushing to the indent stack.',
	blocks.case,
)}
${setTable('CLOSE', 'Keywords that close an indentation block (e.g. `SectionEnd`, `!endif`).', blocks.close)}
${setTable(
	'MID',
	'Keywords printed at the opening keyword’s level without changing the current depth (e.g. `${Else}`).',
	blocks.mid,
)}
${setTable('CLOSE_AFTER', 'Keywords printed at the current level that then close the block (e.g. `${Break}`).', blocks.closeAfter)}`,
};

/**
 * Emitted tables are run through rustfmt, so what `cargo fmt` would produce is what the
 * generator produces — otherwise `--check` would report drift after every `cargo fmt`.
 */
async function emit(path: string, contents: string): Promise<string> {
	await mkdir(dirname(path), { recursive: true });
	await writeFile(path, contents);

	// `--config-path` so a file emitted outside the repo still gets the repo's rustfmt settings.
	const formatted = Bun.spawnSync([
		'rustfmt',
		'--edition',
		'2024',
		'--config-path',
		join(import.meta.dir, '..'),
		path,
	]);

	if (!formatted.success) {
		console.error(`rustfmt failed for ${path}:\n${formatted.stderr.toString()}`);
		process.exit(1);
	}

	return readFile(path, 'utf-8');
}

let stale = 0;

if (check) {
	// Emit into a scratch directory and compare, leaving the working tree untouched.
	const scratch = await mkdtemp(join(tmpdir(), 'ardent-codegen-'));

	for (const [name, contents] of Object.entries(files)) {
		const expected = await emit(join(scratch, name), contents);
		const current = await readFile(join(srcDir, name), 'utf-8').catch(() => '');

		if (current !== expected) {
			console.error(`✗ src/${name} is out of date — run \`mise run spec:codegen\``);
			stale++;
		}
	}

	await rm(scratch, { recursive: true, force: true });
} else {
	for (const [name, contents] of Object.entries(files)) {
		await emit(join(srcDir, name), contents);
		console.log(`✓ src/${name}`);
	}
}

if (stale > 0) {
	process.exit(1);
}

if (check) {
	console.log(`✓ tables are in step with the Dent Style Specification ${specVersion}`);
}
