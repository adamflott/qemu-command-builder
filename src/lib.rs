#![recursion_limit = "256"]

pub mod accel;
pub mod acpitable;
pub mod action;
pub mod addfs;
pub mod audio;
pub mod audiodev;
pub mod blockdev;
pub mod boot;
pub mod chardev;
pub mod common;
pub mod compact;
pub mod cpu;
pub mod cpu_flags;
pub mod cpu_type;
pub mod device;
pub mod display;
pub mod drive;
pub mod fsdev;
pub mod fw_cfg;
pub mod global;
pub mod icount;
pub mod incoming;
pub mod iscsi;
pub mod machine;
pub mod machine_type;
pub mod memory;
pub mod mon;
pub mod msg;
pub mod name;
pub mod netdev;
pub mod numa;
pub mod object;
pub mod overcommit;
pub mod parser;
pub mod parsers;
pub mod plugin;
pub mod rtc;
pub mod runwith;
pub mod sandbox;
pub mod serial;
pub mod set;
pub mod shell_path;
pub mod shell_string;
pub mod smbios;
pub mod smp;
pub mod spice;
pub mod to_command;
pub mod tpmdev;
pub mod trace;
pub mod usb;
pub mod vga;
pub mod virtfs;
pub mod vnc;

use crate::accel::Accel;
use crate::acpitable::AcpiTable;
use crate::action::{Action, WatchdogAction};
use crate::addfs::AddFd;
use crate::audio::Audio;
use crate::audiodev::AudioDev;
use crate::blockdev::BlockDev;
use crate::boot::Boot;
use crate::chardev::CharDev;
use crate::compact::Compact;
use crate::cpu::{CpuAarch64, CpuX86};
use crate::device::Device;
use crate::display::QemuDisplay;
use crate::drive::Drive;
use crate::fsdev::FsDev;
use crate::fw_cfg::FwCfg;
use crate::global::Global;
use crate::icount::Icount;
use crate::incoming::Incoming;
use crate::iscsi::Iscsi;
use crate::memory::Memory;
use crate::mon::Mon;
use crate::msg::Msg;
use crate::name::Name;
use crate::netdev::NetDev;
use crate::numa::NUMA;
use crate::object::Object;
use crate::overcommit::Overcommit;
use crate::parsers::{
    ARG_APPEND, ARG_BIG_D, ARG_BIG_S, ARG_BIOS, ARG_CDROM, ARG_DAEMONIZE, ARG_DTB, ARG_DUMP_VMSTATE, ARG_ECHR, ARG_ENABLE_KVM, ARG_ENABLE_SYNC_PROFILE, ARG_FDA, ARG_FDB, ARG_FULL_SCREEN, ARG_G,
    ARG_GDB, ARG_HDA, ARG_HDB, ARG_HDC, ARG_HDD, ARG_INITRD, ARG_JITDUMP, ARG_K, ARG_KERNEL, ARG_L, ARG_LITTLE_D, ARG_LITTLE_S, ARG_LOADVM, ARG_MEM_PATH, ARG_MEM_PREALLOC, ARG_MTDBLOCK,
    ARG_NO_FD_BOOTCHK, ARG_NO_REBOOT, ARG_NO_SHUTDOWN, ARG_NO_USER_CONFIG, ARG_NODEFAULTS, ARG_NOGRAPHIC, ARG_ONLY_MIGRATABLE, ARG_OPTION_ROM, ARG_PERFMAP, ARG_PFLASH, ARG_PIDFILE, ARG_PRECONFIG,
    ARG_READCONFIG, ARG_SD, ARG_SEED, ARG_SHIM, ARG_SNAPSHOT, ARG_USB, ARG_UUID, ARG_WIN2K_HACK, ARG_XEN_ATTACH, ARG_XEN_DOMID_RESTRICT, ARG_XEN_ID,
};
use crate::plugin::Plugin;
use crate::rtc::Rtc;
use crate::runwith::RunWith;
use crate::sandbox::Sandbox;
use crate::serial::{ARG_PARALLEL, ARG_SERIAL, SpecialDevice};
use crate::set::Set;
use crate::smbios::Smbios;
use crate::smp::SMP;
use crate::spice::Spice;
use crate::to_command::{ToArg, ToCommand};
use crate::tpmdev::TpmDev;
use crate::trace::Trace;
use crate::usb::USBDevice;
use crate::vga::VGA;
use crate::virtfs::Virtfs;
use crate::vnc::VNC;

