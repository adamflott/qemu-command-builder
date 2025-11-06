use std::path::PathBuf;

use bon::Builder;

use crate::common::*;
use crate::to_command::{ToArg, ToCommand};

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default)]
pub enum OnOffSplit {
    #[default]
    On,
    Off,
    Split,
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

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum TCGThreadType {
    Single,
    Multi,
}

impl ToArg for TCGThreadType {
    fn to_arg(&self) -> &str {
        match self {
            TCGThreadType::Single => "single",
            TCGThreadType::Multi => "multi",
        }
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum NotifyVMExitType {
    Run,
    InternalError,
    Disable,
    NotifyWindow(usize),
}

/// This is used to enable an accelerator. Depending on the target
/// architecture, kvm, xen, hvf, nvmm, whpx or tcg can be available. By
/// default, tcg is used. If there is more than one accelerator
/// specified, the next one is used if the previous one fails to
/// initialize.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder)]
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
    notify_vmexit: Option<Vec<NotifyVMExitType>>,

    /// Enable single or multi-threaded TCG
    thread: Option<TCGThreadType>,

    /// Sets the path to the KVM device node. Defaults to ``/dev/kvm``. This
    /// option can be used to pass the KVM device to use via a file descriptor
    /// by setting the value to ``/dev/fdset/NN``.
    device: Option<PathBuf>,
}

impl ToCommand for Accel {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![];
        cmd.push("-accel".to_string());

        let mut args = vec![self.accel_type.to_arg().to_string()];

        if let Some(igd_passthru) = &self.igd_passthru {
            args.push(format!("igd-passthru={}", igd_passthru.to_arg()));
        }
        if let Some(kernel_irqchip) = &self.kernel_irqchip {
            args.push(format!("kernel_irqchip={}", kernel_irqchip.to_arg()));
        }
        if let Some(kvm_shadow_mem) = &self.kvm_shadow_mem {
            args.push(format!("kvm-shadow-mem={}", kvm_shadow_mem));
        }
        if let Some(one_insn_per_tb) = &self.one_insn_per_tb {
            args.push(format!("one-insn-per-tb={}", one_insn_per_tb.to_arg()));
        }
        if let Some(split_wx) = &self.split_wx {
            args.push(format!("split-wx={}", split_wx.to_arg()));
        }
        if let Some(tb_size) = &self.tb_size {
            args.push(format!("tb-size={}", tb_size));
        }
        if let Some(dirty_ring_size) = &self.dirty_ring_size {
            args.push(format!("dirty-ring-size={}", dirty_ring_size));
        }
        if let Some(eager_split_size) = &self.eager_split_size {
            args.push(format!("eager-split-size={}", eager_split_size));
        }
        if let Some(notify_vmexit) = &self.notify_vmexit {
            let mut nvm_args = vec![];
            for opt in notify_vmexit {
                match opt {
                    NotifyVMExitType::Run => {
                        nvm_args.push("run".to_string());
                    }
                    NotifyVMExitType::InternalError => {
                        nvm_args.push("internal-error".to_string());
                    }
                    NotifyVMExitType::Disable => {
                        nvm_args.push("disable".to_string());
                    }
                    NotifyVMExitType::NotifyWindow(window) => {
                        nvm_args.push(format!("notify-window={}", window));
                    }
                }
            }
            args.push(format!("notify-vmexit=={}", nvm_args.join(",")));
        }
        if let Some(thread) = &self.thread {
            args.push(format!("thread={}", thread.to_arg()));
        }
        if let Some(device) = &self.device {
            args.push(format!("device={}", device.display()));
        }
        cmd.push(args.join(","));
        cmd
    }
}
