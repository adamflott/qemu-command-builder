use crate::args::accel::Accel;
use crate::args::acpitable::AcpiTable;
use crate::args::action::{Action, WatchdogAction};
use crate::args::addfs::AddFd;
use crate::args::audio::Audio;
use crate::args::audiodev::AudioDev;
use crate::args::blockdev::BlockDev;
use crate::args::boot::Boot;
use crate::args::chardev::CharDev;
use crate::args::compact::Compact;
use crate::args::device::Device;
use crate::args::display::QemuDisplay;
use crate::args::drive::Drive;
use crate::args::fsdev::FsDev;
use crate::args::fw_cfg::FwCfg;
use crate::args::g::G;
use crate::args::global::Global;
use crate::args::icount::Icount;
use crate::args::incoming::Incoming;
use crate::args::iscsi::Iscsi;
use crate::args::memory::Memory;
use crate::args::mon::Mon;
use crate::args::msg::Msg;
use crate::args::name::Name;
use crate::args::netdev::NetDev;
use crate::args::network_compat::{LegacyNet, Nic};
use crate::args::numa::NUMA;
use crate::args::object::Object;
use crate::args::overcommit::Overcommit;
use crate::args::plugin::Plugin;
use crate::args::rtc::Rtc;
use crate::args::runwith::RunWith;
use crate::args::sandbox::Sandbox;
use crate::args::semihosting::SemihostingConfig;
use crate::args::serial::SpecialDevice;
use crate::args::set::Set;
use crate::args::smbios::Smbios;
use crate::args::smp::SMP;
use crate::args::spice::Spice;
use crate::args::tpmdev::TpmDev;
use crate::args::trace::Trace;
use crate::args::usb::USBDevice;
use crate::args::vga::VGA;
use crate::args::virtfs::Virtfs;
use crate::args::vnc::VNC;
use crate::parsers::{
    ARG_ACCEL, ARG_ACPITABLE, ARG_ACTION, ARG_ADD_FD, ARG_APPEND, ARG_AUDIO, ARG_AUDIODEV, ARG_BIG_D, ARG_BIG_S, ARG_BIOS, ARG_BLOCKDEV, ARG_BOOT, ARG_CDROM, ARG_CHARDEV, ARG_CHROOT, ARG_COMPAT,
    ARG_CPU, ARG_DAEMONIZE, ARG_DEBUGCON, ARG_DEVICE, ARG_DFILTER, ARG_DISPLAY, ARG_DRIVE, ARG_DTB, ARG_DUMP_VMSTATE, ARG_ECHR, ARG_ENABLE_KVM, ARG_ENABLE_SYNC_PROFILE, ARG_FDA, ARG_FDB, ARG_FSDEV,
    ARG_FULL_SCREEN, ARG_FW_CFG, ARG_G, ARG_GDB, ARG_GLOBAL, ARG_HDA, ARG_HDB, ARG_HDC, ARG_HDD, ARG_ICOUNT, ARG_INCOMING, ARG_INITRD, ARG_ISCSI, ARG_JITDUMP, ARG_K, ARG_KERNEL, ARG_L, ARG_LITTLE_D,
    ARG_LITTLE_S, ARG_LOADVM, ARG_MACHINE, ARG_MACHINE_ALIAS, ARG_MEM_PATH, ARG_MEM_PREALLOC, ARG_MEMORY, ARG_MON, ARG_MONITOR, ARG_MSG, ARG_MTDBLOCK, ARG_NAME, ARG_NET, ARG_NETDEV, ARG_NIC,
    ARG_NO_FD_BOOTCHK, ARG_NO_REBOOT, ARG_NO_SHUTDOWN, ARG_NO_USER_CONFIG, ARG_NODEFAULTS, ARG_NOGRAPHIC, ARG_NUMA, ARG_OBJECT, ARG_ONLY_MIGRATABLE, ARG_OPTION_ROM, ARG_OVERCOMMIT, ARG_PARALLEL,
    ARG_PERFMAP, ARG_PFLASH, ARG_PIDFILE, ARG_PLUGIN, ARG_PRECONFIG, ARG_PROM_ENV, ARG_QMP, ARG_QMP_PRETTY, ARG_QTEST, ARG_QTEST_LOG, ARG_READCONFIG, ARG_RTC, ARG_RUN_WITH, ARG_RUNAS, ARG_SANDBOX,
    ARG_SD, ARG_SEED, ARG_SEMIHOSTING, ARG_SEMIHOSTING_CONFIG, ARG_SERIAL, ARG_SET, ARG_SHIM, ARG_SMBIOS, ARG_SMP, ARG_SNAPSHOT, ARG_SPICE, ARG_TPMDEV, ARG_TRACE, ARG_USB, ARG_USBDEVICE, ARG_UUID,
    ARG_VGA, ARG_VIRTFS, ARG_VNC, ARG_WATCHDOG_ACTION, ARG_WIN2K_HACK, ARG_XEN_ATTACH, ARG_XEN_DOMID, ARG_XEN_DOMID_RESTRICT,
};
use crate::shell_string::ShellString;
use crate::{QEMU_BIN_AARCH64, QEMU_BIN_X86_64, QUuid, QemuCommand, QemuInstanceBase, QemuInstanceForAarch64, QemuInstanceForX86_64};
use std::path::PathBuf;
use std::str::FromStr;
use winnow::Result;

