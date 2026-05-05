use pretty_assertions::assert_eq;
use qemu_command_builder::args::mon::{Mon, ReadlineControl};
use qemu_command_builder::common::OnOff;
use qemu_command_builder::shell_string::ShellString;
use qemu_command_builder::to_command::ToCommand;
use std::str::FromStr;

#[test]
fn mon_displays_canonical_qemu_form() {
    let mon = Mon::builder().chardev(ShellString::from("mon0")).mode(ReadlineControl::Control).pretty(OnOff::On).build();

    assert_eq!("chardev=mon0,mode=control,pretty=on", mon.to_args()[0]);
}

#[test]
fn mon_parses_chardev_id_and_mode() {
    let parsed = Mon::from_str("chardev=charmonitor,id=monitor,mode=control").unwrap();

    let expected = Mon::builder()
        .chardev(ShellString::from("charmonitor"))
        .id(ShellString::from("monitor"))
        .mode(ReadlineControl::Control)
        .build();

    assert_eq!(expected, parsed);
    assert_eq!("chardev=charmonitor,id=monitor,mode=control", parsed.to_args()[0]);
    assert_eq!(parsed, Mon::from_str(&parsed.to_args()[0]).unwrap());
}

#[test]
fn mon_accepts_bare_chardev_name() {
    let parsed = Mon::from_str("mon0,mode=readline").unwrap();

    assert_eq!("chardev=mon0,mode=readline", parsed.to_args()[0]);
    assert_eq!(parsed, Mon::from_str(&parsed.to_args()[0]).unwrap());
}

#[test]
fn mon_parses_mixed_order_and_bare_pretty_flag() {
    let parsed = Mon::from_str("chardev=mon0,pretty,mode=control").unwrap();

    assert_eq!("chardev=mon0,mode=control,pretty=on", parsed.to_args()[0]);
    assert_eq!(parsed, Mon::from_str(&parsed.to_args()[0]).unwrap());
}
