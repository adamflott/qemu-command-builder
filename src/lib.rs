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
pub mod plugin;
pub mod rtc;
pub mod runwith;
pub mod sandbox;
pub mod serial;
pub mod set;
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

use std::path::PathBuf;

use bon::Builder;

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
use crate::cpu::CpuX86;
use crate::device::Device;
use crate::display::QemuDisplay;
use crate::drive::Drive;
use crate::fsdev::FsDev;
use crate::fw_cfg::FwCfg;
use crate::global::Global;
use crate::icount::Icount;
use crate::incoming::Incoming;
use crate::iscsi::Iscsi;
use crate::machine::MachineForX86;
use crate::memory::Memory;
use crate::mon::Mon;
use crate::msg::Msg;
use crate::name::Name;
use crate::netdev::NetDev;
use crate::numa::NUMA;
use crate::object::Object;
use crate::overcommit::Overcommit;
use crate::plugin::Plugin;
use crate::rtc::Rtc;
use crate::runwith::RunWith;
use crate::sandbox::Sandbox;
use crate::serial::SpecialDevice;
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

#[derive(Builder)]
pub struct QemuInstanceForX86_64 {
    pub qemu_binary: PathBuf,

    pub machine: Option<MachineForX86>,
    pub cpu: Option<CpuX86>,
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
    pub uuid: Option<String>,
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
    pub append: Option<String>,
    pub initrd: Option<String>,
    pub dtb: Option<PathBuf>,
    pub compact: Option<Compact>,
    pub fw_cfg: Option<FwCfg>,
    pub serial: Option<SpecialDevice>,
    pub parallel: Option<Vec<SpecialDevice>>,
    pub monitor: Option<SpecialDevice>,
    pub qmp: Option<SpecialDevice>,
    pub qmp_pretty: Option<SpecialDevice>,
    pub mon: Option<Mon>,
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

impl ToCommand for QemuInstanceForX86_64 {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![];

