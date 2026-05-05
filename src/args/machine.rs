use crate::parsers::ARG_MACHINE;
use std::str::FromStr;

use bon::Builder;
use proptest_derive::Arbitrary;

use crate::args::machine_type::{MachineTypeAarch64, MachineTypeX86_64};
use crate::common::*;
use crate::parsers::{DELIM_COLON, DELIM_COMMA};
use crate::qao;
use crate::shell_string::{ShellString, ShellStringError};
use crate::to_command::{ToArg, ToCommand};

const KEY_ACCEL: &str = "accel=";
const KEY_TYPE: &str = "type=";
const KEY_VMPORT: &str = "vmport=";
const KEY_DUMP_GUEST_CORE: &str = "dump-guest-core=";
const KEY_MEM_MERGE: &str = "mem-merge=";
const KEY_AES_KEY_WRAP: &str = "aes-key-wrap=";
const KEY_DEA_KEY_WRAP: &str = "dea-key-wrap=";
const KEY_NVDIMM: &str = "nvdimm=";
const KEY_MEMORY_ENCRYPTION: &str = "memory-encryption=";
const KEY_HMAT: &str = "hmat=";
const KEY_AUX_RAM_SHARE: &str = "aux-ram-share=";
const KEY_MEMORY_BACKEND: &str = "memory-backend=";

/// Supported `interleave-granularity=` values for `cxl-fmw`.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Arbitrary)]
pub enum Granularity {
    #[default]
    G256,
    G512,
    G1k,
    G2k,
    G4k,
    G8k,
    G16k,
}

impl ToArg for Granularity {
    fn to_arg(&self) -> &str {
        match self {
            Granularity::G256 => "256",
            Granularity::G512 => "512",
            Granularity::G1k => "1k",
            Granularity::G2k => "2k",
            Granularity::G4k => "4k",
            Granularity::G8k => "8k",
            Granularity::G16k => "16k",
        }
    }
}
/// A CXL fixed memory window definition for `-machine`.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct CxlFmw {
    targets: Vec<String>,
    size: String,
    interleave_granularity: Option<Granularity>,
}

/// Cache topology properties for `-machine smp-cache.*`.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct SmpCache {
    cache: String,
    topology: String,
}

/// Architecture-specific machine types accepted by this crate.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum MachineType {
    X86_64(MachineTypeX86_64),
    Aarch(MachineTypeAarch64),
}

