use pretty_assertions::assert_eq;
use qemu_command_builder::addfs::AddFd;
use qemu_command_builder::shell_string::ShellString;
use qemu_command_builder::to_command::ToCommand;
use std::str::FromStr;

#[test]
fn addfd_displays_canonical_order() {
    let addfd = AddFd::builder().fd(3).set(2).opaque(ShellString::from("rdwr:/path/to/file")).build();

    assert_eq!("fd=3,set=2,opaque=rdwr:/path/to/file", addfd.to_args()[0]);
}

#[test]
fn addfd_parses_mixed_order_into_canonical_output() {
    let parsed = AddFd::from_str("opaque='fd label',set=2,fd=1").unwrap();

    assert_eq!("fd=1,set=2,opaque=fd label", parsed.to_args()[0]);
    assert_eq!(parsed, AddFd::from_str(&parsed.to_args()[0]).unwrap());
}

#[test]
fn addfd_round_trips_shell_quoted_opaque_value() {
    let s = "fd=4,set=7,opaque='rdonly:/path with spaces'";

    let parsed = AddFd::from_str(s).unwrap();

    assert_eq!("fd=4,set=7,opaque=rdonly:/path with spaces", parsed.to_args()[0]);
    assert_eq!("'fd=4,set=7,opaque=rdonly:/path with spaces'", parsed.to_single_arg());
}
