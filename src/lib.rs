#![recursion_limit = "256"]

pub mod args;
pub mod common;
pub mod parser;
pub mod parsers;
pub mod shell_path;
pub mod shell_string;
pub mod to_command;

use std::path::PathBuf;
use std::str::FromStr;

use bon::Builder;
use chrono::{DateTime, TimeZone, Utc};
use newtype_uuid::{TypedUuidKind, TypedUuidTag};
use proptest::prelude::{Arbitrary, Just};
use proptest_derive::Arbitrary;

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
use crate::args::cpu::{CpuAarch64, CpuX86};
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
use crate::args::machine::{MachineAarch64, MachineX86_64};
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
use crate::parser::parse_qemu_command_line;
use crate::parsers::{
    ARG_APPEND, ARG_BIG_D, ARG_BIG_S, ARG_BIOS, ARG_CDROM, ARG_DAEMONIZE, ARG_DEBUGCON, ARG_DFILTER, ARG_DTB, ARG_DUMP_VMSTATE, ARG_ECHR, ARG_ENABLE_KVM, ARG_ENABLE_SYNC_PROFILE, ARG_FDA, ARG_FDB,
    ARG_FULL_SCREEN, ARG_GDB, ARG_HDA, ARG_HDB, ARG_HDC, ARG_HDD, ARG_INITRD, ARG_JITDUMP, ARG_K, ARG_KERNEL, ARG_L, ARG_LITTLE_D, ARG_LITTLE_S, ARG_LOADVM, ARG_MEM_PATH, ARG_MEM_PREALLOC,
    ARG_MONITOR, ARG_MTDBLOCK, ARG_NO_FD_BOOTCHK, ARG_NO_REBOOT, ARG_NO_SHUTDOWN, ARG_NO_USER_CONFIG, ARG_NODEFAULTS, ARG_NOGRAPHIC, ARG_ONLY_MIGRATABLE, ARG_OPTION_ROM, ARG_PARALLEL, ARG_PERFMAP,
    ARG_PFLASH, ARG_PIDFILE, ARG_PRECONFIG, ARG_PROM_ENV, ARG_QMP, ARG_QMP_PRETTY, ARG_QTEST, ARG_QTEST_LOG, ARG_READCONFIG, ARG_SD, ARG_SEED, ARG_SEMIHOSTING, ARG_SERIAL, ARG_SHIM, ARG_SNAPSHOT,
    ARG_USB, ARG_UUID, ARG_WIN2K_HACK, ARG_XEN_ATTACH, ARG_XEN_DOMID, ARG_XEN_DOMID_RESTRICT, DELIM_COMMA,
};
use crate::parsers::{ARG_CHROOT, ARG_RUNAS};
use crate::parsers::{ARG_MIGRATE_MODE_ENABLE, ARG_NO_HPET, ARG_ONLY_CPR_CAPABLE};
use crate::shell_string::ShellString;
use crate::to_command::{ToArg, ToCommand};

/// QEMU binary name for x86_64
const QEMU_BIN_X86_64: &str = "qemu-system-x86_64";