// tokens, ARG_ACCEL, Accel, q.accel
#[macro_export]
macro_rules! ff {
    ($a:expr,$b:ident,$c:ty,$d:expr) => {
        match $a.next() {
            None => {
                return Err(format!("Argument {} expected a token to parse, none found", $b));
            }
            Some(v) => match v.parse::<$c>() {
                Ok(p) => {
                    $d = Some(p);
                }
                Err(parse_err) => {
                    return Err(format!("While trying to parse $b an error was seen: {}", parse_err));
                }
            },
        }
    };
}
#[macro_export]
macro_rules! ffs {
    ($a:expr,$b:ident,$c:ty,$d:expr) => {
        match $a.next() {
            None => {
                return Err(format!("Argument {} expected a token to parse, none found", $b));
            }
            Some(v) => match v.parse::<$c>() {
                Ok(p) => match $d {
                    Some(ref mut vs) => vs.push(p),
                    None => {
                        $d = Some(vec![p]);
                    }
                },
                Err(parse_err) => {
                    return Err(format!("While trying to parse $b an error was seen: {}", parse_err));
                }
            },
        }
    };
}

fn parse_common_qemu_args<Machine: FromStr, Cpu: FromStr>(q: &mut QemuInstanceBase<Machine, Cpu>, tokens: impl IntoIterator<Item = String>) -> Result<(), String>
where
    <Cpu as FromStr>::Err: std::fmt::Display,
    <Machine as FromStr>::Err: std::fmt::Display,
{
    let mut tokens = tokens.into_iter();
    while let Some(token) = tokens.next() {
        match token.as_str() {
            // machine/cpu specific
            ARG_CPU => ff!(tokens, ARG_CPU, Cpu, q.cpu),
            ARG_MACHINE | ARG_MACHINE_ALIAS => ff!(tokens, ARG_MACHINE, Machine, q.machine),
            // common options below
            ARG_ACCEL => ff!(tokens, ARG_ACCEL, Accel, q.accel),
            ARG_SMP => ff!(tokens, ARG_SMP, SMP, q.smp),
            ARG_NUMA => ffs!(tokens, ARG_NUMA, NUMA, q.numa),
            ARG_ADD_FD => ff!(tokens, ARG_ADD_FD, AddFd, q.add_fd),
            ARG_SET => ffs!(tokens, ARG_SET, Set, q.set),
            ARG_GLOBAL => ffs!(tokens, ARG_GLOBAL, Global, q.global),
            ARG_BOOT => ff!(tokens, ARG_BOOT, Boot, q.boot),
            ARG_MEMORY => ff!(tokens, ARG_MEMORY, Memory, q.m),
            ARG_MEM_PATH => ff!(tokens, ARG_MEM_PATH, PathBuf, q.mem_path),
            ARG_MEM_PREALLOC => q.mem_prealloc = Some(true),
            ARG_K => ff!(tokens, ARG_K, String, q.k),
            ARG_AUDIO => ff!(tokens, ARG_AUDIO, Audio, q.audio),
            ARG_AUDIODEV => ff!(tokens, ARG_AUDIODEV, AudioDev, q.audiodev),
            ARG_DEVICE => ffs!(tokens, ARG_DEVICE, Device, q.device),
            ARG_NAME => ff!(tokens, ARG_NAME, Name, q.name),
            ARG_UUID => ff!(tokens, ARG_UUID, newtype_uuid::TypedUuid<QUuid>, q.uuid),
            ARG_FDA => ff!(tokens, ARG_FDA, PathBuf, q.fda),
            ARG_FDB => ff!(tokens, ARG_FDB, PathBuf, q.fdb),
            ARG_HDA => ff!(tokens, ARG_HDA, PathBuf, q.hda),
            ARG_HDB => ff!(tokens, ARG_HDB, PathBuf, q.hdb),
            ARG_HDC => ff!(tokens, ARG_HDC, PathBuf, q.hdc),
            ARG_HDD => ff!(tokens, ARG_HDD, PathBuf, q.hdd),
            ARG_CDROM => ff!(tokens, ARG_CDROM, PathBuf, q.cdrom),
            ARG_BLOCKDEV => ffs!(tokens, ARG_BLOCKDEV, BlockDev, q.blockdev),
            ARG_DRIVE => ffs!(tokens, ARG_DRIVE, Drive, q.drive),
            ARG_MTDBLOCK => ff!(tokens, ARG_MTDBLOCK, PathBuf, q.mdtblock),
            ARG_SD => ff!(tokens, ARG_SD, PathBuf, q.sd),
            ARG_SNAPSHOT => q.snapshot = Some(true),
            ARG_FSDEV => ff!(tokens, ARG_FSDEV, FsDev, q.fsdev),
            ARG_VIRTFS => ff!(tokens, ARG_VIRTFS, Virtfs, q.virtfs),
            ARG_ISCSI => ff!(tokens, ARG_ISCSI, Iscsi, q.iscsi),
            ARG_USB => q.usb = Some(true),
            ARG_USBDEVICE => ff!(tokens, ARG_USBDEVICE, USBDevice, q.usbdevice),
            ARG_DISPLAY => ff!(tokens, ARG_DISPLAY, QemuDisplay, q.display),
            ARG_NOGRAPHIC => q.nographic = Some(true),
            ARG_SPICE => ff!(tokens, ARG_SPICE, Spice, q.spice),
            ARG_VGA => ff!(tokens, ARG_VGA, VGA, q.vga),
            ARG_FULL_SCREEN => q.full_screen = Some(true),
            ARG_G => ff!(tokens, ARG_G, G, q.g),
            ARG_VNC => ff!(tokens, ARG_VNC, VNC, q.vnc),
            ARG_WIN2K_HACK => q.win2k_hack = Some(true),
            ARG_NO_FD_BOOTCHK => q.no_fd_bootchk = Some(true),
            ARG_ACPITABLE => ff!(tokens, ARG_ACPITABLE, AcpiTable, q.acpitable),
            ARG_SMBIOS => ffs!(tokens, ARG_SMBIOS, Smbios, q.smbios),
            ARG_NETDEV => ffs!(tokens, ARG_NETDEV, NetDev, q.netdev),
            ARG_NIC => ffs!(tokens, ARG_NIC, Nic, q.nic),
            ARG_NET => ffs!(tokens, ARG_NET, LegacyNet, q.net),
            ARG_CHARDEV => ffs!(tokens, ARG_CHARDEV, CharDev, q.chardev),
            ARG_TPMDEV => ff!(tokens, ARG_TPMDEV, TpmDev, q.tpmdev),
            ARG_BIOS => ff!(tokens, ARG_BIOS, PathBuf, q.bios),
            ARG_PFLASH => ff!(tokens, ARG_PFLASH, PathBuf, q.pflash),
            ARG_KERNEL => ff!(tokens, ARG_KERNEL, PathBuf, q.kernel),
            ARG_SHIM => ff!(tokens, ARG_SHIM, PathBuf, q.shim),
            ARG_APPEND => ff!(tokens, ARG_APPEND, ShellString, q.append),
            ARG_INITRD => ff!(tokens, ARG_INITRD, PathBuf, q.initrd),
            ARG_DTB => ff!(tokens, ARG_DTB, PathBuf, q.dtb),
            ARG_COMPAT => ff!(tokens, ARG_COMPAT, Compact, q.compact),
            ARG_FW_CFG => ff!(tokens, ARG_FW_CFG, FwCfg, q.fw_cfg),
            ARG_SERIAL => ff!(tokens, ARG_SERIAL, SpecialDevice, q.serial),
            ARG_PARALLEL => ffs!(tokens, ARG_PARALLEL, SpecialDevice, q.parallel),
            ARG_MONITOR => ff!(tokens, ARG_MONITOR, SpecialDevice, q.monitor),
            ARG_QMP => ff!(tokens, ARG_QMP, SpecialDevice, q.qmp),
            ARG_QMP_PRETTY => ff!(tokens, ARG_QMP_PRETTY, SpecialDevice, q.qmp_pretty),
            ARG_MON => ffs!(tokens, ARG_MON, Mon, q.mon),
            ARG_DEBUGCON => ff!(tokens, ARG_DEBUGCON, SpecialDevice, q.debugcon),
            ARG_PIDFILE => ff!(tokens, ARG_PIDFILE, PathBuf, q.pidfile),
            ARG_PRECONFIG => q.preconfig = Some(true),
            ARG_BIG_S => q.big_s = Some(true),
            ARG_OVERCOMMIT => ff!(tokens, ARG_OVERCOMMIT, Overcommit, q.overcommit),
            ARG_GDB => ff!(tokens, ARG_GDB, SpecialDevice, q.gdb),
            ARG_LITTLE_S => q.s = Some(true),
            ARG_LITTLE_D => ffs!(tokens, ARG_LITTLE_D, String, q.d),
            ARG_BIG_D => ff!(tokens, ARG_BIG_D, PathBuf, q.big_d),
            ARG_DFILTER => ffs!(tokens, ARG_DFILTER, String, q.dfilter),
            ARG_SEED => ff!(tokens, ARG_SEED, usize, q.seed),
            ARG_L => ff!(tokens, ARG_L, PathBuf, q.big_l),
            ARG_ENABLE_KVM => q.enable_kvm = Some(true),
            ARG_XEN_DOMID => ff!(tokens, ARG_XEN_DOMID, String, q.xen_domid),
            ARG_XEN_ATTACH => q.xen_attach = Some(true),
            ARG_XEN_DOMID_RESTRICT => q.xen_domid_restrict = Some(true),
            ARG_NO_REBOOT => q.no_reboot = Some(true),
            ARG_NO_SHUTDOWN => q.no_shutdown = Some(true),
            ARG_ACTION => ff!(tokens, ARG_ACTION, Action, q.action),
            ARG_LOADVM => ff!(tokens, ARG_LOADVM, String, q.loadvm),
            ARG_DAEMONIZE => q.daemonize = Some(true),
            ARG_OPTION_ROM => ff!(tokens, ARG_OPTION_ROM, PathBuf, q.option_rom),
            ARG_RTC => ff!(tokens, ARG_RTC, Rtc, q.rtc),
            ARG_ICOUNT => ff!(tokens, ARG_ICOUNT, Icount, q.icount),
            ARG_WATCHDOG_ACTION => ff!(tokens, ARG_WATCHDOG_ACTION, WatchdogAction, q.watchdog_action),
            ARG_ECHR => ff!(tokens, ARG_ECHR, String, q.echr),
            ARG_INCOMING => ffs!(tokens, ARG_INCOMING, Incoming, q.incoming),
            ARG_ONLY_MIGRATABLE => q.only_migratable = Some(true),
            ARG_NODEFAULTS => q.nodefaults = Some(true),
            ARG_SANDBOX => ff!(tokens, ARG_SANDBOX, Sandbox, q.sandbox),
            ARG_READCONFIG => ff!(tokens, ARG_READCONFIG, PathBuf, q.readconfig),
            ARG_NO_USER_CONFIG => q.no_user_config = Some(true),
            ARG_TRACE => ff!(tokens, ARG_TRACE, Trace, q.trace),
            ARG_PLUGIN => ff!(tokens, ARG_PLUGIN, Plugin, q.plugin),
            ARG_SEMIHOSTING => q.semihosting = Some(true),
            ARG_SEMIHOSTING_CONFIG => ff!(tokens, ARG_SEMIHOSTING_CONFIG, SemihostingConfig, q.semihosting_config),
            ARG_QTEST => ff!(tokens, ARG_QTEST, String, q.qtest),
            ARG_QTEST_LOG => ff!(tokens, ARG_QTEST_LOG, PathBuf, q.qtest_log),
            ARG_PROM_ENV => ffs!(tokens, ARG_PROM_ENV, String, q.prom_env),
            ARG_RUNAS => ff!(tokens, ARG_RUNAS, String, q.runas),
            ARG_RUN_WITH => ff!(tokens, ARG_RUN_WITH, RunWith, q.run_with),
            ARG_CHROOT => ff!(tokens, ARG_CHROOT, PathBuf, q.chroot),
            ARG_MSG => ff!(tokens, ARG_MSG, Msg, q.msg),
            ARG_DUMP_VMSTATE => ff!(tokens, ARG_DUMP_VMSTATE, PathBuf, q.dump_vmstate),
            ARG_ENABLE_SYNC_PROFILE => q.enable_sync_profile = Some(true),
            ARG_PERFMAP => ff!(tokens, ARG_PERFMAP, PathBuf, q.perfmap),
            ARG_JITDUMP => ff!(tokens, ARG_JITDUMP, PathBuf, q.jitdump),
            ARG_OBJECT => ffs!(tokens, ARG_OBJECT, Object, q.object),
            other => {
                return Err(format!("unsupported argument: {other}"));
            }
        }
    }
    Ok(())
}

