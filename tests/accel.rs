use pretty_assertions::assert_eq;
use qemu_command_builder::accel::{Accel, NotifyVMExit, OnOffSplit, TCGThreadType};
use qemu_command_builder::common::{AccelType, OnOff, OnOffDefaultOff};
use qemu_command_builder::shell_path::ShellPath;
use qemu_command_builder::to_command::ToCommand;
use std::str::FromStr;

#[test]
fn accel_displays_qemu_hyphenated_keys() {
    let accel = Accel::builder()
        .accel_type(AccelType::Kvm)
        .kernel_irqchip(OnOffSplit::Split)
        .kvm_shadow_mem(1048576)
        .dirty_ring_size(4096)
        .device(ShellPath::from("/dev/fdset/7"))
        .build();

    assert_eq!(
        "kvm,kernel-irqchip=split,kvm-shadow-mem=1048576,dirty-ring-size=4096,device=/dev/fdset/7",
        accel.to_single_arg()
    );
}

#[test]
fn accel_round_trips_mixed_property_order() {
    let s = "tcg,thread=multi,split-wx=off,notify-vmexit=run,notify-window=7,one-insn-per-tb=on,tb-size=32";

    let parsed = Accel::from_str(s).unwrap();

    assert_eq!(
        "tcg,one-insn-per-tb=on,split-wx=off,tb-size=32,notify-vmexit=run,notify-window=7,thread=multi",
        parsed.to_single_arg()
    );
    assert_eq!(parsed, Accel::from_str(&parsed.to_single_arg()).unwrap());
}

#[test]
fn accel_parses_optional_accel_prefix() {
    let parsed =
        Accel::from_str("accel=xen,igd-passthru=off,kernel-irqchip=on").unwrap();

    let expected = Accel::builder()
        .accel_type(AccelType::Xen)
        .igd_passthru(OnOffDefaultOff::Off)
        .kernel_irqchip(OnOffSplit::On)
        .build();

    assert_eq!(expected, parsed);
    assert_eq!("xen,igd-passthru=off,kernel-irqchip=on", parsed.to_single_arg());
}

#[test]
fn accel_round_trips_all_supported_properties() {
    let accel = Accel::builder()
        .accel_type(AccelType::Tcg)
        .igd_passthru(OnOffDefaultOff::On)
        .kernel_irqchip(OnOffSplit::Off)
        .kvm_shadow_mem(4096)
        .one_insn_per_tb(OnOff::On)
        .split_wx(OnOff::Off)
        .tb_size(64)
        .dirty_ring_size(2048)
        .eager_split_size(512)
        .notify_vmexit(NotifyVMExit::Run(Some(9)))
        .thread(TCGThreadType::Single)
        .device(ShellPath::from("/dev/kvm"))
        .build();

    let rendered = accel.to_single_arg();
    let reparsed = Accel::from_str(&rendered).unwrap();

    assert_eq!(
        "tcg,igd-passthru=on,kernel-irqchip=off,kvm-shadow-mem=4096,one-insn-per-tb=on,split-wx=off,tb-size=64,dirty-ring-size=2048,eager-split-size=512,notify-vmexit=run,notify-window=9,thread=single,device=/dev/kvm",
        rendered
    );
    assert_eq!(accel, reparsed);
}
