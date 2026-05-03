use crate::accel::{ARG_ACCEL, Accel};
use crate::acpitable::{ARG_ACPITABLE, AcpiTable};
use crate::action::{ARG_ACTION, Action};
use crate::addfs::{ARG_ADD_FD, AddFd};
use crate::boot::{ARG_BOOT, Boot};
use crate::chardev::{ARG_CHARDEV, CharDev};
use crate::cpu::{ARG_CPU, CpuX86};
use crate::device::{ARG_DEVICE, Device};
use crate::display::{ARG_DISPLAY, QemuDisplay};
use crate::drive::{ARG_DRIVE, Drive};
use crate::machine::{ARG_MACHINE, MachineX86_64};
use crate::memory::{ARG_MEMORY, Memory};
use crate::mon::{ARG_MON, Mon};
use crate::msg::{ARG_MSG, Msg};
use crate::name::{ARG_NAME, Name};
use crate::netdev::{ARG_NETDEV, NetDev};
use crate::parsers::{
    ARG_APPEND, ARG_BIG_D, ARG_BIG_S, ARG_BIOS, ARG_CDROM, ARG_DAEMONIZE, ARG_DTB, ARG_DUMP_VMSTATE, ARG_ECHR, ARG_ENABLE_KVM, ARG_ENABLE_SYNC_PROFILE, ARG_FDA, ARG_FDB, ARG_FULL_SCREEN, ARG_HDA,
    ARG_HDB, ARG_HDC, ARG_HDD, ARG_INITRD, ARG_JITDUMP, ARG_K, ARG_KERNEL, ARG_L, ARG_LITTLE_D, ARG_LITTLE_S, ARG_LOADVM, ARG_MEM_PATH, ARG_MEM_PREALLOC, ARG_MTDBLOCK, ARG_NO_FD_BOOTCHK,
    ARG_NO_REBOOT, ARG_NO_SHUTDOWN, ARG_NO_USER_CONFIG, ARG_NODEFAULTS, ARG_NOGRAPHIC, ARG_ONLY_MIGRATABLE, ARG_OPTION_ROM, ARG_PERFMAP, ARG_PFLASH, ARG_PIDFILE, ARG_PRECONFIG, ARG_READCONFIG,
    ARG_SD, ARG_SEED, ARG_SHIM, ARG_SNAPSHOT, ARG_USB, ARG_WIN2K_HACK, ARG_XEN_ATTACH, ARG_XEN_DOMID_RESTRICT, ARG_XEN_ID,
};
use crate::rtc::{ARG_RTC, Rtc};
use crate::runwith::{ARG_RUN_WITH, RunWith};
use crate::serial::{ARG_PARALLEL, ARG_SERIAL, SpecialDevice};
use crate::shell_string::ShellString;
use crate::smbios::{ARG_SMBIOS, Smbios};
use crate::smp::{ARG_SMP, SMP};
use crate::vga::{ARG_VGA, VGA};
use crate::{QEMU_BIN_AARCH64, QEMU_BIN_X86_64, QemuInstanceBase, QemuInstanceForAarch64, QemuInstanceForX86_64};
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
                        let mut xx = Vec::new();
                        xx.push(p);
                        $d = Some(xx);
                    }
                },
                Err(parse_err) => {
                    return Err(format!("While trying to parse $b an error was seen: {}", parse_err));
                }
            },
        }
    };
}

