use scaffold_cli::cli;
use scaffold_cli::color;

#[test]
fn styles_wrap_when_enabled() {
    color::set_enabled(true);
    assert_eq!(color::bold("hi"), "\x1b[1mhi\x1b[0m");
    assert_eq!(color::green("ok"), "\x1b[32mok\x1b[0m");
    assert_eq!(color::red("bad"), "\x1b[31mbad\x1b[0m");
    assert_eq!(color::yellow("warn"), "\x1b[33mwarn\x1b[0m");
    assert_eq!(color::cyan("name"), "\x1b[36mname\x1b[0m");
    assert_eq!(color::dim("note"), "\x1b[2mnote\x1b[0m");
}

#[test]
fn styles_pass_through_when_disabled() {
    color::set_enabled(false);
    assert_eq!(color::bold("hi"), "hi");
    assert_eq!(color::green("ok"), "ok");
}

#[test]
fn flag_is_detected_and_stripped() {
    let v = |xs: &[&str]| xs.iter().map(|s| s.to_string()).collect::<Vec<_>>();
    assert!(cli::no_color_requested(&v(&["list", "--no-color"])));
    assert!(!cli::no_color_requested(&v(&["list"])));
    let parsed = cli::parse(&v(&["--no-color", "list"]));
    assert!(parsed.is_ok());
    let parsed = cli::parse(&v(&["list", "--no-color"]));
    assert!(parsed.is_ok());
}

#[test]
fn the_decision_covers_flag_env_and_tty() {
    assert!(color::decide(false, false, true));
    assert!(!color::decide(true, false, true));
    assert!(!color::decide(false, true, true));
    assert!(!color::decide(false, false, false));
}
