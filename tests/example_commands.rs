use std::str::FromStr;

use pretty_assertions::assert_eq;
use qemu_command_builder::{QemuInstanceForX86_64, to_command::ToCommand};

fn assert_example_round_trips(source: &str, canonical_fixture: &str) {
    let parsed = QemuInstanceForX86_64::from_str(source.trim()).expect("example command must parse");
    let generated = parsed.to_single_command();

    assert_eq!(canonical_fixture.trim(), generated, "generated command differs from the canonical fixture");

    let reparsed = QemuInstanceForX86_64::from_str(&generated).expect("generated command must parse");

    assert_eq!(parsed, reparsed, "generated command dropped or changed an argument");
    assert_eq!(generated, reparsed.to_single_command(), "generated command must be deterministic");
}

#[test]
fn libvirt_example_round_trips() {
    assert_example_round_trips(include_str!("fixtures/libvirt-example.sh"), include_str!("fixtures/libvirt-example.canonical.sh"));
}

#[test]
fn qemu_example_round_trips() {
    assert_example_round_trips(include_str!("fixtures/qemu-example.cmd"), include_str!("fixtures/qemu-example.canonical.cmd"));
}

#[test]
fn confidential_vm_example_round_trips() {
    assert_example_round_trips(
        include_str!("fixtures/qemu-confidential-vm-example.cmd"),
        include_str!("fixtures/qemu-confidential-vm-example.canonical.cmd"),
    );
}
