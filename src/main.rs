use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::Instant;

use clap::{CommandFactory, Parser, Subcommand, ValueEnum};
use glob::glob;

use ardent::{DentOptions, EndOfLines, Formatter};

#[derive(Parser)]
#[command(
	name = "ardent",
	version,
	about = "Opinionated formatter for NSIS scripts"
)]
struct Cli {
	#[arg(short = 'D', long, help = "Print debug messages")]
	debug: bool,

	#[command(subcommand)]
	command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
	#[command(about = "Format NSIS scripts")]
	Format {
		#[arg(help = "Files or glob patterns to format")]
		files: Vec<String>,

		#[arg(short, long, help = "Edit files in-place")]
		write: bool,

		#[command(flatten)]
		formatting: FormattingArgs,
	},

	#[command(about = "Check if NSIS scripts are formatted correctly")]
	Check {
		#[arg(help = "Files or glob patterns to check")]
		files: Vec<String>,

		#[arg(short, long, help = "Edit files in-place, if check fails")]
		write: bool,

		#[command(flatten)]
		formatting: FormattingArgs,
	},
}

#[derive(Parser, Debug)]
struct FormattingArgs {
	#[arg(
		short,
		long,
		value_enum,
		help = "Control how line-breaks are represented"
	)]
	eol: Option<EolArg>,

	#[arg(
		short,
		long,
		default_value_t = 2,
		help = "Number of units per indentation level"
	)]
	indent_size: usize,

	#[arg(short = 's', long, help = "Indent with spaces instead of tabs")]
	use_spaces: bool,

	#[arg(short, long, default_value_t = true, help = "Trim empty lines")]
	trim: bool,
}

#[derive(Clone, Debug, ValueEnum)]
enum EolArg {
	Crlf,
	Lf,
}

fn default_eol() -> EndOfLines {
	if cfg!(windows) {
		EndOfLines::Crlf
	} else {
		EndOfLines::Lf
	}
}

fn dent_options_from(args: &FormattingArgs) -> DentOptions {
	DentOptions {
		end_of_lines: Some(args.eol.as_ref().map_or_else(default_eol, |e| match e {
			EolArg::Crlf => EndOfLines::Crlf,
			EolArg::Lf => EndOfLines::Lf,
		})),
		indent_size: args.indent_size,
		trim_empty_lines: args.trim,
		use_tabs: !args.use_spaces,
	}
}

fn resolve_files(patterns: &[String]) -> Vec<PathBuf> {
	let mut files = Vec::new();
	for pattern in patterns {
		match glob(pattern) {
			Ok(paths) => {
				for entry in paths.flatten() {
					files.push(entry);
				}
			}
			Err(e) => {
				eprintln!("Warning: invalid glob pattern '{}': {}", pattern, e);
			}
		}
	}
	files
}

fn is_nsis_file(path: &Path) -> bool {
	path.extension()
		.and_then(|e| e.to_str())
		.is_some_and(|ext| ext == "nsi" || ext == "nsh")
}

fn run_format(
	patterns: &[String],
	write: bool,
	formatting: &FormattingArgs,
	debug: bool,
) -> ExitCode {
	if debug {
		eprintln!("Debug: args={:?}, options={:?}", patterns, formatting);
	}

	if !formatting.use_spaces && formatting.indent_size != 2 {
		eprintln!("Warning: the \"indent-size\" option is ignored when \"use-spaces\" is not set.");
	}

	if patterns.is_empty() {
		Cli::command()
			.find_subcommand_mut("format")
			.unwrap()
			.print_help()
			.unwrap();
		return ExitCode::from(2);
	}

	let files = resolve_files(patterns);
	if files.is_empty() {
		eprintln!("Error: no valid input files provided, exiting.");
		return ExitCode::from(1);
	}

	let formatter = match Formatter::new(dent_options_from(formatting)) {
		Ok(f) => f,
		Err(e) => {
			eprintln!("Error: {e}");
			return ExitCode::from(1);
		}
	};

	if write {
		eprintln!(
			"Formatting {} {}...",
			files.len(),
			if files.len() == 1 { "file" } else { "files" }
		);
	}

	let outer_start = Instant::now();

	for file in &files {
		if !is_nsis_file(file) {
			eprintln!(
				"Warning: {} is not an NSIS script, skipping.",
				file.display()
			);
			continue;
		}

		if !file.exists() {
			eprintln!("Warning: {} does not exist, skipping.", file.display());
			continue;
		}

		let start = Instant::now();
		let raw_contents = match fs::read_to_string(file) {
			Ok(c) => c,
			Err(e) => {
				eprintln!("Error reading {}: {e}", file.display());
				continue;
			}
		};

		let result = match formatter.check(&raw_contents) {
			Ok(r) => r,
			Err(e) => {
				eprintln!("Error parsing {}: {e}", file.display());
				continue;
			}
		};

		let duration = start.elapsed().as_millis();

		if write {
			if let Some(formatted) = result {
				if let Err(e) = fs::write(file, &formatted) {
					eprintln!("Error writing {}: {e}", file.display());
					continue;
				}
				eprintln!("{} formatted ({}ms)", file.display(), duration);
			} else {
				eprintln!("{} already formatted ({}ms)", file.display(), duration);
			}
		} else {
			let output = result.as_deref().unwrap_or(&raw_contents);
			let _ = io::stdout().write_all(output.as_bytes());
		}
	}

	if write {
		let outer_duration = outer_start.elapsed().as_millis();
		eprintln!("Completed in {}ms.", outer_duration);
	}

	ExitCode::SUCCESS
}

