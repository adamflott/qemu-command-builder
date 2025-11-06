use bon::Builder;

use crate::common::*;
use crate::machine_type::{MachineAarch64, MachineX86_64};
use crate::to_command::{ToArg, ToCommand};

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default)]
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
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder)]
pub struct CxlFmw {
    targets: Vec<String>,
    size: String,
    interleave_granularity: Option<Granularity>,
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder)]
pub struct SmpCache {
    cache: String,
    topology: String,
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
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder)]
pub struct MachineFor<T> {
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
    memory_encryption: Option<String>, // TODO find out actual values

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
    memory_backend: Option<String>, // TODO find out actual values

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
    cxl_fmw: Option<CxlFmw>,

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
    smp_cache: Option<Vec<SmpCache>>,
}

pub type MachineForX86 = MachineFor<MachineX86_64>;
pub type MachineForAarch64 = MachineFor<MachineAarch64>;

impl<M: ToCommand + ToArg> ToCommand for MachineFor<M> {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![];
        cmd.push("-machine".to_string());

        let mut args = vec![self.machine_type.to_arg().to_string()];

        if let Some(accels) = &self.accel {
            let accel_strs: Vec<&str> = accels.iter().map(|a| a.to_arg()).collect();
            args.push(format!("accel={}", accel_strs.join(":")));
        }
        if let Some(vmport) = &self.vmport {
            args.push(format!("vmport={}", vmport.to_arg()));
        }
        if let Some(dump_guest_core) = &self.dump_guest_core {
            args.push(format!("dump-guest-core={}", dump_guest_core.to_arg()));
        }
        if let Some(mem_merge) = &self.mem_merge {
            args.push(format!("mem-merge={}", mem_merge.to_arg()));
        }
        if let Some(aes_key_wrap) = &self.aes_key_wrap {
            args.push(format!("aes-key-wrap={}", aes_key_wrap.to_arg()));
        }
        if let Some(dea_key_wrap) = &self.dea_key_wrap {
            args.push(format!("dea-key-wrap={}", dea_key_wrap.to_arg()));
        }
        if let Some(nvdimm) = &self.nvdimm {
            args.push(format!("nvdimm={}", nvdimm.to_arg()));
        }
        if let Some(memory_encryption) = &self.memory_encryption {
            args.push(format!("memory-encryption={}", memory_encryption));
        }
        if let Some(hmat) = &self.hmat {
            args.push(format!("hmat={}", hmat.to_arg()));
        }
        if let Some(aux_ram_share) = &self.aux_ram_share {
            args.push(format!("aux-ram-share={}", aux_ram_share.to_arg()));
        }
        if let Some(memory_backend) = &self.memory_backend {
            args.push(format!("memory-backend={}", memory_backend));
        }
        if let Some(cxl_fmw) = &self.cxl_fmw {
            for (idx, target) in cxl_fmw.targets.iter().enumerate() {
                args.push(format!("cxl-fmw.0.targets.{}={}", idx, target));
            }
            args.push(format!("cxl-fmw.0.size={}", cxl_fmw.size));
            if let Some(granularity) = &cxl_fmw.interleave_granularity {
                args.push(format!(
                    "cxl-fmw.0.interleave-granularity={}",
                    granularity.to_arg()
                ));
            }
        }
        if let Some(smp_caches) = &self.smp_cache {
            for (idx, smp_cache) in smp_caches.iter().enumerate() {
                args.push(format!("smp-cache.{}.cache={}", idx, smp_cache.cache));
                args.push(format!("smp-cache.{}.topology={}", idx, smp_cache.topology));
            }
        }
        cmd.push(args.join(","));

        cmd
    }
}
