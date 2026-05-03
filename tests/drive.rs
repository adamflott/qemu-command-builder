use pretty_assertions::assert_eq;
use qemu_command_builder::common::{IgnoreUnmap, OnOff, OnOffUnmap};
use qemu_command_builder::drive::{Drive, DriveAIOType, DriveCacheType, DriveErrorAction, DriveInterface, DriveMedia};
use qemu_command_builder::shell_path::ShellPath;
use qemu_command_builder::shell_string::ShellString;
use qemu_command_builder::to_command::ToCommand;
use std::str::FromStr;

#[test]
fn drive_displays_qemu_readonly_and_copy_on_read_keys() {
    let drive = Drive::builder()
        .file(ShellPath::from("/tmp/disk.img"))
        .interface(DriveInterface::None)
        .media(DriveMedia::Disk)
        .cache(DriveCacheType::Writeback)
        .aio(DriveAIOType::Native)
        .format(ShellString::from("raw"))
        .id(ShellString::from("disk0"))
        .read_only(OnOff::Off)
        .copy_on_read(OnOff::On)
        .build();

    assert_eq!(
        "file=/tmp/disk.img,if=none,media=disk,cache=writeback,aio=native,format=raw,id=disk0,readonly=off,copy-on-read=on",
        drive.to_args()[0]
    );
}

#[test]
fn drive_parses_mixed_order_into_canonical_output() {
    let parsed = Drive::from_str(
        "detect-zeroes=unmap,werror=enospc,file=/tmp/disk.img,copy-on-read=off,readonly=on,discard=unmap,if=virtio",
    )
    .unwrap();

    assert_eq!(
        "file=/tmp/disk.img,if=virtio,werror=enospc,readonly=on,copy-on-read=off,discard=unmap,detect-zeroes=unmap",
        parsed.to_args()[0]
    );
    assert_eq!(parsed, Drive::from_str(&parsed.to_args()[0]).unwrap());
}

#[test]
fn drive_accepts_legacy_read_only_and_copy_on_read_input() {
    let parsed = Drive::from_str("file=/tmp/disk.img,read-only=on,copy-on-read=off,rerror=report").unwrap();

    let expected = Drive::builder()
        .file(ShellPath::from("/tmp/disk.img"))
        .read_only(OnOff::On)
        .copy_on_read(OnOff::Off)
        .rerror(DriveErrorAction::Report)
        .build();

    assert_eq!(expected, parsed);
    assert_eq!("file=/tmp/disk.img,rerror=report,readonly=on,copy-on-read=off", parsed.to_args()[0]);
}

#[test]
fn drive_round_trips_throttling_and_group_options() {
    let drive = Drive::builder()
        .file(ShellPath::from("/tmp/disk.img"))
        .bps(1024)
        .bps_rd(2048)
        .iops(100)
        .iops_max(200)
        .iops_size(4096)
        .group(ShellString::from("quota0"))
        .snapshot(OnOff::Off)
        .discard(IgnoreUnmap::Unmap)
        .detect_zeroes(OnOffUnmap::On)
        .build();

    let rendered = drive.to_args()[0].clone();

    assert_eq!(drive, Drive::from_str(&rendered).unwrap());
}
