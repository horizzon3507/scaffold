use std::io::Cursor;

use scaffold_cli::pick::{choose, parse_choice};
use scaffold_cli::preset::all;

#[test]
fn parses_numbers_names_and_empty() {
    let presets = all();
    assert_eq!(parse_choice("", &presets), Some(None));
    assert_eq!(parse_choice("1", &presets), Some(Some("hory".to_string())));
    assert_eq!(
        parse_choice("rust", &presets),
        Some(Some("rust".to_string()))
    );
    assert_eq!(parse_choice("999", &presets), None);
    assert_eq!(parse_choice("nope", &presets), None);
}

#[test]
fn choose_reads_a_selection() {
    let presets = all();
    let mut input = Cursor::new("2\n".as_bytes());
    let mut out = Vec::new();
    let picked = choose(&presets, "hory", &mut input, &mut out).unwrap();
    assert_eq!(picked, "rust");
    let shown = String::from_utf8(out).unwrap();
    assert!(shown.contains("pick a preset:"));
    assert!(shown.contains("(default)"));
}

#[test]
fn empty_input_and_eof_take_the_default() {
    let presets = all();
    let mut input = Cursor::new("\n".as_bytes());
    let mut out = Vec::new();
    assert_eq!(
        choose(&presets, "hory", &mut input, &mut out).unwrap(),
        "hory"
    );

    let mut input = Cursor::new("".as_bytes());
    let mut out = Vec::new();
    assert_eq!(
        choose(&presets, "hory", &mut input, &mut out).unwrap(),
        "hory"
    );
}

#[test]
fn bad_input_reprompts_then_accepts() {
    let presets = all();
    let mut input = Cursor::new("nope\nrust\n".as_bytes());
    let mut out = Vec::new();
    assert_eq!(
        choose(&presets, "hory", &mut input, &mut out).unwrap(),
        "rust"
    );
    let shown = String::from_utf8(out).unwrap();
    assert!(shown.contains("unknown choice"));
}