fn run_check(
	patterns: &[String],
	write: bool,
	formatting: &FormattingArgs,
	debug: bool,
) -> ExitCode {
	if debug {
		eprintln!("Debug: args={:?}, options={:?}", patterns, formatting);
	}

	if !formatting.use_spaces && formatting.indent_size != 2 {
		eprintln!("Warning: the \"indent-size\" option is ignored when \"use-spaces\" is not set.");
	}

	if patterns.is_empty() {
		Cli::command()
			.find_subcommand_mut("check")
			.unwrap()
			.print_help()
			.unwrap();
		return ExitCode::from(2);
	}

	let files = resolve_files(patterns);
	if files.is_empty() {
		eprintln!("Error: no valid input files provided, exiting.");
		return ExitCode::from(2);
	}

	let formatter = match Formatter::new(dent_options_from(formatting)) {
		Ok(f) => f,
		Err(e) => {
			eprintln!("Error: {e}");
			return ExitCode::from(1);
		}
	};

	eprintln!(
		"Checking {} {}...",
		files.len(),
		if files.len() == 1 { "file" } else { "files" }
	);

	let outer_start = Instant::now();
	let mut drifted: Vec<PathBuf> = Vec::new();

	for file in &files {
		if !is_nsis_file(file) {
			eprintln!(
				"Warning: {} is not an NSIS script, skipping.",
				file.display()
			);
			continue;
		}

		if !file.exists() {
			eprintln!("Warning: {} does not exist, skipping.", file.display());
			continue;
		}

		let start = Instant::now();
		let raw_contents = match fs::read_to_string(file) {
			Ok(c) => c,
			Err(e) => {
				eprintln!("Error reading {}: {e}", file.display());
				continue;
			}
		};

		let result = match formatter.check(&raw_contents) {
			Ok(r) => r,
			Err(e) => {
				eprintln!("Error parsing {}: {e}", file.display());
				continue;
			}
		};

		let duration = start.elapsed().as_millis();

		if let Some(formatted) = result {
			drifted.push(file.clone());
			if write {
				if let Err(e) = fs::write(file, &formatted) {
					eprintln!("Error writing {}: {e}", file.display());
					continue;
				}
				eprintln!("{} formatted ({}ms)", file.display(), duration);
			} else {
				eprintln!("Warning: {} has issues ({}ms)", file.display(), duration);
			}
		} else {
			eprintln!("{} already formatted ({}ms)", file.display(), duration);
		}
	}

	let outer_duration = outer_start.elapsed().as_millis();
	eprintln!("Completed in {}ms.", outer_duration);

	if !drifted.is_empty() {
		ExitCode::from(1)
	} else {
		ExitCode::SUCCESS
	}
}

fn main() -> ExitCode {
	let cli = Cli::parse();

	match cli.command {
		Some(Commands::Format {
			files,
			write,
			formatting,
		}) => run_format(&files, write, &formatting, cli.debug),
		Some(Commands::Check {
			files,
			write,
			formatting,
		}) => run_check(&files, write, &formatting, cli.debug),
		None => {
			Cli::command().print_help().unwrap();
			ExitCode::from(2)
		}
	}
}
