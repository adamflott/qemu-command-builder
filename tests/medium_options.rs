use pretty_assertions::assert_eq;
use qemu_command_builder::args::fsdev::SecurityModel;
use qemu_command_builder::args::incoming::Incoming;
use qemu_command_builder::args::iscsi::Iscsi;
use qemu_command_builder::args::rtc::Rtc;
use qemu_command_builder::args::runwith::RunWith;
use qemu_command_builder::args::sandbox::{AllowDeny, AllowDenyChildren, Sandbox};
use qemu_command_builder::args::serial::{SpecialDevice, Tcp as SerialTcp, VC as SerialVc};
use qemu_command_builder::args::tpmdev::{Emulator, Passthrough, TpmDev};
use qemu_command_builder::args::usb::USBDevice;
use qemu_command_builder::args::virtfs::{Local as VirtfsLocal, Synth as VirtfsSynth, Virtfs};
use qemu_command_builder::common::OnOff;
use qemu_command_builder::to_command::ToCommand;
use std::path::PathBuf;
use std::str::FromStr;

#[test]
fn rtc_parses_mixed_order_and_date_only_base() {
    let parsed = Rtc::from_str("clock=vm,base=2026-05-03,driftfix=slew").unwrap();
    assert_eq!("base=2026-05-03T00:00:00,clock=vm,driftfix=slew", parsed.to_args()[0]);
    assert_eq!(parsed, Rtc::from_str(&parsed.to_args()[0]).unwrap());
}

#[test]
fn serial_special_devices_round_trip_network_forms() {
    let tcp = SpecialDevice::Tcp(SerialTcp::builder().host("127.0.0.1".to_string()).port(4444).server(OnOff::On).wait(OnOff::Off).build());
    let vc = SpecialDevice::VC(Some(SerialVc::builder().is_pixel(false).w(80).h(24).build()));

    assert_eq!("tcp:127.0.0.1:4444,server=on,wait=off", tcp.to_args()[0]);
    assert_eq!(tcp, SpecialDevice::from_str(&tcp.to_args()[0]).unwrap());
    assert_eq!("vc:80Cx24C", vc.to_args()[0]);
    assert_eq!(vc, SpecialDevice::from_str(&vc.to_args()[0]).unwrap());
}

#[test]
fn tpmdev_round_trips_both_backends() {
    let passthrough = TpmDev::Passthrough(
        Passthrough::builder()
            .id("tpm0".to_string())
            .path(PathBuf::from("/dev/tpm0"))
            .cancel_path(PathBuf::from("/sys/class/misc/tpm0/device/cancel"))
            .build(),
    );
    let emulator = TpmDev::Emulator(Emulator::builder().id("tpm1".to_string()).chardev("chrtpm".to_string()).build());

    assert_eq!("passthrough,id=tpm0,path=/dev/tpm0,cancel-path=/sys/class/misc/tpm0/device/cancel", passthrough.to_args()[0]);
    assert_eq!(passthrough, TpmDev::from_str(&passthrough.to_args()[0]).unwrap());
    assert_eq!("emulator,id=tpm1,chardev=chrtpm", emulator.to_args()[0]);
    assert_eq!(emulator, TpmDev::from_str(&emulator.to_args()[0]).unwrap());
}

#[test]
fn usbdevice_round_trips() {
    let device = USBDevice::from_str("wacom-tablet").unwrap();
    assert_eq!("wacom-tablet", device.to_args()[0]);
}

#[test]
fn virtfs_round_trips_local_and_synth() {
    let local = Virtfs::Local(
        VirtfsLocal::builder()
            .path(PathBuf::from("/exports/share"))
            .mount_tag("hostshare".to_string())
            .security_mode(SecurityModel::MappedXAttr)
            .id("fs0".to_string())
            .writeout(())
            .readonly(true)
            .max_xattr(512)
            .build(),
    );
    let synth = Virtfs::Synth(VirtfsSynth::builder().mount_tag("tag0".to_string()).id("syn0".to_string()).readonly(true).max_xattr(0).build());

    assert_eq!(
        "local,path=/exports/share,mount_tag=hostshare,security_model=mapped-xattr,id=fs0,writeout=immediate,readonly=on,max_xattr=512",
        local.to_args()[0]
    );
    assert_eq!(local, Virtfs::from_str(&local.to_args()[0]).unwrap());
    assert_eq!("synth,mount_tag=tag0,id=syn0,readonly=on,max_xattr=0", synth.to_args()[0]);
    assert_eq!(synth, Virtfs::from_str(&synth.to_args()[0]).unwrap());
}

#[test]
fn incoming_round_trips_tcp_and_file() {
    let tcp = Incoming::from_str("tcp::4444,ipv4=on,to=5555").unwrap();
    let file = Incoming::from_str("file:/tmp/mig.bin,offset=4096").unwrap();

    assert_eq!("tcp::4444,to=5555,ipv4=on", tcp.to_args()[0]);
    assert_eq!(tcp, Incoming::from_str(&tcp.to_args()[0]).unwrap());
    assert_eq!("file:/tmp/mig.bin,offset=4096", file.to_args()[0]);
    assert_eq!(file, Incoming::from_str(&file.to_args()[0]).unwrap());
}

#[test]
fn iscsi_round_trips() {
    let iscsi = Iscsi::from_str("user=alice,password-secret=sec0,header-digest=CRC32C,initiator-name=iqn.2026-05.test:id,id=sess0,timeout=30").unwrap();
    assert_eq!(
        "user=alice,password-secret=sec0,header-digest=CRC32C,initiator-name=iqn.2026-05.test:id,id=sess0,timeout=30",
        iscsi.to_args()[0]
    );
    assert_eq!(iscsi, Iscsi::from_str(&iscsi.to_args()[0]).unwrap());
}

#[test]
fn sandbox_round_trips() {
    let sandbox = Sandbox::builder()
        .mode(OnOff::On)
        .obsolete(AllowDeny::Deny)
        .elevateprivileges(AllowDenyChildren::Children)
        .spawn(AllowDeny::Allow)
        .resourcecontrol(AllowDeny::Deny)
        .build();

    assert_eq!("on,obsolete=deny,elevateprivileges=children,spawn=allow,resourcecontrol=deny", sandbox.to_args()[0]);
    assert_eq!(sandbox, Sandbox::from_str(&sandbox.to_args()[0]).unwrap());
}

#[test]
fn run_with_round_trips_exit_with_parent() {
    let run_with = RunWith::from_str("async-teardown=on,chroot=/vmroot,exit-with-parent=on,user=1000:1000").unwrap();

    assert_eq!("async-teardown=on,chroot=/vmroot,exit-with-parent=on,user=1000:1000", run_with.to_args()[0]);
    assert_eq!(run_with, RunWith::from_str(&run_with.to_args()[0]).unwrap());
}
