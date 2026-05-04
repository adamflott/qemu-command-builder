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
    let parsed = Drive::from_str("detect-zeroes=unmap,werror=enospc,file=/tmp/disk.img,copy-on-read=off,readonly=on,discard=unmap,if=virtio").unwrap();

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
fn drive_parses_blockdev_style_cache_and_read_only_options() {
    let parsed = Drive::from_str("file=/dev/mapper/vg1-1,if=none,media=disk,id=drive-scsi-disk-0,aio=native,cache.direct=on,format=raw,read-only=off,auto-read-only=off").unwrap();

    let expected = Drive::builder()
        .file(ShellPath::from("/dev/mapper/vg1-1"))
        .interface(DriveInterface::None)
        .media(DriveMedia::Disk)
        .cache_direct(OnOff::On)
        .aio(DriveAIOType::Native)
        .format(ShellString::from("raw"))
        .id(ShellString::from("drive-scsi-disk-0"))
        .read_only(OnOff::Off)
        .auto_read_only(OnOff::Off)
        .build();

    assert_eq!(expected, parsed);
    assert_eq!(
        "file=/devr/mapper/vg1-1,if=none,media=disk,cache.direct=on,aio=native,format=raw,id=drive-scsi-disk-0,readonly=off,auto-read-only=off",
        parsed.to_args()[0]
    );
    assert_eq!(parsed, Drive::from_str(&parsed.to_args()[0]).unwrap());
}

#[test]
fn drive_parses_rbd_encryption_and_throttling_options() {
    let parsed = Drive::from_str(
        r#"file=rbd:block-volumes/144349:id=bs-cluster1-labkrk2:auth_supported=cephx;none:mon_host=[2600\:3c1f\:2\:3\:31\:\:704];[2600\:3c1f\:2\:3\:31\:\:705];[2600\:3c1f\:2\:3\:31\:\:706];[2600\:3c1f\:2\:3\:31\:\:707];[2600\:3c1f\:2\:3\:31\:\:708]:rbd_cache=true:rbd_cache_size=32000000,encrypt.format=luks2,encrypt.key-secret=secret-144349,if=none,media=disk,id=test6,aio=native,cache=writeback,format=rbd,throttling.bps-total=367001600,throttling.iops-total-max=12000,throttling.iops-total=8000,throttling.iops-total-max-length=60,throttling.bps-total-max=550502400,throttling.bps-total-max-length=60,read-only=off,auto-read-only=off"#,
    )
    .unwrap();

    assert_eq!(
        r#"file=rbd:block-volumes/144349:id=bs-cluster1-labkrk2:auth_supported=cephx;none:mon_host=[2600\:3c1f\:2\:3\:31\:\:704];[2600\:3c1f\:2\:3\:31\:\:705];[2600\:3c1f\:2\:3\:31\:\:706];[2600\:3c1f\:2\:3\:31\:\:707];[2600\:3c1f\:2\:3\:31\:\:708]:rbd_cache=true:rbd_cache_size=32000000,if=none,media=disk,cache=writeback,aio=native,format=rbd,encrypt.format=luks2,encrypt.key-secret=secret-144349,id=test6,readonly=off,auto-read-only=off,throttling.bps-total=367001600,throttling.bps-total-max=550502400,throttling.bps-total-max-length=60,throttling.iops-total=8000,throttling.iops-total-max=12000,throttling.iops-total-max-length=60"#,
        parsed.to_args()[0]
    );
    assert_eq!(parsed, Drive::from_str(&parsed.to_args()[0]).unwrap());
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
