use pretty_assertions::assert_eq;
use qemu_command_builder::QemuInstanceForX86_64;
use qemu_command_builder::args::compact::{AcceptHide, AcceptRejectCrash, Compact, DeprecatedInput, UnstableInput};
use qemu_command_builder::to_command::ToCommand;
use std::str::FromStr;

#[test]
fn compat_deprecated_round_trips() {
    let compat = Compact::DeprecatedInput(DeprecatedInput::builder().deprecated_input(AcceptRejectCrash::Reject).deprecated_output(AcceptHide::Hide).build());

    let rendered = compat.to_args()[0].clone();
    assert_eq!("deprecated-input=reject,deprecated-output=hide", rendered);
    assert_eq!(compat, Compact::from_str(&rendered).unwrap());
}

#[test]
fn compat_unstable_round_trips() {
    let compat = Compact::UnstableInput(UnstableInput::builder().unstable_input(AcceptRejectCrash::Crash).unstable_output(AcceptHide::Accept).build());

    let rendered = compat.to_args()[0].clone();
    assert_eq!("unstable-input=crash,unstable-output=accept", rendered);
    assert_eq!(compat, Compact::from_str(&rendered).unwrap());
}

#[test]
fn qemu_instance_parses_compat() {
    let cmd = "/usr/bin/qemu-system-x86_64 -compat deprecated-input=reject,deprecated-output=hide -nodefaults";
    let parsed = QemuInstanceForX86_64::from_str(cmd).unwrap();

    assert_eq!(cmd, parsed.to_single_command());
}
