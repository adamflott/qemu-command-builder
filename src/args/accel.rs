use crate::parsers::ARG_ACCEL;
use bon::Builder;
use proptest_derive::Arbitrary;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::common::*;
use crate::parsers::DELIM_COMMA;
use crate::qao;
use crate::shell_path::ShellPath;
use crate::to_command::{ToArg, ToCommand};

const KEY_IGD_PASSTHRU: &str = "igd-passthru=";
const KEY_KERNEL_IRQCHIP: &str = "kernel-irqchip=";
const KEY_KVM_SHADOW_MEM: &str = "kvm-shadow-mem=";
const KEY_ONE_INSN_PER_TB: &str = "one-insn-per-tb=";
const KEY_SPLIT_WX: &str = "split-wx=";
const KEY_TB_SIZE: &str = "tb-size=";
const KEY_DIRTY_RING_SIZE: &str = "dirty-ring-size=";
const KEY_EAGER_SPLIT_SIZE: &str = "eager-split-size=";
const KEY_NOTIFY_VMEXIT: &str = "notify-vmexit=";
const KEY_THREAD: &str = "thread=";
const KEY_DEVICE: &str = "device=";

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Arbitrary)]
/// QEMU `kernel-irqchip=` values for `-accel`.
pub enum OnOffSplit {
    #[default]
    On,
    Off,
    Split,
}

impl Display for OnOffSplit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_arg())
    }
}

impl FromStr for OnOffSplit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "on" => Ok(OnOffSplit::On),
            "off" => Ok(OnOffSplit::Off),
            "split" => Ok(OnOffSplit::Split),
            _ => Err(()),
        }
    }
}
impl ToArg for OnOffSplit {
    fn to_arg(&self) -> &str {
        match self {
            OnOffSplit::On => "on",
            OnOffSplit::Off => "off",
            OnOffSplit::Split => "split",
        }
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
/// QEMU `thread=` values for TCG acceleration.
pub enum TCGThreadType {
    Single,
    Multi,
}

impl FromStr for TCGThreadType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "single" => Ok(TCGThreadType::Single),
            "multi" => Ok(TCGThreadType::Multi),
            _ => Err(()),
        }
    }
}
impl Display for TCGThreadType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TCGThreadType::Single => write!(f, "single"),
            TCGThreadType::Multi => write!(f, "multi"),
        }
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
/// QEMU `notify-vmexit=` values.
///
/// The `run` mode may also carry the companion `notify-window=` property.
pub enum NotifyVMExit {
    Run(Option<usize>),
    InternalError,
    Disable,
}

impl FromStr for NotifyVMExit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "run" => Ok(NotifyVMExit::Run(None)),
            "internal-error" => Ok(NotifyVMExit::InternalError),
            "disable" => Ok(NotifyVMExit::Disable),
            _ => Err(format!("invalid notify-vmexit value: {s}")),
        }
    }
}
impl Display for NotifyVMExit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NotifyVMExit::Run(n) => {
                if let Some(n) = n {
                    write!(f, "run,notify-window={}", n)
                } else {
                    write!(f, "run")
                }
            }
            NotifyVMExit::InternalError => write!(f, "internal-error"),
            NotifyVMExit::Disable => write!(f, "disable"),
        }
    }
}