/// QEMU binary name for aarch64
const QEMU_BIN_AARCH64: &str = "qemu-system-aarch64";

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct QemuInstanceBase<Machine, Cpu> {
    pub qemu_binary: PathBuf,

    pub machine: Option<Machine>,
    pub cpu: Option<Cpu>,

    pub accel: Option<Accel>,
    pub smp: Option<SMP>,
    pub numa: Option<Vec<NUMA>>,
    pub add_fd: Option<Vec<AddFd>>,
    pub set: Option<Vec<Set>>,
    pub global: Option<Vec<Global>>,
    pub boot: Option<Boot>,
    pub m: Option<Memory>,
    pub mem_path: Option<PathBuf>,
    pub mem_prealloc: Option<bool>,
    pub k: Option<String>,
    pub audio: Option<Audio>,
    pub audiodev: Option<Vec<AudioDev>>,
    pub device: Option<Vec<Device>>,
    pub name: Option<Name>,
    pub uuid: Option<newtype_uuid::TypedUuid<QUuid>>,
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
    pub fsdev: Option<Vec<FsDev>>,
    pub virtfs: Option<Vec<Virtfs>>,
    pub iscsi: Option<Iscsi>,
    pub usb: Option<bool>,
    pub usbdevice: Option<USBDevice>,
    pub display: Option<QemuDisplay>,
    pub nographic: Option<bool>,
    pub spice: Option<Spice>,
    pub vga: Option<VGA>,
    pub full_screen: Option<bool>,
    pub g: Option<G>,
    pub vnc: Option<VNC>,
    pub win2k_hack: Option<bool>,
    pub no_fd_bootchk: Option<bool>,
    pub acpitable: Option<Vec<AcpiTable>>,
    pub smbios: Option<Vec<Smbios>>,
    pub netdev: Option<Vec<NetDev>>,
    pub nic: Option<Vec<Nic>>,
    pub net: Option<Vec<LegacyNet>>,
    pub chardev: Option<Vec<CharDev>>,
    pub tpmdev: Option<Vec<TpmDev>>,
    pub bios: Option<PathBuf>,
    pub pflash: Option<PathBuf>,
    pub kernel: Option<PathBuf>,
    pub shim: Option<PathBuf>,
    pub append: Option<ShellString>,
    pub initrd: Option<PathBuf>,
    pub dtb: Option<PathBuf>,
    pub compact: Option<Compact>,
    pub fw_cfg: Option<Vec<FwCfg>>,
    pub serial: Option<Vec<SpecialDevice>>,
    pub parallel: Option<Vec<SpecialDevice>>,
    pub monitor: Option<Vec<SpecialDevice>>,
    pub qmp: Option<Vec<SpecialDevice>>,
    pub qmp_pretty: Option<Vec<SpecialDevice>>,
    pub mon: Option<Vec<Mon>>,
    pub debugcon: Option<SpecialDevice>,
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
    pub xen_domid: Option<String>,
    pub xen_attach: Option<bool>,
    pub xen_domid_restrict: Option<bool>,
    pub no_reboot: Option<bool>,
    pub no_hpet: Option<bool>,
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
    pub only_cpr_capable: Option<bool>,
    pub migrate_mode_enable: Option<String>,
    pub nodefaults: Option<bool>,
    pub sandbox: Option<Sandbox>,
    pub readconfig: Option<PathBuf>,
    pub no_user_config: Option<bool>,
    pub trace: Option<Trace>,
    pub plugin: Option<Vec<Plugin>>,
    pub semihosting: Option<bool>,
    pub semihosting_config: Option<SemihostingConfig>,
    pub qtest: Option<String>,
    pub qtest_log: Option<PathBuf>,
    pub prom_env: Option<Vec<String>>,
    pub run_with: Option<RunWith>,
    // compat w/ qemu 7.x+
    pub runas: Option<String>,
    // compat w/ qemu 7.x+
    pub chroot: Option<PathBuf>,
    pub msg: Option<Msg>,
    pub dump_vmstate: Option<PathBuf>,
    pub enable_sync_profile: Option<bool>,
    pub perfmap: Option<PathBuf>,
    pub jitdump: Option<PathBuf>,
    pub object: Option<Vec<Object>>,
}