use crate::machine::MachineX86_64;
use crate::machine_type::MachineAarch64;
use crate::shell_string::ShellString;
use bon::Builder;
use chrono::{DateTime, TimeZone, Utc};
use newtype_uuid::{TypedUuidKind, TypedUuidTag};
use proptest::prelude::{Arbitrary, Just};
use proptest_derive::Arbitrary;
use std::path::PathBuf;
use std::str::FromStr;

/// QEMU binary name for x86_64
const QEMU_BIN_X86_64: &'static str = "qemu-system-x86_64";

/// QEMU binary name for aarch64
const QEMU_BIN_AARCH64: &'static str = "qemu-system-aarch64";

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct QemuInstanceBase<M, C> {
    pub qemu_binary: PathBuf,

    pub machine: Option<M>,
    pub cpu: Option<C>,

    pub accel: Option<Accel>,
    pub smp: Option<SMP>,
    pub numa: Option<Vec<NUMA>>,
    pub add_fd: Option<AddFd>,
    pub set: Option<Vec<Set>>,
    pub global: Option<Vec<Global>>,
    pub boot: Option<Boot>,
    pub m: Option<Memory>,
    pub mem_path: Option<PathBuf>,
    pub mem_prealloc: Option<bool>,
    pub k: Option<String>,
    pub audio: Option<Audio>,
    pub audiodev: Option<AudioDev>,
    pub device: Option<Vec<Device>>,
    pub name: Option<Name>,
    pub uuid: Option<newtype_uuid::TypedUuid<Xuuid>>,
    pub fda: Option<PathBuf>,
    pub fdb: Option<PathBuf>,
    pub hda: Option<PathBuf>,
    pub hdb: Option<PathBuf>,
    pub hdc: Option<PathBuf>,
    pub hdd: Option<PathBuf>,
    pub cdrom: Option<PathBuf>,
    pub blockdev: Option<Vec<BlockDev>>,
    pub drive: Option<Vec<Drive>>,
    pub mdtblock: Option<PathBuf>,
    pub sd: Option<PathBuf>,
    pub snapshot: Option<bool>,
    pub fsdev: Option<FsDev>,
    pub virtfs: Option<Virtfs>,
    pub iscsi: Option<Iscsi>,
    pub usb: Option<bool>,
    pub usbdevice: Option<USBDevice>,
    pub display: Option<QemuDisplay>,
    pub nographic: Option<bool>,
    pub spice: Option<Spice>,
    pub vga: Option<VGA>,
    pub full_screen: Option<bool>,
    pub g: Option<(usize, usize, Option<usize>)>,
    pub vnc: Option<VNC>,
    pub win2k_hack: Option<bool>,
    pub no_fd_bootchk: Option<bool>,
    pub acpitable: Option<AcpiTable>,
    pub smbios: Option<Vec<Smbios>>,
    pub netdev: Option<Vec<NetDev>>,
    pub chardev: Option<Vec<CharDev>>,
    pub tpmdev: Option<TpmDev>,
    pub bios: Option<PathBuf>,
    pub pflash: Option<PathBuf>,
    pub kernel: Option<PathBuf>,
    pub shim: Option<PathBuf>,
    pub append: Option<ShellString>,
    pub initrd: Option<PathBuf>,
    pub dtb: Option<PathBuf>,
    pub compact: Option<Compact>,
    pub fw_cfg: Option<FwCfg>,
    pub serial: Option<SpecialDevice>,
    pub parallel: Option<Vec<SpecialDevice>>,
    pub monitor: Option<SpecialDevice>,
    pub qmp: Option<SpecialDevice>,
    pub qmp_pretty: Option<SpecialDevice>,
    pub mon: Option<Vec<Mon>>,
    pub debugcon: Option<CharDev>,
    pub pidfile: Option<PathBuf>,
    pub preconfig: Option<bool>,
    pub big_s: Option<bool>,
    pub overcommit: Option<Overcommit>,
    pub gdb: Option<SpecialDevice>,
    pub s: Option<bool>,
    pub d: Option<Vec<String>>,
    pub big_d: Option<PathBuf>,
    pub dfilter: Option<Vec<String>>,
    pub seed: Option<usize>,
    pub big_l: Option<PathBuf>,
    pub enable_kvm: Option<bool>,
    pub xen_id: Option<String>,
    pub xen_attach: Option<bool>,
    pub xen_domid_restrict: Option<bool>,
    pub no_reboot: Option<bool>,
    pub no_shutdown: Option<bool>,
    pub action: Option<Action>,
    pub loadvm: Option<String>,
    pub daemonize: Option<bool>,
    pub option_rom: Option<PathBuf>,
    pub rtc: Option<Rtc>,
    pub icount: Option<Icount>,
    pub watchdog_action: Option<WatchdogAction>,
    pub echr: Option<String>,
    pub incoming: Option<Vec<Incoming>>,
    pub only_migratable: Option<bool>,
    pub nodefaults: Option<bool>,
    pub sandbox: Option<Sandbox>,
    pub readconfig: Option<PathBuf>,
    pub no_user_config: Option<bool>,
    pub trace: Option<Trace>,
    pub plugin: Option<Plugin>,
    pub run_with: Option<RunWith>,
    pub msg: Option<Msg>,
    pub dump_vmstate: Option<PathBuf>,
    pub enable_sync_profile: Option<bool>,
    pub perfmap: Option<PathBuf>,
    pub jitdump: Option<PathBuf>,
    pub object: Option<Vec<Object>>,
}

