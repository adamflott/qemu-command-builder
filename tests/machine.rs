use pretty_assertions::assert_eq;
use qemu_command_builder::common::{AccelType, OnOffAuto, OnOffDefaultOff, OnOffDefaultOn};
use qemu_command_builder::machine::{Machine, MachineAarch64, MachineX86_64};
use qemu_command_builder::machine_type::{MachineTypeAarch64, MachineTypeX86_64};
use qemu_command_builder::shell_string::ShellString;
use qemu_command_builder::to_command::ToCommand;
use std::str::FromStr;

#[test]
fn machine_displays_canonical_qemu_order() {
    let machine = MachineX86_64::builder()
        .m(Machine::builder()
            .machine_type(MachineTypeX86_64::Q35)
            .accel(vec![AccelType::Kvm, AccelType::Tcg])
            .vmport(OnOffAuto::Auto)
            .dump_guest_core(OnOffDefaultOn::Off)
            .nvdimm(OnOffDefaultOff::On)
            .memory_encryption(ShellString::from("sev0"))
            .memory_backend(ShellString::from("pc.ram"))
            .build())
        .build();

    assert_eq!(
        "q35,accel=kvm:tcg,vmport=auto,dump-guest-core=off,nvdimm=on,memory-encryption=sev0,memory-backend=pc.ram",
        machine.to_args()[0]
    );
}

#[test]
fn machine_parses_mixed_order_into_canonical_output() {
    let parsed = MachineX86_64::from_str("q35,memory-backend=pc.ram,nvdimm=on,accel=kvm:tcg,memory-encryption=sev0,vmport=auto,dump-guest-core=off").unwrap();

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

#[test]
fn machine_aarch64_displays_canonical_qemu_order() {
    let machine = MachineAarch64::builder()
        .m(Machine::builder()
            .machine_type(MachineTypeAarch64::Virt)
            .accel(vec![AccelType::Kvm, AccelType::Tcg])
            .dump_guest_core(OnOffDefaultOn::Off)
            .mem_merge(OnOffDefaultOn::On)
            .nvdimm(OnOffDefaultOff::On)
            .memory_backend(ShellString::from("virt.ram"))
            .build())
        .build();

    assert_eq!("virt,accel=kvm:tcg,dump-guest-core=off,mem-merge=on,nvdimm=on,memory-backend=virt.ram", machine.to_args()[0]);
}

#[test]
fn machine_aarch64_parses_supported_options() {
    let parsed = MachineAarch64::from_str("type=virt,memory-backend=virt.ram,nvdimm=on,accel=kvm:tcg,dump-guest-core=off,mem-merge=on").unwrap();

    assert_eq!("virt,accel=kvm:tcg,dump-guest-core=off,mem-merge=on,nvdimm=on,memory-backend=virt.ram", parsed.to_args()[0]);
    assert_eq!(parsed, MachineAarch64::from_str(&parsed.to_args()[0]).unwrap());
}

#[test]
fn machine_aarch64_rejects_x86_only_options() {
    let err = MachineAarch64::from_str("virt,vmport=auto").unwrap_err().to_string();

    assert!(err.contains("unsupported aarch64 machine option: vmport"));
}
