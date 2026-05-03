use pretty_assertions::assert_eq;
use qemu_command_builder::common::{AccelType, OnOffAuto, OnOffDefaultOff, OnOffDefaultOn};
use qemu_command_builder::machine::{Machine, MachineX86_64};
use qemu_command_builder::machine_type::MachineTypeX86_64;
use qemu_command_builder::shell_string::ShellString;
use qemu_command_builder::to_command::ToCommand;
use std::str::FromStr;

#[test]
fn machine_displays_canonical_qemu_order() {
    let machine = MachineX86_64::builder()
        .m(
            Machine::builder()
                .machine_type(MachineTypeX86_64::Q35)
                .accel(vec![AccelType::Kvm, AccelType::Tcg])
                .vmport(OnOffAuto::Auto)
                .dump_guest_core(OnOffDefaultOn::Off)
                .nvdimm(OnOffDefaultOff::On)
                .memory_encryption(ShellString::from("sev0"))
                .memory_backend(ShellString::from("pc.ram"))
                .build(),
        )
        .build();

    assert_eq!(
        "q35,accel=kvm:tcg,vmport=auto,dump-guest-core=off,nvdimm=on,memory-encryption=sev0,memory-backend=pc.ram",
        machine.to_args()[0]
    );
}

#[test]
fn machine_parses_mixed_order_into_canonical_output() {
    let parsed = MachineX86_64::from_str(
        "q35,memory-backend=pc.ram,nvdimm=on,accel=kvm:tcg,memory-encryption=sev0,vmport=auto,dump-guest-core=off",
    )
    .unwrap();

    assert_eq!(
        "q35,accel=kvm:tcg,vmport=auto,dump-guest-core=off,nvdimm=on,memory-encryption=sev0,memory-backend=pc.ram",
        parsed.to_args()[0]
    );
    assert_eq!(parsed, MachineX86_64::from_str(&parsed.to_args()[0]).unwrap());
}

#[test]
fn machine_accepts_type_prefix_alias() {
    let parsed = MachineX86_64::from_str("type=q35,accel=kvm,hmat=on,aux-ram-share=off").unwrap();

    assert_eq!("q35,accel=kvm,hmat=on,aux-ram-share=off", parsed.to_args()[0]);
    assert_eq!(parsed, MachineX86_64::from_str(&parsed.to_args()[0]).unwrap());
}
