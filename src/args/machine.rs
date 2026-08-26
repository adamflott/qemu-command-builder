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
const KEY_CONFIDENTIAL_GUEST_SUPPORT: &str = "confidential-guest-support=";
const KEY_HMAT: &str = "hmat=";
const KEY_SPCR: &str = "spcr=";
const KEY_AUX_RAM_SHARE: &str = "aux-ram-share=";
const KEY_MEMORY_BACKEND: &str = "memory-backend=";
const KEY_IGVM_CFG: &str = "igvm-cfg=";

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

impl FromStr for Granularity {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "256" => Ok(Self::G256),
            "512" => Ok(Self::G512),
            "1k" => Ok(Self::G1k),
            "2k" => Ok(Self::G2k),
            "4k" => Ok(Self::G4k),
            "8k" => Ok(Self::G8k),
            "16k" => Ok(Self::G16k),
            other => Err(format!("invalid cxl-fmw interleave-granularity: {other}")),
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

/// An SGX EPC section definition for `-machine sgx-epc.N.*`.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct SgxEpc {
    memdev: ShellString,
    node: u64,
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
    memory_encryption: Option<ShellString>,

    /// Confidential guest support object to use. The default is none.
    confidential_guest_support: Option<ShellString>,

    /// Enables or disables ACPI Heterogeneous Memory Attribute Table
    /// (HMAT) support. The default is off.
    hmat: Option<OnOffDefaultOff>,

    /// Enables or disables ACPI Serial Port Console Redirection Table
    /// (SPCR) support. The default is on.
    spcr: Option<OnOffDefaultOn>,

    /// Allocate auxiliary guest RAM as an anonymous file that is
    /// shareable with an external process.  This option applies to
    /// memory allocated as a side effect of creating various devices.
    /// It does not apply to memory-backend-objects, whether explicitly
    /// specified on the command line, or implicitly created by the -m
    /// command line option.  The default is off.
    aux_ram_share: Option<OnOffDefaultOff>,

    /// An alternative to legacy ``-mem-path`` and ``mem-prealloc`` options.
    /// Allows to use a memory backend as main RAM.
    memory_backend: Option<ShellString>,

    /// CXL fixed memory windows.
    cxl_fmw: Option<Vec<CxlFmw>>,

    /// IGVM configuration object to use for initial guest state.
    igvm_cfg: Option<ShellString>,

    /// SGX EPC sections.
    sgx_epc: Option<Vec<SgxEpc>>,