fn _parse_qemu_command_line<Machine: std::str::FromStr, Cpu: std::str::FromStr>(bin_name: &str, s: &str, q: &mut QemuInstanceBase<Machine, Cpu>) -> Result<(), String>
where
    <Cpu as FromStr>::Err: std::fmt::Display,
    <Machine as FromStr>::Err: std::fmt::Display,
{
    let mut tokens = shellish_parse::parse(s, shellish_parse::ParseOptions::new()).map_err(|e| e.to_string())?.into_iter();

    let binary = tokens.next();
    match binary {
        None => {
            return Err(String::from("No next token for determining binary name"));
        }
        Some(path) => {
            if path.contains(bin_name) {
                q.qemu_binary = PathBuf::from(path);
            } else {
                return Err(format!("{} not found in string {}", bin_name, path));
            }
        }
    }

    parse_common_qemu_args(q, tokens)?;

    Ok(())
}

impl FromStr for QemuInstanceForX86_64 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut q = QemuInstanceBase::builder().qemu_binary(PathBuf::from("")).build();
        _parse_qemu_command_line(QEMU_BIN_X86_64, s, &mut q)?;
        Ok(q)
    }
}

impl FromStr for QemuInstanceForAarch64 {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut q = QemuInstanceBase::builder().qemu_binary(PathBuf::from("")).build();
        _parse_qemu_command_line(QEMU_BIN_AARCH64, s, &mut q)?;
        Ok(q)
    }
}

pub fn parse_qemu_command_line(cmd: &str) -> Result<QemuCommand, String> {
    match cmd.split_once(" ") {
        None => Err(String::from("No next token for determining binary name")),
        Some((bin, _rest)) => {
            if bin.ends_with(QEMU_BIN_AARCH64) {
                return QemuInstanceForAarch64::from_str(cmd).map(QemuCommand::Aarch64);
            }
            if bin.ends_with(QEMU_BIN_X86_64) {
                return QemuInstanceForX86_64::from_str(cmd).map(QemuCommand::X86_64);
            }
            Err(format!("binary {} is an unsupported architecture", bin))
        }
    }
}