impl FromStr for QemuInstanceForX86_64 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut q: QemuInstanceForX86_64 = QemuInstanceBase::builder().qemu_binary(PathBuf::from("")).build();

        let mut tokens = shellish_parse::parse(s, shellish_parse::ParseOptions::new()).map_err(|e| e.to_string())?.into_iter();

        let binary = tokens.next();
        match binary {
            None => {
                return Err(String::from("No next token for determining binary name"));
            }
            Some(path) => {
                if path.contains(QEMU_BIN_X86_64) {
                    q.qemu_binary = PathBuf::from(path);
                } else {
                    return Err(format!("{} not found in string", QEMU_BIN_X86_64));
                }
            }
        }

        while let Some(token) = tokens.next() {
            match token.as_str() {
                ARG_ACCEL => ff!(tokens, ARG_ACCEL, Accel, q.accel),
                ARG_APPEND => ff!(tokens, ARG_APPEND, ShellString, q.append),
                ARG_ACPITABLE => ff!(tokens, ARG_ACPITABLE, AcpiTable, q.acpitable),
                ARG_ACTION => ff!(tokens, ARG_ACTION, Action, q.action),
                ARG_ADD_FD => ff!(tokens, ARG_ADD_FD, AddFd, q.add_fd),
                ARG_BIG_S => q.big_s = Some(true),
                ARG_RTC => ff!(tokens, ARG_RTC, Rtc, q.rtc),
                ARG_MEM_PATH => ff!(tokens, ARG_MEM_PATH, PathBuf, q.mem_path),
                ARG_K => ff!(tokens, ARG_K, String, q.k),
                ARG_FDA => ff!(tokens, ARG_FDA, PathBuf, q.fda),
                ARG_FDB => ff!(tokens, ARG_FDB, PathBuf, q.fdb),
                ARG_HDA => ff!(tokens, ARG_HDA, PathBuf, q.hda),
                ARG_HDB => ff!(tokens, ARG_HDB, PathBuf, q.hdb),
                ARG_HDC => ff!(tokens, ARG_HDC, PathBuf, q.hdc),
                ARG_HDD => ff!(tokens, ARG_HDD, PathBuf, q.hdd),
                ARG_CDROM => ff!(tokens, ARG_CDROM, PathBuf, q.cdrom),
                ARG_MTDBLOCK => ff!(tokens, ARG_MTDBLOCK, PathBuf, q.mdtblock),
                ARG_BIOS => ff!(tokens, ARG_BIOS, PathBuf, q.bios),
                ARG_PFLASH => ff!(tokens, ARG_PFLASH, PathBuf, q.pflash),
                ARG_KERNEL => ff!(tokens, ARG_KERNEL, PathBuf, q.kernel),
                ARG_SHIM => ff!(tokens, ARG_SHIM, PathBuf, q.shim),
                ARG_INITRD => ff!(tokens, ARG_INITRD, PathBuf, q.initrd),
                ARG_DTB => ff!(tokens, ARG_DTB, PathBuf, q.dtb),
                ARG_SD => ff!(tokens, ARG_SD, PathBuf, q.sd),
                ARG_PIDFILE => ff!(tokens, ARG_PIDFILE, PathBuf, q.pidfile),
                ARG_BIG_D => ff!(tokens, ARG_BIG_D, PathBuf, q.big_d),
                ARG_DRIVE => ffs!(tokens, ARG_DRIVE, Drive, q.drive),
                ARG_RUN_WITH => ff!(tokens, ARG_RUN_WITH, RunWith, q.run_with),
                // TODO split on :
                ARG_LITTLE_D => ffs!(tokens, ARG_LITTLE_D, String, q.d),
                ARG_SEED => ff!(tokens, ARG_SEED, usize, q.seed),
                ARG_L => ff!(tokens, ARG_L, PathBuf, q.big_l),
                ARG_MACHINE => ff!(tokens, ARG_MACHINE, MachineX86_64, q.machine),
                ARG_OPTION_ROM => ff!(tokens, ARG_OPTION_ROM, PathBuf, q.option_rom),
                ARG_LOADVM => ff!(tokens, ARG_LOADVM, String, q.loadvm),
                ARG_XEN_ID => ff!(tokens, ARG_XEN_ID, String, q.xen_id),
                ARG_DUMP_VMSTATE => ff!(tokens, ARG_DUMP_VMSTATE, PathBuf, q.dump_vmstate),
                ARG_PERFMAP => ff!(tokens, ARG_PERFMAP, PathBuf, q.perfmap),
                ARG_JITDUMP => ff!(tokens, ARG_JITDUMP, PathBuf, q.jitdump),
                // ARG_G => ff!(tokens, ARG_G, PathBuf, q.g),
                // ARG_GDB => ff!(tokens, ARG_GDB, PathBuf, q.gdb),
                ARG_ECHR => ff!(tokens, ARG_ECHR, String, q.echr),
                ARG_READCONFIG => ff!(tokens, ARG_READCONFIG, PathBuf, q.readconfig),
                ARG_BOOT => ff!(tokens, ARG_BOOT, Boot, q.boot),
                ARG_CPU => ff!(tokens, ARG_CPU, CpuX86, q.cpu),
                ARG_DAEMONIZE => q.daemonize = Some(true),
                ARG_DEVICE => ffs!(tokens, ARG_DEVICE, Device, q.device),
                ARG_DISPLAY => ff!(tokens, ARG_DISPLAY, QemuDisplay, q.display),
                ARG_ENABLE_KVM => q.enable_kvm = Some(true),
                ARG_ENABLE_SYNC_PROFILE => q.enable_sync_profile = Some(true),
                ARG_FULL_SCREEN => q.full_screen = Some(true),
                ARG_LITTLE_S => q.s = Some(true),
                ARG_MEMORY => ff!(tokens, ARG_MEMORY, Memory, q.m),
                ARG_MEM_PREALLOC => q.mem_prealloc = Some(true),
                ARG_MSG => ff!(tokens, ARG_MSG, Msg, q.msg),
                ARG_NAME => ff!(tokens, ARG_NAME, Name, q.name),
                ARG_NODEFAULTS => q.nodefaults = Some(true),
                ARG_NOGRAPHIC => q.nographic = Some(true),
                ARG_NO_FD_BOOTCHK => q.no_fd_bootchk = Some(true),
                ARG_NO_REBOOT => q.no_reboot = Some(true),
                ARG_NO_SHUTDOWN => q.no_shutdown = Some(true),
                ARG_NO_USER_CONFIG => q.no_user_config = Some(true),
                ARG_ONLY_MIGRATABLE => q.only_migratable = Some(true),
                ARG_PRECONFIG => q.preconfig = Some(true),
                ARG_MON => ffs!(tokens, ARG_MON, Mon, q.mon),
                ARG_NETDEV => ffs!(tokens, ARG_NETDEV, NetDev, q.netdev),
                ARG_CHARDEV => ffs!(tokens, ARG_CHARDEV, CharDev, q.chardev),
                ARG_SERIAL => ff!(tokens, ARG_SERIAL, SpecialDevice, q.serial),
                ARG_PARALLEL => ffs!(tokens, ARG_PARALLEL, SpecialDevice, q.parallel),
                ARG_SMBIOS => ffs!(tokens, ARG_SMBIOS, Smbios, q.smbios),
                ARG_SMP => ff!(tokens, ARG_SMP, SMP, q.smp),
                ARG_SNAPSHOT => q.snapshot = Some(true),
                ARG_USB => q.usb = Some(true),
                ARG_VGA => ff!(tokens, ARG_VGA, VGA, q.vga),
                ARG_WIN2K_HACK => q.win2k_hack = Some(true),
                ARG_XEN_ATTACH => q.xen_attach = Some(true),
                ARG_XEN_DOMID_RESTRICT => q.xen_domid_restrict = Some(true),
                other => {
                    return Err(format!("unsupported argument: {other}"));
                }
            }
        }

        Ok(q)
    }
}

impl FromStr for QemuInstanceForAarch64 {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut q: QemuInstanceForAarch64 = QemuInstanceBase::builder().qemu_binary(PathBuf::from("")).build();
        let mut tokens = s.split_whitespace();

        let binary = tokens.next();
        match binary {
            None => {
                return Err(String::from("No next token for determining binary name"));
            }
            Some(path) => {
                if path.contains(QEMU_BIN_AARCH64) {
                    q.qemu_binary = PathBuf::from(path);
                } else {
                    return Err(format!("{} not found in string", QEMU_BIN_AARCH64));
                }
            }
        }

        Ok(q)
    }
}