impl ToArg for MachineType {
    fn to_arg(&self) -> &str {
        match self {
            MachineType::X86_64(mt) => mt.to_arg(),
            MachineType::Aarch(mt) => mt.to_arg(),
        }
    }
}
/// Select the emulated machine by name. Use ``-machine help`` to list
/// available machines.
///
/// For architectures which aim to support live migration compatibility
/// across releases, each release will introduce a new versioned machine
/// type. For example, the 2.8.0 release introduced machine types
/// "pc-i440fx-2.8" and "pc-q35-2.8" for the x86\_64/i686 architectures.
///
/// To allow live migration of guests from QEMU version 2.8.0, to QEMU
/// version 2.9.0, the 2.9.0 version must support the "pc-i440fx-2.8"
/// and "pc-q35-2.8" machines too. To allow users live migrating VMs to
/// skip multiple intermediate releases when upgrading, new releases of
/// QEMU will support machine types from many previous versions.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct Machine<T> {
    /// The QEMU machine type name.
    machine_type: T,

    /// This is used to enable an accelerator. Depending on the target
    /// architecture, kvm, xen, hvf, nvmm, whpx or tcg can be available.
    /// By default, tcg is used. If there is more than one accelerator
    /// specified, the next one is used if the previous one fails to
    /// initialize.
    accel: Option<Vec<AccelType>>,

    /// Enables emulation of VMWare IO port, for vmmouse etc. auto says
    //  to select the value based on accel and i8042. For accel=xen or
    //  i8042=off the default is off otherwise the default is on.
    vmport: Option<OnOffAuto>,

    /// Include guest memory in a core dump. The default is on.
    dump_guest_core: Option<OnOffDefaultOn>,

    /// Enables or disables memory merge support. This feature, when
    //  supported by the host, de-duplicates identical memory pages
    //  among VMs instances (enabled by default).
    mem_merge: Option<OnOffDefaultOn>,

    /// Enables or disables AES key wrapping support on s390-ccw hosts.
    /// This feature controls whether AES wrapping keys will be created
    /// to allow execution of AES cryptographic functions. The default
    /// is on.
    aes_key_wrap: Option<OnOffDefaultOn>,

    /// Enables or disables DEA key wrapping support on s390-ccw hosts.
    /// This feature controls whether DEA wrapping keys will be created
    /// to allow execution of DEA cryptographic functions. The default
    /// is on.
    dea_key_wrap: Option<OnOffDefaultOn>,

    /// Enables or disables NVDIMM support. The default is off.
    nvdimm: Option<OnOffDefaultOff>,

    /// Memory encryption object to use. The default is none.
    memory_encryption: Option<ShellString>, // TODO find out actual values

    /// Enables or disables ACPI Heterogeneous Memory Attribute Table
    /// (HMAT) support. The default is off.
    hmat: Option<OnOffDefaultOff>,

    /// Allocate auxiliary guest RAM as an anonymous file that is
    /// shareable with an external process.  This option applies to
    /// memory allocated as a side effect of creating various devices.
    /// It does not apply to memory-backend-objects, whether explicitly
    /// specified on the command line, or implicitly created by the -m
    /// command line option.  The default is off.
    aux_ram_share: Option<OnOffDefaultOff>,

    /// An alternative to legacy ``-mem-path`` and ``mem-prealloc`` options.
    /// Allows to use a memory backend as main RAM.
    memory_backend: Option<ShellString>, // TODO find out actual values
                                         /*
                                           /// Define a CXL Fixed Memory Window (CFMW).
                                           ///
                                           /// Described in the CXL 2.0 ECN: CEDT CFMWS & QTG _DSM.
                                           ///
                                           /// They are regions of Host Physical Addresses (HPA) on a system which
                                           /// may be interleaved across one or more CXL host bridges.  The system
                                           /// software will assign particular devices into these windows and
                                           /// configure the downstream Host-managed Device Memory (HDM) decoders
                                           /// in root ports, switch ports and devices appropriately to meet the
                                           /// interleave requirements before enabling the memory devices.
                                           ///
                                           /// ``targets.X=target`` provides the mapping to CXL host bridges
                                           /// which may be identified by the id provided in the -device entry.
                                           /// Multiple entries are needed to specify all the targets when
                                           /// the fixed memory window represents interleaved memory. X is the
                                           /// target index from 0.
                                           ///
                                           /// ``size=size`` sets the size of the CFMW. This must be a multiple of
                                           /// 256MiB. The region will be aligned to 256MiB but the location is
                                           /// platform and configuration dependent.
                                           ///
                                           /// ``interleave-granularity=granularity`` sets the granularity of
                                           /// interleave. Default 256 (bytes). Only 256, 512, 1k, 2k,
                                           /// 4k, 8k and 16k granularities supported.
                                         // TOOD  cxl_fmw: Option<CxlFmw>,

                                           /// Define cache properties for SMP system.
                                           ///
                                           /// ``cache=cachename`` specifies the cache that the properties will be
                                           /// applied on. This field is the combination of cache level and cache
                                           /// type. It supports ``l1d`` (L1 data cache), ``l1i`` (L1 instruction
                                           /// cache), ``l2`` (L2 unified cache) and ``l3`` (L3 unified cache).
                                           ///
                                           /// ``topology=topologylevel`` sets the cache topology level. It accepts
                                           /// CPU topology levels including ``core``, ``module``, ``cluster``, ``die``,
                                           /// ``socket``, ``book``, ``drawer`` and a special value ``default``. If
                                           /// ``default`` is set, then the cache topology will follow the architecture's
                                           /// default cache topology model. If another topology level is set, the cache
                                           /// will be shared at corresponding CPU topology level. For example,
                                           /// ``topology=core`` makes the cache shared by all threads within a core.
                                           /// The omitting cache will default to using the ``default`` level.
                                           ///
                                           /// The default cache topology model for an i386 PC machine is as follows:
                                           /// ``l1d``, ``l1i``, and ``l2`` caches are per ``core``, while the ``l3``
                                           /// cache is per ``die``.
                                           */
                                         // TODO   smp_cache: Option<Vec<SmpCache>>,
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct MachineX86_64 {
    /// The x86_64-specific `-machine` payload.
    pub m: Machine<MachineTypeX86_64>,
}