/// This is used to enable an accelerator. Depending on the target
/// architecture, kvm, xen, hvf, nvmm, whpx or tcg can be available. By
/// default, tcg is used. If there is more than one accelerator
/// specified, the next one is used if the previous one fails to
/// initialize.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct Accel {
    accel_type: AccelType,

    /// When Xen is in use, this option controls whether Intel
    /// integrated graphics devices can be passed through to the guest
    /// (default=off)
    igd_passthru: Option<OnOffDefaultOff>,

    /// Controls KVM in-kernel irqchip support. The default is full
    /// acceleration of the interrupt controllers. On x86, split irqchip
    /// reduces the kernel attack surface, at a performance cost for
    /// non-MSI interrupts. Disabling the in-kernel irqchip completely
    /// is not recommended except for debugging purposes.
    kernel_irqchip: Option<OnOffSplit>,

    /// Defines the size of the KVM shadow MMU.
    kvm_shadow_mem: Option<usize>,

    /// Makes the TCG accelerator put only one guest instruction into
    /// each translation block. This slows down emulation a lot, but
    /// can be useful in some situations, such as when trying to analyse
    /// the logs produced by the ``-d`` option.
    one_insn_per_tb: Option<OnOff>,

    /// Controls the use of split w^x mapping for the TCG code generation
    /// buffer. Some operating systems require this to be enabled, and in
    /// such a case this will default on. On other operating systems, this
    /// will default off, but one may enable this for testing or debugging.
    split_wx: Option<OnOff>,

    /// Controls the size (in MiB) of the TCG translation block cache.
    tb_size: Option<usize>,

    /// When the KVM accelerator is used, it controls the size of the per-vCPU
    /// dirty page ring buffer (number of entries for each vCPU). It should
    /// be a value that is power of two, and it should be 1024 or bigger (but
    /// still less than the maximum value that the kernel supports).  4096
    /// could be a good initial value if you have no idea which is the best.
    /// Set this value to 0 to disable the feature.  By default, this feature
    /// is disabled (dirty-ring-size=0).  When enabled, KVM will instead
    /// record dirty pages in a bitmap.
    dirty_ring_size: Option<usize>,

    /// KVM implements dirty page logging at the PAGE_SIZE granularity and
    /// enabling dirty-logging on a huge-page requires breaking it into
    /// PAGE_SIZE pages in the first place. KVM on ARM does this splitting
    /// lazily by default. There are performance benefits in doing huge-page
    /// split eagerly, especially in situations where TLBI costs associated
    /// with break-before-make sequences are considerable and also if guest
    /// workloads are read intensive. The size here specifies how many pages
    /// to break at a time and needs to be a valid block size which is
    /// 1GB/2MB/4KB, 32MB/16KB and 512MB/64KB for 4KB/16KB/64KB PAGE_SIZE
    /// respectively. Be wary of specifying a higher size as it will have an
    /// impact on the memory. By default, this feature is disabled
    /// (eager-split-size=0).
    eager_split_size: Option<usize>,

    /// Enables or disables notify VM exit support on x86 host and specify
    /// the corresponding notify window to trigger the VM exit if enabled.
    /// ``run`` option enables the feature. It does nothing and continue
    /// if the exit happens. ``internal-error`` option enables the feature.
    /// It raises a internal error. ``disable`` option doesn't enable the feature.
    /// This feature can mitigate the CPU stuck issue due to event windows don't
    /// open up for a specified of time (i.e. notify-window).
    /// Default: notify-vmexit=run,notify-window=0.
    notify_vmexit: Option<NotifyVMExit>,

    /// Enable single or multi-threaded TCG
    thread: Option<TCGThreadType>,

    // Sets the path to the KVM device node. Defaults to ``/dev/kvm``. This
    // option can be used to pass the KVM device to use via a file descriptor
    // by setting the value to ``/dev/fdset/NN``.
    device: Option<ShellPath>,
}

impl Accel {
    /// Creates an accelerator configuration for the given backend.
    pub fn new(accel_type: AccelType) -> Self {
        Self {
            accel_type,
            igd_passthru: None,
            kernel_irqchip: None,
            kvm_shadow_mem: None,
            one_insn_per_tb: None,
            split_wx: None,
            tb_size: None,
            dirty_ring_size: None,
            eager_split_size: None,
            notify_vmexit: None,
            thread: None,
            device: None,
        }
    }
}

impl ToCommand for Accel {
    fn command(&self) -> String {
        ARG_ACCEL.to_string()
    }

