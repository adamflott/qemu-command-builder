use pretty_assertions::assert_eq;
use qemu_command_builder::blockdev::BlockDev;
use qemu_command_builder::common::{OnOff, OnOffUnmap};
use qemu_command_builder::to_command::ToCommand;
use std::collections::BTreeMap;
use std::str::FromStr;

#[test]
fn blockdev_displays_generic_and_driver_specific_options() {
    let blockdev = BlockDev::builder()
        .driver("file".to_string())
        .node_name("disk".to_string())
        .read_only(OnOff::Off)
        .detect_zeroes(OnOffUnmap::Unmap)
        .driver_opts(BTreeMap::from([("aio".to_string(), "native".to_string()), ("filename".to_string(), "/tmp/disk.img".to_string())]))
        .build();

    assert_eq!("driver=file,node-name=disk,read-only=off,detect-zeroes=unmap,aio=native,filename=/tmp/disk.img", blockdev.to_args()[0]);
}

#[test]
fn blockdev_parses_bare_driver_and_round_trips() {
    let parsed = BlockDev::from_str("raw,node-name=disk,file=disk_file,cache.direct=on").unwrap();

    assert_eq!("driver=raw,node-name=disk,cache.direct=on,file=disk_file", parsed.to_args()[0]);
    assert_eq!(parsed, BlockDev::from_str(&parsed.to_args()[0]).unwrap());
}

#[test]
fn blockdev_parses_mixed_generic_options_and_driver_opts() {
    let parsed = BlockDev::from_str("driver=qcow2,file=my_file,cache-size=16777216,discard=unmap,cache.no-flush=off,force-share=on,detect-zeroes=on").unwrap();

    let reparsed = BlockDev::from_str(&parsed.to_args()[0]).unwrap();

    assert_eq!(
        "driver=qcow2,discard=unmap,cache.no-flush=off,force-share=on,detect-zeroes=on,cache-size=16777216,file=my_file",
        parsed.to_args()[0]
    );
    assert_eq!(parsed, reparsed);
}
