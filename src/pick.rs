//! The interactive preset picker: a numbered list and one line of input.

use std::io::{BufRead, Write};

use crate::color;
use crate::preset::Preset;

/// Interpret one answer: empty picks nothing, a number picks by position,
/// and a preset name picks by name.
pub fn parse_choice(answer: &str, presets: &[Preset]) -> Option<Option<String>> {
    let answer = answer.trim();
    if answer.is_empty() {
        return Some(None);
    }
    if let Ok(n) = answer.parse::<usize>() {
        if (1..=presets.len()).contains(&n) {
            return Some(Some(presets[n - 1].name.to_string()));
        }
        return None;
    }
    if presets.iter().any(|p| p.name == answer) {
        return Some(Some(answer.to_string()));
    }
    None
}

/// Show the presets and ask for one. Empty input means the default (None).
pub fn choose(
    presets: &[Preset],
    default: &str,
    input: &mut impl BufRead,
    out: &mut impl Write,
) -> std::io::Result<String> {
    writeln!(out, "pick a preset:")?;
    for (i, p) in presets.iter().enumerate() {
        let marker = if p.name == default { " (default)" } else { "" };
        writeln!(out, "  {:>2}) {}{}", i + 1, color::cyan(p.name), marker)?;
    }
    loop {
        write!(out, "[{default}] > ")?;
        out.flush()?;
        let mut line = String::new();
        input.read_line(&mut line)?;
        if line.is_empty() {
            // EOF: fall back to the default.
            return Ok(default.to_string());
        }
        match parse_choice(&line, presets) {
            Some(Some(name)) => return Ok(name),
            Some(None) => return Ok(default.to_string()),
            None => writeln!(out, "unknown choice; try a number or a preset name")?,
        }
    }
}
