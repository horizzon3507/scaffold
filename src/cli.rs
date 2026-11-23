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
}

/// Arguments for `scaffold new`.
pub struct NewArgs {
    /// Project name; also the default directory name.
    pub name: String,
    /// Preset to use.
    pub preset: String,
    /// Directory to generate into. Defaults to ./<name>.
    pub dir: Option<PathBuf>,
    /// Overwrite files that already exist.
    pub force: bool,
    /// Print the plan without writing anything.
    pub dry_run: bool,
    /// Version control to initialize.
    pub vcs: Vcs,
    /// Author name recorded in generated metadata.
    pub author: Option<String>,
}

/// Arguments for `scaffold list`.
pub struct ListArgs {
    /// Show each preset's description too.
    pub verbose: bool,
}

/// Arguments for `scaffold describe`.
pub struct DescribeArgs {
    /// Preset to describe.
    pub preset: String,
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
pub fn parse(argv: &[String]) -> Result<Parsed, String> {
    let Some(first) = argv.first() else {
        return Ok(Parsed::Help(HELP));
    };
    match first.as_str() {
        "new" => parse_new(&argv[1..]),
        "list" => parse_list(&argv[1..]),
        "describe" => parse_describe(&argv[1..]),
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
        preset: f.preset.unwrap_or_else(|| "hory".to_string()),
        dir: f.dir,
        force: f.force,
        dry_run: f.dry_run,
        vcs: f.vcs.unwrap_or(Vcs::Git),
        author: f.author,
    })))
}

fn parse_list(args: &[String]) -> Result<Parsed, String> {
    let mut verbose = false;
    for arg in args {
        match arg.as_str() {
            "-v" | "--verbose" => verbose = true,
            "-h" | "--help" => return Ok(Parsed::Help(HELP)),
            other => return Err(format!("unexpected argument '{other}'")),
        }
    }
    Ok(Parsed::Run(Command::List(ListArgs { verbose })))
}

fn parse_describe(args: &[String]) -> Result<Parsed, String> {
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
        assert_eq!(args.preset, "hory");
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
        assert_eq!(args.preset, "rust");
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
        assert_eq!(args.preset, "rust");
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
