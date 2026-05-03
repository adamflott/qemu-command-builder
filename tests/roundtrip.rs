use pretty_assertions::assert_eq;
use qemu_command_builder::QemuInstanceForX86_64;
use qemu_command_builder::display::QemuDisplay;
use qemu_command_builder::netdev::NetDev;
use qemu_command_builder::serial::SpecialDevice;
use qemu_command_builder::smbios::{Smbios, SmbiosType0, SmbiosType1, SmbiosType2};
use qemu_command_builder::to_command::ToCommand;
use std::str::FromStr;

//proptest! {
/*
    #[test]
    fn roundtrip_accel(v: Accel) {
        let parsed = Accel::from_str(&v.to_single_arg()).unwrap();
        prop_assert!(v == parsed);
    }
    #[test]
    fn roundtrip_acpitable(v: AcpiTable) {
        let parsed = AcpiTable::from_str(&v.to_single_arg()).unwrap();
        prop_assert!(v == parsed);
    }
    #[test]
    fn roundtrip_action(v: Action) {
        let parsed = Action::from_str(&v.to_single_arg()).unwrap();
        prop_assert!(v == parsed);
    }
    #[test]
    fn roundtrip_addfd(v: AddFd) {
        let parsed = AddFd::from_str(&v.to_single_arg()).unwrap();
        prop_assert!(v == parsed);
    }
    #[test]
    fn roundtrip_boot(v: Boot) {
        let parsed = Boot::from_str(&v.to_single_arg()).unwrap();
        prop_assert!(v == parsed);
    }
    #[test]
    fn roundtrip_device(v: Device) {
        let parsed = Device::from_str(&v.to_single_arg()).unwrap();
        prop_assert!(v == parsed);
    }
    #[test]
    fn roundtrip_smp(v: SMP) {
        let parsed = SMP::from_str(&v.to_single_arg()).unwrap();
        prop_assert!(v == parsed);
    }
    #[test]
    fn roundtrip_m(v: Memory) {
        let parsed = Memory::from_str(&v.to_single_arg()).unwrap();
        prop_assert!(v == parsed);
    }
    #[test]
    fn roundtrip_name(v: Name) {
        let parsed = Name::from_str(&v.to_single_arg()).unwrap();
        prop_assert!(v == parsed);
    }
    #[test]
    fn roundtrip_msg(v: Msg) {
        let parsed = Msg::from_str(&v.to_single_arg()).unwrap();
        prop_assert!(v == parsed);
    }
    #[test]
    fn roundtrip_mon(v: Mon) {
        let parsed = Mon::from_str(&v.to_single_arg()).unwrap();
        prop_assert!(v == parsed);
    }
    #[test]
    fn roundtrip_drive(v: Drive) {
        let parsed = Drive::from_str(&v.to_single_arg()).unwrap();
        prop_assert!(v == parsed);
    }
    #[test]
    fn roundtrip_run_with(v: RunWith) {
        let parsed = RunWith::from_str(&v.to_single_arg()).unwrap();
        prop_assert!(v == parsed);
    }
    #[test]
    fn roundtrip_rtc(v: Rtc) {
        let parsed = Rtc::from_str(&v.to_single_arg()).unwrap();
        prop_assert!(v == parsed);
    }
}
*/
/*
#[test]
fn roundtrip_full() {
    let s = r#"qemu-system-x86_64 -cpu host -smp 4 -add-fd fd=1,set=2 -boot order=nc -m 1024M -device blah -name test,process=test -drive file=/dev/vg1/drive0-27573,if=none,media=disk,cache=writeback,aio=native,format=raw,id=drive-scsi-disk-0,readonly=off -nographic -vga std -acpitable sig=whatever -kernel /kernel.img -append "console=tty1 ro" -mon chardev=charmonitor,mode=control -pidfile /run/qemu.pid -d unimp -D /run/qemu-debug.log -enable-kvm -no-reboot -rtc clock=vm -nodefaults -no-user-config -run-with chroot=/chroot,user=vmuser -msg timestamp=on"#;
    match QemuInstanceForX86_64::from_str(s) {
        Ok(parsed) => {
            eprintln!("{:#?}", parsed);
            assert_eq!(s, parsed.to_single_command());
        }
        Err(err) => {
            panic!("{}", err);
        }
    }
}
*/

