//! Hand-rolled command-line parsing: no dependencies, small binary.

use std::path::PathBuf;

/// What scaffold should do.
pub enum Command {
    /// Create a new project from a preset.
    New(NewArgs),
    /// List the available presets.
    List(ListArgs),
    /// Show the files a preset generates.
    Describe(DescribeArgs),
    /// Print a shell completion script.
    Completions(CompletionsArgs),
}

/// Arguments for `scaffold new`.
pub struct NewArgs {
    /// Project name; also the default directory name.
    pub name: String,
    /// Preset to use. `None` defers to the config file, then `hory`.
    pub preset: Option<String>,
    /// Directory to generate into. Defaults to ./<name>.
    pub dir: Option<PathBuf>,
    /// Overwrite files that already exist.
    pub force: bool,
    /// Print the plan without writing anything.
    pub dry_run: bool,
    /// Version control to initialize. `None` defers to the config file, then git.
    pub vcs: Option<Vcs>,
    /// Author name recorded in generated metadata.
    pub author: Option<String>,
    /// Never prompt; non-interactive use.
    pub no_input: bool,
}

/// Arguments for `scaffold list`.
pub struct ListArgs {
    /// Show each preset's description too.
    pub verbose: bool,
    /// Print machine-readable JSON.
    pub json: bool,
}

/// Arguments for `scaffold completions`.
pub struct CompletionsArgs {
    /// Target shell.
    pub shell: String,
}

/// Arguments for `scaffold describe`.
pub struct DescribeArgs {
    /// Preset to describe.
    pub preset: String,
    /// Print machine-readable JSON.
    pub json: bool,
    /// Print the files as a tree instead of a table.
    pub tree: bool,
}

/// Version control backend for a new project.
#[derive(Clone, Copy)]
pub enum Vcs {
    /// Run `git init` in the new project.
    Git,
    /// Do not initialize version control.
    None,
}

#[derive(Default)]
struct Flags {
    preset: Option<String>,
    dir: Option<PathBuf>,
    force: bool,
    dry_run: bool,
    vcs: Option<Vcs>,
    author: Option<String>,
    no_input: bool,
}

fn no_value(flag: &str, inline: Option<String>) -> Result<(), String> {
    match inline {
        Some(_) => Err(format!("flag {flag} takes no value")),
        None => Ok(()),
    }
}

fn take_value(
    flag: &str,
    inline: Option<String>,
    args: &[String],
    i: &mut usize,
) -> Result<String, String> {
    match inline {
        Some(v) => Ok(v),
        None => {
            *i += 1;
            args.get(*i)
                .cloned()
                .ok_or_else(|| format!("flag {flag} needs a value"))
        }
    }
}

fn split_flag(arg: &str) -> (&str, Option<String>) {
    match arg.split_once('=') {
        Some((k, v)) => (k, Some(v.to_string())),
        None => (arg, None),
    }
}

fn parse_vcs(value: &str) -> Result<Vcs, String> {
    match value {
        "git" => Ok(Vcs::Git),
        "none" => Ok(Vcs::None),
        other => Err(format!(
            "invalid value '{other}' for --vcs (expected git or none)"
        )),
    }
}

const HELP: &str = "\
scaffold - scaffolds new projects from opinionated presets

USAGE:
    scaffold <COMMAND>

COMMANDS:
    new       Create a new project from a preset
    list      List the available presets
    describe  Show the files a preset generates

OPTIONS:
    -h, --help       Print help
    -V, --version    Print version
        --no-color   Disable colored output (also honors NO_COLOR)
        --quiet      Print only errors
";

const NEW_HELP: &str = "\
scaffold-new - create a new project from a preset

USAGE:
    scaffold new <NAME> [OPTIONS]

ARGS:
    <NAME>    Project name; also the default directory name

OPTIONS:
    -p, --preset <PRESET>    Preset to use [default: hory]
    -d, --dir <DIR>          Directory to generate into [default: ./<name>]
    -f, --force              Overwrite files that already exist
        --dry-run            Print the plan without writing anything
        --vcs <VCS>          Version control to initialize [default: git] [values: git, none]
        --author <AUTHOR>    Author name recorded in generated metadata
        --no-input           Never prompt for a preset
    -h, --help               Print help