    fn to_args(&self) -> Vec<String> {
        let mut args = vec![self.accel_type.to_arg().to_string()];

        qao!(&self.igd_passthru, args, KEY_IGD_PASSTHRU);
        qao!(&self.kernel_irqchip, args, KEY_KERNEL_IRQCHIP);
        qao!(&self.kvm_shadow_mem, args, KEY_KVM_SHADOW_MEM);
        qao!(&self.one_insn_per_tb, args, KEY_ONE_INSN_PER_TB);
        qao!(&self.split_wx, args, KEY_SPLIT_WX);
        qao!(&self.tb_size, args, KEY_TB_SIZE);
        qao!(&self.dirty_ring_size, args, KEY_DIRTY_RING_SIZE);
        qao!(&self.eager_split_size, args, KEY_EAGER_SPLIT_SIZE);
        qao!(&self.notify_vmexit, args, KEY_NOTIFY_VMEXIT);
        qao!(&self.thread, args, KEY_THREAD);
        if let Some(device) = &self.device {
            args.push(format!("{}{}", KEY_DEVICE, device.as_ref()));
        }

        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for Accel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(DELIM_COMMA);
        let accel_token = parts.next().ok_or_else(|| "empty accel argument".to_string())?;
        let accel_name = accel_token.strip_prefix("accel=").unwrap_or(accel_token);
        let accel_type = accel_name.parse::<AccelType>().map_err(|_| format!("invalid accel type: {accel_name}"))?;

        let mut accel = Accel::new(accel_type);
        let mut pending_notify_window = None;

        for part in parts {
            let (key, value) = part.split_once('=').ok_or_else(|| format!("invalid accel option: {part}"))?;
            match key {
                "igd-passthru" => {
                    accel.igd_passthru = Some(value.parse::<OnOffDefaultOff>().map_err(|_| format!("invalid igd-passthru value: {value}"))?);
                }
                "kernel-irqchip" => {
                    accel.kernel_irqchip = Some(value.parse::<OnOffSplit>().map_err(|_| format!("invalid kernel-irqchip value: {value}"))?);
                }
                "kvm-shadow-mem" => {
                    accel.kvm_shadow_mem = Some(value.parse::<usize>().map_err(|e| e.to_string())?);
                }
                "one-insn-per-tb" => {
                    accel.one_insn_per_tb = Some(value.parse::<OnOff>().map_err(|_| format!("invalid one-insn-per-tb value: {value}"))?);
                }
                "split-wx" => {
                    accel.split_wx = Some(value.parse::<OnOff>().map_err(|_| format!("invalid split-wx value: {value}"))?);
                }
                "tb-size" => {
                    accel.tb_size = Some(value.parse::<usize>().map_err(|e| e.to_string())?);
                }
                "dirty-ring-size" => {
                    accel.dirty_ring_size = Some(value.parse::<usize>().map_err(|e| e.to_string())?);
                }
                "eager-split-size" => {
                    accel.eager_split_size = Some(value.parse::<usize>().map_err(|e| e.to_string())?);
                }
                "notify-vmexit" => {
                    accel.notify_vmexit = Some(value.parse::<NotifyVMExit>()?);
                }
                "notify-window" => {
                    pending_notify_window = Some(value.parse::<usize>().map_err(|e| e.to_string())?);
                }
                "thread" => {
                    accel.thread = Some(value.parse::<TCGThreadType>().map_err(|_| format!("invalid thread value: {value}"))?);
                }
                "device" => {
                    accel.device = Some(ShellPath::from(value));
                }
                other => return Err(format!("unsupported accel option: {other}")),
            }
        }

        if let Some(window) = pending_notify_window {
            accel.notify_vmexit = match accel.notify_vmexit.take() {
                Some(NotifyVMExit::Run(_)) => Some(NotifyVMExit::Run(Some(window))),
                Some(other) => return Err(format!("notify-window requires notify-vmexit=run, got {other}")),
                None => return Err("notify-window requires notify-vmexit=run".to_string()),
            };
        }

        Ok(accel)
    }
}