impl<M: ToCommand, C: ToCommand> ToCommand for QemuInstanceBase<M, C>
where
    QemuInstanceBase<M, C>: FromStr,
{
    fn command(&self) -> String {
        self.qemu_binary.display().to_string()
    }

    fn to_args(&self) -> Vec<String> {
        let mut cmd = vec![];

        if let Some(machine) = &self.machine {
            cmd.append(&mut machine.to_command());
        }
        if let Some(cpu) = &self.cpu {
            cmd.append(&mut cpu.to_command());
        }
        if let Some(accel) = &self.accel {
            cmd.append(&mut accel.to_command());
        }
        if let Some(smp) = &self.smp {
            cmd.append(&mut smp.to_command());
        }
        if let Some(numas) = &self.numa {
            for numa in numas {
                cmd.append(&mut numa.to_command());
            }
        }
        if let Some(add_fd) = &self.add_fd {
            cmd.append(&mut add_fd.to_command());
        }
        if let Some(sets) = &self.set {
            for set in sets {
                cmd.append(&mut set.to_command());
            }
        }
        if let Some(globals) = &self.global {
            for global in globals {
                cmd.append(&mut global.to_command());
            }
        }
        if let Some(boot) = &self.boot {
            cmd.append(&mut boot.to_command());
        }
        if let Some(m) = &self.m {
            cmd.append(&mut m.to_command());
        }
        if let Some(path) = &self.mem_path {
            cmd.push(ARG_MEM_PATH.to_string());
            cmd.push(path.display().to_string());
        }
        if let Some(state) = self.mem_prealloc
            && state
        {
            cmd.push(ARG_MEM_PREALLOC.to_string());
        }
        if let Some(lang) = &self.k {
            cmd.push(ARG_K.to_string());
            cmd.push(lang.to_string());
        }
        if let Some(audio) = &self.audio {
            cmd.append(&mut audio.to_command());
        }
        if let Some(audiodev) = &self.audiodev {
            cmd.append(&mut audiodev.to_command());
        }
        if let Some(devices) = &self.device {
            for device in devices {
                cmd.append(&mut device.to_command());
            }
        }
        if let Some(name) = &self.name {
            cmd.append(&mut name.to_command());
        }
        if let Some(uuid) = &self.uuid {
            cmd.push(ARG_UUID.to_string());
            cmd.push(uuid.to_string());
        }
        if let Some(fda) = &self.fda {
            cmd.push(ARG_FDA.to_string());
            cmd.push(fda.display().to_string());
        }
        if let Some(fdb) = &self.fdb {
            cmd.push(ARG_FDB.to_string());
            cmd.push(fdb.display().to_string());
        }
        if let Some(hda) = &self.hda {
            cmd.push(ARG_HDA.to_string());
            cmd.push(hda.display().to_string());
        }
        if let Some(hdb) = &self.hdb {
            cmd.push(ARG_HDB.to_string());
            cmd.push(hdb.display().to_string());
        }
        if let Some(hdc) = &self.hdc {
            cmd.push(ARG_HDC.to_string());
            cmd.push(hdc.display().to_string());
        }
        if let Some(hdd) = &self.hdd {
            cmd.push(ARG_HDD.to_string());
            cmd.push(hdd.display().to_string());
        }
        if let Some(cdrom) = &self.cdrom {
            cmd.push(ARG_CDROM.to_string());
            cmd.push(cdrom.display().to_string());
        }
        if let Some(blockdevs) = &self.blockdev {
            for blockdev in blockdevs {
                cmd.append(&mut blockdev.to_command());
            }
        }
        if let Some(drives) = &self.drive {
            for drive in drives {
                cmd.append(&mut drive.to_command());
            }
        }
        if let Some(mdtblock) = &self.mdtblock {
            cmd.push(ARG_MTDBLOCK.to_string());
            cmd.push(mdtblock.display().to_string());
        }
        if let Some(sd) = &self.sd {
            cmd.push(ARG_SD.to_string());
            cmd.push(sd.display().to_string());
        }
        if let Some(state) = &self.snapshot
            && *state
        {
            cmd.push(ARG_SNAPSHOT.to_string());
        }
        if let Some(fsdev) = &self.fsdev {
            cmd.append(&mut fsdev.to_command());
        }
        if let Some(virtfs) = &self.virtfs {
            cmd.append(&mut virtfs.to_command());
        }
        if let Some(iscsi) = &self.iscsi {
            cmd.append(&mut iscsi.to_command());
        }
        if let Some(state) = &self.usb
            && *state
        {
            cmd.push(ARG_USB.to_string());
        }
        if let Some(usbdevice) = &self.usbdevice {
            cmd.append(&mut usbdevice.to_command());
        }
        if let Some(display) = &self.display {
            cmd.append(&mut display.to_command());
        }
        if let Some(state) = &self.nographic
            && *state
        {
            cmd.push(ARG_NOGRAPHIC.to_string());
        }
        if let Some(spice) = &self.spice {
            cmd.append(&mut spice.to_command());
        }
        if let Some(vga) = &self.vga {
            cmd.append(&mut vga.to_command());
        }
        if let Some(full_screen) = &self.full_screen
            && *full_screen
        {
            cmd.push(ARG_FULL_SCREEN.to_string());
        }
        if let Some((w, h, d)) = &self.g {
            cmd.push(ARG_G.to_string());
            let mut dimensions = format!("{}x{}", w, h);

            if let Some(d) = d {
                dimensions.push_str(format!("x{}", &d.to_string()).as_str());
            }
            cmd.push(dimensions);
        }
        if let Some(vnc) = &self.vnc {
            cmd.append(&mut vnc.to_command());
        }
        if let Some(win2k_hack) = &self.win2k_hack
            && *win2k_hack
        {
            cmd.push(ARG_WIN2K_HACK.to_string());
        }
        if let Some(no_fd_bootchk) = &self.no_fd_bootchk
            && *no_fd_bootchk
        {
            cmd.push(ARG_NO_FD_BOOTCHK.to_string());
        }
        if let Some(acpitable) = &self.acpitable {
            cmd.append(&mut acpitable.to_command());
        }
        if let Some(smbioss) = &self.smbios {
            for smbios in smbioss {
                cmd.append(&mut smbios.to_command());
            }
        }
        if let Some(netdevs) = &self.netdev {
            for netdev in netdevs {
                cmd.append(&mut netdev.to_command());
            }
        }
        if let Some(chardevs) = &self.chardev {
            for chardev in chardevs {
                cmd.append(&mut chardev.to_command());
            }
        }
        if let Some(tpmdev) = &self.tpmdev {
            cmd.append(&mut tpmdev.to_command());
        }
        if let Some(bios) = &self.bios {
            cmd.push(ARG_BIOS.to_string());
            cmd.push(bios.display().to_string());
        }
        if let Some(pflash) = &self.pflash {
            cmd.push(ARG_PFLASH.to_string());
            cmd.push(pflash.display().to_string());
        }
        if let Some(kernel) = &self.kernel {
            cmd.push(ARG_KERNEL.to_string());
            cmd.push(kernel.display().to_string());
        }
        if let Some(shim) = &self.shim {
            cmd.push(ARG_SHIM.to_string());
            cmd.push(shim.display().to_string());
        }
        if let Some(append) = &self.append {
            cmd.push(ARG_APPEND.to_string());
            cmd.push(append.to_string());
        }
        if let Some(initrd) = &self.initrd {
            cmd.push(ARG_INITRD.to_string());
            cmd.push(initrd.display().to_string());
        }
        if let Some(dtb) = &self.dtb {
            cmd.push(ARG_DTB.to_string());
            cmd.push(dtb.display().to_string());
        }
        if let Some(compact) = &self.compact {
            cmd.append(&mut compact.to_command());
        }
        if let Some(fw_cfg) = &self.fw_cfg {
            cmd.append(&mut fw_cfg.to_command());
        }
        if let Some(serial) = &self.serial {
            cmd.push(ARG_SERIAL.to_string());
            cmd.append(&mut serial.to_args());
        }
        if let Some(parallels) = &self.parallel {
            for parallel in parallels {
                cmd.push(ARG_PARALLEL.to_string());
                cmd.append(&mut parallel.to_args());
            }
        }
        if let Some(monitor) = &self.monitor {
            cmd.push("-monitor".to_string());
            cmd.append(&mut monitor.to_command());
        }
        if let Some(qmp) = &self.qmp {
            cmd.push("-qmp".to_string());
            cmd.append(&mut qmp.to_command());
        }
        if let Some(qmp_pretty) = &self.qmp_pretty {
            cmd.push("-qmp-pretty".to_string());
            cmd.append(&mut qmp_pretty.to_command());
        }
        if let Some(mons) = &self.mon {
            for mon in mons {
                cmd.append(&mut mon.to_command());
            }
        }
        if let Some(debugcon) = &self.debugcon {
            cmd.push("-debugcon".to_string());
            cmd.append(&mut debugcon.to_command());
        }
        if let Some(pidfile) = &self.pidfile {
            cmd.push(ARG_PIDFILE.to_string());
            cmd.push(pidfile.display().to_string());
        }
        if let Some(preconfig) = &self.preconfig
            && *preconfig
        {
            cmd.push(ARG_PRECONFIG.to_string());
        }
        if let Some(s) = &self.big_s
            && *s
        {
            cmd.push(ARG_BIG_S.to_string());
        }
        if let Some(overcommit) = &self.overcommit {
            cmd.append(&mut overcommit.to_command());
        }
        if let Some(gdb) = &self.gdb {
            cmd.push(ARG_GDB.to_string());
            cmd.append(&mut gdb.to_command());
        }
        if let Some(s) = &self.s
            && *s
        {
            cmd.push(ARG_LITTLE_S.to_string());
        }
        if let Some(d) = &self.d {
            cmd.push(ARG_LITTLE_D.to_string());
            cmd.push(d.join(","));
        }
        if let Some(big_d) = &self.big_d {
            cmd.push(ARG_BIG_D.to_string());
            cmd.push(big_d.display().to_string());
        }
        if let Some(dfilter) = &self.dfilter {
            cmd.push("-dfilter".to_string());
            cmd.push(dfilter.join(","));
        }
        if let Some(seed) = &self.seed {
            cmd.push(ARG_SEED.to_string());
            cmd.push(seed.to_string());
        }
        if let Some(l) = &self.big_l {
            cmd.push(ARG_L.to_string());
            cmd.push(l.display().to_string());
        }
        if let Some(enable_kvm) = &self.enable_kvm
            && *enable_kvm
        {
            cmd.push(ARG_ENABLE_KVM.to_string());
        }
        if let Some(xen_id) = &self.xen_id {
            cmd.push(ARG_XEN_ID.to_string());
            cmd.push(xen_id.to_string());
        }
        if let Some(xen_attach) = &self.xen_attach
            && *xen_attach
        {
            cmd.push(ARG_XEN_ATTACH.to_string());
        }
        if let Some(xen_domid_restrict) = &self.xen_domid_restrict
            && *xen_domid_restrict
        {
            cmd.push(ARG_XEN_DOMID_RESTRICT.to_string());
        }
        if let Some(no_reboot) = &self.no_reboot
            && *no_reboot
        {
            cmd.push(ARG_NO_REBOOT.to_string());
        }
        if let Some(no_shutdown) = &self.no_shutdown
            && *no_shutdown
        {
            cmd.push(ARG_NO_SHUTDOWN.to_string());
        }
        if let Some(action) = &self.action {
            cmd.append(&mut action.to_command());
        }
        if let Some(loadvm) = &self.loadvm {
            cmd.push(ARG_LOADVM.to_string());
            cmd.push(loadvm.to_string());
        }
        if let Some(daemonize) = &self.daemonize
            && *daemonize
        {
            cmd.push(ARG_DAEMONIZE.to_string());
        }
        if let Some(option_rom) = &self.option_rom {
            cmd.push(ARG_OPTION_ROM.to_string());
            cmd.push(option_rom.display().to_string());
        }
        if let Some(rtc) = &self.rtc {
            cmd.append(&mut rtc.to_command());
        }
        if let Some(icount) = &self.icount {
            cmd.append(&mut icount.to_command());
        }
        if let Some(watchdog_action) = &self.watchdog_action {
            cmd.push("-watchdog-action".to_string());
            cmd.push(watchdog_action.to_arg().to_string());
        }
        if let Some(echr) = &self.echr {
            cmd.push(ARG_ECHR.to_string());
            cmd.push(echr.to_string());
        }
        if let Some(incomings) = &self.incoming {
            for incoming in incomings {
                cmd.append(&mut incoming.to_command());
            }
        }
        if let Some(only_migratable) = &self.only_migratable
            && *only_migratable
        {
            cmd.push(ARG_ONLY_MIGRATABLE.to_string());
        }
        if let Some(nodefaults) = &self.nodefaults
            && *nodefaults
        {
            cmd.push(ARG_NODEFAULTS.to_string());
        }
        if let Some(sandbox) = &self.sandbox {
            cmd.append(&mut sandbox.to_command());
        }
        if let Some(readconfig) = &self.readconfig {
            cmd.push(ARG_READCONFIG.to_string());
            cmd.push(readconfig.display().to_string());
        }
        if let Some(no_user_config) = &self.no_user_config
            && *no_user_config
        {
            cmd.push(ARG_NO_USER_CONFIG.to_string());
        }
        if let Some(trace) = &self.trace {
            cmd.append(&mut trace.to_command());
        }
        if let Some(plugin) = &self.plugin {
            cmd.append(&mut plugin.to_command());
        }
        if let Some(run_with) = &self.run_with {
            cmd.append(&mut run_with.to_command());
        }
        if let Some(msg) = &self.msg {
            cmd.append(&mut msg.to_command());
        }
        if let Some(dump_vmstate) = &self.dump_vmstate {
            cmd.push(ARG_DUMP_VMSTATE.to_string());
            cmd.push(dump_vmstate.display().to_string());
        }
        if let Some(enable_sync_profile) = &self.enable_sync_profile
            && *enable_sync_profile
        {
            cmd.push(ARG_ENABLE_SYNC_PROFILE.to_string());
        }
        if let Some(perfmap) = &self.perfmap {
            cmd.push(ARG_PERFMAP.to_string());
            cmd.push(perfmap.display().to_string());
        }
        if let Some(jitdump) = &self.jitdump {
            cmd.push(ARG_JITDUMP.to_string());
            cmd.push(jitdump.display().to_string());
        }
        if let Some(objects) = &self.object {
            for object in objects {
                cmd.append(&mut object.to_command());
            }
        }
        cmd
    }
}

