use qemu_command_builder::chardev::{CharDev, CharSocket, CharSocketUds, CharStdio};
use qemu_command_builder::common::AccelType;
use qemu_command_builder::common::OnOff;
use qemu_command_builder::cpu::CpuX86;
use qemu_command_builder::cpu_flags::CPUFlags;
use qemu_command_builder::cpu_type::CpuTypeX86_64;
use qemu_command_builder::device::Device;
use qemu_command_builder::display::QemuDisplay;
use qemu_command_builder::drive::{
    Drive, DriveAIOType, DriveCacheType, DriveInterface, DriveMedia,
};
use qemu_command_builder::machine::MachineForX86;
use qemu_command_builder::machine_type::MachineX86_64;
use qemu_command_builder::memory::Memory;
use qemu_command_builder::memory::MemoryUnit;
use qemu_command_builder::mon::{Mon, ReadlineControl};
use qemu_command_builder::msg::Msg;
use qemu_command_builder::name::Name;
use qemu_command_builder::netdev::Tap;
use qemu_command_builder::netdev::{NetDev, ScriptOrNot};
use qemu_command_builder::rtc::{Rtc, RtcClock};
use qemu_command_builder::runwith::{RunWith, UserOrIds};
use qemu_command_builder::serial::SpecialDevice;
use qemu_command_builder::smbios::{Smbios, SmbiosType0, SmbiosType1, SmbiosType2};
use qemu_command_builder::smp::SMP;
use qemu_command_builder::to_command::ToCommand;
use qemu_command_builder::vga::VGA;
use qemu_command_builder::*;
use std::path::PathBuf;