        cmd.push(self.qemu_binary.display().to_string());

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
            cmd.push("-mem-path".to_string());
            cmd.push(path.display().to_string());
        }
        if let Some(state) = self.mem_prealloc
            && state
        {
            cmd.push("-mem-prealloc".to_string());
        }
        if let Some(lang) = &self.k {
            cmd.push("-k".to_string());
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
            cmd.push("-uuid".to_string());
            cmd.push(uuid.to_string());
        }
        if let Some(fda) = &self.fda {
            cmd.push("-fda".to_string());
            cmd.push(fda.display().to_string());
        }
        if let Some(fdb) = &self.fdb {
            cmd.push("-fdb".to_string());
            cmd.push(fdb.display().to_string());
        }
        if let Some(hda) = &self.hda {
            cmd.push("-hda".to_string());
            cmd.push(hda.display().to_string());
        }
        if let Some(hdb) = &self.hdb {
            cmd.push("-hdb".to_string());
            cmd.push(hdb.display().to_string());
        }
        if let Some(hdc) = &self.hdc {
            cmd.push("-hdc".to_string());
            cmd.push(hdc.display().to_string());
        }
        if let Some(hdd) = &self.hdd {
            cmd.push("-hdd".to_string());
            cmd.push(hdd.display().to_string());
        }
        if let Some(cdrom) = &self.cdrom {
            cmd.push("-cdrom".to_string());
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
            cmd.push("-mtdblock".to_string());
            cmd.push(mdtblock.display().to_string());
        }
        if let Some(sd) = &self.sd {
            cmd.push("-sd".to_string());
            cmd.push(sd.display().to_string());
        }
        if let Some(state) = &self.snapshot
            && *state
        {
            cmd.push("-snapshot".to_string());
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
            cmd.push("-usb".to_string());
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
            cmd.push("-nographic".to_string());
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
            cmd.push("-full-screen".to_string());
        }
        if let Some((w, h, d)) = &self.g {
            cmd.push("-g".to_string());
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
            cmd.push("-win2k-hack".to_string());
        }
        if let Some(no_fd_bootchk) = &self.no_fd_bootchk
            && *no_fd_bootchk
        {
            cmd.push("-no-fd-bootchk".to_string());
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
            cmd.push("-bios".to_string());
            cmd.push(bios.display().to_string());
        }
        if let Some(pflash) = &self.pflash {
            cmd.push("-pflash".to_string());
            cmd.push(pflash.display().to_string());
        }
        if let Some(kernel) = &self.kernel {
            cmd.push("-kernel".to_string());
            cmd.push(kernel.display().to_string());
        }
        if let Some(shim) = &self.shim {
            cmd.push("-shim".to_string());
            cmd.push(shim.display().to_string());
        }
        if let Some(append) = &self.append {
            cmd.push("-append".to_string());
            cmd.push(append.clone());
        }
        if let Some(initrd) = &self.initrd {
            cmd.push("-initrd".to_string());
            cmd.push(initrd.to_string());
        }
        if let Some(dtb) = &self.dtb {
            cmd.push("-dtb".to_string());
            cmd.push(dtb.display().to_string());
        }
        if let Some(compact) = &self.compact {
            cmd.append(&mut compact.to_command());
        }
        if let Some(fw_cfg) = &self.fw_cfg {
            cmd.append(&mut fw_cfg.to_command());
        }
        if let Some(serial) = &self.serial {
            cmd.push("-serial".to_string());
            cmd.append(&mut serial.to_command());
        }
        if let Some(parallels) = &self.parallel {
            for parallel in parallels {
                cmd.push("-parallel".to_string());
                cmd.append(&mut parallel.to_command());
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
        if let Some(mon) = &self.mon {
            cmd.append(&mut mon.to_command());
        }
        if let Some(debugcon) = &self.debugcon {
            cmd.push("-debugcon".to_string());
            cmd.append(&mut debugcon.to_command());
        }
        if let Some(pidfile) = &self.pidfile {
            cmd.push("-pidfile".to_string());
            cmd.push(pidfile.display().to_string());
        }
        if let Some(preconfig) = &self.preconfig
            && *preconfig
        {
            cmd.push("--preconfig".to_string());
        }
        if let Some(s) = &self.big_s
            && *s
        {
            cmd.push("-S".to_string());
        }
        if let Some(overcommit) = &self.overcommit {
            cmd.append(&mut overcommit.to_command());
        }
        if let Some(gdb) = &self.gdb {
            cmd.push("-gdb".to_string());
            cmd.append(&mut gdb.to_command());
        }
        if let Some(s) = &self.s
            && *s
        {
            cmd.push("-s".to_string());
        }
        if let Some(d) = &self.d {
            cmd.push("-d".to_string());
            cmd.push(d.join(","));
        }
        if let Some(big_d) = &self.big_d {
            cmd.push("-D".to_string());
            cmd.push(big_d.display().to_string());
        }
        if let Some(dfilter) = &self.dfilter {
            cmd.push("-dfilter".to_string());
            cmd.push(dfilter.join(","));
        }
        if let Some(seed) = &self.seed {
            cmd.push("-seed".to_string());
            cmd.push(seed.to_string());
        }
        if let Some(seed) = &self.seed {
            cmd.push("-seed".to_string());
            cmd.push(seed.to_string());
        }
        if let Some(l) = &self.big_l {
            cmd.push("-L".to_string());
            cmd.push(l.display().to_string());
        }
        if let Some(enable_kvm) = &self.enable_kvm
            && *enable_kvm
        {
            cmd.push("-enable-kvm".to_string());
        }
        if let Some(xen_id) = &self.xen_id {
            cmd.push("-xen-id".to_string());
            cmd.push(xen_id.to_string());
        }
        if let Some(xen_attach) = &self.xen_attach
            && *xen_attach
        {
            cmd.push("-xen-attach".to_string());
        }
        if let Some(xen_domid_restrict) = &self.xen_domid_restrict
            && *xen_domid_restrict
        {
            cmd.push("-xen-domid-restrict".to_string());
        }
        if let Some(no_reboot) = &self.no_reboot
            && *no_reboot
        {
            cmd.push("-no-reboot".to_string());
        }
        if let Some(no_shutdown) = &self.no_shutdown
            && *no_shutdown
        {
            cmd.push("-no-shutdown".to_string());
        }
        if let Some(action) = &self.action {
            cmd.append(&mut action.to_command());
        }
        if let Some(loadvm) = &self.loadvm {
            cmd.push("-loadvm".to_string());
            cmd.push(loadvm.to_string());
        }
        if let Some(daemonize) = &self.daemonize
            && *daemonize
        {
            cmd.push("-daemonize".to_string());
        }
        if let Some(option_rom) = &self.option_rom {
            cmd.push("-option-rom".to_string());
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
            cmd.push("-echr".to_string());
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
            cmd.push("-only-migratable".to_string());
        }
        if let Some(nodefaults) = &self.nodefaults
            && *nodefaults
        {
            cmd.push("-nodefaults".to_string());
        }
        if let Some(sandbox) = &self.sandbox {
            cmd.append(&mut sandbox.to_command());
        }
        if let Some(readconfig) = &self.readconfig {
            cmd.push("-readconfig".to_string());
            cmd.push(readconfig.display().to_string());
        }
        if let Some(no_user_config) = &self.no_user_config
            && *no_user_config
        {
            cmd.push("-no-user-config".to_string());
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
            cmd.push("-dump-vmstate".to_string());
            cmd.push(dump_vmstate.display().to_string());
        }
        if let Some(enable_sync_profile) = &self.enable_sync_profile
            && *enable_sync_profile
        {
            cmd.push("-enable-sync-profile".to_string());
        }
        if let Some(perfmap) = &self.perfmap {
            cmd.push("-perfmap".to_string());
            cmd.push(perfmap.display().to_string());
        }
        if let Some(jitdump) = &self.jitdump {
            cmd.push("-jitdump".to_string());
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
