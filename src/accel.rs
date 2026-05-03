use bon::Builder;
use proptest_derive::Arbitrary;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use winnow::ascii::{alphanumeric1, dec_uint};
use winnow::combinator::{alt, opt};
use winnow::prelude::*;
use winnow::token::literal;

use crate::common::*;
use crate::parsers::DELIM_COMMA;
use crate::shell_path::{ShellPath, shell_path_until_comma};
use crate::shell_string::ShellStringError;
use crate::to_command::{ToArg, ToCommand};
use crate::{pco, ppo, qao};

pub(crate) const ARG_ACCEL: &str = "-accel";

const KEY_IGD_PASSTHRU: &str = "igd-passthru=";
const KEY_KERNEL_IRQCHIP: &str = "kernel_irqchip=";
const KEY_KVM_SHADOW_MEM: &str = "kvm-shadow-mem=";
const KEY_ONE_INSN_PER_TB: &str = "one-insn-per-tb=";
const KEY_SPLIT_WX: &str = "split-wx=";
const KEY_TB_SIZE: &str = "tb_size=";
const KEY_DIRTY_RING_SIZE: &str = "dirty-ring-size=";
const KEY_EAGER_SPLIT_SIZE: &str = "eager-split-size=";
const KEY_NOTIFY_VMEXIT: &str = "notify-vmexit=";
const KEY_THREAD: &str = "thread=";
const KEY_DEVICE: &str = "device=";

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Arbitrary)]
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
pub enum NotifyVMExit {
    Run(Option<usize>),
    InternalError,
    Disable,
}

fn nvt_run(s: &mut &str) -> ModalResult<NotifyVMExit> {
    let _ = literal("run").parse_next(s)?;
    let n = opt(nvt_nw).parse_next(s)?;
    Ok(NotifyVMExit::Run(n))
}
fn nvt_ie(s: &mut &str) -> ModalResult<NotifyVMExit> {
    literal("internal-error").parse_next(s).map(|_| Ok(NotifyVMExit::InternalError))?
}
fn nvt_disable(s: &mut &str) -> ModalResult<NotifyVMExit> {
    literal("disable").parse_next(s).map(|_| Ok(NotifyVMExit::Disable))?
}
fn nvt_nw(s: &mut &str) -> ModalResult<usize> {
    let _ = literal(DELIM_COMMA).parse_next(s)?;
    let _ = literal("notify-window=").parse_next(s)?;
    let n = dec_uint.parse_next(s)?;
    Ok(n)
}
fn notify_vm_exit_type(s: &mut &str) -> ModalResult<NotifyVMExit> {
    alt((nvt_run, nvt_ie, nvt_disable)).parse_next(s)
}

impl FromStr for NotifyVMExit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match notify_vm_exit_type.parse(s) {
            Ok(v) => Ok(v),
            Err(_) => Err(()),
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
    kvm_shadow_mem: Option<usize>, // TODO convert to a byte type

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
    tb_size: Option<usize>, // TODO convert to a byte type

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
    type Err = ShellStringError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        accel.parse(s).map_err(|e| ShellStringError::from_parse(e))
    }
}

pco!(igd_passthru, alphanumeric1, OnOffDefaultOff, KEY_IGD_PASSTHRU);
pco!(kernel_irqchip, alphanumeric1, OnOffSplit, KEY_KERNEL_IRQCHIP);
ppo!(kvm_shadow_mem, dec_uint, usize, KEY_KVM_SHADOW_MEM);
pco!(one_insn_per_tb, alphanumeric1, OnOff, KEY_ONE_INSN_PER_TB);
pco!(split_wx, alphanumeric1, OnOff, KEY_SPLIT_WX);
ppo!(tb_size, dec_uint, usize, KEY_TB_SIZE);
ppo!(dirty_ring_size, dec_uint, usize, KEY_DIRTY_RING_SIZE);
ppo!(eager_split_size, dec_uint, usize, KEY_EAGER_SPLIT_SIZE);
fn notify_vmexit(s: &mut &str) -> ModalResult<NotifyVMExit> {
    let _ = literal(DELIM_COMMA).parse_next(s)?;
    let _ = literal(KEY_NOTIFY_VMEXIT).parse_next(s)?;
    notify_vm_exit_type.parse_next(s)
}
pco!(thread, alphanumeric1, TCGThreadType, KEY_THREAD);
pco!(device, shell_path_until_comma, ShellPath, KEY_DEVICE);

fn accel(s: &mut &str) -> ModalResult<Accel> {
    let accel_type = alphanumeric1.parse_to::<AccelType>().parse_next(s)?;
    let igd_passthru = opt(igd_passthru).parse_next(s)?;
    let kernel_irqchip = opt(kernel_irqchip).parse_next(s)?;
    let kvm_shadow_mem = opt(kvm_shadow_mem).parse_next(s)?;
    let one_insn_per_tb = opt(one_insn_per_tb).parse_next(s)?;
    let split_wx = opt(split_wx).parse_next(s)?;
    let tb_size = opt(tb_size).parse_next(s)?;
    let dirty_ring_size = opt(dirty_ring_size).parse_next(s)?;
    let eager_split_size = opt(eager_split_size).parse_next(s)?;
    let notify_vmexit = opt(notify_vmexit).parse_next(s)?;
    let thread = opt(thread).parse_next(s)?;
    let device = opt(device).parse_next(s)?;
    Ok(Accel {
        accel_type,
        igd_passthru,
        kernel_irqchip,
        kvm_shadow_mem,
        one_insn_per_tb,
        split_wx,
        tb_size,
        dirty_ring_size,
        eager_split_size,
        notify_vmexit,
        thread,
        device,
    })
}
