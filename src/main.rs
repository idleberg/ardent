use std::fs;
use std::io::{self, IsTerminal, Read, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::Instant;

use clap::{CommandFactory, Parser, Subcommand, ValueEnum};
use glob::glob;

use ardent::{EndOfLine, Formatter, FormatterOptions};

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

	#[arg(
		short = 'q',
		long,
		help = "Prefer single quotes instead of double quotes"
	)]
	single_quote: bool,

	#[arg(short = 'T', long = "no-trim", help = "Do not trim empty lines")]
	no_trim: bool,

	#[arg(
		short = 'p',
		long,
		default_value_t = 120,
		help = "Maximum line width before wrapping with line continuations (0 to disable)"
	)]
	print_width: usize,
}

#[derive(Clone, Debug, ValueEnum)]
enum EolArg {
	Crlf,
	Lf,
}

fn default_eol() -> EndOfLine {
	if cfg!(windows) {
		EndOfLine::Crlf
	} else {
		EndOfLine::Lf
	}
}

fn dent_options_from(args: &FormattingArgs) -> FormatterOptions {
	FormatterOptions {
		end_of_line: Some(args.eol.as_ref().map_or_else(default_eol, |e| match e {
			EolArg::Crlf => EndOfLine::Crlf,
			EolArg::Lf => EndOfLine::Lf,
		})),
		indent_size: args.indent_size,
		trim_empty_lines: !args.no_trim,
		use_tabs: !args.use_spaces,
		print_width: args.print_width,
		single_quote: args.single_quote,
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

fn has_stdin() -> bool {
	!io::stdin().is_terminal()
}

fn read_stdin() -> io::Result<String> {
	let mut buf = String::new();
	io::stdin().read_to_string(&mut buf)?;
	Ok(buf)
}

fn init_formatter(
	patterns: &[String],
	formatting: &FormattingArgs,
	debug: bool,
	subcommand: &str,
) -> Result<Formatter, ExitCode> {
	if debug {
		logger_debug!("args={:?}, options={:?}", patterns, formatting);
	}

	if !formatting.use_spaces && formatting.indent_size != 2 {
		logger_warn!("the \"indent-size\" option is ignored when \"use-spaces\" is not set.");
	}

	if patterns.is_empty() && !has_stdin() {
		Cli::command()
			.find_subcommand_mut(subcommand)
			.unwrap()
			.print_help()
			.unwrap();
		return Err(ExitCode::from(2));
	}

	Formatter::new(dent_options_from(formatting)).map_err(|e| {
		logger_error!("{e}");
		ExitCode::from(1)
	})
}

fn format_stdin(formatter: &Formatter) -> Result<(String, Option<String>), ExitCode> {
	let raw_contents = read_stdin().map_err(|e| {
		logger_error!("reading stdin: {e}");
		ExitCode::from(1)
	})?;
	let result = formatter.check(&raw_contents).map_err(|e| {
		logger_error!("parsing stdin: {e}");
		ExitCode::from(1)
	})?;
	Ok((raw_contents, result))
}

fn for_each_file(
	files: &[PathBuf],
	formatter: &Formatter,
	mut on_result: impl FnMut(&Path, &str, Option<String>, u128),
) {
	for file in files {
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
		on_result(file, &raw_contents, result, duration);
	}
}

fn run_format(
	patterns: &[String],
	write: bool,
	formatting: &FormattingArgs,
	debug: bool,
) -> ExitCode {
	let formatter = match init_formatter(patterns, formatting, debug, "format") {
		Ok(f) => f,
		Err(code) => return code,
	};

	if patterns.is_empty() {
		let (raw_contents, result) = match format_stdin(&formatter) {
			Ok(r) => r,
			Err(code) => return code,
		};
		let output = result.as_deref().unwrap_or(&raw_contents);
		let _ = io::stdout().write_all(output.as_bytes());
		return ExitCode::SUCCESS;
	}

	let files = resolve_files(patterns);
	if files.is_empty() {
		logger_error!("no valid input files provided, exiting.");
		return ExitCode::from(1);
	}

	if write {
		logger_start!(
			"Formatting {} {}...",
			files.len(),
			if files.len() == 1 { "file" } else { "files" }
		);
	}

	let outer_start = Instant::now();
	let mut num_formatted: usize = 0;
	let mut num_unchanged: usize = 0;

	for_each_file(
		&files,
		&formatter,
		|file, raw_contents, result, duration| {
			if write {
				if let Some(formatted) = result {
					num_formatted += 1;
					if let Err(e) = fs::write(file, &formatted) {
						logger_error!("writing {}: {e}", blue(&file.display()));
						return;
					}
					logger_info!(
						"{} formatted {}",
						blue(&file.display()),
						dim(&format_args!("({}ms)", duration))
					);
				} else {
					num_unchanged += 1;
					logger_info!(
						"{} already formatted {}",
						blue(&file.display()),
						dim(&format_args!("({}ms)", duration))
					);
				}
			} else {
				let output = result.as_deref().unwrap_or(raw_contents);
				let _ = io::stdout().write_all(output.as_bytes());
			}
		},
	);

	if write {
		let outer_duration = outer_start.elapsed().as_millis();
		let total = num_formatted + num_unchanged;
		let summary = if num_formatted == 0 {
			format!(
				"All {} {} already formatted.",
				total,
				if total == 1 { "file was" } else { "files" }
			)
		} else {
			format!(
				"Formatted {} of {} {}.",
				num_formatted,
				total,
				if total == 1 { "file" } else { "files" }
			)
		};
		logger_success!("Completed in {}ms. {}", outer_duration, summary);
	}

	ExitCode::SUCCESS
}

fn run_check(
	patterns: &[String],
	write: bool,
	formatting: &FormattingArgs,
	debug: bool,
) -> ExitCode {
	let formatter = match init_formatter(patterns, formatting, debug, "check") {
		Ok(f) => f,
		Err(code) => return code,
	};

	if patterns.is_empty() {
		logger_start!("Checking standard input...");
		if write {
			logger_warn!("the \"--write\" option is ignored when reading from stdin.");
		}
		let start = Instant::now();
		let (_raw_contents, result) = match format_stdin(&formatter) {
			Ok(r) => r,
			Err(code) => return code,
		};
		let duration = start.elapsed().as_millis();
		return if result.is_some() {
			logger_warn!(
				"Script has issues {}",
				dim(&format_args!("({}ms)", duration))
			);
			logger_success!("Completed in {}ms.", duration);
			ExitCode::from(1)
		} else {
			logger_info!(
				"Script already formatted {}",
				dim(&format_args!("({}ms)", duration))
			);
			logger_success!("Completed in {}ms.", duration);
			ExitCode::SUCCESS
		};
	}

	let files = resolve_files(patterns);
	if files.is_empty() {
		logger_error!("no valid input files provided, exiting.");
		return ExitCode::from(2);
	}

	logger_start!(
		"Checking {} {}...",
		files.len(),
		if files.len() == 1 { "file" } else { "files" }
	);

	let outer_start = Instant::now();
	let mut num_issues: usize = 0;
	let mut num_unchanged: usize = 0;

	for_each_file(
		&files,
		&formatter,
		|file, _raw_contents, result, duration| {
			if let Some(formatted) = result {
				num_issues += 1;
				if write {
					if let Err(e) = fs::write(file, &formatted) {
						logger_error!("writing {}: {e}", blue(&file.display()));
						return;
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
				num_unchanged += 1;
				logger_info!(
					"{} already formatted {}",
					blue(&file.display()),
					dim(&format_args!("({}ms)", duration))
				);
			}
		},
	);

	let outer_duration = outer_start.elapsed().as_millis();
	let total = num_issues + num_unchanged;
	let summary = if num_issues == 0 {
		format!(
			"All {} {} formatted correctly.",
			total,
			if total == 1 { "file" } else { "files" }
		)
	} else {
		format!(
			"Found formatting issues in {} of {} {}.",
			num_issues,
			total,
			if total == 1 { "file" } else { "files" }
		)
	};
	logger_success!("Completed in {}ms. {}", outer_duration, summary);

	if num_issues > 0 {
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