    /// Machine-type-specific properties accepted by the selected board.
    extra_properties: Option<Vec<(ShellString, ShellString)>>,
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
        if let Some(confidential_guest_support) = &self.m.confidential_guest_support {
            args.push(format!("{}{}", KEY_CONFIDENTIAL_GUEST_SUPPORT, confidential_guest_support.as_ref()));
        }
        qao!(&self.m.hmat, args, KEY_HMAT);
        qao!(&self.m.spcr, args, KEY_SPCR);
        qao!(&self.m.aux_ram_share, args, KEY_AUX_RAM_SHARE);
        if let Some(memory_backend) = &self.m.memory_backend {
            args.push(format!("{}{}", KEY_MEMORY_BACKEND, memory_backend.as_ref()));
        }
        push_cxl_fmw_args(&mut args, &self.m.cxl_fmw);
        if let Some(igvm_cfg) = &self.m.igvm_cfg {
            args.push(format!("{}{}", KEY_IGVM_CFG, igvm_cfg.as_ref()));
        }
        if let Some(sgx_epcs) = &self.m.sgx_epc {
            for (idx, sgx_epc) in sgx_epcs.iter().enumerate() {
                args.push(format!("sgx-epc.{}.memdev={}", idx, sgx_epc.memdev.as_ref()));
                args.push(format!("sgx-epc.{}.node={}", idx, sgx_epc.node));
            }
        }
        if let Some(properties) = &self.m.extra_properties {
            for (key, value) in properties {
                args.push(format!("{}={}", key.as_ref(), value.as_ref()));
            }
        }

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
    let mut confidential_guest_support = None;
    let mut hmat = None;
    let mut spcr = None;
    let mut aux_ram_share = None;
    let mut memory_backend = None;
    let mut cxl_fmw = std::collections::BTreeMap::<u64, CxlFmwParts>::new();
    let mut igvm_cfg = None;
    let mut sgx_epc = std::collections::BTreeMap::<u64, (Option<ShellString>, Option<u64>)>::new();
    let mut extra_properties = Vec::new();

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
            &mut confidential_guest_support,
            &mut hmat,
            &mut spcr,
            &mut aux_ram_share,
            &mut memory_backend,
            &mut cxl_fmw,
            &mut igvm_cfg,
            &mut sgx_epc,
            &mut extra_properties,
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
            &mut confidential_guest_support,
            &mut hmat,
            &mut spcr,
            &mut aux_ram_share,
            &mut memory_backend,
            &mut cxl_fmw,
            &mut igvm_cfg,
            &mut sgx_epc,
            &mut extra_properties,
        )?;
    }

    let machine_type = machine_type.ok_or_else(|| "machine type is required".to_string())?;
    let cxl_fmw = build_cxl_fmw(cxl_fmw)?;
    let sgx_epc = build_sgx_epc(sgx_epc)?;

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
            confidential_guest_support,
            hmat,
            spcr,
            aux_ram_share,
            memory_backend,
            cxl_fmw,
            igvm_cfg,
            sgx_epc,
            extra_properties: (!extra_properties.is_empty()).then_some(extra_properties),
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
    confidential_guest_support: &mut Option<ShellString>,
    hmat: &mut Option<OnOffDefaultOff>,
    spcr: &mut Option<OnOffDefaultOn>,
    aux_ram_share: &mut Option<OnOffDefaultOff>,
    memory_backend: &mut Option<ShellString>,
    cxl_fmw: &mut std::collections::BTreeMap<u64, CxlFmwParts>,
    igvm_cfg: &mut Option<ShellString>,
    sgx_epc: &mut std::collections::BTreeMap<u64, (Option<ShellString>, Option<u64>)>,
    extra_properties: &mut Vec<(ShellString, ShellString)>,
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
        "confidential-guest-support" => *confidential_guest_support = Some(ShellString::new(value)),
        "hmat" => *hmat = Some(value.parse::<OnOffDefaultOff>().map_err(|_| format!("invalid hmat value: {value}"))?),
        "spcr" => *spcr = Some(value.parse::<OnOffDefaultOn>().map_err(|_| format!("invalid spcr value: {value}"))?),
        "aux-ram-share" => *aux_ram_share = Some(value.parse::<OnOffDefaultOff>().map_err(|_| format!("invalid aux-ram-share value: {value}"))?),
        "memory-backend" => *memory_backend = Some(ShellString::new(value)),
        _ if key.starts_with("cxl-fmw.") => parse_cxl_fmw_option(key, value, cxl_fmw)?,
        "igvm-cfg" => *igvm_cfg = Some(ShellString::new(value)),
        _ if key.starts_with("sgx-epc.") => parse_sgx_epc_option(key, value, sgx_epc)?,
        other => extra_properties.push((ShellString::new(other), ShellString::new(value))),
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
        qao!(&self.m.spcr, args, KEY_SPCR);
        if let Some(confidential_guest_support) = &self.m.confidential_guest_support {
            args.push(format!("{}{}", KEY_CONFIDENTIAL_GUEST_SUPPORT, confidential_guest_support.as_ref()));
        }
        if let Some(memory_backend) = &self.m.memory_backend {
            args.push(format!("{}{}", KEY_MEMORY_BACKEND, memory_backend.as_ref()));
        }
        push_cxl_fmw_args(&mut args, &self.m.cxl_fmw);
        if let Some(igvm_cfg) = &self.m.igvm_cfg {
            args.push(format!("{}{}", KEY_IGVM_CFG, igvm_cfg.as_ref()));
        }
        if let Some(sgx_epcs) = &self.m.sgx_epc {
            for (idx, sgx_epc) in sgx_epcs.iter().enumerate() {
                args.push(format!("sgx-epc.{}.memdev={}", idx, sgx_epc.memdev.as_ref()));
                args.push(format!("sgx-epc.{}.node={}", idx, sgx_epc.node));
            }
        }
        if let Some(properties) = &self.m.extra_properties {
            for (key, value) in properties {
                args.push(format!("{}={}", key.as_ref(), value.as_ref()));
            }
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
    let mut spcr = None;
    let mut confidential_guest_support = None;
    let mut memory_backend = None;
    let mut cxl_fmw = std::collections::BTreeMap::<u64, CxlFmwParts>::new();
    let mut igvm_cfg = None;
    let mut sgx_epc = std::collections::BTreeMap::<u64, (Option<ShellString>, Option<u64>)>::new();
    let mut extra_properties = Vec::new();

    if let Some(value) = first.strip_prefix(KEY_TYPE) {
        machine_type = Some(parse_machine_type_aarch64(value)?);
    } else if first.contains('=') {
        parse_machine_aarch64_option(
            first,
            &mut machine_type,
            &mut accel,
            &mut dump_guest_core,
            &mut mem_merge,
            &mut nvdimm,
            &mut spcr,
            &mut confidential_guest_support,
            &mut memory_backend,
            &mut cxl_fmw,
            &mut igvm_cfg,
            &mut sgx_epc,
            &mut extra_properties,
        )?;
    } else {
        machine_type = Some(parse_machine_type_aarch64(first)?);
    }

    for part in parts {
        parse_machine_aarch64_option(
            part,
            &mut machine_type,
            &mut accel,
            &mut dump_guest_core,
            &mut mem_merge,
            &mut nvdimm,
            &mut spcr,
            &mut confidential_guest_support,
            &mut memory_backend,
            &mut cxl_fmw,
            &mut igvm_cfg,
            &mut sgx_epc,
            &mut extra_properties,
        )?;
    }

    let machine_type = machine_type.ok_or_else(|| "machine type is required".to_string())?;
    let cxl_fmw = build_cxl_fmw(cxl_fmw)?;
    let sgx_epc = build_sgx_epc(sgx_epc)?;

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
            confidential_guest_support,
            hmat: None,
            spcr,
            aux_ram_share: None,
            memory_backend,
            cxl_fmw,
            igvm_cfg,
            sgx_epc,
            extra_properties: (!extra_properties.is_empty()).then_some(extra_properties),
        },
    })
}