";

/// The outcome of parsing: run something, or print and exit.
pub enum Parsed {
    /// A command to run.
    Run(Command),
    /// Help text to print.
    Help(&'static str),
    /// Print the version.
    Version,
}

/// Parse arguments (without argv[0]) into an outcome.
/// True when `--no-color` appears anywhere in the arguments.
pub fn no_color_requested(argv: &[String]) -> bool {
    argv.iter().any(|a| a == "--no-color")
}

/// True when `--quiet` appears anywhere in the arguments.
pub fn quiet_requested(argv: &[String]) -> bool {
    argv.iter().any(|a| a == "--quiet")
}

pub fn parse(argv: &[String]) -> Result<Parsed, String> {
    let argv: Vec<String> = argv
        .iter()
        .filter(|a| a.as_str() != "--no-color" && a.as_str() != "--quiet")
        .cloned()
        .collect();
    let argv = argv.as_slice();
    let Some(first) = argv.first() else {
        return Ok(Parsed::Help(HELP));
    };
    match first.as_str() {
        "new" => parse_new(&argv[1..]),
        "list" => parse_list(&argv[1..]),
        "describe" => parse_describe(&argv[1..]),
        "completions" => parse_completions(&argv[1..]),
        "-h" | "--help" => Ok(Parsed::Help(HELP)),
        "-V" | "--version" => Ok(Parsed::Version),
        other => Err(format!("unknown command '{other}' (try --help)")),
    }
}

fn parse_new(args: &[String]) -> Result<Parsed, String> {
    let mut name = None;
    let mut f = Flags::default();
    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        if let Some(long) = arg.strip_prefix("--") {
            let (key, inline) = split_flag(long);
            match key {
                "preset" => f.preset = Some(take_value("--preset", inline, args, &mut i)?),
                "dir" => f.dir = Some(PathBuf::from(take_value("--dir", inline, args, &mut i)?)),
                "force" => {
                    no_value("--force", inline)?;
                    f.force = true;
                }
                "dry-run" => {
                    no_value("--dry-run", inline)?;
                    f.dry_run = true;
                }
                "no-input" => {
                    no_value("--no-input", inline)?;
                    f.no_input = true;
                }
                "vcs" => f.vcs = Some(parse_vcs(&take_value("--vcs", inline, args, &mut i)?)?),
                "author" => f.author = Some(take_value("--author", inline, args, &mut i)?),
                "help" => return Ok(Parsed::Help(NEW_HELP)),
                other => return Err(format!("unknown flag --{other}")),
            }
        } else if let Some(short) = arg.strip_prefix('-') {
            let (key, inline) = split_flag(short);
            match key {
                "p" => f.preset = Some(take_value("-p", inline, args, &mut i)?),
                "d" => f.dir = Some(PathBuf::from(take_value("-d", inline, args, &mut i)?)),
                "f" => {
                    no_value("-f", inline)?;
                    f.force = true;
                }
                "h" => return Ok(Parsed::Help(NEW_HELP)),
                other => return Err(format!("unknown flag -{other}")),
            }
        } else if name.is_some() {
            return Err(format!("unexpected argument '{arg}'"));
        } else {
            name = Some(arg.clone());
        }
        i += 1;
    }
    let Some(name) = name else {
        return Err("missing project name (try scaffold new --help)".to_string());
    };
    Ok(Parsed::Run(Command::New(NewArgs {
        name,
        preset: f.preset,
        dir: f.dir,
        force: f.force,
        dry_run: f.dry_run,
        vcs: f.vcs,
        author: f.author,
        no_input: f.no_input,
    })))
}

fn parse_completions(args: &[String]) -> Result<Parsed, String> {
    let mut shell = None;
    for a in args {
        if a.starts_with('-') {
            return Err(format!("unknown flag '{a}' for completions"));
        }
        if shell.replace(a.clone()).is_some() {
            return Err("completions takes exactly one shell name".to_string());
        }
    }
    let shell = shell.ok_or_else(|| "completions needs a shell: bash, zsh, or fish".to_string())?;
    Ok(Parsed::Run(Command::Completions(CompletionsArgs { shell })))
}