pub type QemuInstanceForX86_64 = QemuInstanceBase<MachineX86_64, CpuX86>;
pub type QemuInstanceForAarch64 = QemuInstanceBase<MachineAarch64, CpuAarch64>;

pub struct Xuuid {}

impl TypedUuidKind for Xuuid {
    fn tag() -> TypedUuidTag {
        // Tags are required to be ASCII identifiers, with underscores
        // and dashes also supported. The validity of a tag can be checked
        // at compile time by assigning it to a const, like so:
        const TAG: TypedUuidTag = TypedUuidTag::new("my_kind");
        TAG
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct XDateTime(DateTime<Utc>);

impl Arbitrary for XDateTime {
    type Parameters = ();
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        let dt = Utc.with_ymd_and_hms(1984, 10, 22, 0, 1, 5).unwrap();
        Just(XDateTime(dt))
    }

    type Strategy = proptest::strategy::Just<Self>;
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder)]
pub struct Ipv4Net {
    ip: ipnet::Ipv4Net,
}

impl Arbitrary for Ipv4Net {
    type Parameters = ();
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        todo!()
    }

    type Strategy = proptest::strategy::Just<Self>;
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder)]
pub struct Ipv6Net {
    ip: ipnet::Ipv6Net,
}

impl Arbitrary for Ipv6Net {
    type Parameters = ();
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        todo!()
    }

    type Strategy = proptest::strategy::Just<Self>;
}
