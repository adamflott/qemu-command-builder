use pretty_assertions::assert_eq;
use qemu_command_builder::numa::NUMA;
use qemu_command_builder::to_command::ToCommand;
use qemu_command_builder::QemuInstanceForX86_64;
use std::str::FromStr;

#[test]
fn numa_node_mem_round_trips() {
    let rendered = "node,mem=4096,cpu=0-3,nodeid=0,initiator=1";
    let parsed = NUMA::from_str(rendered).unwrap();

    assert_eq!(rendered, parsed.to_args()[0]);
    assert_eq!(parsed, NUMA::from_str(&parsed.to_args()[0]).unwrap());
}

#[test]
fn numa_node_memdev_round_trips() {
    let rendered = "node,memdev=7,cpu=4-7,nodeid=1";
    let parsed = NUMA::from_str(rendered).unwrap();

    assert_eq!(rendered, parsed.to_args()[0]);
    assert_eq!(parsed, NUMA::from_str(&parsed.to_args()[0]).unwrap());
}

#[test]
fn numa_hmat_cache_round_trips() {
    let rendered = "hmat-cache,node-id=0,size=32768,level=1,associativity=direct,policy=write-back,line=64";
    let parsed = NUMA::from_str(rendered).unwrap();

    assert_eq!(rendered, parsed.to_args()[0]);
    assert_eq!(parsed, NUMA::from_str(&parsed.to_args()[0]).unwrap());
}

#[test]
fn numa_hmat_lb_round_trips() {
    let rendered = "hmat-lb,initiator=0,target=1,hierarchy=memory,data-type=access-latency,latency=10,bandwidth=20";
    let parsed = NUMA::from_str(rendered).unwrap();

    assert_eq!(rendered, parsed.to_args()[0]);
    assert_eq!(parsed, NUMA::from_str(&parsed.to_args()[0]).unwrap());
}

#[test]
fn qemu_instance_parses_numa() {
    let cmd = "/usr/bin/qemu-system-x86_64 -numa node,mem=4096,nodeid=0 -nodefaults";
    let parsed = QemuInstanceForX86_64::from_str(cmd).unwrap();

    assert_eq!(cmd, parsed.to_single_command());
}