fn parse_list(args: &[String]) -> Result<Parsed, String> {
    let mut verbose = false;
    let mut json = false;
    for arg in args {
        match arg.as_str() {
            "-v" | "--verbose" => verbose = true,
            "--json" => json = true,
            "-h" | "--help" => return Ok(Parsed::Help(HELP)),
            other => return Err(format!("unexpected argument '{other}'")),
        }
    }
    Ok(Parsed::Run(Command::List(ListArgs { verbose, json })))
}

fn parse_describe(args: &[String]) -> Result<Parsed, String> {
    let json = args.iter().any(|a| a == "--json");
    let tree = args.iter().any(|a| a == "--tree");
    let args: Vec<&String> = args
        .iter()
        .filter(|a| a.as_str() != "--json" && a.as_str() != "--tree")
        .collect();
    let args: Vec<String> = args.into_iter().cloned().collect();
    let args = args.as_slice();
    let Some(preset) = args.first() else {
        return Err("missing preset name (try scaffold describe --help)".to_string());
    };
    if preset == "-h" || preset == "--help" {
        return Ok(Parsed::Help(HELP));
    }
    if let Some(extra) = args.get(1) {
        return Err(format!("unexpected argument '{extra}'"));
    }
    Ok(Parsed::Run(Command::Describe(DescribeArgs {
        preset: preset.clone(),
        json,
        tree,
    })))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(parts: &[&str]) -> Vec<String> {
        parts.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn parses_new_with_defaults() {
        let Ok(Parsed::Run(Command::New(args))) = parse(&v(&["new", "demo"])) else {
            panic!("expected new")
        };
        assert_eq!(args.name, "demo");
        assert!(args.preset.is_none());
        assert!(!args.force);
        assert!(!args.dry_run);
    }

    #[test]
    fn parses_flags_and_aliases() {
        let Ok(Parsed::Run(Command::New(args))) = parse(&v(&[
            "new",
            "demo",
            "-p",
            "rust",
            "-d",
            "/tmp/x",
            "-f",
            "--dry-run",
            "--vcs",
            "none",
            "--author",
            "hory",
        ])) else {
            panic!("expected new")
        };
        assert_eq!(args.preset.as_deref(), Some("rust"));
        assert_eq!(args.dir.unwrap().to_str().unwrap(), "/tmp/x");
        assert!(args.force && args.dry_run);
        assert_eq!(args.author.as_deref(), Some("hory"));
    }

    #[test]
    fn parses_list_and_describe() {
        assert!(matches!(
            parse(&v(&["list", "-v"])),
            Ok(Parsed::Run(Command::List(_)))
        ));
        assert!(matches!(
            parse(&v(&["describe", "hory"])),
            Ok(Parsed::Run(Command::Describe(_)))
        ));
        assert!(parse(&v(&["nope"])).is_err());
    }

    #[test]
    fn parses_the_equals_form() {
        let Ok(Parsed::Run(Command::New(args))) =
            parse(&v(&["new", "demo", "--preset=rust", "--author=hory"]))
        else {
            panic!("expected new")
        };
        assert_eq!(args.preset.as_deref(), Some("rust"));
        assert_eq!(args.author.as_deref(), Some("hory"));
    }

    #[test]
    fn handles_help_and_version() {
        assert!(matches!(parse(&v(&[])), Ok(Parsed::Help(_))));
        assert!(matches!(parse(&v(&["--help"])), Ok(Parsed::Help(_))));
        assert!(matches!(parse(&v(&["-V"])), Ok(Parsed::Version)));
        assert!(matches!(parse(&v(&["new", "--help"])), Ok(Parsed::Help(_))));
    }

    #[test]
    fn rejects_values_on_boolean_flags() {
        assert!(parse(&v(&["new", "demo", "--force=yes"])).is_err());
        assert!(parse(&v(&["new", "demo", "--dry-run=1"])).is_err());
    }

    #[test]
    fn rejects_missing_values_and_extra_args() {
        assert!(parse(&v(&["new", "demo", "--preset"])).is_err());
        assert!(parse(&v(&["new", "a", "b"])).is_err());
        assert!(parse(&v(&["new"])).is_err());
    }
}