impl ToCommand for MachineX86_64 {
    fn command(&self) -> String {
        ARG_MACHINE.to_string()
    }

    fn to_args(&self) -> Vec<String> {
        let mut args = vec![self.m.machine_type.to_arg().to_string()];

        if let Some(accels) = &self.m.accel {
            let accel_strs: Vec<&str> = accels.iter().map(|a| a.to_arg()).collect();
            args.push(format!("{}{}", KEY_ACCEL, accel_strs.join(":")));
        }
        qao!(&self.m.vmport, args, KEY_VMPORT);
        qao!(&self.m.dump_guest_core, args, KEY_DUMP_GUEST_CORE);
        qao!(&self.m.mem_merge, args, KEY_MEM_MERGE);
        qao!(&self.m.aes_key_wrap, args, KEY_AES_KEY_WRAP);
        qao!(&self.m.dea_key_wrap, args, KEY_DEA_KEY_WRAP);
        qao!(&self.m.nvdimm, args, KEY_NVDIMM);
        if let Some(memory_encryption) = &self.m.memory_encryption {
            args.push(format!("{}{}", KEY_MEMORY_ENCRYPTION, memory_encryption.as_ref()));
        }
        qao!(&self.m.hmat, args, KEY_HMAT);
        qao!(&self.m.aux_ram_share, args, KEY_AUX_RAM_SHARE);
        if let Some(memory_backend) = &self.m.memory_backend {
            args.push(format!("{}{}", KEY_MEMORY_BACKEND, memory_backend.as_ref()));
        }

        /*
        if let Some(cxl_fmw) = &self.m.cxl_fmw {
            for (idx, target) in cxl_fmw.targets.iter().enumerate() {
                args.push(format!("cxl-fmw.0.targets.{}={}", idx, target));
            }
            args.push(format!("cxl-fmw.0.size={}", cxl_fmw.size));
            if let Some(granularity) = &cxl_fmw.interleave_granularity {
                args.push(format!("cxl-fmw.0.interleave-granularity={}", granularity.to_arg()));
            }
        }
        if let Some(smp_caches) = &self.m.smp_cache {
            for (idx, smp_cache) in smp_caches.iter().enumerate() {
                args.push(format!("smp-cache.{}.cache={}", idx, smp_cache.cache));
                args.push(format!("smp-cache.{}.topology={}", idx, smp_cache.topology));
            }
        }
         */
        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for MachineX86_64 {
    type Err = ShellStringError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_machine_x86_64(s).map_err(ShellStringError::new)
    }
}

fn parse_machine_x86_64(s: &str) -> Result<MachineX86_64, String> {
    let mut parts = s.split(DELIM_COMMA);
    let first = parts.next().ok_or_else(|| "empty machine argument".to_string())?;

    let mut machine_type = None;
    let mut accel = None;
    let mut vmport = None;
    let mut dump_guest_core = None;
    let mut mem_merge = None;
    let mut aes_key_wrap = None;
    let mut dea_key_wrap = None;
    let mut nvdimm = None;
    let mut memory_encryption = None;
    let mut hmat = None;
    let mut aux_ram_share = None;
    let mut memory_backend = None;

    if let Some(value) = first.strip_prefix(KEY_TYPE) {
        machine_type = Some(parse_machine_type(value)?);
    } else if first.contains('=') {
        parse_machine_option(
            first,
            &mut machine_type,
            &mut accel,
            &mut vmport,
            &mut dump_guest_core,
            &mut mem_merge,
            &mut aes_key_wrap,
            &mut dea_key_wrap,
            &mut nvdimm,
            &mut memory_encryption,
            &mut hmat,
            &mut aux_ram_share,
            &mut memory_backend,
        )?;
    } else {
        machine_type = Some(parse_machine_type(first)?);
    }

    for part in parts {
        parse_machine_option(
            part,
            &mut machine_type,
            &mut accel,
            &mut vmport,
            &mut dump_guest_core,
            &mut mem_merge,
            &mut aes_key_wrap,
            &mut dea_key_wrap,
            &mut nvdimm,
            &mut memory_encryption,
            &mut hmat,
            &mut aux_ram_share,
            &mut memory_backend,
        )?;
    }

    let machine_type = machine_type.ok_or_else(|| "machine type is required".to_string())?;

    Ok(MachineX86_64 {
        m: Machine {
            machine_type,
            accel,
            vmport,
            dump_guest_core,
            mem_merge,
            aes_key_wrap,
            dea_key_wrap,
            nvdimm,
            memory_encryption,
            hmat,
            aux_ram_share,
            memory_backend,
        },
    })
}

#[allow(clippy::too_many_arguments)]
fn parse_machine_option(
    part: &str,
    machine_type: &mut Option<MachineTypeX86_64>,
    accel: &mut Option<Vec<AccelType>>,
    vmport: &mut Option<OnOffAuto>,
    dump_guest_core: &mut Option<OnOffDefaultOn>,
    mem_merge: &mut Option<OnOffDefaultOn>,
    aes_key_wrap: &mut Option<OnOffDefaultOn>,
    dea_key_wrap: &mut Option<OnOffDefaultOn>,
    nvdimm: &mut Option<OnOffDefaultOff>,
    memory_encryption: &mut Option<ShellString>,
    hmat: &mut Option<OnOffDefaultOff>,
    aux_ram_share: &mut Option<OnOffDefaultOff>,
    memory_backend: &mut Option<ShellString>,
) -> Result<(), String> {
    let (key, value) = part.split_once('=').ok_or_else(|| format!("invalid machine option: {part}"))?;
    match key {
        "type" => *machine_type = Some(parse_machine_type(value)?),
        "accel" => {
            let accels = value
                .split(DELIM_COLON)
                .map(|v| v.parse::<AccelType>().map_err(|_| format!("invalid accel value: {v}")))
                .collect::<Result<Vec<_>, _>>()?;
            *accel = Some(accels);
        }
        "vmport" => *vmport = Some(value.parse::<OnOffAuto>().map_err(|_| format!("invalid vmport value: {value}"))?),
        "dump-guest-core" => *dump_guest_core = Some(value.parse::<OnOffDefaultOn>().map_err(|_| format!("invalid dump-guest-core value: {value}"))?),
        "mem-merge" => *mem_merge = Some(value.parse::<OnOffDefaultOn>().map_err(|_| format!("invalid mem-merge value: {value}"))?),
        "aes-key-wrap" => *aes_key_wrap = Some(value.parse::<OnOffDefaultOn>().map_err(|_| format!("invalid aes-key-wrap value: {value}"))?),
        "dea-key-wrap" => *dea_key_wrap = Some(value.parse::<OnOffDefaultOn>().map_err(|_| format!("invalid dea-key-wrap value: {value}"))?),
        "nvdimm" => *nvdimm = Some(value.parse::<OnOffDefaultOff>().map_err(|_| format!("invalid nvdimm value: {value}"))?),
        "memory-encryption" => *memory_encryption = Some(ShellString::new(value)),
        "hmat" => *hmat = Some(value.parse::<OnOffDefaultOff>().map_err(|_| format!("invalid hmat value: {value}"))?),
        "aux-ram-share" => *aux_ram_share = Some(value.parse::<OnOffDefaultOff>().map_err(|_| format!("invalid aux-ram-share value: {value}"))?),
        "memory-backend" => *memory_backend = Some(ShellString::new(value)),
        other => return Err(format!("unsupported machine option: {other}")),
    }

    Ok(())
}

fn parse_machine_type(value: &str) -> Result<MachineTypeX86_64, String> {
    value.parse::<MachineTypeX86_64>()
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct MachineAarch64 {
    pub m: Machine<MachineTypeAarch64>,
}
impl ToCommand for MachineAarch64 {
    fn command(&self) -> String {
        ARG_MACHINE.to_string()
    }

    fn to_args(&self) -> Vec<String> {
        let mut args = vec![self.m.machine_type.to_arg().to_string()];

        if let Some(accels) = &self.m.accel {
            let accel_strs: Vec<&str> = accels.iter().map(|a| a.to_arg()).collect();
            args.push(format!("{}{}", KEY_ACCEL, accel_strs.join(":")));
        }
        qao!(&self.m.dump_guest_core, args, KEY_DUMP_GUEST_CORE);
        qao!(&self.m.mem_merge, args, KEY_MEM_MERGE);
        qao!(&self.m.nvdimm, args, KEY_NVDIMM);
        if let Some(memory_backend) = &self.m.memory_backend {
            args.push(format!("{}{}", KEY_MEMORY_BACKEND, memory_backend.as_ref()));
        }

        vec![args.join(DELIM_COMMA)]
    }
}
impl FromStr for MachineAarch64 {
    type Err = ShellStringError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_machine_aarch64(s).map_err(ShellStringError::new)
    }
}

fn parse_machine_aarch64(s: &str) -> Result<MachineAarch64, String> {
    let mut parts = s.split(DELIM_COMMA);
    let first = parts.next().ok_or_else(|| "empty machine argument".to_string())?;

    let mut machine_type = None;
    let mut accel = None;
    let mut dump_guest_core = None;
    let mut mem_merge = None;
    let mut nvdimm = None;
    let mut memory_backend = None;

    if let Some(value) = first.strip_prefix(KEY_TYPE) {
        machine_type = Some(parse_machine_type_aarch64(value)?);
    } else if first.contains('=') {
        parse_machine_aarch64_option(first, &mut machine_type, &mut accel, &mut dump_guest_core, &mut mem_merge, &mut nvdimm, &mut memory_backend)?;
    } else {
        machine_type = Some(parse_machine_type_aarch64(first)?);
    }

    for part in parts {
        parse_machine_aarch64_option(part, &mut machine_type, &mut accel, &mut dump_guest_core, &mut mem_merge, &mut nvdimm, &mut memory_backend)?;
    }

    let machine_type = machine_type.ok_or_else(|| "machine type is required".to_string())?;

    Ok(MachineAarch64 {
        m: Machine {
            machine_type,
            accel,
            vmport: None,
            dump_guest_core,
            mem_merge,
            aes_key_wrap: None,
            dea_key_wrap: None,
            nvdimm,
            memory_encryption: None,
            hmat: None,
            aux_ram_share: None,
            memory_backend,
        },
    })
}

fn parse_machine_aarch64_option(
    part: &str,
    machine_type: &mut Option<MachineTypeAarch64>,
    accel: &mut Option<Vec<AccelType>>,
    dump_guest_core: &mut Option<OnOffDefaultOn>,
    mem_merge: &mut Option<OnOffDefaultOn>,
    nvdimm: &mut Option<OnOffDefaultOff>,
    memory_backend: &mut Option<ShellString>,
) -> Result<(), String> {
    let (key, value) = part.split_once('=').ok_or_else(|| format!("invalid machine option: {part}"))?;
    match key {
        "type" => *machine_type = Some(parse_machine_type_aarch64(value)?),
        "accel" => {
            let accels = value
                .split(DELIM_COLON)
                .map(|v| v.parse::<AccelType>().map_err(|_| format!("invalid accel value: {v}")))
                .collect::<Result<Vec<_>, _>>()?;
            *accel = Some(accels);
        }
        "dump-guest-core" => *dump_guest_core = Some(value.parse::<OnOffDefaultOn>().map_err(|_| format!("invalid dump-guest-core value: {value}"))?),
        "mem-merge" => *mem_merge = Some(value.parse::<OnOffDefaultOn>().map_err(|_| format!("invalid mem-merge value: {value}"))?),
        "nvdimm" => *nvdimm = Some(value.parse::<OnOffDefaultOff>().map_err(|_| format!("invalid nvdimm value: {value}"))?),
        "memory-backend" => *memory_backend = Some(ShellString::new(value)),
        other => return Err(format!("unsupported aarch64 machine option: {other}")),
    }

    Ok(())
}

fn parse_machine_type_aarch64(value: &str) -> Result<MachineTypeAarch64, String> {
    value.parse::<MachineTypeAarch64>().map_err(|_| format!("{value} is not a supported aarch64 machine type"))
}