#[allow(clippy::too_many_arguments)]
fn parse_machine_aarch64_option(
    part: &str,
    machine_type: &mut Option<MachineTypeAarch64>,
    accel: &mut Option<Vec<AccelType>>,
    dump_guest_core: &mut Option<OnOffDefaultOn>,
    mem_merge: &mut Option<OnOffDefaultOn>,
    nvdimm: &mut Option<OnOffDefaultOff>,
    spcr: &mut Option<OnOffDefaultOn>,
    confidential_guest_support: &mut Option<ShellString>,
    memory_backend: &mut Option<ShellString>,
    cxl_fmw: &mut std::collections::BTreeMap<u64, CxlFmwParts>,
    igvm_cfg: &mut Option<ShellString>,
    sgx_epc: &mut std::collections::BTreeMap<u64, (Option<ShellString>, Option<u64>)>,
    extra_properties: &mut Vec<(ShellString, ShellString)>,
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
        "spcr" => *spcr = Some(value.parse::<OnOffDefaultOn>().map_err(|_| format!("invalid spcr value: {value}"))?),
        "confidential-guest-support" => *confidential_guest_support = Some(ShellString::new(value)),
        "memory-backend" => *memory_backend = Some(ShellString::new(value)),
        _ if key.starts_with("cxl-fmw.") => parse_cxl_fmw_option(key, value, cxl_fmw)?,
        "igvm-cfg" => *igvm_cfg = Some(ShellString::new(value)),
        _ if key.starts_with("sgx-epc.") => parse_sgx_epc_option(key, value, sgx_epc)?,
        "vmport" | "aes-key-wrap" | "dea-key-wrap" | "memory-encryption" | "hmat" | "aux-ram-share" => {
            return Err(format!("unsupported aarch64 machine option: {key}"));
        }
        other => extra_properties.push((ShellString::new(other), ShellString::new(value))),
    }

    Ok(())
}

#[derive(Debug, Default)]
struct CxlFmwParts {
    targets: std::collections::BTreeMap<u64, String>,
    size: Option<String>,
    interleave_granularity: Option<Granularity>,
}

fn push_cxl_fmw_args(args: &mut Vec<String>, cxl_fmw: &Option<Vec<CxlFmw>>) {
    if let Some(cxl_fmws) = cxl_fmw {
        for (idx, cxl_fmw) in cxl_fmws.iter().enumerate() {
            for (target_idx, target) in cxl_fmw.targets.iter().enumerate() {
                args.push(format!("cxl-fmw.{}.targets.{}={}", idx, target_idx, target));
            }
            args.push(format!("cxl-fmw.{}.size={}", idx, cxl_fmw.size));
            if let Some(granularity) = &cxl_fmw.interleave_granularity {
                args.push(format!("cxl-fmw.{}.interleave-granularity={}", idx, granularity.to_arg()));
            }
        }
    }
}

