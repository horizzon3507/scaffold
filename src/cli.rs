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
    verbose: bool,
}

fn take_value(
    flag: &str,
    inline: Option<String>,
    rest: &mut std::slice::Iter<String>,
) -> Result<String, String> {
    match inline {
        Some(v) => Ok(v),
        None => rest
            .next()
            .cloned()
            .ok_or_else(|| format!("flag {flag} needs a value")),
    }
}

fn parse_vcs(value: &str) -> Result<Vcs, String> {
    match value {
        "git" => Ok(Vcs::Git),
        "none" => Ok(Vcs::None),
        other => Err(format!("invalid value '{other}' for --vcs (expected git or none)")),
    }
}

/// Parse arguments (without argv[0]) into a command.
pub fn parse(argv: &[String]) -> Result<Command, String> {
    let mut args = argv.iter();
    let Some(first) = args.next() else {
        return Err("missing command (try --help)".to_string());
    };
    match first.as_str() {
        "new" => parse_new(args),
        "list" => parse_list(args),
        "describe" => parse_describe(args),
        other => Err(format!("unknown command '{other}' (try --help)")),
    }
}

fn parse_new(mut args: std::slice::Iter<String>) -> Result<Command, String> {
    let mut name = None;
    let mut f = Flags::default();
    for arg in args.by_ref() {
        if let Some(long) = arg.strip_prefix("--") {
            match long {
                "preset" => f.preset = Some(take_value("--preset", None, &mut args)?),
                "dir" => f.dir = Some(PathBuf::from(take_value("--dir", None, &mut args)?)),
                "force" => f.force = true,
                "dry-run" => f.dry_run = true,
                "vcs" => f.vcs = Some(parse_vcs(&take_value("--vcs", None, &mut args)?)?),
                "author" => f.author = Some(take_value("--author", None, &mut args)?),
                other => return Err(format!("unknown flag --{other}")),
            }
        } else if let Some(short) = arg.strip_prefix('-') {
            match short {
                "p" => f.preset = Some(take_value("-p", None, &mut args)?),
                "d" => f.dir = Some(PathBuf::from(take_value("-d", None, &mut args)?)),
                "f" => f.force = true,
                other => return Err(format!("unknown flag -{other}")),
            }
        } else if name.is_some() {
            return Err(format!("unexpected argument '{arg}'"));
        } else {
            name = Some(arg.clone());
        }
    }
    let Some(name) = name else {
        return Err("missing project name (try scaffold new --help)".to_string());
    };
    Ok(Command::New(NewArgs {
        name,
        preset: f.preset.unwrap_or_else(|| "hory".to_string()),
        dir: f.dir,
        force: f.force,
        dry_run: f.dry_run,
        vcs: f.vcs.unwrap_or(Vcs::Git),
        author: f.author,
    }))
}

fn parse_list(mut args: std::slice::Iter<String>) -> Result<Command, String> {
    let mut verbose = false;
    for arg in args.by_ref() {
        match arg.as_str() {
            "-v" | "--verbose" => verbose = true,
            other => return Err(format!("unexpected argument '{other}'")),
        }
    }
    Ok(Command::List(ListArgs { verbose }))
}

fn parse_describe(mut args: std::slice::Iter<String>) -> Result<Command, String> {
    let Some(preset) = args.next() else {
        return Err("missing preset name (try scaffold describe --help)".to_string());
    };
    if let Some(extra) = args.next() {
        return Err(format!("unexpected argument '{extra}'"));
    }
    Ok(Command::Describe(DescribeArgs {
        preset: preset.clone(),
    }))
}
