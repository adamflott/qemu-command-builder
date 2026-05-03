use pretty_assertions::assert_eq;
use qemu_command_builder::memory::{Memory, MemoryUnit};
use qemu_command_builder::to_command::ToCommand;
use std::str::FromStr;

#[test]
fn memory_displays_canonical_hotplug_form() {
    let memory = Memory::builder()
        .mem(MemoryUnit::GigaBytes(1))
        .slots(3)
        .maxmem(MemoryUnit::GigaBytes(4))
        .build();

    assert_eq!("1G,slots=3,maxmem=4G", memory.to_args()[0]);
}

#[test]
fn memory_accepts_size_prefix_alias() {
    let parsed = Memory::from_str("size=1G,slots=3,maxmem=4G").unwrap();

    assert_eq!("1G,slots=3,maxmem=4G", parsed.to_args()[0]);
    assert_eq!(parsed, Memory::from_str(&parsed.to_args()[0]).unwrap());
}

#[test]
fn memory_parses_hotplug_options_in_any_order() {
    let parsed = Memory::from_str("1024M,maxmem=4G,slots=8").unwrap();

    assert_eq!("1024M,slots=8,maxmem=4G", parsed.to_args()[0]);
    assert_eq!(parsed, Memory::from_str(&parsed.to_args()[0]).unwrap());
}