impl<Machine: ToCommand, Cpu: ToCommand> ToCommand for QemuInstanceBase<Machine, Cpu>
where
    QemuInstanceBase<Machine, Cpu>: FromStr,
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
        if let Some(add_fds) = &self.add_fd {
            for add_fd in add_fds {
                cmd.append(&mut add_fd.to_command());
            }
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
        if let Some(audiodevs) = &self.audiodev {
            for audiodev in audiodevs {
                cmd.append(&mut audiodev.to_command());
            }
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
        if let Some(fsdevs) = &self.fsdev {
            for fsdev in fsdevs {
                cmd.append(&mut fsdev.to_command());
            }
        }
        if let Some(virtfses) = &self.virtfs {
            for virtfs in virtfses {
                cmd.append(&mut virtfs.to_command());
            }
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
        if let Some(g) = &self.g {
            cmd.append(&mut g.to_command());
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
        if let Some(acpitables) = &self.acpitable {
            for acpitable in acpitables {
                cmd.append(&mut acpitable.to_command());
            }
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
        if let Some(nics) = &self.nic {
            for nic in nics {
                cmd.append(&mut nic.to_command());
            }
        }
        if let Some(nets) = &self.net {
            for net in nets {
                cmd.append(&mut net.to_command());
            }
        }
        if let Some(chardevs) = &self.chardev {
            for chardev in chardevs {
                cmd.append(&mut chardev.to_command());
            }
        }
        if let Some(tpmdevs) = &self.tpmdev {
            for tpmdev in tpmdevs {
                cmd.append(&mut tpmdev.to_command());
            }
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
            cmd.push(append.as_ref().to_string());
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
        if let Some(fw_cfgs) = &self.fw_cfg {
            for fw_cfg in fw_cfgs {
                cmd.append(&mut fw_cfg.to_command());
            }
        }
        if let Some(serials) = &self.serial {
            for serial in serials {
                cmd.push(ARG_SERIAL.to_string());
                cmd.append(&mut serial.to_args());
            }
        }
        if let Some(parallels) = &self.parallel {
            for parallel in parallels {
                cmd.push(ARG_PARALLEL.to_string());
                cmd.append(&mut parallel.to_args());
            }
        }
        if let Some(monitors) = &self.monitor {
            for monitor in monitors {
                cmd.push(ARG_MONITOR.to_string());
                cmd.append(&mut monitor.to_args());
            }
        }
        if let Some(qmps) = &self.qmp {
            for qmp in qmps {
                cmd.push(ARG_QMP.to_string());
                cmd.append(&mut qmp.to_args());
            }
        }
        if let Some(qmp_pretties) = &self.qmp_pretty {
            for qmp_pretty in qmp_pretties {
                cmd.push(ARG_QMP_PRETTY.to_string());
                cmd.append(&mut qmp_pretty.to_args());
            }
        }
        if let Some(mons) = &self.mon {
            for mon in mons {
                cmd.append(&mut mon.to_command());
            }
        }
        if let Some(debugcon) = &self.debugcon {
            cmd.push(ARG_DEBUGCON.to_string());
            cmd.append(&mut debugcon.to_args());
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
            cmd.append(&mut gdb.to_args());
        }
        if let Some(s) = &self.s
            && *s
        {
            cmd.push(ARG_LITTLE_S.to_string());
        }
        if let Some(d) = &self.d {
            cmd.push(ARG_LITTLE_D.to_string());
            cmd.push(d.join(DELIM_COMMA));
        }
        if let Some(big_d) = &self.big_d {
            cmd.push(ARG_BIG_D.to_string());
            cmd.push(big_d.display().to_string());
        }
        if let Some(dfilter) = &self.dfilter {
            cmd.push(ARG_DFILTER.to_string());
            cmd.push(dfilter.join(DELIM_COMMA));
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
        if let Some(xen_domid) = &self.xen_domid {
            cmd.push(ARG_XEN_DOMID.to_string());
            cmd.push(xen_domid.to_string());
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
        if self.no_hpet == Some(true) {
            cmd.push(ARG_NO_HPET.to_string());
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
        if self.only_cpr_capable == Some(true) {
            cmd.push(ARG_ONLY_CPR_CAPABLE.to_string());
        }
        if let Some(mode) = &self.migrate_mode_enable {
            cmd.push(ARG_MIGRATE_MODE_ENABLE.to_string());
            cmd.push(mode.clone());
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
        if let Some(plugins) = &self.plugin {
            for plugin in plugins {
                cmd.append(&mut plugin.to_command());
            }
        }
        if self.semihosting == Some(true) {
            cmd.push(ARG_SEMIHOSTING.to_string());
        }
        if let Some(config) = &self.semihosting_config {
            cmd.append(&mut config.to_command());
        }
        if let Some(qtest) = &self.qtest {
            cmd.push(ARG_QTEST.to_string());
            cmd.push(qtest.clone());
        }
        if let Some(qtest_log) = &self.qtest_log {
            cmd.push(ARG_QTEST_LOG.to_string());
            cmd.push(qtest_log.display().to_string());
        }
        if let Some(prom_envs) = &self.prom_env {
            for prom_env in prom_envs {
                cmd.push(ARG_PROM_ENV.to_string());
                cmd.push(prom_env.clone());
            }
        }
        if let Some(run_with) = &self.run_with {
            cmd.append(&mut run_with.to_command());
        }
        if let Some(runas) = &self.runas {
            cmd.push(ARG_RUNAS.to_string());
            cmd.push(runas.to_string());
        }
        if let Some(chroot) = &self.chroot {
            cmd.push(ARG_CHROOT.to_string());
            cmd.push(chroot.display().to_string());
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

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum QemuCommand {
    X86_64(QemuInstanceBase<MachineX86_64, CpuX86>),
    Aarch64(QemuInstanceBase<MachineAarch64, CpuAarch64>),
}

impl ToCommand for QemuCommand {
    fn to_args(&self) -> Vec<String> {
        match self {
            QemuCommand::X86_64(instance) => instance.to_args(),
            QemuCommand::Aarch64(instance) => instance.to_args(),
        }
    }

    fn to_command(&self) -> Vec<String> {
        match self {
            QemuCommand::X86_64(instance) => instance.to_command(),
            QemuCommand::Aarch64(instance) => instance.to_command(),
        }
    }
}

impl FromStr for QemuCommand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_qemu_command_line(s)
    }
}

pub struct QUuid {}

impl TypedUuidKind for QUuid {
    fn tag() -> TypedUuidTag {
        // Tags are required to be ASCII identifiers, with underscores
        // and dashes also supported. The validity of a tag can be checked
        // at compile time by assigning it to a const, like so:
        const TAG: TypedUuidTag = TypedUuidTag::new("my_kind");
        TAG
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct QDateTime(DateTime<Utc>);

impl Arbitrary for QDateTime {
    type Parameters = ();
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        let dt = Utc.with_ymd_and_hms(1984, 10, 22, 0, 1, 5).unwrap();
        Just(QDateTime(dt))
    }

    type Strategy = proptest::strategy::Just<Self>;
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder)]
pub struct QIpv4Net {
    ip: ipnet::Ipv4Net,
}

impl Arbitrary for QIpv4Net {
    type Parameters = ();
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        todo!()
    }

    type Strategy = proptest::strategy::Just<Self>;
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder)]
pub struct QIpv6Net {
    ip: ipnet::Ipv6Net,
}

impl Arbitrary for QIpv6Net {
    type Parameters = ();
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        todo!()
    }

    type Strategy = proptest::strategy::Just<Self>;
}
