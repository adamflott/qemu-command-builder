use pretty_assertions::assert_eq;
use qemu_command_builder::args::addfs::AddFd;
use qemu_command_builder::args::boot::Boot;
use qemu_command_builder::shell_string::ShellString;
use qemu_command_builder::to_command::ToCommand;
use std::str::FromStr;

#[test]
fn display_emits_shell_quoted_token() {
    let s = ShellString::from("console=tty1 ro");
    assert_eq!(s.to_string(), "'console=tty1 ro'");
}

#[test]
fn from_str_parses_shell_quoted_token() {
    let parsed = ShellString::from_str(r#"'console=tty1 ro'"#).unwrap();
    assert_eq!(parsed.as_ref(), "console=tty1 ro");
}

#[test]
fn boot_round_trips_quoted_shell_string_fields() {
    let parsed = Boot::from_str("order='cad b',once=d").unwrap();
    assert_eq!(parsed.to_args(), vec!["order=cad b,once=d".to_string()]);
    assert_eq!(parsed.to_single_command(), "-boot 'order=cad b,once=d'");
}

#[test]
fn add_fd_parses_ps_style_quoted_opaque_value() {
    let parsed = AddFd::from_str("fd=1,set=2,opaque='fd label'").unwrap();
    assert_eq!(parsed.to_args(), vec!["fd=1,set=2,opaque=fd label".to_string()]);
    assert_eq!(parsed.to_single_command(), "-add-fd 'fd=1,set=2,opaque=fd label'");
}