#[test]
fn full_command_line() {
    let mut cpu = CpuX86::new(CpuTypeX86_64::Host);
    cpu.migratable(true);
    cpu.flags(vec![CPUFlags::Vmx, CPUFlags::Svm]);

    let m = Memory::builder().mem(MemoryUnit::MegaBytes(1024)).build();

    let name = Name::builder().name("crate_test".to_string()).build();

    let mut virio_scsi_pci_device_id0 = Device::new("virtio-scsi-pci");
    virio_scsi_pci_device_id0.add_prop("id", "scsi0");

    let mut scsi_hd_id0 = Device::new("scsi-hd");
    scsi_hd_id0
        .add_prop("bus", "scsi0.0")
        .add_prop("scsi-id", "0")
        .add_prop("channel", "0")
        .add_prop("lun", "0")
        .add_prop("drive", "drive-scsi-disk-0")
        .add_prop("id", "drive0")
        .add_prop("bootindex", "1")
        .add_prop("rotation_rate", "1");

    let mut virio_scsi_pci_device_id1 = Device::new("virtio-scsi-pci");
    virio_scsi_pci_device_id1.add_prop("id", "scsi1");

    let mut scsi_hd_id1 = Device::new("scsi-hd");
    scsi_hd_id1
        .add_prop("bus", "scsi1.0")
        .add_prop("scsi-id", "1")
        .add_prop("channel", "0")
        .add_prop("lun", "2")
        .add_prop("drive", "drive-scsi-disk-1")
        .add_prop("id", "drive1")
        .add_prop("rotation_rate", "1");

    let mut virio_net_pci = Device::new("virtio-net-pci");
    virio_net_pci
        .add_prop("netdev", "net0")
        .add_prop("mac", "f2:3c:93:6e:bb:d4")
        .add_prop("vectors", "18")
        .add_prop("mq", "on");

    let drive_0 = Drive::builder()
        .file(PathBuf::from("/dev/vg1/drive0-27573"))
        .interface(DriveInterface::None)
        .media(DriveMedia::Disk)
        .id("drive-scsi-disk-0".to_string())
        .aio(DriveAIOType::Native)
        .cache(DriveCacheType::Writeback)
        .format("raw".to_string())
        .read_only(OnOff::Off)
        .auto_read_only(OnOff::Off)
        .build();

    let smbios_type0 = Smbios::Type0(
        SmbiosType0::builder()
            .vendor("some vendor".to_string())
            .version("unknown version".to_string())
            .build(),
    );
    let smbios_type1 = Smbios::Type1(
        SmbiosType1::builder()
            .manufacturer("".to_string())
            .product("widget".to_string())
            .version("alpha".to_string())
            .family("none".to_string())
            .serial("1234".to_string())
            .build(),
    );
    let smbios_type2 = Smbios::Type2(SmbiosType2::builder().serial("".to_string()).build());

    let chardev0 = CharDev::Socket(CharSocket::Uds(
        CharSocketUds::builder()
            .id("charmonitor".to_string())
            .path(PathBuf::from("/qemumon.sock"))
            .server(OnOff::On)
            .wait(OnOff::Off)
            .logfile(PathBuf::from("/run/qemumon.log"))
            .logappend(OnOff::Off)
            .build(),
    ));

    let chardev1 = CharDev::Stdio(
        CharStdio::builder()
            .id("serial0".to_string())
            .mux(OnOff::Off)
            .signal(OnOff::Off)
            .build(),
    );

    let display = QemuDisplay::Vnc {
        vnc: "unix:/run/vnc.socket".to_string(),
        optargs: None,
    };

    let tap = Tap::builder()
        .id("net0".to_string())
        .ifname("tap0".to_string())
        .script(ScriptOrNot::None)
        .downscript(ScriptOrNot::None)
        .vhost(OnOff::On)
        .queues(8)
        .build();
    let netdev = NetDev::Tap(tap);

    let serial = SpecialDevice::Chardev("serial0".to_string());

    let mon = Mon::builder()
        .chardev("charmonitor".to_string())
        .mode(ReadlineControl::Control)
        .build();

    let rtc = Rtc::builder().clock(RtcClock::Vm).build();

    let msg = Msg::builder().timestamp(OnOff::On).build();

    let run_with = RunWith::builder()
        .chroot(PathBuf::from("/chroot"))
        .user(UserOrIds::User("vmuser".to_string()))
        .build();

    let qemu = QemuInstanceForX86_64::builder()
        .qemu_binary(PathBuf::from("/usr/bin/qemu-system-x86_64"))
        .machine(
            MachineForX86::builder()
                .machine_type(MachineX86_64::Q35)
                .accel(vec![AccelType::Kvm])
                .build(),
        )
        .cpu(cpu)
        .smp(SMP::new(1))
        .m(m)
        .name(name)
        .nographic(true)
        .vga(VGA::Std)
        .display(display)
        .drive(vec![drive_0])
        .smbios(vec![smbios_type0, smbios_type1, smbios_type2])
        .kernel(PathBuf::from("/kernel.img"))
        .append(String::from("console=tty1 ro"))
        .device(vec![
            virio_scsi_pci_device_id0,
            scsi_hd_id0,
            virio_scsi_pci_device_id1,
            scsi_hd_id1,
            virio_net_pci,
        ])
        .netdev(vec![netdev])
        .parallel(vec![SpecialDevice::None])
        .serial(serial)
        .pidfile(PathBuf::from("/run/qemu.pid"))
        .mon(vec![mon])
        .d(vec!["unimp".to_string()])
        .enable_kvm(true)
        .big_d(PathBuf::from("/run/qemu-debug.log"))
        .no_reboot(true)
        .rtc(rtc)
        .nodefaults(true)
        .no_user_config(true)
        .msg(msg)
        .run_with(run_with)
        .chardev(vec![chardev0, chardev1])
        .build();

    let qemu_expected_args = vec![
        "/usr/bin/qemu-system-x86_64",
        "-machine",
        "q35,accel=kvm",
        "-cpu",
        "host,migratable=yes,-vmx,-svm",
        "-smp",
        "1",
        "-m",
        "1024M",
        "-device",
        "virtio-scsi-pci,id=scsi0",
        "-device",
        "scsi-hd,bus=scsi0.0,scsi-id=0,channel=0,lun=0,drive=drive-scsi-disk-0,id=drive0,bootindex=1,rotation_rate=1",
        "-device",
        "virtio-scsi-pci,id=scsi1",
        "-device",
        "scsi-hd,bus=scsi1.0,scsi-id=1,channel=0,lun=2,drive=drive-scsi-disk-1,id=drive1,rotation_rate=1",
        "-device",
        "virtio-net-pci,netdev=net0,mac=f2:3c:93:6e:bb:d4,vectors=18,mq=on",
        "-name",
        "crate_test",
        "-drive",
        "file=/dev/vg1/drive0-27573,if=none,media=disk,cache=writeback,id=drive-scsi-disk-0,aio=native,format=raw,read-only=off,auto-read-only=off",
        "-display",
        "vnc=unix:/run/vnc.socket",
        "-nographic",
        "-vga",
        "std",
        "-smbios",
        "type=0,vendor=some vendor,version=unknown version",
        "-smbios",
        "type=1,manufacturer=,product=widget,version=alpha,serial=1234,family=none",
        "-smbios",
        "type=2,serial=",
        "-netdev",
        "tap,id=net0,ifname=tap0,script=no,downscript=no,vhost=on,queues=8",
        "-chardev",
        "socket,id=charmonitor,path=/qemumon.sock,server=on,wait=off,logfile=/run/qemumon.log,logappend=off",
        "-chardev",
        "stdio,id=serial0,mux=off,signal=off",
        "-kernel",
        "/kernel.img",
        "-append",
        "console=tty1 ro",
        "-serial",
        "chardev:serial0",
        "-parallel",
        "none",
        "-mon",
        "chardev=charmonitor,mode=control",
        "-pidfile",
        "/run/qemu.pid",
        "-d",
        "unimp",
        "-D",
        "/run/qemu-debug.log",
        "-enable-kvm",
        "-no-reboot",
        "-rtc",
        "clock=vm",
        "-nodefaults",
        "-no-user-config",
        "-run-with",
        "chroot=/chroot,user=vmuser",
        "-msg",
        "timestamp=on",
    ];

    assert_eq!(qemu.to_command(), qemu_expected_args);
}
