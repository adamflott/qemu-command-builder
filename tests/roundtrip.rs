use pretty_assertions::assert_eq;
use proptest::prelude::*;
use qemu_command_builder::QemuInstanceForX86_64;
use qemu_command_builder::accel::Accel;
use qemu_command_builder::acpitable::AcpiTable;
use qemu_command_builder::action::Action;
use qemu_command_builder::addfs::AddFd;
use qemu_command_builder::boot::Boot;
use qemu_command_builder::device::Device;
use qemu_command_builder::drive::Drive;
use qemu_command_builder::memory::Memory;
use qemu_command_builder::mon::Mon;
use qemu_command_builder::msg::Msg;
use qemu_command_builder::name::Name;
use qemu_command_builder::rtc::Rtc;
use qemu_command_builder::runwith::RunWith;
use qemu_command_builder::smp::SMP;
use qemu_command_builder::to_command::ToCommand;
use std::str::FromStr;

proptest! {
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
*/

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

#[test]
fn roundtrip_full() {
    let s = r#"qemu-system-x86_64 -cpu host -smp 4 -add-fd fd=1,set=2 -boot order=nc -m 1024M -device blah -name test,process=test -drive file=/dev/vg1/drive0-27573,if=none,media=disk,cache=writeback,aio=native,format=raw,id=drive-scsi-disk-0,read-only=off -nographic -vga std -acpitable sig=whatever -kernel /kernel.img -append "console=tty1 ro" -mon chardev=charmonitor,mode=control -enable-kvm -no-reboot -rtc clock=vm -nodefaults -no-user-config -run-with chroot=/chroot,user=vmuser -msg timestamp=on"#;
    match QemuInstanceForX86_64::from_str(s) {
        Ok(parsed) => {
            eprintln!("{:#?}", parsed);
            assert_eq!(s, parsed.to_single_command());
        }
        Err(err) => {
            panic!("{}", err);
        }
    }

    assert_eq!(1, 1);
}

/*
proptest! {
    #[test]
    fn roundtrip_cpux86_64(v: CpuX86) {
        let parsed = CpuX86::from_str(&v.to_single_arg()).unwrap();
        prop_assert!(v == parsed);
    }
}
*/
