//! `scaffold doctor`: sanity-check the environment.

use crate::color;

/// One environment check.
pub struct Check {
    /// What was checked.
    pub name: &'static str,
    /// Whether it passed.
    pub ok: bool,
    /// Extra detail, shown dimmed.
    pub detail: String,
}

/// Run every check.
pub fn checks() -> Vec<Check> {
    vec![git_on_path(), config_readable(), counts()]
}

fn git_on_path() -> Check {
    match std::process::Command::new("git").arg("--version").output() {
        Ok(out) if out.status.success() => {
            let version = String::from_utf8_lossy(&out.stdout).trim().to_string();
            Check {
                name: "git on PATH",
                ok: true,
                detail: version,
            }
        }
        _ => Check {
            name: "git on PATH",
            ok: false,
            detail: "new projects use git init by default".to_string(),
        },
    }
}

fn config_readable() -> Check {
    match crate::config::path() {
        Some(path) if path.exists() => match std::fs::read_to_string(&path) {
            Ok(_) => Check {
                name: "config file",
                ok: true,
                detail: path.display().to_string(),
            },
            Err(err) => Check {
                name: "config file",
                ok: false,
                detail: format!("{}: {err}", path.display()),
            },
        },
        _ => Check {
            name: "config file",
            ok: true,
            detail: "none (optional)".to_string(),
        },
    }
}

fn counts() -> Check {
    Check {
        name: "presets available",
        ok: !crate::preset::all().is_empty(),
        detail: format!("{} presets", crate::preset::all().len()),
    }
}

/// Print the checks; Err when any of them failed.
pub fn run() -> crate::error::AppResult<()> {
    let mut failed = 0;
    println!("scaffold {}", env!("CARGO_PKG_VERSION"));
    for check in checks() {
        let mark = if check.ok {
            color::green("ok")
        } else {
            failed += 1;
            color::red("FAIL")
        };
        crate::out!("  {mark}  {:<16} {}", check.name, color::dim(&check.detail));
    }
    if failed > 0 {
        return Err(format!("{failed} check(s) failed").into());
    }
    Ok(())
}
