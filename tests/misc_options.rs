use pretty_assertions::assert_eq;
use qemu_command_builder::QemuInstanceForX86_64;
use qemu_command_builder::args::g::G;
use qemu_command_builder::args::msg::Msg;
use qemu_command_builder::args::name::Name;
use qemu_command_builder::args::object::Object;
use qemu_command_builder::args::overcommit::{OnOffOnfault, Overcommit};
use qemu_command_builder::args::plugin::Plugin;
use qemu_command_builder::args::set::Set;
use qemu_command_builder::args::trace::Trace;
use qemu_command_builder::common::OnOff;
use qemu_command_builder::to_command::ToCommand;
use std::path::PathBuf;
use std::str::FromStr;

#[test]
fn msg_parses_bare_flags_and_canonicalizes() {
    let parsed = Msg::from_str("guest-name,timestamp=off").unwrap();
    assert_eq!("timestamp=off,guest-name=on", parsed.to_args()[0]);
    assert_eq!(parsed, Msg::from_str(&parsed.to_args()[0]).unwrap());
}

#[test]
fn name_parses_mixed_order() {
    let parsed = Name::from_str("guest,debug-threads=off,process=qemu-vm").unwrap();
    assert_eq!("guest,process=qemu-vm,debug-threads=off", parsed.to_args()[0]);
    assert_eq!(parsed, Name::from_str(&parsed.to_args()[0]).unwrap());
}

#[test]
fn object_uses_object_command_and_round_trips() {
    let object = Object::builder()
        .typename("memory-backend-ram".to_string())
        .properties(vec![("id".to_string(), "ram0".to_string()), ("size".to_string(), "1G".to_string())])
        .build();

    assert_eq!("-object", object.command());
    assert_eq!("memory-backend-ram,id=ram0,size=1G", object.to_args()[0]);
    assert_eq!(object, Object::from_str(&object.to_args()[0]).unwrap());
}

#[test]
fn qemu_instance_parses_confidential_guest_support_with_sev_object() {
    let cmd = "/usr/bin/qemu-system-x86_64 -machine q35,confidential-guest-support=sev0 -object sev-guest,id=sev0,cbitpos=47,reduced-phys-bits=1";
    let parsed = QemuInstanceForX86_64::from_str(cmd).unwrap();

    assert_eq!(cmd, parsed.to_single_command());
}

#[test]
fn overcommit_emits_qemu_keys_and_round_trips() {
    let mem_lock = Overcommit::MemLock(OnOffOnfault::Onfault);
    let cpu_pm = Overcommit::CpuPm(OnOff::On);

    assert_eq!("mem-lock=on-fault", mem_lock.to_args()[0]);
    assert_eq!("cpu-pm=on", cpu_pm.to_args()[0]);
    assert_eq!(mem_lock, Overcommit::from_str("mem-lock=on-fault").unwrap());
    assert_eq!(cpu_pm, Overcommit::from_str("cpu-pm=on").unwrap());
}

#[test]
fn plugin_does_not_duplicate_command_and_round_trips() {
    let plugin = Plugin::builder().file(PathBuf::from("/tmp/libtrace.so")).args(vec![("arg1".to_string(), "value1".to_string())]).build();

    assert_eq!("file=/tmp/libtrace.so,arg1=value1", plugin.to_args()[0]);
    assert_eq!(plugin, Plugin::from_str("file=/tmp/libtrace.so,arg1=value1").unwrap());
    assert_eq!(plugin, Plugin::from_str("/tmp/libtrace.so,arg1=value1").unwrap());
}

#[test]
fn set_serializes_as_single_argument() {
    let set = Set::builder().group("drive.disk0.file".to_string()).value("/disk.img".to_string()).build();

    assert_eq!("drive.disk0.file=/disk.img", set.to_args()[0]);
    assert_eq!(set, Set::from_str("drive.disk0.file=/disk.img").unwrap());
}

#[test]
fn trace_supports_bare_enable_pattern_and_mixed_order() {
    let parsed = Trace::from_str("events=/tmp/events,file=/tmp/trace.log,net_*").unwrap();
    assert_eq!("enable=net_*,events=/tmp/events,file=/tmp/trace.log", parsed.to_args()[0]);
    assert_eq!(parsed, Trace::from_str(&parsed.to_args()[0]).unwrap());
}

#[test]
fn g_parses_resolution_with_optional_depth() {
    let without_depth = G::from_str("800x600").unwrap();
    assert_eq!("800x600", without_depth.to_args()[0]);
    assert_eq!(without_depth, G::from_str(&without_depth.to_args()[0]).unwrap());

    let with_depth = G::from_str("1024x768x24").unwrap();
    assert_eq!("1024x768x24", with_depth.to_args()[0]);
    assert_eq!(with_depth, G::from_str(&with_depth.to_args()[0]).unwrap());
}

#[test]
fn g_rejects_invalid_resolution() {
    assert!(G::from_str("").is_err());
    assert!(G::from_str("800").is_err());
    assert!(G::from_str("800x").is_err());
    assert!(G::from_str("800x600x24x1").is_err());
    assert!(G::from_str("0x600").is_err());
}

#[test]
fn qemu_instance_parses_and_emits_g() {
    let parsed = QemuInstanceForX86_64::from_str("/usr/bin/qemu-system-x86_64 -g 1024x768x24").unwrap();

    assert_eq!(Some(G::from_str("1024x768x24").unwrap()), parsed.g);
    assert_eq!("/usr/bin/qemu-system-x86_64 -g 1024x768x24", parsed.to_single_command());
}
