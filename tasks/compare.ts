import { $, Glob } from "bun";
import { statSync } from "fs";
import { tmpdir } from "os";
import { join } from "path";
import { unlinkSync, writeFileSync } from "fs";

const args = process.argv.slice(2);

if (args.length === 0) {
	console.log("Usage: mise run compare -- <file.nsi|dir|glob ...>");
	process.exit(1);
}

const files: string[] = [];

for (const arg of args) {
	try {
		if (statSync(arg).isDirectory()) {
			const g = new Glob("**/*.{nsi,nsh}");

			for (const match of g.scanSync({ cwd: arg })) {
				files.push(join(arg, match));
			}

			continue;
		}
	} catch {}

	if (arg.includes("*") || arg.includes("?")) {
		const g = new Glob(arg);

		for (const match of g.scanSync(".")) {
			if (match.endsWith(".nsi") || match.endsWith(".nsh")) {
				files.push(match);
			}
		}
	} else {
		files.push(arg);
	}
}

if (files.length === 0) {
	console.error("No .nsi/.nsh files matched");
	process.exit(1);
}

let pass = 0;
let fail = 0;

for (const f of files) {
	const bunOut = join(tmpdir(), `compare-bun-${Bun.hash(f)}`);
	const rustOut = join(tmpdir(), `compare-rust-${Bun.hash(f)}`);

	try {
		const bunResult =
			await $`node /Users/jan/Repositories/_nsis/node-dent/packages/dent-cli/src/main.ts format ${f}`.quiet().nothrow();
			// await $`bunx --bun -p @nsis/dent-cli dent format ${f}`.quiet().nothrow();
		const rustResult =
			await $`./target/release/ardent format ${f}`.quiet().nothrow();

		if (bunResult.exitCode !== 0 || rustResult.exitCode !== 0) {
			const errors: string[] = [];
			if (bunResult.exitCode !== 0) errors.push(`dent (exit ${bunResult.exitCode}): ${bunResult.stderr.toString().trim()}`);
			if (rustResult.exitCode !== 0) errors.push(`ardent (exit ${rustResult.exitCode}): ${rustResult.stderr.toString().trim()}`);
			console.log(`ERROR: ${f}`);
			errors.forEach((e) => console.log(`  ${e}`));
			console.log("---");
			fail++;
			continue;
		}

		writeFileSync(bunOut, bunResult.stdout);
		writeFileSync(rustOut, rustResult.stdout);

		const diff = await $`diff -u ${bunOut} ${rustOut}`.quiet().nothrow();

		if (diff.exitCode === 0) {
			pass++;
		} else {
			console.log(`DIFF: ${f}`);
			console.log(diff.stdout.toString().split("\n").slice(0, 20).join("\n"));
			console.log("---");
			fail++;
		}
	} finally {
		try { unlinkSync(bunOut); } catch {}
		try { unlinkSync(rustOut); } catch {}
	}
}

console.log();
console.log(`Results: ${pass} passed, ${fail} failed`);

if (fail > 0) process.exit(1);