fn parse_cxl_fmw_option(key: &str, value: &str, cxl_fmw: &mut std::collections::BTreeMap<u64, CxlFmwParts>) -> Result<(), String> {
    let mut parts = key.split('.');
    let prefix = parts.next();
    let index = parts.next().ok_or_else(|| format!("invalid CXL FMW option: {key}"))?;
    if prefix != Some("cxl-fmw") {
        return Err(format!("invalid CXL FMW option: {key}"));
    }

    let index = index.parse::<u64>().map_err(|e| format!("invalid CXL FMW index: {e}"))?;
    let entry = cxl_fmw.entry(index).or_default();

    match parts.next() {
        Some("targets") => {
            let target_index = parts.next().ok_or_else(|| format!("invalid CXL FMW target option: {key}"))?;
            if parts.next().is_some() {
                return Err(format!("invalid CXL FMW target option: {key}"));
            }
            let target_index = target_index.parse::<u64>().map_err(|e| format!("invalid CXL FMW target index: {e}"))?;
            entry.targets.insert(target_index, value.to_string());
        }
        Some("size") => {
            if parts.next().is_some() {
                return Err(format!("invalid CXL FMW size option: {key}"));
            }
            entry.size = Some(value.to_string());
        }
        Some("interleave-granularity") => {
            if parts.next().is_some() {
                return Err(format!("invalid CXL FMW interleave-granularity option: {key}"));
            }
            entry.interleave_granularity = Some(value.parse::<Granularity>()?);
        }
        Some(other) => return Err(format!("unsupported CXL FMW option: {other}")),
        None => return Err(format!("invalid CXL FMW option: {key}")),
    }

    Ok(())
}

fn build_cxl_fmw(cxl_fmw: std::collections::BTreeMap<u64, CxlFmwParts>) -> Result<Option<Vec<CxlFmw>>, String> {
    if cxl_fmw.is_empty() {
        return Ok(None);
    }

    let mut windows = Vec::new();
    for (index, parts) in cxl_fmw {
        if parts.targets.is_empty() {
            return Err(format!("cxl-fmw.{index} requires at least one target"));
        }
        windows.push(CxlFmw {
            targets: parts.targets.into_values().collect(),
            size: parts.size.ok_or_else(|| format!("cxl-fmw.{index} requires size"))?,
            interleave_granularity: parts.interleave_granularity,
        });
    }
    Ok(Some(windows))
}

fn parse_sgx_epc_option(key: &str, value: &str, sgx_epc: &mut std::collections::BTreeMap<u64, (Option<ShellString>, Option<u64>)>) -> Result<(), String> {
    let mut parts = key.split('.');
    let prefix = parts.next();
    let index = parts.next().ok_or_else(|| format!("invalid SGX EPC option: {key}"))?;
    let field = parts.next().ok_or_else(|| format!("invalid SGX EPC option: {key}"))?;
    if prefix != Some("sgx-epc") || parts.next().is_some() {
        return Err(format!("invalid SGX EPC option: {key}"));
    }

    let index = index.parse::<u64>().map_err(|e| format!("invalid SGX EPC index: {e}"))?;
    let entry = sgx_epc.entry(index).or_default();
    match field {
        "memdev" => entry.0 = Some(ShellString::new(value)),
        "node" => entry.1 = Some(value.parse::<u64>().map_err(|e| format!("invalid SGX EPC node: {e}"))?),
        other => return Err(format!("unsupported SGX EPC option: {other}")),
    }
    Ok(())
}

fn build_sgx_epc(sgx_epc: std::collections::BTreeMap<u64, (Option<ShellString>, Option<u64>)>) -> Result<Option<Vec<SgxEpc>>, String> {
    if sgx_epc.is_empty() {
        return Ok(None);
    }

    let mut entries = Vec::new();
    for (index, (memdev, node)) in sgx_epc {
        entries.push(SgxEpc {
            memdev: memdev.ok_or_else(|| format!("sgx-epc.{index} requires memdev"))?,
            node: node.ok_or_else(|| format!("sgx-epc.{index} requires node"))?,
        });
    }
    Ok(Some(entries))
}

fn parse_machine_type_aarch64(value: &str) -> Result<MachineTypeAarch64, String> {
    value.parse::<MachineTypeAarch64>().map_err(|_| format!("{value} is not a supported aarch64 machine type"))
}