#[test]
fn roundtrip_full_wip() {
    let s = "/usr/bin/qemu-system-x86_64 -machine q35,accel=kvm -cpu host,migratable=yes,svm,vmx -smp 1 -m 1024M -device virtio-scsi-pci,id=scsi0 -device scsi-hd,bootindex=1,bus=scsi0.0,channel=0,drive=drive-scsi-disk-0,id=drive0,lun=0,rotation_rate=1,scsi-id=0 -device virtio-scsi-pci,id=scsi1 -device scsi-hd,bus=scsi1.0,channel=0,drive=drive-scsi-disk-1,id=drive1,lun=2,rotation_rate=1,scsi-id=1 -device virtio-net-pci,mac=f2:3c:93:6e:bb:d4,mq=on,netdev=net0,vectors=18 -name crate_test -drive file=/dev/vg1/drive0-27573,if=none,media=disk,cache=writeback,aio=native,format=raw,id=drive-scsi-disk-0,readonly=off -display vnc=unix:/run/vnc.socket -nographic -vga std -smbios 'type=0,vendor=some vendor,version=unknown version' -smbios type=1,manufacturer=,product=widget,version=alpha,serial=1234,family=none -smbios type=2,serial= -netdev tap,id=net0,ifname=tap0,script=no,downscript=no,vhost=on,queues=8 -chardev socket,id=charmonitor,path=/qemumon.sock,server=on,wait=off,logfile=/run/qemumon.log,logappend=off -chardev stdio,id=serial0,mux=off,signal=off -kernel /kernel.img -append 'console=tty1 ro' -serial chardev:serial0 -parallel none -mon chardev=charmonitor,mode=control -pidfile /run/qemu.pid -d unimp -D /run/qemu-debug.log -enable-kvm -no-reboot -rtc clock=vm -nodefaults -no-user-config -run-with chroot=/chroot,user=vmuser -msg timestamp=on";

    match QemuInstanceForX86_64::from_str(s) {
        Ok(parsed) => {
            assert_eq!(s, parsed.to_single_command());
        }
        Err(err) => {
            panic!("{}", err);
        }
    }
}

#[test]
fn roundtrip_display_vnc() {
    let display = QemuDisplay::from_str("vnc=unix:/run/vnc.socket").unwrap();
    assert_eq!("vnc=unix:/run/vnc.socket", display.to_single_arg());
}

#[test]
fn roundtrip_smbios_variants() {
    let type0 = "type=0,vendor=some vendor,version=unknown version";
    let type1 = "type=1,manufacturer=,product=widget,version=alpha,serial=1234,family=none";
    let type2 = "type=2,serial=";

    assert_eq!(type0, Smbios::Type0(SmbiosType0::from_str(type0).unwrap()).to_args()[0]);
    assert_eq!(type1, Smbios::Type1(SmbiosType1::from_str(type1).unwrap()).to_args()[0]);
    assert_eq!(type2, Smbios::Type2(SmbiosType2::from_str(type2).unwrap()).to_args()[0]);
}

#[test]
fn roundtrip_tap_and_special_devices() {
    let tap = NetDev::from_str("tap,id=net0,ifname=tap0,script=no,downscript=no,vhost=on,queues=8").unwrap();
    assert_eq!("tap,id=net0,ifname=tap0,script=no,downscript=no,vhost=on,queues=8", tap.to_args()[0]);

    let serial = SpecialDevice::from_str("chardev:serial0").unwrap();
    assert_eq!("chardev:serial0", serial.to_args()[0]);

    let parallel = SpecialDevice::from_str("none").unwrap();
    assert_eq!("none", parallel.to_args()[0]);
}
