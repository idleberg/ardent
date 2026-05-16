use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::Instant;

use clap::{CommandFactory, Parser, Subcommand, ValueEnum};
use glob::glob;

use ardent::{EndOfLines, Formatter, FormatterOptions};

mod logger;
use logger::*;

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

	#[arg(short = 'T', long = "no-trim", help = "Do not trim empty lines")]
	no_trim: bool,
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

fn dent_options_from(args: &FormattingArgs) -> FormatterOptions {
	FormatterOptions {
		end_of_lines: Some(args.eol.as_ref().map_or_else(default_eol, |e| match e {
			EolArg::Crlf => EndOfLines::Crlf,
			EolArg::Lf => EndOfLines::Lf,
		})),
		indent_size: args.indent_size,
		trim_empty_lines: !args.no_trim,
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
				logger_warn!("invalid glob pattern '{}': {}", pattern, e);
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
		logger_debug!("args={:?}, options={:?}", patterns, formatting);
	}

	if !formatting.use_spaces && formatting.indent_size != 2 {
		logger_warn!("the \"indent-size\" option is ignored when \"use-spaces\" is not set.");
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
		logger_error!("no valid input files provided, exiting.");
		return ExitCode::from(1);
	}

	let formatter = match Formatter::new(dent_options_from(formatting)) {
		Ok(f) => f,
		Err(e) => {
			logger_error!("{e}");
			return ExitCode::from(1);
		}
	};

	if write {
		logger_start!(
			"Formatting {} {}...",
			files.len(),
			if files.len() == 1 { "file" } else { "files" }
		);
	}

	let outer_start = Instant::now();

	for file in &files {
		if !is_nsis_file(file) {
			logger_warn!("{} is not an NSIS script, skipping.", blue(&file.display()));
			continue;
		}

		if !file.exists() {
			logger_warn!("{} does not exist, skipping.", blue(&file.display()));
			continue;
		}

		let start = Instant::now();
		let raw_contents = match fs::read_to_string(file) {
			Ok(c) => c,
			Err(e) => {
				logger_error!("reading {}: {e}", blue(&file.display()));
				continue;
			}
		};

		let result = match formatter.check(&raw_contents) {
			Ok(r) => r,
			Err(e) => {
				logger_error!("parsing {}: {e}", blue(&file.display()));
				continue;
			}
		};

		let duration = start.elapsed().as_millis();

		if write {
			if let Some(formatted) = result {
				if let Err(e) = fs::write(file, &formatted) {
					logger_error!("writing {}: {e}", blue(&file.display()));
					continue;
				}
				logger_info!(
					"{} formatted {}",
					blue(&file.display()),
					dim(&format_args!("({}ms)", duration))
				);
			} else {
				logger_info!(
					"{} already formatted {}",
					blue(&file.display()),
					dim(&format_args!("({}ms)", duration))
				);
			}
		} else {
			let output = result.as_deref().unwrap_or(&raw_contents);
			let _ = io::stdout().write_all(output.as_bytes());
		}
	}

	if write {
		let outer_duration = outer_start.elapsed().as_millis();
		logger_success!("Completed in {}ms.", outer_duration);
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
		logger_debug!("args={:?}, options={:?}", patterns, formatting);
	}

	if !formatting.use_spaces && formatting.indent_size != 2 {
		logger_warn!("the \"indent-size\" option is ignored when \"use-spaces\" is not set.");
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
		logger_error!("no valid input files provided, exiting.");
		return ExitCode::from(2);
	}

	let formatter = match Formatter::new(dent_options_from(formatting)) {
		Ok(f) => f,
		Err(e) => {
			logger_error!("{e}");
			return ExitCode::from(1);
		}
	};

	logger_start!(
		"Checking {} {}...",
		files.len(),
		if files.len() == 1 { "file" } else { "files" }
	);

	let outer_start = Instant::now();
	let mut drifted: Vec<PathBuf> = Vec::new();

	for file in &files {
		if !is_nsis_file(file) {
			logger_warn!("{} is not an NSIS script, skipping.", blue(&file.display()));
			continue;
		}

		if !file.exists() {
			logger_warn!("{} does not exist, skipping.", blue(&file.display()));
			continue;
		}

		let start = Instant::now();
		let raw_contents = match fs::read_to_string(file) {
			Ok(c) => c,
			Err(e) => {
				logger_error!("reading {}: {e}", blue(&file.display()));
				continue;
			}
		};

		let result = match formatter.check(&raw_contents) {
			Ok(r) => r,
			Err(e) => {
				logger_error!("parsing {}: {e}", blue(&file.display()));
				continue;
			}
		};

		let duration = start.elapsed().as_millis();

		if let Some(formatted) = result {
			drifted.push(file.clone());
			if write {
				if let Err(e) = fs::write(file, &formatted) {
					logger_error!("writing {}: {e}", blue(&file.display()));
					continue;
				}
				logger_info!(
					"{} formatted {}",
					blue(&file.display()),
					dim(&format_args!("({}ms)", duration))
				);
			} else {
				logger_warn!(
					"{} has issues {}",
					blue(&file.display()),
					dim(&format_args!("({}ms)", duration))
				);
			}
		} else {
			logger_info!(
				"{} already formatted {}",
				blue(&file.display()),
				dim(&format_args!("({}ms)", duration))
			);
		}
	}

	let outer_duration = outer_start.elapsed().as_millis();
	logger_success!("Completed in {}ms.", outer_duration);

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
