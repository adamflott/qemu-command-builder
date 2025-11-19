use std::str::FromStr;

use proptest_derive::Arbitrary;

use crate::to_command::{ToArg, ToCommand};

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum CPUFlag {
    /// 3dnow
    X3dnow,
    /// 3dnowext
    X3dnowext,
    /// 3dnowprefetch
    X3dnowprefetch,
    /// abm
    Abm,
    /// ace2
    Ace2,
    /// ace2-en
    Ace2en,
    /// acpi
    Acpi,
    /// adx
    Adx,
    /// aes
    Aes,
    /// amd-no-ssb
    Amdnossb,
    /// amd-psfd
    Amdpsfd,
    /// amd-ssbd
    Amdssbd,
    /// amd-stibp
    Amdstibp,
    /// amx-bf16
    Amxbf16,
    /// amx-complex
    Amxcomplex,
    /// amx-fp16
    Amxfp16,
    /// amx-int8
    Amxint8,
    /// amx-tile
    Amxtile,
    /// apic
    Apic,
    /// arat
    Arat,
    /// arch-capabilities
    Archcapabilities,
    /// arch-lbr
    Archlbr,
    /// auto-ibrs
    Autoibrs,
    /// avic
    Avic,
    /// avx
    Avx,
    /// avx-ifma
    Avxifma,
    /// avx-ne-convert
    Avxneconvert,
    /// avx-vnni
    AvxVnni,
    /// avx-vnni-int16
    AvxVnniint16,
    /// avx-vnni-int8
    AvxVnniint8,
    /// avx10
    Avx10,
    /// avx10-128
    Avx10128,
    /// avx10-256
    Avx10256,
    /// avx10-512
    Avx10512,
    /// avx2
    Avx2,
    /// avx512-4fmaps
    Avx5124fmaps,
    /// avx512-4vnniw
    Avx5124vnniw,
    /// avx512-bf16
    Avx512bf16,
    /// avx512-fp16
    Avx512fp16,
    /// avx512-vp2intersect
    Avx512Vp2intersect,
    /// avx512-vpopcntdq
    Avx512Vpopcntdq,
    /// avx512bitalg
    Avx512bitalg,
    /// avx512bw
    Avx512bw,
    /// avx512cd
    Avx512cd,
    /// avx512dq
    Avx512dq,
    /// avx512er
    Avx512er,
    /// avx512f
    Avx512f,
    /// avx512ifma
    Avx512ifma,
    /// avx512pf
    Avx512pf,
    /// avx512vbmi
    Avx512vbmi,
    /// avx512vbmi2
    Avx512vbmi2,
    /// avx512vl
    Avx512vl,
    /// avx512vnni
    Avx512vnni,
    /// bhi-ctrl
    Bhictrl,
    /// bhi-no
    Bhino,
    /// bmi1
    Bmi1,
    /// bmi2
    Bmi2,
    /// bus-lock-detect
    Buslockdetect,
    /// cid
    Cid,
    /// cldemote
    Cldemote,
    /// clflush
    Clflush,
    /// clflushopt
    Clflushopt,
    /// clwb
    Clwb,
    /// clzero
    Clzero,
    /// cmov
    Cmov,
    /// cmp-legacy
    Cmplegacy,
    /// cmpccxadd
    Cmpccxadd,
    /// core-capability
    Corecapability,
    /// cr8legacy
    Cr8legacy,
    /// cx16
    Cx16,
    /// cx8
    Cx8,
    /// dca
    Dca,
    /// ddpd-u
    Ddpdu,
    /// de
    De,
    /// decodeassists
    Decodeassists,
    /// ds
    Ds,
    /// ds-cpl
    Dscpl,
    /// dtes64
    Dtes64,
    /// eraps
    Eraps,
    /// erms
    Erms,
    /// est
    Est,
    /// extapic
    Extapic,
    /// f16c
    F16c,
    /// fb-clear
    Fbclear,
    /// fbsdp-no
    Fbsdpno,
    /// fdp-excptn-only
    Fdpexcptnonly,
    /// flush-l1d
    Flushl1d,
    /// flushbyasid
    Flushbyasid,
    /// fma
    Fma,
    /// fma4
    Fma4,
    /// fpu
    Fpu,
    /// fred
    Fred,
    /// fsgsbase
    Fsgsbase,
    /// fsrc
    Fsrc,
    /// fsrm
    Fsrm,
    /// fsrs
    Fsrs,
    /// full-width-write
    Fullwidthwrite,
    /// fxsr
    Fxsr,
    /// fxsr-opt
    Fxsropt,
    /// fzrm
    Fzrm,
    /// gds-no
    Gdsno,
    /// gfni
    Gfni,
    /// hle
    Hle,
    /// ht
    Ht,
    /// hypervisor
    Hypervisor,
    /// ia64
    Ia64,
    /// ibpb
    Ibpb,
    /// ibpb-brtype
    Ibpbbrtype,
    /// ibrs
    Ibrs,
    /// ibrs-all
    Ibrsall,
    /// ibs
    Ibs,
    /// intel-psfd
    Intelpsfd,
    /// intel-pt
    Intelpt,
    /// intel-pt-lip
    Intelptlip,
    /// invpcid
    Invpcid,
    /// invtsc
    Invtsc,
    /// ipred-ctrl
    Ipredctrl,
    /// kvm-asyncpf
    Kvmasyncpf,
    /// kvm-asyncpf-int
    Kvmasyncpfint,
    /// kvm-asyncpf-vmexit
    KvmasyncpfVmexit,
    /// kvm-hint-dedicated
    Kvmhintdedicated,
    /// kvm-mmu
    Kvmmmu,
    /// kvm-msi-ext-dest-id
    Kvmmsiextdestid,
    /// kvm-nopiodelay
    Kvmnopiodelay,
    /// kvm-poll-control
    Kvmpollcontrol,
    /// kvm-pv-eoi
    Kvmpveoi,
    /// kvm-pv-ipi
    Kvmpvipi,
    /// kvm-pv-sched-yield
    Kvmpvschedyield,
    /// kvm-pv-tlb-flush
    Kvmpvtlbflush,
    /// kvm-pv-unhalt
    Kvmpvunhalt,
    /// kvm-steal-time
    Kvmstealtime,
    /// kvmclock
    Kvmclock,
    /// kvmclock-stable-bit
    Kvmclockstablebit,
    /// la57
    La57,
    /// lahf-lm
    Lahflm,
    /// lam
    Lam,
    /// lbrv
    Lbrv,
    /// lfence-always-serializing
    Lfencealwaysserializing,
    /// lkgs
    Lkgs,
    /// lm
    Lm,
    /// lwp
    Lwp,
    /// mca
    Mca,
    /// mcdt-no
    Mcdtno,
    /// mce
    Mce,
    /// md-clear
    Mdclear,
    /// mds-no
    Mdsno,
    /// misalignsse
    Misalignsse,
    /// mmx
    Mmx,
    /// mmxext
    Mmxext,
    /// monitor
    Monitor,
    /// movbe
    Movbe,
    /// movdir64b
    Movdir64b,
    /// movdiri
    Movdiri,
    /// mpx
    Mpx,
    /// msr
    Msr,
    /// mtrr
    Mtrr,
    /// no-nested-data-bp
    Nonesteddatabp,
    /// nodeid-msr
    Nodeidmsr,
    /// npt
    Npt,
    /// nrip-save
    Nripsave,
    /// null-sel-clr-base
    Nullselclrbase,
    /// nx
    Nx,
    /// osvw
    Osvw,
    /// overflow-recov
    Overflowrecov,
    /// pae
    Pae,
    /// pat
    Pat,
    /// pause-filter
    Pausefilter,
    /// pbe
    Pbe,
    /// pbrsb-no
    Pbrsbno,
    /// pcid
    Pcid,
    /// pclmulqdq
    Pclmulqdq,
    /// pcommit
    Pcommit,
    /// pdcm
    Pdcm,
    /// pdpe1gb
    Pdpe1gb,
    /// perfctr-core
    Perfctrcore,
    /// perfctr-nb
    Perfctrnb,
    /// perfmon-v2
    PerfmonV2,
    /// pfthreshold
    Pfthreshold,
    /// pge
    Pge,
    /// phe
    Phe,
    /// phe-en
    Pheen,
    /// pks
    Pks,
    /// pku
    Pku,
    /// pmm
    Pmm,
    /// pmm-en
    Pmmen,
    /// pn
    Pn,
    /// pni
    Pni,
    /// popcnt
    Popcnt,
    /// prefetchiti
    Prefetchiti,
    /// pschange-mc-no
    Pschangemcno,
    /// psdp-no
    Psdpno,
    /// pse
    Pse,
    /// pse36
    Pse36,
    /// rdctl-no
    Rdctlno,
    /// rdpid
    Rdpid,
    /// rdrand
    Rdrand,
    /// rdseed
    Rdseed,
    /// rdtscp
    Rdtscp,
    /// rfds-clear
    Rfdsclear,
    /// rfds-no
    Rfdsno,
    /// rrsba-ctrl
    Rrsbactrl,
    /// rsba
    Rsba,
    /// rtm
    Rtm,
    /// sbdr-ssdp-no
    Sbdrssdpno,
    /// sbpb
    Sbpb,
    /// sep
    Sep,
    /// serialize
    Serialize,
    /// sgx
    Sgx,
    /// sgx-aex-notify
    Sgxaexnotify,
    /// sgx-debug
    Sgxdebug,
    /// sgx-edeccssa
    Sgxedeccssa,
    /// sgx-exinfo
    Sgxexinfo,
    /// sgx-kss
    Sgxkss,
    /// sgx-mode64
    Sgxmode64,
    /// sgx-provisionkey
    Sgxprovisionkey,
    /// sgx-tokenkey
    Sgxtokenkey,
    /// sgx1
    Sgx1,
    /// sgx2
    Sgx2,
    /// sgxlc
    Sgxlc,
    /// sha-ni
    Shani,
    /// sha512
    Sha512,
    /// skinit
    Skinit,
    /// skip-l1dfl-vmentry
    Skipl1dflVmentry,
    /// sm3
    Sm3,
    /// sm4
    Sm4,
    /// smap
    Smap,
    /// smep
    Smep,
    /// smx
    Smx,
    /// spec-ctrl
    Specctrl,
    /// split-lock-detect
    Splitlockdetect,
    /// srso-no
    Srsono,
    /// srso-user-kernel-no
    Srsouserkernelno,
    /// ss
    Ss,
    /// ssb-no
    Ssbno,
    /// ssbd
    Ssbd,
    /// sse
    Sse,
    /// sse2
    Sse2,
    /// sse4.1
    Sse4_1,
    /// sse4.2
    Sse4_2,
    /// sse4a
    Sse4a,
    /// ssse3
    Ssse3,
    /// stibp
    Stibp,
    /// stibp-always-on
    Stibpalwayson,
    /// succor
    Succor,
    /// svm
    Svm,
    /// svm-lock
    Svmlock,
    /// svme-addr-chk
    Svmeaddrchk,
    /// syscall
    Syscall,
    /// taa-no
    Taano,
    /// tbm
    Tbm,
    /// tce
    Tce,
    /// tm
    Tm,
    /// tm2
    Tm2,
    /// topoext
    Topoext,
    /// tsc
    Tsc,
    /// tsc-adjust
    Tscadjust,
    /// tsc-deadline
    Tscdeadline,
    /// tsc-scale
    Tscscale,
    /// tsx-ctrl
    Tsxctrl,
    /// tsx-ldtrk
    Tsxldtrk,
    /// umip
    Umip,
    /// v-vmsave-vmload
    VVmsavevmload,
    /// vaes
    Vaes,
    /// vgif
    Vgif,
    /// virt-ssbd
    Virtssbd,
    /// vmcb-clean
    Vmcbclean,
    /// vme
    Vme,
    /// vmx
    Vmx,
    /// vmx-activity-hlt
    Vmxactivityhlt,
    /// vmx-activity-shutdown
    Vmxactivityshutdown,
    /// vmx-activity-wait-sipi
    Vmxactivitywaitsipi,
    /// vmx-any-errcode
    Vmxanyerrcode,
    /// vmx-apicv-register
    Vmxapicvregister,
    /// vmx-apicv-vid
    VmxapicvVid,
    /// vmx-apicv-x2apic
    Vmxapicvx2apic,
    /// vmx-apicv-xapic
    Vmxapicvxapic,
    /// vmx-cr3-load-noexit
    Vmxcr3loadnoexit,
    /// vmx-cr3-store-noexit
    Vmxcr3storenoexit,
    /// vmx-cr8-load-exit
    Vmxcr8loadexit,
    /// vmx-cr8-store-exit
    Vmxcr8storeexit,
    /// vmx-desc-exit
    Vmxdescexit,
    /// vmx-enable-user-wait-pause
    Vmxenableuserwaitpause,
    /// vmx-encls-exit
    Vmxenclsexit,
    /// vmx-entry-ia32e-mode
    Vmxentryia32emode,
    /// vmx-entry-load-bndcfgs
    Vmxentryloadbndcfgs,
    /// vmx-entry-load-efer
    Vmxentryloadefer,
    /// vmx-entry-load-fred
    Vmxentryloadfred,
    /// vmx-entry-load-pat
    Vmxentryloadpat,
    /// vmx-entry-load-perf-global-ctrl
    Vmxentryloadperfglobalctrl,
    /// vmx-entry-load-pkrs
    Vmxentryloadpkrs,
    /// vmx-entry-load-rtit-ctl
    Vmxentryloadrtitctl,
    /// vmx-entry-noload-debugctl
    Vmxentrynoloaddebugctl,
    /// vmx-ept
    Vmxept,
    /// vmx-ept-1gb
    Vmxept1gb,
    /// vmx-ept-2mb
    Vmxept2mb,
    /// vmx-ept-advanced-exitinfo
    Vmxeptadvancedexitinfo,
    /// vmx-ept-execonly
    Vmxeptexeconly,
    /// vmx-eptad
    Vmxeptad,
    /// vmx-eptp-switching
    Vmxeptpswitching,
    /// vmx-exit-ack-intr
    Vmxexitackintr,
    /// vmx-exit-clear-bndcfgs
    Vmxexitclearbndcfgs,
    /// vmx-exit-clear-rtit-ctl
    Vmxexitclearrtitctl,
    /// vmx-exit-load-efer
    Vmxexitloadefer,
    /// vmx-exit-load-pat
    Vmxexitloadpat,
    /// vmx-exit-load-perf-global-ctrl
    Vmxexitloadperfglobalctrl,
    /// vmx-exit-load-pkrs
    Vmxexitloadpkrs,
    /// vmx-exit-nosave-debugctl
    Vmxexitnosavedebugctl,
    /// vmx-exit-save-efer
    Vmxexitsaveefer,
    /// vmx-exit-save-pat
    Vmxexitsavepat,
    /// vmx-exit-save-preemption-timer
    Vmxexitsavepreemptiontimer,
    /// vmx-exit-secondary-ctls
    Vmxexitsecondaryctls,
    /// vmx-flexpriority
    Vmxflexpriority,
    /// vmx-hlt-exit
    Vmxhltexit,
    /// vmx-ins-outs
    Vmxinsouts,
    /// vmx-intr-exit
    Vmxintrexit,
    /// vmx-invept
    Vmxinvept,
    /// vmx-invept-all-context
    Vmxinveptallcontext,
    /// vmx-invept-single-context
    Vmxinveptsinglecontext,
    /// vmx-invept-single-context-noglobals
    Vmxinveptsinglecontextnoglobals,
    /// vmx-invlpg-exit
    Vmxinvlpgexit,
    /// vmx-invpcid-exit
    Vmxinvpcidexit,
    /// vmx-invvpid
    Vmxinvvpid,
    /// vmx-invvpid-all-context
    Vmxinvvpidallcontext,
    /// vmx-invvpid-single-addr
    Vmxinvvpidsingleaddr,
    /// vmx-io-bitmap
    Vmxiobitmap,
    /// vmx-io-exit
    Vmxioexit,
    /// vmx-monitor-exit
    Vmxmonitorexit,
    /// vmx-movdr-exit
    Vmxmovdrexit,
    /// vmx-msr-bitmap
    Vmxmsrbitmap,
    /// vmx-mtf
    Vmxmtf,
    /// vmx-mwait-exit
    Vmxmwaitexit,
    /// vmx-nested-exception
    Vmxnestedexception,
    /// vmx-nmi-exit
    Vmxnmiexit,
    /// vmx-page-walk-4
    Vmxpagewalk4,
    /// vmx-page-walk-5
    Vmxpagewalk5,
    /// vmx-pause-exit
    Vmxpauseexit,
    /// vmx-ple
    Vmxple,
    /// vmx-pml
    Vmxpml,
    /// vmx-posted-intr
    Vmxpostedintr,
    /// vmx-preemption-timer
    Vmxpreemptiontimer,
    /// vmx-rdpmc-exit
    Vmxrdpmcexit,
    /// vmx-rdrand-exit
    Vmxrdrandexit,
    /// vmx-rdseed-exit
    Vmxrdseedexit,
    /// vmx-rdtsc-exit
    Vmxrdtscexit,
    /// vmx-rdtscp-exit
    Vmxrdtscpexit,
    /// vmx-secondary-ctls
    Vmxsecondaryctls,
    /// vmx-shadow-vmcs
    VmxshadowVmcs,
    /// vmx-store-lma
    Vmxstorelma,
    /// vmx-true-ctls
    Vmxtruectls,
    /// vmx-tsc-offset
    Vmxtscoffset,
    /// vmx-tsc-scaling
    Vmxtscscaling,
    /// vmx-unrestricted-guest
    Vmxunrestrictedguest,
    /// vmx-vintr-pending
    VmxVintrpending,
    /// vmx-vmfunc
    VmxVmfunc,
    /// vmx-vmwrite-vmexit-fields
    VmxVmwritevmexitfields,
    /// vmx-vnmi
    VmxVnmi,
    /// vmx-vnmi-pending
    VmxVnmipending,
    /// vmx-vpid
    VmxVpid,
    /// vmx-wbinvd-exit
    Vmxwbinvdexit,
    /// vmx-xsaves
    Vmxxsaves,
    /// vmx-zero-len-inject
    Vmxzeroleninject,
    /// vnmi
    Vnmi,
    /// vpclmulqdq
    Vpclmulqdq,
    /// waitpkg
    Waitpkg,
    /// wbnoinvd
    Wbnoinvd,
    /// wdt
    Wdt,
    /// wrmsrns
    Wrmsrns,
    /// x2apic
    X2apic,
    /// xcrypt
    Xcrypt,
    /// xcrypt-en
    Xcrypten,
    /// xfd
    Xfd,
    /// xgetbv1
    Xgetbv1,
    /// xop
    Xop,
    /// xsave
    Xsave,
    /// xsavec
    Xsavec,
    /// xsaveerptr
    Xsaveerptr,
    /// xsaveopt
    Xsaveopt,
    /// xsaves
    Xsaves,
    /// xstore
    Xstore,
    /// xstore-en
    Xstoreen,
    /// xtpr
    Xtpr,
    /// zero-fcs-fds
    Zerofcsfds,
}

impl ToCommand for CPUFlag {
    fn to_args(&self) -> Vec<String> {
        let mut cmd = vec![];

        match self {
            CPUFlag::X3dnow => cmd.push("3dnow".to_string()),
            CPUFlag::X3dnowext => cmd.push("3dnowext".to_string()),
            CPUFlag::X3dnowprefetch => cmd.push("3dnowprefetch".to_string()),
            CPUFlag::Abm => cmd.push("abm".to_string()),
            CPUFlag::Ace2 => cmd.push("ace2".to_string()),
            CPUFlag::Ace2en => cmd.push("ace2-en".to_string()),
            CPUFlag::Acpi => cmd.push("acpi".to_string()),
            CPUFlag::Adx => cmd.push("adx".to_string()),
            CPUFlag::Aes => cmd.push("aes".to_string()),
            CPUFlag::Amdnossb => cmd.push("amd-no-ssb".to_string()),
            CPUFlag::Amdpsfd => cmd.push("amd-psfd".to_string()),
            CPUFlag::Amdssbd => cmd.push("amd-ssbd".to_string()),
            CPUFlag::Amdstibp => cmd.push("amd-stibp".to_string()),
            CPUFlag::Amxbf16 => cmd.push("amx-bf16".to_string()),
            CPUFlag::Amxcomplex => cmd.push("amx-complex".to_string()),
            CPUFlag::Amxfp16 => cmd.push("amx-fp16".to_string()),
            CPUFlag::Amxint8 => cmd.push("amx-int8".to_string()),
            CPUFlag::Amxtile => cmd.push("amx-tile".to_string()),
            CPUFlag::Apic => cmd.push("apic".to_string()),
            CPUFlag::Arat => cmd.push("arat".to_string()),
            CPUFlag::Archcapabilities => cmd.push("arch-capabilities".to_string()),
            CPUFlag::Archlbr => cmd.push("arch-lbr".to_string()),
            CPUFlag::Autoibrs => cmd.push("auto-ibrs".to_string()),
            CPUFlag::Avic => cmd.push("avic".to_string()),
            CPUFlag::Avx => cmd.push("avx".to_string()),
            CPUFlag::Avxifma => cmd.push("avx-ifma".to_string()),
            CPUFlag::Avxneconvert => cmd.push("avx-ne-convert".to_string()),
            CPUFlag::AvxVnni => cmd.push("avx-vnni".to_string()),
            CPUFlag::AvxVnniint16 => cmd.push("avx-vnni-int16".to_string()),
            CPUFlag::AvxVnniint8 => cmd.push("avx-vnni-int8".to_string()),
            CPUFlag::Avx10 => cmd.push("avx10".to_string()),
            CPUFlag::Avx10128 => cmd.push("avx10-128".to_string()),
            CPUFlag::Avx10256 => cmd.push("avx10-256".to_string()),
            CPUFlag::Avx10512 => cmd.push("avx10-512".to_string()),
            CPUFlag::Avx2 => cmd.push("avx2".to_string()),
            CPUFlag::Avx5124fmaps => cmd.push("avx512-4fmaps".to_string()),
            CPUFlag::Avx5124vnniw => cmd.push("avx512-4vnniw".to_string()),
            CPUFlag::Avx512bf16 => cmd.push("avx512-bf16".to_string()),
            CPUFlag::Avx512fp16 => cmd.push("avx512-fp16".to_string()),
            CPUFlag::Avx512Vp2intersect => cmd.push("avx512-vp2intersect".to_string()),
            CPUFlag::Avx512Vpopcntdq => cmd.push("avx512-vpopcntdq".to_string()),
            CPUFlag::Avx512bitalg => cmd.push("avx512bitalg".to_string()),
            CPUFlag::Avx512bw => cmd.push("avx512bw".to_string()),
            CPUFlag::Avx512cd => cmd.push("avx512cd".to_string()),
            CPUFlag::Avx512dq => cmd.push("avx512dq".to_string()),
            CPUFlag::Avx512er => cmd.push("avx512er".to_string()),
            CPUFlag::Avx512f => cmd.push("avx512f".to_string()),
            CPUFlag::Avx512ifma => cmd.push("avx512ifma".to_string()),
            CPUFlag::Avx512pf => cmd.push("avx512pf".to_string()),
            CPUFlag::Avx512vbmi => cmd.push("avx512vbmi".to_string()),
            CPUFlag::Avx512vbmi2 => cmd.push("avx512vbmi2".to_string()),
            CPUFlag::Avx512vl => cmd.push("avx512vl".to_string()),
            CPUFlag::Avx512vnni => cmd.push("avx512vnni".to_string()),
            CPUFlag::Bhictrl => cmd.push("bhi-ctrl".to_string()),
            CPUFlag::Bhino => cmd.push("bhi-no".to_string()),
            CPUFlag::Bmi1 => cmd.push("bmi1".to_string()),
            CPUFlag::Bmi2 => cmd.push("bmi2".to_string()),
            CPUFlag::Buslockdetect => cmd.push("bus-lock-detect".to_string()),
            CPUFlag::Cid => cmd.push("cid".to_string()),
            CPUFlag::Cldemote => cmd.push("cldemote".to_string()),
            CPUFlag::Clflush => cmd.push("clflush".to_string()),
            CPUFlag::Clflushopt => cmd.push("clflushopt".to_string()),
            CPUFlag::Clwb => cmd.push("clwb".to_string()),
            CPUFlag::Clzero => cmd.push("clzero".to_string()),
            CPUFlag::Cmov => cmd.push("cmov".to_string()),
            CPUFlag::Cmplegacy => cmd.push("cmp-legacy".to_string()),
            CPUFlag::Cmpccxadd => cmd.push("cmpccxadd".to_string()),
            CPUFlag::Corecapability => cmd.push("core-capability".to_string()),
            CPUFlag::Cr8legacy => cmd.push("cr8legacy".to_string()),
            CPUFlag::Cx16 => cmd.push("cx16".to_string()),
            CPUFlag::Cx8 => cmd.push("cx8".to_string()),
            CPUFlag::Dca => cmd.push("dca".to_string()),
            CPUFlag::Ddpdu => cmd.push("ddpd-u".to_string()),
            CPUFlag::De => cmd.push("de".to_string()),
            CPUFlag::Decodeassists => cmd.push("decodeassists".to_string()),
            CPUFlag::Ds => cmd.push("ds".to_string()),
            CPUFlag::Dscpl => cmd.push("ds-cpl".to_string()),
            CPUFlag::Dtes64 => cmd.push("dtes64".to_string()),
            CPUFlag::Eraps => cmd.push("eraps".to_string()),
            CPUFlag::Erms => cmd.push("erms".to_string()),
            CPUFlag::Est => cmd.push("est".to_string()),
            CPUFlag::Extapic => cmd.push("extapic".to_string()),
            CPUFlag::F16c => cmd.push("f16c".to_string()),
            CPUFlag::Fbclear => cmd.push("fb-clear".to_string()),
            CPUFlag::Fbsdpno => cmd.push("fbsdp-no".to_string()),
            CPUFlag::Fdpexcptnonly => cmd.push("fdp-excptn-only".to_string()),
            CPUFlag::Flushl1d => cmd.push("flush-l1d".to_string()),
            CPUFlag::Flushbyasid => cmd.push("flushbyasid".to_string()),
            CPUFlag::Fma => cmd.push("fma".to_string()),
            CPUFlag::Fma4 => cmd.push("fma4".to_string()),
            CPUFlag::Fpu => cmd.push("fpu".to_string()),
            CPUFlag::Fred => cmd.push("fred".to_string()),
            CPUFlag::Fsgsbase => cmd.push("fsgsbase".to_string()),
            CPUFlag::Fsrc => cmd.push("fsrc".to_string()),
            CPUFlag::Fsrm => cmd.push("fsrm".to_string()),
            CPUFlag::Fsrs => cmd.push("fsrs".to_string()),
            CPUFlag::Fullwidthwrite => cmd.push("full-width-write".to_string()),
            CPUFlag::Fxsr => cmd.push("fxsr".to_string()),
            CPUFlag::Fxsropt => cmd.push("fxsr-opt".to_string()),
            CPUFlag::Fzrm => cmd.push("fzrm".to_string()),
            CPUFlag::Gdsno => cmd.push("gds-no".to_string()),
            CPUFlag::Gfni => cmd.push("gfni".to_string()),
            CPUFlag::Hle => cmd.push("hle".to_string()),
            CPUFlag::Ht => cmd.push("ht".to_string()),
            CPUFlag::Hypervisor => cmd.push("hypervisor".to_string()),
            CPUFlag::Ia64 => cmd.push("ia64".to_string()),
            CPUFlag::Ibpb => cmd.push("ibpb".to_string()),
            CPUFlag::Ibpbbrtype => cmd.push("ibpb-brtype".to_string()),
            CPUFlag::Ibrs => cmd.push("ibrs".to_string()),
            CPUFlag::Ibrsall => cmd.push("ibrs-all".to_string()),
            CPUFlag::Ibs => cmd.push("ibs".to_string()),
            CPUFlag::Intelpsfd => cmd.push("intel-psfd".to_string()),
            CPUFlag::Intelpt => cmd.push("intel-pt".to_string()),
            CPUFlag::Intelptlip => cmd.push("intel-pt-lip".to_string()),
            CPUFlag::Invpcid => cmd.push("invpcid".to_string()),
            CPUFlag::Invtsc => cmd.push("invtsc".to_string()),
            CPUFlag::Ipredctrl => cmd.push("ipred-ctrl".to_string()),
            CPUFlag::Kvmasyncpf => cmd.push("kvm-asyncpf".to_string()),
            CPUFlag::Kvmasyncpfint => cmd.push("kvm-asyncpf-int".to_string()),
            CPUFlag::KvmasyncpfVmexit => cmd.push("kvm-asyncpf-vmexit".to_string()),
            CPUFlag::Kvmhintdedicated => cmd.push("kvm-hint-dedicated".to_string()),
            CPUFlag::Kvmmmu => cmd.push("kvm-mmu".to_string()),
            CPUFlag::Kvmmsiextdestid => cmd.push("kvm-msi-ext-dest-id".to_string()),
            CPUFlag::Kvmnopiodelay => cmd.push("kvm-nopiodelay".to_string()),
            CPUFlag::Kvmpollcontrol => cmd.push("kvm-poll-control".to_string()),
            CPUFlag::Kvmpveoi => cmd.push("kvm-pv-eoi".to_string()),
            CPUFlag::Kvmpvipi => cmd.push("kvm-pv-ipi".to_string()),
            CPUFlag::Kvmpvschedyield => cmd.push("kvm-pv-sched-yield".to_string()),
            CPUFlag::Kvmpvtlbflush => cmd.push("kvm-pv-tlb-flush".to_string()),
            CPUFlag::Kvmpvunhalt => cmd.push("kvm-pv-unhalt".to_string()),
            CPUFlag::Kvmstealtime => cmd.push("kvm-steal-time".to_string()),
            CPUFlag::Kvmclock => cmd.push("kvmclock".to_string()),
            CPUFlag::Kvmclockstablebit => cmd.push("kvmclock-stable-bit".to_string()),
            CPUFlag::La57 => cmd.push("la57".to_string()),
            CPUFlag::Lahflm => cmd.push("lahf-lm".to_string()),
            CPUFlag::Lam => cmd.push("lam".to_string()),
            CPUFlag::Lbrv => cmd.push("lbrv".to_string()),
            CPUFlag::Lfencealwaysserializing => cmd.push("lfence-always-serializing".to_string()),
            CPUFlag::Lkgs => cmd.push("lkgs".to_string()),
            CPUFlag::Lm => cmd.push("lm".to_string()),
            CPUFlag::Lwp => cmd.push("lwp".to_string()),
            CPUFlag::Mca => cmd.push("mca".to_string()),
            CPUFlag::Mcdtno => cmd.push("mcdt-no".to_string()),
            CPUFlag::Mce => cmd.push("mce".to_string()),
            CPUFlag::Mdclear => cmd.push("md-clear".to_string()),
            CPUFlag::Mdsno => cmd.push("mds-no".to_string()),
            CPUFlag::Misalignsse => cmd.push("misalignsse".to_string()),
            CPUFlag::Mmx => cmd.push("mmx".to_string()),
            CPUFlag::Mmxext => cmd.push("mmxext".to_string()),
            CPUFlag::Monitor => cmd.push("monitor".to_string()),
            CPUFlag::Movbe => cmd.push("movbe".to_string()),
            CPUFlag::Movdir64b => cmd.push("movdir64b".to_string()),
            CPUFlag::Movdiri => cmd.push("movdiri".to_string()),
            CPUFlag::Mpx => cmd.push("mpx".to_string()),
            CPUFlag::Msr => cmd.push("msr".to_string()),
            CPUFlag::Mtrr => cmd.push("mtrr".to_string()),
            CPUFlag::Nonesteddatabp => cmd.push("no-nested-data-bp".to_string()),
            CPUFlag::Nodeidmsr => cmd.push("nodeid-msr".to_string()),
            CPUFlag::Npt => cmd.push("npt".to_string()),
            CPUFlag::Nripsave => cmd.push("nrip-save".to_string()),
            CPUFlag::Nullselclrbase => cmd.push("null-sel-clr-base".to_string()),
            CPUFlag::Nx => cmd.push("nx".to_string()),
            CPUFlag::Osvw => cmd.push("osvw".to_string()),
            CPUFlag::Overflowrecov => cmd.push("overflow-recov".to_string()),
            CPUFlag::Pae => cmd.push("pae".to_string()),
            CPUFlag::Pat => cmd.push("pat".to_string()),
            CPUFlag::Pausefilter => cmd.push("pause-filter".to_string()),
            CPUFlag::Pbe => cmd.push("pbe".to_string()),
            CPUFlag::Pbrsbno => cmd.push("pbrsb-no".to_string()),
            CPUFlag::Pcid => cmd.push("pcid".to_string()),
            CPUFlag::Pclmulqdq => cmd.push("pclmulqdq".to_string()),
            CPUFlag::Pcommit => cmd.push("pcommit".to_string()),
            CPUFlag::Pdcm => cmd.push("pdcm".to_string()),
            CPUFlag::Pdpe1gb => cmd.push("pdpe1gb".to_string()),
            CPUFlag::Perfctrcore => cmd.push("perfctr-core".to_string()),
            CPUFlag::Perfctrnb => cmd.push("perfctr-nb".to_string()),
            CPUFlag::PerfmonV2 => cmd.push("perfmon-v2".to_string()),
            CPUFlag::Pfthreshold => cmd.push("pfthreshold".to_string()),
            CPUFlag::Pge => cmd.push("pge".to_string()),
            CPUFlag::Phe => cmd.push("phe".to_string()),
            CPUFlag::Pheen => cmd.push("phe-en".to_string()),
            CPUFlag::Pks => cmd.push("pks".to_string()),
            CPUFlag::Pku => cmd.push("pku".to_string()),
            CPUFlag::Pmm => cmd.push("pmm".to_string()),
            CPUFlag::Pmmen => cmd.push("pmm-en".to_string()),
            CPUFlag::Pn => cmd.push("pn".to_string()),
            CPUFlag::Pni => cmd.push("pni".to_string()),
            CPUFlag::Popcnt => cmd.push("popcnt".to_string()),
            CPUFlag::Prefetchiti => cmd.push("prefetchiti".to_string()),
            CPUFlag::Pschangemcno => cmd.push("pschange-mc-no".to_string()),
            CPUFlag::Psdpno => cmd.push("psdp-no".to_string()),
            CPUFlag::Pse => cmd.push("pse".to_string()),
            CPUFlag::Pse36 => cmd.push("pse36".to_string()),
            CPUFlag::Rdctlno => cmd.push("rdctl-no".to_string()),
            CPUFlag::Rdpid => cmd.push("rdpid".to_string()),
            CPUFlag::Rdrand => cmd.push("rdrand".to_string()),
            CPUFlag::Rdseed => cmd.push("rdseed".to_string()),
            CPUFlag::Rdtscp => cmd.push("rdtscp".to_string()),
            CPUFlag::Rfdsclear => cmd.push("rfds-clear".to_string()),
            CPUFlag::Rfdsno => cmd.push("rfds-no".to_string()),
            CPUFlag::Rrsbactrl => cmd.push("rrsba-ctrl".to_string()),
            CPUFlag::Rsba => cmd.push("rsba".to_string()),
            CPUFlag::Rtm => cmd.push("rtm".to_string()),
            CPUFlag::Sbdrssdpno => cmd.push("sbdr-ssdp-no".to_string()),
            CPUFlag::Sbpb => cmd.push("sbpb".to_string()),
            CPUFlag::Sep => cmd.push("sep".to_string()),
            CPUFlag::Serialize => cmd.push("serialize".to_string()),
            CPUFlag::Sgx => cmd.push("sgx".to_string()),
            CPUFlag::Sgxaexnotify => cmd.push("sgx-aex-notify".to_string()),
            CPUFlag::Sgxdebug => cmd.push("sgx-debug".to_string()),
            CPUFlag::Sgxedeccssa => cmd.push("sgx-edeccssa".to_string()),
            CPUFlag::Sgxexinfo => cmd.push("sgx-exinfo".to_string()),
            CPUFlag::Sgxkss => cmd.push("sgx-kss".to_string()),
            CPUFlag::Sgxmode64 => cmd.push("sgx-mode64".to_string()),
            CPUFlag::Sgxprovisionkey => cmd.push("sgx-provisionkey".to_string()),
            CPUFlag::Sgxtokenkey => cmd.push("sgx-tokenkey".to_string()),
            CPUFlag::Sgx1 => cmd.push("sgx1".to_string()),
            CPUFlag::Sgx2 => cmd.push("sgx2".to_string()),
            CPUFlag::Sgxlc => cmd.push("sgxlc".to_string()),
            CPUFlag::Shani => cmd.push("sha-ni".to_string()),
            CPUFlag::Sha512 => cmd.push("sha512".to_string()),
            CPUFlag::Skinit => cmd.push("skinit".to_string()),
            CPUFlag::Skipl1dflVmentry => cmd.push("skip-l1dfl-vmentry".to_string()),
            CPUFlag::Sm3 => cmd.push("sm3".to_string()),
            CPUFlag::Sm4 => cmd.push("sm4".to_string()),
            CPUFlag::Smap => cmd.push("smap".to_string()),
            CPUFlag::Smep => cmd.push("smep".to_string()),
            CPUFlag::Smx => cmd.push("smx".to_string()),
            CPUFlag::Specctrl => cmd.push("spec-ctrl".to_string()),
            CPUFlag::Splitlockdetect => cmd.push("split-lock-detect".to_string()),
            CPUFlag::Srsono => cmd.push("srso-no".to_string()),
            CPUFlag::Srsouserkernelno => cmd.push("srso-user-kernel-no".to_string()),
            CPUFlag::Ss => cmd.push("ss".to_string()),
            CPUFlag::Ssbno => cmd.push("ssb-no".to_string()),
            CPUFlag::Ssbd => cmd.push("ssbd".to_string()),
            CPUFlag::Sse => cmd.push("sse".to_string()),
            CPUFlag::Sse2 => cmd.push("sse2".to_string()),
            CPUFlag::Sse4_1 => cmd.push("sse4.1".to_string()),
            CPUFlag::Sse4_2 => cmd.push("sse4.2".to_string()),
            CPUFlag::Sse4a => cmd.push("sse4a".to_string()),
            CPUFlag::Ssse3 => cmd.push("ssse3".to_string()),
            CPUFlag::Stibp => cmd.push("stibp".to_string()),
            CPUFlag::Stibpalwayson => cmd.push("stibp-always-on".to_string()),
            CPUFlag::Succor => cmd.push("succor".to_string()),
            CPUFlag::Svm => cmd.push("svm".to_string()),
            CPUFlag::Svmlock => cmd.push("svm-lock".to_string()),
            CPUFlag::Svmeaddrchk => cmd.push("svme-addr-chk".to_string()),
            CPUFlag::Syscall => cmd.push("syscall".to_string()),
            CPUFlag::Taano => cmd.push("taa-no".to_string()),
            CPUFlag::Tbm => cmd.push("tbm".to_string()),
            CPUFlag::Tce => cmd.push("tce".to_string()),
            CPUFlag::Tm => cmd.push("tm".to_string()),
            CPUFlag::Tm2 => cmd.push("tm2".to_string()),
            CPUFlag::Topoext => cmd.push("topoext".to_string()),
            CPUFlag::Tsc => cmd.push("tsc".to_string()),
            CPUFlag::Tscadjust => cmd.push("tsc-adjust".to_string()),
            CPUFlag::Tscdeadline => cmd.push("tsc-deadline".to_string()),
            CPUFlag::Tscscale => cmd.push("tsc-scale".to_string()),
            CPUFlag::Tsxctrl => cmd.push("tsx-ctrl".to_string()),
            CPUFlag::Tsxldtrk => cmd.push("tsx-ldtrk".to_string()),
            CPUFlag::Umip => cmd.push("umip".to_string()),
            CPUFlag::VVmsavevmload => cmd.push("v-vmsave-vmload".to_string()),
            CPUFlag::Vaes => cmd.push("vaes".to_string()),
            CPUFlag::Vgif => cmd.push("vgif".to_string()),
            CPUFlag::Virtssbd => cmd.push("virt-ssbd".to_string()),
            CPUFlag::Vmcbclean => cmd.push("vmcb-clean".to_string()),
            CPUFlag::Vme => cmd.push("vme".to_string()),
            CPUFlag::Vmx => cmd.push("vmx".to_string()),
            CPUFlag::Vmxactivityhlt => cmd.push("vmx-activity-hlt".to_string()),
            CPUFlag::Vmxactivityshutdown => cmd.push("vmx-activity-shutdown".to_string()),
            CPUFlag::Vmxactivitywaitsipi => cmd.push("vmx-activity-wait-sipi".to_string()),
            CPUFlag::Vmxanyerrcode => cmd.push("vmx-any-errcode".to_string()),
            CPUFlag::Vmxapicvregister => cmd.push("vmx-apicv-register".to_string()),
            CPUFlag::VmxapicvVid => cmd.push("vmx-apicv-vid".to_string()),
            CPUFlag::Vmxapicvx2apic => cmd.push("vmx-apicv-x2apic".to_string()),
            CPUFlag::Vmxapicvxapic => cmd.push("vmx-apicv-xapic".to_string()),
            CPUFlag::Vmxcr3loadnoexit => cmd.push("vmx-cr3-load-noexit".to_string()),
            CPUFlag::Vmxcr3storenoexit => cmd.push("vmx-cr3-store-noexit".to_string()),
            CPUFlag::Vmxcr8loadexit => cmd.push("vmx-cr8-load-exit".to_string()),
            CPUFlag::Vmxcr8storeexit => cmd.push("vmx-cr8-store-exit".to_string()),
            CPUFlag::Vmxdescexit => cmd.push("vmx-desc-exit".to_string()),
            CPUFlag::Vmxenableuserwaitpause => cmd.push("vmx-enable-user-wait-pause".to_string()),
            CPUFlag::Vmxenclsexit => cmd.push("vmx-encls-exit".to_string()),
            CPUFlag::Vmxentryia32emode => cmd.push("vmx-entry-ia32e-mode".to_string()),
            CPUFlag::Vmxentryloadbndcfgs => cmd.push("vmx-entry-load-bndcfgs".to_string()),
            CPUFlag::Vmxentryloadefer => cmd.push("vmx-entry-load-efer".to_string()),
            CPUFlag::Vmxentryloadfred => cmd.push("vmx-entry-load-fred".to_string()),
            CPUFlag::Vmxentryloadpat => cmd.push("vmx-entry-load-pat".to_string()),
            CPUFlag::Vmxentryloadperfglobalctrl => cmd.push("vmx-entry-load-perf-global-ctrl".to_string()),
            CPUFlag::Vmxentryloadpkrs => cmd.push("vmx-entry-load-pkrs".to_string()),
            CPUFlag::Vmxentryloadrtitctl => cmd.push("vmx-entry-load-rtit-ctl".to_string()),
            CPUFlag::Vmxentrynoloaddebugctl => cmd.push("vmx-entry-noload-debugctl".to_string()),
            CPUFlag::Vmxept => cmd.push("vmx-ept".to_string()),
            CPUFlag::Vmxept1gb => cmd.push("vmx-ept-1gb".to_string()),
            CPUFlag::Vmxept2mb => cmd.push("vmx-ept-2mb".to_string()),
            CPUFlag::Vmxeptadvancedexitinfo => cmd.push("vmx-ept-advanced-exitinfo".to_string()),
            CPUFlag::Vmxeptexeconly => cmd.push("vmx-ept-execonly".to_string()),
            CPUFlag::Vmxeptad => cmd.push("vmx-eptad".to_string()),
            CPUFlag::Vmxeptpswitching => cmd.push("vmx-eptp-switching".to_string()),
            CPUFlag::Vmxexitackintr => cmd.push("vmx-exit-ack-intr".to_string()),
            CPUFlag::Vmxexitclearbndcfgs => cmd.push("vmx-exit-clear-bndcfgs".to_string()),
            CPUFlag::Vmxexitclearrtitctl => cmd.push("vmx-exit-clear-rtit-ctl".to_string()),
            CPUFlag::Vmxexitloadefer => cmd.push("vmx-exit-load-efer".to_string()),
            CPUFlag::Vmxexitloadpat => cmd.push("vmx-exit-load-pat".to_string()),
            CPUFlag::Vmxexitloadperfglobalctrl => cmd.push("vmx-exit-load-perf-global-ctrl".to_string()),
            CPUFlag::Vmxexitloadpkrs => cmd.push("vmx-exit-load-pkrs".to_string()),
            CPUFlag::Vmxexitnosavedebugctl => cmd.push("vmx-exit-nosave-debugctl".to_string()),
            CPUFlag::Vmxexitsaveefer => cmd.push("vmx-exit-save-efer".to_string()),
            CPUFlag::Vmxexitsavepat => cmd.push("vmx-exit-save-pat".to_string()),
            CPUFlag::Vmxexitsavepreemptiontimer => cmd.push("vmx-exit-save-preemption-timer".to_string()),
            CPUFlag::Vmxexitsecondaryctls => cmd.push("vmx-exit-secondary-ctls".to_string()),
            CPUFlag::Vmxflexpriority => cmd.push("vmx-flexpriority".to_string()),
            CPUFlag::Vmxhltexit => cmd.push("vmx-hlt-exit".to_string()),
            CPUFlag::Vmxinsouts => cmd.push("vmx-ins-outs".to_string()),
            CPUFlag::Vmxintrexit => cmd.push("vmx-intr-exit".to_string()),
            CPUFlag::Vmxinvept => cmd.push("vmx-invept".to_string()),
            CPUFlag::Vmxinveptallcontext => cmd.push("vmx-invept-all-context".to_string()),
            CPUFlag::Vmxinveptsinglecontext => cmd.push("vmx-invept-single-context".to_string()),
            CPUFlag::Vmxinveptsinglecontextnoglobals => cmd.push("vmx-invept-single-context-noglobals".to_string()),
            CPUFlag::Vmxinvlpgexit => cmd.push("vmx-invlpg-exit".to_string()),
            CPUFlag::Vmxinvpcidexit => cmd.push("vmx-invpcid-exit".to_string()),
            CPUFlag::Vmxinvvpid => cmd.push("vmx-invvpid".to_string()),
            CPUFlag::Vmxinvvpidallcontext => cmd.push("vmx-invvpid-all-context".to_string()),
            CPUFlag::Vmxinvvpidsingleaddr => cmd.push("vmx-invvpid-single-addr".to_string()),
            CPUFlag::Vmxiobitmap => cmd.push("vmx-io-bitmap".to_string()),
            CPUFlag::Vmxioexit => cmd.push("vmx-io-exit".to_string()),
            CPUFlag::Vmxmonitorexit => cmd.push("vmx-monitor-exit".to_string()),
            CPUFlag::Vmxmovdrexit => cmd.push("vmx-movdr-exit".to_string()),
            CPUFlag::Vmxmsrbitmap => cmd.push("vmx-msr-bitmap".to_string()),
            CPUFlag::Vmxmtf => cmd.push("vmx-mtf".to_string()),
            CPUFlag::Vmxmwaitexit => cmd.push("vmx-mwait-exit".to_string()),
            CPUFlag::Vmxnestedexception => cmd.push("vmx-nested-exception".to_string()),
            CPUFlag::Vmxnmiexit => cmd.push("vmx-nmi-exit".to_string()),
            CPUFlag::Vmxpagewalk4 => cmd.push("vmx-page-walk-4".to_string()),
            CPUFlag::Vmxpagewalk5 => cmd.push("vmx-page-walk-5".to_string()),
            CPUFlag::Vmxpauseexit => cmd.push("vmx-pause-exit".to_string()),
            CPUFlag::Vmxple => cmd.push("vmx-ple".to_string()),
            CPUFlag::Vmxpml => cmd.push("vmx-pml".to_string()),
            CPUFlag::Vmxpostedintr => cmd.push("vmx-posted-intr".to_string()),
            CPUFlag::Vmxpreemptiontimer => cmd.push("vmx-preemption-timer".to_string()),
            CPUFlag::Vmxrdpmcexit => cmd.push("vmx-rdpmc-exit".to_string()),
            CPUFlag::Vmxrdrandexit => cmd.push("vmx-rdrand-exit".to_string()),
            CPUFlag::Vmxrdseedexit => cmd.push("vmx-rdseed-exit".to_string()),
            CPUFlag::Vmxrdtscexit => cmd.push("vmx-rdtsc-exit".to_string()),
            CPUFlag::Vmxrdtscpexit => cmd.push("vmx-rdtscp-exit".to_string()),
            CPUFlag::Vmxsecondaryctls => cmd.push("vmx-secondary-ctls".to_string()),
            CPUFlag::VmxshadowVmcs => cmd.push("vmx-shadow-vmcs".to_string()),
            CPUFlag::Vmxstorelma => cmd.push("vmx-store-lma".to_string()),
            CPUFlag::Vmxtruectls => cmd.push("vmx-true-ctls".to_string()),
            CPUFlag::Vmxtscoffset => cmd.push("vmx-tsc-offset".to_string()),
            CPUFlag::Vmxtscscaling => cmd.push("vmx-tsc-scaling".to_string()),
            CPUFlag::Vmxunrestrictedguest => cmd.push("vmx-unrestricted-guest".to_string()),
            CPUFlag::VmxVintrpending => cmd.push("vmx-vintr-pending".to_string()),
            CPUFlag::VmxVmfunc => cmd.push("vmx-vmfunc".to_string()),
            CPUFlag::VmxVmwritevmexitfields => cmd.push("vmx-vmwrite-vmexit-fields".to_string()),
            CPUFlag::VmxVnmi => cmd.push("vmx-vnmi".to_string()),
            CPUFlag::VmxVnmipending => cmd.push("vmx-vnmi-pending".to_string()),
            CPUFlag::VmxVpid => cmd.push("vmx-vpid".to_string()),
            CPUFlag::Vmxwbinvdexit => cmd.push("vmx-wbinvd-exit".to_string()),
            CPUFlag::Vmxxsaves => cmd.push("vmx-xsaves".to_string()),
            CPUFlag::Vmxzeroleninject => cmd.push("vmx-zero-len-inject".to_string()),
            CPUFlag::Vnmi => cmd.push("vnmi".to_string()),
            CPUFlag::Vpclmulqdq => cmd.push("vpclmulqdq".to_string()),
            CPUFlag::Waitpkg => cmd.push("waitpkg".to_string()),
            CPUFlag::Wbnoinvd => cmd.push("wbnoinvd".to_string()),
            CPUFlag::Wdt => cmd.push("wdt".to_string()),
            CPUFlag::Wrmsrns => cmd.push("wrmsrns".to_string()),
            CPUFlag::X2apic => cmd.push("x2apic".to_string()),
            CPUFlag::Xcrypt => cmd.push("xcrypt".to_string()),
            CPUFlag::Xcrypten => cmd.push("xcrypt-en".to_string()),
            CPUFlag::Xfd => cmd.push("xfd".to_string()),
            CPUFlag::Xgetbv1 => cmd.push("xgetbv1".to_string()),
            CPUFlag::Xop => cmd.push("xop".to_string()),
            CPUFlag::Xsave => cmd.push("xsave".to_string()),
            CPUFlag::Xsavec => cmd.push("xsavec".to_string()),
            CPUFlag::Xsaveerptr => cmd.push("xsaveerptr".to_string()),
            CPUFlag::Xsaveopt => cmd.push("xsaveopt".to_string()),
            CPUFlag::Xsaves => cmd.push("xsaves".to_string()),
            CPUFlag::Xstore => cmd.push("xstore".to_string()),
            CPUFlag::Xstoreen => cmd.push("xstore-en".to_string()),
            CPUFlag::Xtpr => cmd.push("xtpr".to_string()),
            CPUFlag::Zerofcsfds => cmd.push("zero-fcs-fds".to_string()),
        }
        cmd
    }
}

impl ToArg for CPUFlag {
    fn to_arg(&self) -> &str {
        match self {
            CPUFlag::X3dnow => "3dnow",
            CPUFlag::X3dnowext => "3dnowext",
            CPUFlag::X3dnowprefetch => "3dnowprefetch",
            CPUFlag::Abm => "abm",
            CPUFlag::Ace2 => "ace2",
            CPUFlag::Ace2en => "ace2-en",
            CPUFlag::Acpi => "acpi",
            CPUFlag::Adx => "adx",
            CPUFlag::Aes => "aes",
            CPUFlag::Amdnossb => "amd-no-ssb",
            CPUFlag::Amdpsfd => "amd-psfd",
            CPUFlag::Amdssbd => "amd-ssbd",
            CPUFlag::Amdstibp => "amd-stibp",
            CPUFlag::Amxbf16 => "amx-bf16",
            CPUFlag::Amxcomplex => "amx-complex",
            CPUFlag::Amxfp16 => "amx-fp16",
            CPUFlag::Amxint8 => "amx-int8",
            CPUFlag::Amxtile => "amx-tile",
            CPUFlag::Apic => "apic",
            CPUFlag::Arat => "arat",
            CPUFlag::Archcapabilities => "arch-capabilities",
            CPUFlag::Archlbr => "arch-lbr",
            CPUFlag::Autoibrs => "auto-ibrs",
            CPUFlag::Avic => "avic",
            CPUFlag::Avx => "avx",
            CPUFlag::Avxifma => "avx-ifma",
            CPUFlag::Avxneconvert => "avx-ne-convert",
            CPUFlag::AvxVnni => "avx-vnni",
            CPUFlag::AvxVnniint16 => "avx-vnni-int16",
            CPUFlag::AvxVnniint8 => "avx-vnni-int8",
            CPUFlag::Avx10 => "avx10",
            CPUFlag::Avx10128 => "avx10-128",
            CPUFlag::Avx10256 => "avx10-256",
            CPUFlag::Avx10512 => "avx10-512",
            CPUFlag::Avx2 => "avx2",
            CPUFlag::Avx5124fmaps => "avx512-4fmaps",
            CPUFlag::Avx5124vnniw => "avx512-4vnniw",
            CPUFlag::Avx512bf16 => "avx512-bf16",
            CPUFlag::Avx512fp16 => "avx512-fp16",
            CPUFlag::Avx512Vp2intersect => "avx512-vp2intersect",
            CPUFlag::Avx512Vpopcntdq => "avx512-vpopcntdq",
            CPUFlag::Avx512bitalg => "avx512bitalg",
            CPUFlag::Avx512bw => "avx512bw",
            CPUFlag::Avx512cd => "avx512cd",
            CPUFlag::Avx512dq => "avx512dq",
            CPUFlag::Avx512er => "avx512er",
            CPUFlag::Avx512f => "avx512f",
            CPUFlag::Avx512ifma => "avx512ifma",
            CPUFlag::Avx512pf => "avx512pf",
            CPUFlag::Avx512vbmi => "avx512vbmi",
            CPUFlag::Avx512vbmi2 => "avx512vbmi2",
            CPUFlag::Avx512vl => "avx512vl",
            CPUFlag::Avx512vnni => "avx512vnni",
            CPUFlag::Bhictrl => "bhi-ctrl",
            CPUFlag::Bhino => "bhi-no",
            CPUFlag::Bmi1 => "bmi1",
            CPUFlag::Bmi2 => "bmi2",
            CPUFlag::Buslockdetect => "bus-lock-detect",
            CPUFlag::Cid => "cid",
            CPUFlag::Cldemote => "cldemote",
            CPUFlag::Clflush => "clflush",
            CPUFlag::Clflushopt => "clflushopt",
            CPUFlag::Clwb => "clwb",
            CPUFlag::Clzero => "clzero",
            CPUFlag::Cmov => "cmov",
            CPUFlag::Cmplegacy => "cmp-legacy",
            CPUFlag::Cmpccxadd => "cmpccxadd",
            CPUFlag::Corecapability => "core-capability",
            CPUFlag::Cr8legacy => "cr8legacy",
            CPUFlag::Cx16 => "cx16",
            CPUFlag::Cx8 => "cx8",
            CPUFlag::Dca => "dca",
            CPUFlag::Ddpdu => "ddpd-u",
            CPUFlag::De => "de",
            CPUFlag::Decodeassists => "decodeassists",
            CPUFlag::Ds => "ds",
            CPUFlag::Dscpl => "ds-cpl",
            CPUFlag::Dtes64 => "dtes64",
            CPUFlag::Eraps => "eraps",
            CPUFlag::Erms => "erms",
            CPUFlag::Est => "est",
            CPUFlag::Extapic => "extapic",
            CPUFlag::F16c => "f16c",
            CPUFlag::Fbclear => "fb-clear",
            CPUFlag::Fbsdpno => "fbsdp-no",
            CPUFlag::Fdpexcptnonly => "fdp-excptn-only",
            CPUFlag::Flushl1d => "flush-l1d",
            CPUFlag::Flushbyasid => "flushbyasid",
            CPUFlag::Fma => "fma",
            CPUFlag::Fma4 => "fma4",
            CPUFlag::Fpu => "fpu",
            CPUFlag::Fred => "fred",
            CPUFlag::Fsgsbase => "fsgsbase",
            CPUFlag::Fsrc => "fsrc",
            CPUFlag::Fsrm => "fsrm",
            CPUFlag::Fsrs => "fsrs",
            CPUFlag::Fullwidthwrite => "full-width-write",
            CPUFlag::Fxsr => "fxsr",
            CPUFlag::Fxsropt => "fxsr-opt",
            CPUFlag::Fzrm => "fzrm",
            CPUFlag::Gdsno => "gds-no",
            CPUFlag::Gfni => "gfni",
            CPUFlag::Hle => "hle",
            CPUFlag::Ht => "ht",
            CPUFlag::Hypervisor => "hypervisor",
            CPUFlag::Ia64 => "ia64",
            CPUFlag::Ibpb => "ibpb",
            CPUFlag::Ibpbbrtype => "ibpb-brtype",
            CPUFlag::Ibrs => "ibrs",
            CPUFlag::Ibrsall => "ibrs-all",
            CPUFlag::Ibs => "ibs",
            CPUFlag::Intelpsfd => "intel-psfd",
            CPUFlag::Intelpt => "intel-pt",
            CPUFlag::Intelptlip => "intel-pt-lip",
            CPUFlag::Invpcid => "invpcid",
            CPUFlag::Invtsc => "invtsc",
            CPUFlag::Ipredctrl => "ipred-ctrl",
            CPUFlag::Kvmasyncpf => "kvm-asyncpf",
            CPUFlag::Kvmasyncpfint => "kvm-asyncpf-int",
            CPUFlag::KvmasyncpfVmexit => "kvm-asyncpf-vmexit",
            CPUFlag::Kvmhintdedicated => "kvm-hint-dedicated",
            CPUFlag::Kvmmmu => "kvm-mmu",
            CPUFlag::Kvmmsiextdestid => "kvm-msi-ext-dest-id",
            CPUFlag::Kvmnopiodelay => "kvm-nopiodelay",
            CPUFlag::Kvmpollcontrol => "kvm-poll-control",
            CPUFlag::Kvmpveoi => "kvm-pv-eoi",
            CPUFlag::Kvmpvipi => "kvm-pv-ipi",
            CPUFlag::Kvmpvschedyield => "kvm-pv-sched-yield",
            CPUFlag::Kvmpvtlbflush => "kvm-pv-tlb-flush",
            CPUFlag::Kvmpvunhalt => "kvm-pv-unhalt",
            CPUFlag::Kvmstealtime => "kvm-steal-time",
            CPUFlag::Kvmclock => "kvmclock",
            CPUFlag::Kvmclockstablebit => "kvmclock-stable-bit",
            CPUFlag::La57 => "la57",
            CPUFlag::Lahflm => "lahf-lm",
            CPUFlag::Lam => "lam",
            CPUFlag::Lbrv => "lbrv",
            CPUFlag::Lfencealwaysserializing => "lfence-always-serializing",
            CPUFlag::Lkgs => "lkgs",
            CPUFlag::Lm => "lm",
            CPUFlag::Lwp => "lwp",
            CPUFlag::Mca => "mca",
            CPUFlag::Mcdtno => "mcdt-no",
            CPUFlag::Mce => "mce",
            CPUFlag::Mdclear => "md-clear",
            CPUFlag::Mdsno => "mds-no",
            CPUFlag::Misalignsse => "misalignsse",
            CPUFlag::Mmx => "mmx",
            CPUFlag::Mmxext => "mmxext",
            CPUFlag::Monitor => "monitor",
            CPUFlag::Movbe => "movbe",
            CPUFlag::Movdir64b => "movdir64b",
            CPUFlag::Movdiri => "movdiri",
            CPUFlag::Mpx => "mpx",
            CPUFlag::Msr => "msr",
            CPUFlag::Mtrr => "mtrr",
            CPUFlag::Nonesteddatabp => "no-nested-data-bp",
            CPUFlag::Nodeidmsr => "nodeid-msr",
            CPUFlag::Npt => "npt",
            CPUFlag::Nripsave => "nrip-save",
            CPUFlag::Nullselclrbase => "null-sel-clr-base",
            CPUFlag::Nx => "nx",
            CPUFlag::Osvw => "osvw",
            CPUFlag::Overflowrecov => "overflow-recov",
            CPUFlag::Pae => "pae",
            CPUFlag::Pat => "pat",
            CPUFlag::Pausefilter => "pause-filter",
            CPUFlag::Pbe => "pbe",
            CPUFlag::Pbrsbno => "pbrsb-no",
            CPUFlag::Pcid => "pcid",
            CPUFlag::Pclmulqdq => "pclmulqdq",
            CPUFlag::Pcommit => "pcommit",
            CPUFlag::Pdcm => "pdcm",
            CPUFlag::Pdpe1gb => "pdpe1gb",
            CPUFlag::Perfctrcore => "perfctr-core",
            CPUFlag::Perfctrnb => "perfctr-nb",
            CPUFlag::PerfmonV2 => "perfmon-v2",
            CPUFlag::Pfthreshold => "pfthreshold",
            CPUFlag::Pge => "pge",
            CPUFlag::Phe => "phe",
            CPUFlag::Pheen => "phe-en",
            CPUFlag::Pks => "pks",
            CPUFlag::Pku => "pku",
            CPUFlag::Pmm => "pmm",
            CPUFlag::Pmmen => "pmm-en",
            CPUFlag::Pn => "pn",
            CPUFlag::Pni => "pni",
            CPUFlag::Popcnt => "popcnt",
            CPUFlag::Prefetchiti => "prefetchiti",
            CPUFlag::Pschangemcno => "pschange-mc-no",
            CPUFlag::Psdpno => "psdp-no",
            CPUFlag::Pse => "pse",
            CPUFlag::Pse36 => "pse36",
            CPUFlag::Rdctlno => "rdctl-no",
            CPUFlag::Rdpid => "rdpid",
            CPUFlag::Rdrand => "rdrand",
            CPUFlag::Rdseed => "rdseed",
            CPUFlag::Rdtscp => "rdtscp",
            CPUFlag::Rfdsclear => "rfds-clear",
            CPUFlag::Rfdsno => "rfds-no",
            CPUFlag::Rrsbactrl => "rrsba-ctrl",
            CPUFlag::Rsba => "rsba",
            CPUFlag::Rtm => "rtm",
            CPUFlag::Sbdrssdpno => "sbdr-ssdp-no",
            CPUFlag::Sbpb => "sbpb",
            CPUFlag::Sep => "sep",
            CPUFlag::Serialize => "serialize",
            CPUFlag::Sgx => "sgx",
            CPUFlag::Sgxaexnotify => "sgx-aex-notify",
            CPUFlag::Sgxdebug => "sgx-debug",
            CPUFlag::Sgxedeccssa => "sgx-edeccssa",
            CPUFlag::Sgxexinfo => "sgx-exinfo",
            CPUFlag::Sgxkss => "sgx-kss",
            CPUFlag::Sgxmode64 => "sgx-mode64",
            CPUFlag::Sgxprovisionkey => "sgx-provisionkey",
            CPUFlag::Sgxtokenkey => "sgx-tokenkey",
            CPUFlag::Sgx1 => "sgx1",
            CPUFlag::Sgx2 => "sgx2",
            CPUFlag::Sgxlc => "sgxlc",
            CPUFlag::Shani => "sha-ni",
            CPUFlag::Sha512 => "sha512",
            CPUFlag::Skinit => "skinit",
            CPUFlag::Skipl1dflVmentry => "skip-l1dfl-vmentry",
            CPUFlag::Sm3 => "sm3",
            CPUFlag::Sm4 => "sm4",
            CPUFlag::Smap => "smap",
            CPUFlag::Smep => "smep",
            CPUFlag::Smx => "smx",
            CPUFlag::Specctrl => "spec-ctrl",
            CPUFlag::Splitlockdetect => "split-lock-detect",
            CPUFlag::Srsono => "srso-no",
            CPUFlag::Srsouserkernelno => "srso-user-kernel-no",
            CPUFlag::Ss => "ss",
            CPUFlag::Ssbno => "ssb-no",
            CPUFlag::Ssbd => "ssbd",
            CPUFlag::Sse => "sse",
            CPUFlag::Sse2 => "sse2",
            CPUFlag::Sse4_1 => "sse4.1",
            CPUFlag::Sse4_2 => "sse4.2",
            CPUFlag::Sse4a => "sse4a",
            CPUFlag::Ssse3 => "ssse3",
            CPUFlag::Stibp => "stibp",
            CPUFlag::Stibpalwayson => "stibp-always-on",
            CPUFlag::Succor => "succor",
            CPUFlag::Svm => "svm",
            CPUFlag::Svmlock => "svm-lock",
            CPUFlag::Svmeaddrchk => "svme-addr-chk",
            CPUFlag::Syscall => "syscall",
            CPUFlag::Taano => "taa-no",
            CPUFlag::Tbm => "tbm",
            CPUFlag::Tce => "tce",
            CPUFlag::Tm => "tm",
            CPUFlag::Tm2 => "tm2",
            CPUFlag::Topoext => "topoext",
            CPUFlag::Tsc => "tsc",
            CPUFlag::Tscadjust => "tsc-adjust",
            CPUFlag::Tscdeadline => "tsc-deadline",
            CPUFlag::Tscscale => "tsc-scale",
            CPUFlag::Tsxctrl => "tsx-ctrl",
            CPUFlag::Tsxldtrk => "tsx-ldtrk",
            CPUFlag::Umip => "umip",
            CPUFlag::VVmsavevmload => "v-vmsave-vmload",
            CPUFlag::Vaes => "vaes",
            CPUFlag::Vgif => "vgif",
            CPUFlag::Virtssbd => "virt-ssbd",
            CPUFlag::Vmcbclean => "vmcb-clean",
            CPUFlag::Vme => "vme",
            CPUFlag::Vmx => "vmx",
            CPUFlag::Vmxactivityhlt => "vmx-activity-hlt",
            CPUFlag::Vmxactivityshutdown => "vmx-activity-shutdown",
            CPUFlag::Vmxactivitywaitsipi => "vmx-activity-wait-sipi",
            CPUFlag::Vmxanyerrcode => "vmx-any-errcode",
            CPUFlag::Vmxapicvregister => "vmx-apicv-register",
            CPUFlag::VmxapicvVid => "vmx-apicv-vid",
            CPUFlag::Vmxapicvx2apic => "vmx-apicv-x2apic",
            CPUFlag::Vmxapicvxapic => "vmx-apicv-xapic",
            CPUFlag::Vmxcr3loadnoexit => "vmx-cr3-load-noexit",
            CPUFlag::Vmxcr3storenoexit => "vmx-cr3-store-noexit",
            CPUFlag::Vmxcr8loadexit => "vmx-cr8-load-exit",
            CPUFlag::Vmxcr8storeexit => "vmx-cr8-store-exit",
            CPUFlag::Vmxdescexit => "vmx-desc-exit",
            CPUFlag::Vmxenableuserwaitpause => "vmx-enable-user-wait-pause",
            CPUFlag::Vmxenclsexit => "vmx-encls-exit",
            CPUFlag::Vmxentryia32emode => "vmx-entry-ia32e-mode",
            CPUFlag::Vmxentryloadbndcfgs => "vmx-entry-load-bndcfgs",
            CPUFlag::Vmxentryloadefer => "vmx-entry-load-efer",
            CPUFlag::Vmxentryloadfred => "vmx-entry-load-fred",
            CPUFlag::Vmxentryloadpat => "vmx-entry-load-pat",
            CPUFlag::Vmxentryloadperfglobalctrl => "vmx-entry-load-perf-global-ctrl",
            CPUFlag::Vmxentryloadpkrs => "vmx-entry-load-pkrs",
            CPUFlag::Vmxentryloadrtitctl => "vmx-entry-load-rtit-ctl",
            CPUFlag::Vmxentrynoloaddebugctl => "vmx-entry-noload-debugctl",
            CPUFlag::Vmxept => "vmx-ept",
            CPUFlag::Vmxept1gb => "vmx-ept-1gb",
            CPUFlag::Vmxept2mb => "vmx-ept-2mb",
            CPUFlag::Vmxeptadvancedexitinfo => "vmx-ept-advanced-exitinfo",
            CPUFlag::Vmxeptexeconly => "vmx-ept-execonly",
            CPUFlag::Vmxeptad => "vmx-eptad",
            CPUFlag::Vmxeptpswitching => "vmx-eptp-switching",
            CPUFlag::Vmxexitackintr => "vmx-exit-ack-intr",
            CPUFlag::Vmxexitclearbndcfgs => "vmx-exit-clear-bndcfgs",
            CPUFlag::Vmxexitclearrtitctl => "vmx-exit-clear-rtit-ctl",
            CPUFlag::Vmxexitloadefer => "vmx-exit-load-efer",
            CPUFlag::Vmxexitloadpat => "vmx-exit-load-pat",
            CPUFlag::Vmxexitloadperfglobalctrl => "vmx-exit-load-perf-global-ctrl",
            CPUFlag::Vmxexitloadpkrs => "vmx-exit-load-pkrs",
            CPUFlag::Vmxexitnosavedebugctl => "vmx-exit-nosave-debugctl",
            CPUFlag::Vmxexitsaveefer => "vmx-exit-save-efer",
            CPUFlag::Vmxexitsavepat => "vmx-exit-save-pat",
            CPUFlag::Vmxexitsavepreemptiontimer => "vmx-exit-save-preemption-timer",
            CPUFlag::Vmxexitsecondaryctls => "vmx-exit-secondary-ctls",
            CPUFlag::Vmxflexpriority => "vmx-flexpriority",
            CPUFlag::Vmxhltexit => "vmx-hlt-exit",
            CPUFlag::Vmxinsouts => "vmx-ins-outs",
            CPUFlag::Vmxintrexit => "vmx-intr-exit",
            CPUFlag::Vmxinvept => "vmx-invept",
            CPUFlag::Vmxinveptallcontext => "vmx-invept-all-context",
            CPUFlag::Vmxinveptsinglecontext => "vmx-invept-single-context",
            CPUFlag::Vmxinveptsinglecontextnoglobals => "vmx-invept-single-context-noglobals",
            CPUFlag::Vmxinvlpgexit => "vmx-invlpg-exit",
            CPUFlag::Vmxinvpcidexit => "vmx-invpcid-exit",
            CPUFlag::Vmxinvvpid => "vmx-invvpid",
            CPUFlag::Vmxinvvpidallcontext => "vmx-invvpid-all-context",
            CPUFlag::Vmxinvvpidsingleaddr => "vmx-invvpid-single-addr",
            CPUFlag::Vmxiobitmap => "vmx-io-bitmap",
            CPUFlag::Vmxioexit => "vmx-io-exit",
            CPUFlag::Vmxmonitorexit => "vmx-monitor-exit",
            CPUFlag::Vmxmovdrexit => "vmx-movdr-exit",
            CPUFlag::Vmxmsrbitmap => "vmx-msr-bitmap",
            CPUFlag::Vmxmtf => "vmx-mtf",
            CPUFlag::Vmxmwaitexit => "vmx-mwait-exit",
            CPUFlag::Vmxnestedexception => "vmx-nested-exception",
            CPUFlag::Vmxnmiexit => "vmx-nmi-exit",
            CPUFlag::Vmxpagewalk4 => "vmx-page-walk-4",
            CPUFlag::Vmxpagewalk5 => "vmx-page-walk-5",
            CPUFlag::Vmxpauseexit => "vmx-pause-exit",
            CPUFlag::Vmxple => "vmx-ple",
            CPUFlag::Vmxpml => "vmx-pml",
            CPUFlag::Vmxpostedintr => "vmx-posted-intr",
            CPUFlag::Vmxpreemptiontimer => "vmx-preemption-timer",
            CPUFlag::Vmxrdpmcexit => "vmx-rdpmc-exit",
            CPUFlag::Vmxrdrandexit => "vmx-rdrand-exit",
            CPUFlag::Vmxrdseedexit => "vmx-rdseed-exit",
            CPUFlag::Vmxrdtscexit => "vmx-rdtsc-exit",
            CPUFlag::Vmxrdtscpexit => "vmx-rdtscp-exit",
            CPUFlag::Vmxsecondaryctls => "vmx-secondary-ctls",
            CPUFlag::VmxshadowVmcs => "vmx-shadow-vmcs",
            CPUFlag::Vmxstorelma => "vmx-store-lma",
            CPUFlag::Vmxtruectls => "vmx-true-ctls",
            CPUFlag::Vmxtscoffset => "vmx-tsc-offset",
            CPUFlag::Vmxtscscaling => "vmx-tsc-scaling",
            CPUFlag::Vmxunrestrictedguest => "vmx-unrestricted-guest",
            CPUFlag::VmxVintrpending => "vmx-vintr-pending",
            CPUFlag::VmxVmfunc => "vmx-vmfunc",
            CPUFlag::VmxVmwritevmexitfields => "vmx-vmwrite-vmexit-fields",
            CPUFlag::VmxVnmi => "vmx-vnmi",
            CPUFlag::VmxVnmipending => "vmx-vnmi-pending",
            CPUFlag::VmxVpid => "vmx-vpid",
            CPUFlag::Vmxwbinvdexit => "vmx-wbinvd-exit",
            CPUFlag::Vmxxsaves => "vmx-xsaves",
            CPUFlag::Vmxzeroleninject => "vmx-zero-len-inject",
            CPUFlag::Vnmi => "vnmi",
            CPUFlag::Vpclmulqdq => "vpclmulqdq",
            CPUFlag::Waitpkg => "waitpkg",
            CPUFlag::Wbnoinvd => "wbnoinvd",
            CPUFlag::Wdt => "wdt",
            CPUFlag::Wrmsrns => "wrmsrns",
            CPUFlag::X2apic => "x2apic",
            CPUFlag::Xcrypt => "xcrypt",
            CPUFlag::Xcrypten => "xcrypt-en",
            CPUFlag::Xfd => "xfd",
            CPUFlag::Xgetbv1 => "xgetbv1",
            CPUFlag::Xop => "xop",
            CPUFlag::Xsave => "xsave",
            CPUFlag::Xsavec => "xsavec",
            CPUFlag::Xsaveerptr => "xsaveerptr",
            CPUFlag::Xsaveopt => "xsaveopt",
            CPUFlag::Xsaves => "xsaves",
            CPUFlag::Xstore => "xstore",
            CPUFlag::Xstoreen => "xstore-en",
            CPUFlag::Xtpr => "xtpr",
            CPUFlag::Zerofcsfds => "zero-fcs-fds",
        }
    }
}

impl FromStr for CPUFlag {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "3dnow" => Ok(CPUFlag::X3dnow),
            "3dnowext" => Ok(CPUFlag::X3dnowext),
            "3dnowprefetch" => Ok(CPUFlag::X3dnowprefetch),
            "abm" => Ok(CPUFlag::Abm),
            "ace2" => Ok(CPUFlag::Ace2),
            "ace2-en" => Ok(CPUFlag::Ace2en),
            "acpi" => Ok(CPUFlag::Acpi),
            "adx" => Ok(CPUFlag::Adx),
            "aes" => Ok(CPUFlag::Aes),
            "amd-no-ssb" => Ok(CPUFlag::Amdnossb),
            "amd-psfd" => Ok(CPUFlag::Amdpsfd),
            "amd-ssbd" => Ok(CPUFlag::Amdssbd),
            "amd-stibp" => Ok(CPUFlag::Amdstibp),
            "amx-bf16" => Ok(CPUFlag::Amxbf16),
            "amx-complex" => Ok(CPUFlag::Amxcomplex),
            "amx-fp16" => Ok(CPUFlag::Amxfp16),
            "amx-int8" => Ok(CPUFlag::Amxint8),
            "amx-tile" => Ok(CPUFlag::Amxtile),
            "apic" => Ok(CPUFlag::Apic),
            "arat" => Ok(CPUFlag::Arat),
            "arch-capabilities" => Ok(CPUFlag::Archcapabilities),
            "arch-lbr" => Ok(CPUFlag::Archlbr),
            "auto-ibrs" => Ok(CPUFlag::Autoibrs),
            "avic" => Ok(CPUFlag::Avic),
            "avx" => Ok(CPUFlag::Avx),
            "avx-ifma" => Ok(CPUFlag::Avxifma),
            "avx-ne-convert" => Ok(CPUFlag::Avxneconvert),
            "avx-vnni" => Ok(CPUFlag::AvxVnni),
            "avx-vnni-int16" => Ok(CPUFlag::AvxVnniint16),
            "avx-vnni-int8" => Ok(CPUFlag::AvxVnniint8),
            "avx10" => Ok(CPUFlag::Avx10),
            "avx10-128" => Ok(CPUFlag::Avx10128),
            "avx10-256" => Ok(CPUFlag::Avx10256),
            "avx10-512" => Ok(CPUFlag::Avx10512),
            "avx2" => Ok(CPUFlag::Avx2),
            "avx512-4fmaps" => Ok(CPUFlag::Avx5124fmaps),
            "avx512-4vnniw" => Ok(CPUFlag::Avx5124vnniw),
            "avx512-bf16" => Ok(CPUFlag::Avx512bf16),
            "avx512-fp16" => Ok(CPUFlag::Avx512fp16),
            "avx512-vp2intersect" => Ok(CPUFlag::Avx512Vp2intersect),
            "avx512-vpopcntdq" => Ok(CPUFlag::Avx512Vpopcntdq),
            "avx512bitalg" => Ok(CPUFlag::Avx512bitalg),
            "avx512bw" => Ok(CPUFlag::Avx512bw),
            "avx512cd" => Ok(CPUFlag::Avx512cd),
            "avx512dq" => Ok(CPUFlag::Avx512dq),
            "avx512er" => Ok(CPUFlag::Avx512er),
            "avx512f" => Ok(CPUFlag::Avx512f),
            "avx512ifma" => Ok(CPUFlag::Avx512ifma),
            "avx512pf" => Ok(CPUFlag::Avx512pf),
            "avx512vbmi" => Ok(CPUFlag::Avx512vbmi),
            "avx512vbmi2" => Ok(CPUFlag::Avx512vbmi2),
            "avx512vl" => Ok(CPUFlag::Avx512vl),
            "avx512vnni" => Ok(CPUFlag::Avx512vnni),
            "bhi-ctrl" => Ok(CPUFlag::Bhictrl),
            "bhi-no" => Ok(CPUFlag::Bhino),
            "bmi1" => Ok(CPUFlag::Bmi1),
            "bmi2" => Ok(CPUFlag::Bmi2),
            "bus-lock-detect" => Ok(CPUFlag::Buslockdetect),
            "cid" => Ok(CPUFlag::Cid),
            "cldemote" => Ok(CPUFlag::Cldemote),
            "clflush" => Ok(CPUFlag::Clflush),
            "clflushopt" => Ok(CPUFlag::Clflushopt),
            "clwb" => Ok(CPUFlag::Clwb),
            "clzero" => Ok(CPUFlag::Clzero),
            "cmov" => Ok(CPUFlag::Cmov),
            "cmp-legacy" => Ok(CPUFlag::Cmplegacy),
            "cmpccxadd" => Ok(CPUFlag::Cmpccxadd),
            "core-capability" => Ok(CPUFlag::Corecapability),
            "cr8legacy" => Ok(CPUFlag::Cr8legacy),
            "cx16" => Ok(CPUFlag::Cx16),
            "cx8" => Ok(CPUFlag::Cx8),
            "dca" => Ok(CPUFlag::Dca),
            "ddpd-u" => Ok(CPUFlag::Ddpdu),
            "de" => Ok(CPUFlag::De),
            "decodeassists" => Ok(CPUFlag::Decodeassists),
            "ds" => Ok(CPUFlag::Ds),
            "ds-cpl" => Ok(CPUFlag::Dscpl),
            "dtes64" => Ok(CPUFlag::Dtes64),
            "eraps" => Ok(CPUFlag::Eraps),
            "erms" => Ok(CPUFlag::Erms),
            "est" => Ok(CPUFlag::Est),
            "extapic" => Ok(CPUFlag::Extapic),
            "f16c" => Ok(CPUFlag::F16c),
            "fb-clear" => Ok(CPUFlag::Fbclear),
            "fbsdp-no" => Ok(CPUFlag::Fbsdpno),
            "fdp-excptn-only" => Ok(CPUFlag::Fdpexcptnonly),
            "flush-l1d" => Ok(CPUFlag::Flushl1d),
            "flushbyasid" => Ok(CPUFlag::Flushbyasid),
            "fma" => Ok(CPUFlag::Fma),
            "fma4" => Ok(CPUFlag::Fma4),
            "fpu" => Ok(CPUFlag::Fpu),
            "fred" => Ok(CPUFlag::Fred),
            "fsgsbase" => Ok(CPUFlag::Fsgsbase),
            "fsrc" => Ok(CPUFlag::Fsrc),
            "fsrm" => Ok(CPUFlag::Fsrm),
            "fsrs" => Ok(CPUFlag::Fsrs),
            "full-width-write" => Ok(CPUFlag::Fullwidthwrite),
            "fxsr" => Ok(CPUFlag::Fxsr),
            "fxsr-opt" => Ok(CPUFlag::Fxsropt),
            "fzrm" => Ok(CPUFlag::Fzrm),
            "gds-no" => Ok(CPUFlag::Gdsno),
            "gfni" => Ok(CPUFlag::Gfni),
            "hle" => Ok(CPUFlag::Hle),
            "ht" => Ok(CPUFlag::Ht),
            "hypervisor" => Ok(CPUFlag::Hypervisor),
            "ia64" => Ok(CPUFlag::Ia64),
            "ibpb" => Ok(CPUFlag::Ibpb),
            "ibpb-brtype" => Ok(CPUFlag::Ibpbbrtype),
            "ibrs" => Ok(CPUFlag::Ibrs),
            "ibrs-all" => Ok(CPUFlag::Ibrsall),
            "ibs" => Ok(CPUFlag::Ibs),
            "intel-psfd" => Ok(CPUFlag::Intelpsfd),
            "intel-pt" => Ok(CPUFlag::Intelpt),
            "intel-pt-lip" => Ok(CPUFlag::Intelptlip),
            "invpcid" => Ok(CPUFlag::Invpcid),
            "invtsc" => Ok(CPUFlag::Invtsc),
            "ipred-ctrl" => Ok(CPUFlag::Ipredctrl),
            "kvm-asyncpf" => Ok(CPUFlag::Kvmasyncpf),
            "kvm-asyncpf-int" => Ok(CPUFlag::Kvmasyncpfint),
            "kvm-asyncpf-vmexit" => Ok(CPUFlag::KvmasyncpfVmexit),
            "kvm-hint-dedicated" => Ok(CPUFlag::Kvmhintdedicated),
            "kvm-mmu" => Ok(CPUFlag::Kvmmmu),
            "kvm-msi-ext-dest-id" => Ok(CPUFlag::Kvmmsiextdestid),
            "kvm-nopiodelay" => Ok(CPUFlag::Kvmnopiodelay),
            "kvm-poll-control" => Ok(CPUFlag::Kvmpollcontrol),
            "kvm-pv-eoi" => Ok(CPUFlag::Kvmpveoi),
            "kvm-pv-ipi" => Ok(CPUFlag::Kvmpvipi),
            "kvm-pv-sched-yield" => Ok(CPUFlag::Kvmpvschedyield),
            "kvm-pv-tlb-flush" => Ok(CPUFlag::Kvmpvtlbflush),
            "kvm-pv-unhalt" => Ok(CPUFlag::Kvmpvunhalt),
            "kvm-steal-time" => Ok(CPUFlag::Kvmstealtime),
            "kvmclock" => Ok(CPUFlag::Kvmclock),
            "kvmclock-stable-bit" => Ok(CPUFlag::Kvmclockstablebit),
            "la57" => Ok(CPUFlag::La57),
            "lahf-lm" => Ok(CPUFlag::Lahflm),
            "lam" => Ok(CPUFlag::Lam),
            "lbrv" => Ok(CPUFlag::Lbrv),
            "lfence-always-serializing" => Ok(CPUFlag::Lfencealwaysserializing),
            "lkgs" => Ok(CPUFlag::Lkgs),
            "lm" => Ok(CPUFlag::Lm),
            "lwp" => Ok(CPUFlag::Lwp),
            "mca" => Ok(CPUFlag::Mca),
            "mcdt-no" => Ok(CPUFlag::Mcdtno),
            "mce" => Ok(CPUFlag::Mce),
            "md-clear" => Ok(CPUFlag::Mdclear),
            "mds-no" => Ok(CPUFlag::Mdsno),
            "misalignsse" => Ok(CPUFlag::Misalignsse),
            "mmx" => Ok(CPUFlag::Mmx),
            "mmxext" => Ok(CPUFlag::Mmxext),
            "monitor" => Ok(CPUFlag::Monitor),
            "movbe" => Ok(CPUFlag::Movbe),
            "movdir64b" => Ok(CPUFlag::Movdir64b),
            "movdiri" => Ok(CPUFlag::Movdiri),
            "mpx" => Ok(CPUFlag::Mpx),
            "msr" => Ok(CPUFlag::Msr),
            "mtrr" => Ok(CPUFlag::Mtrr),
            "no-nested-data-bp" => Ok(CPUFlag::Nonesteddatabp),
            "nodeid-msr" => Ok(CPUFlag::Nodeidmsr),
            "npt" => Ok(CPUFlag::Npt),
            "nrip-save" => Ok(CPUFlag::Nripsave),
            "null-sel-clr-base" => Ok(CPUFlag::Nullselclrbase),
            "nx" => Ok(CPUFlag::Nx),
            "osvw" => Ok(CPUFlag::Osvw),
            "overflow-recov" => Ok(CPUFlag::Overflowrecov),
            "pae" => Ok(CPUFlag::Pae),
            "pat" => Ok(CPUFlag::Pat),
            "pause-filter" => Ok(CPUFlag::Pausefilter),
            "pbe" => Ok(CPUFlag::Pbe),
            "pbrsb-no" => Ok(CPUFlag::Pbrsbno),
            "pcid" => Ok(CPUFlag::Pcid),
            "pclmulqdq" => Ok(CPUFlag::Pclmulqdq),
            "pcommit" => Ok(CPUFlag::Pcommit),
            "pdcm" => Ok(CPUFlag::Pdcm),
            "pdpe1gb" => Ok(CPUFlag::Pdpe1gb),
            "perfctr-core" => Ok(CPUFlag::Perfctrcore),
            "perfctr-nb" => Ok(CPUFlag::Perfctrnb),
            "perfmon-v2" => Ok(CPUFlag::PerfmonV2),
            "pfthreshold" => Ok(CPUFlag::Pfthreshold),
            "pge" => Ok(CPUFlag::Pge),
            "phe" => Ok(CPUFlag::Phe),
            "phe-en" => Ok(CPUFlag::Pheen),
            "pks" => Ok(CPUFlag::Pks),
            "pku" => Ok(CPUFlag::Pku),
            "pmm" => Ok(CPUFlag::Pmm),
            "pmm-en" => Ok(CPUFlag::Pmmen),
            "pn" => Ok(CPUFlag::Pn),
            "pni" => Ok(CPUFlag::Pni),
            "popcnt" => Ok(CPUFlag::Popcnt),
            "prefetchiti" => Ok(CPUFlag::Prefetchiti),
            "pschange-mc-no" => Ok(CPUFlag::Pschangemcno),
            "psdp-no" => Ok(CPUFlag::Psdpno),
            "pse" => Ok(CPUFlag::Pse),
            "pse36" => Ok(CPUFlag::Pse36),
            "rdctl-no" => Ok(CPUFlag::Rdctlno),
            "rdpid" => Ok(CPUFlag::Rdpid),
            "rdrand" => Ok(CPUFlag::Rdrand),
            "rdseed" => Ok(CPUFlag::Rdseed),
            "rdtscp" => Ok(CPUFlag::Rdtscp),
            "rfds-clear" => Ok(CPUFlag::Rfdsclear),
            "rfds-no" => Ok(CPUFlag::Rfdsno),
            "rrsba-ctrl" => Ok(CPUFlag::Rrsbactrl),
            "rsba" => Ok(CPUFlag::Rsba),
            "rtm" => Ok(CPUFlag::Rtm),
            "sbdr-ssdp-no" => Ok(CPUFlag::Sbdrssdpno),
            "sbpb" => Ok(CPUFlag::Sbpb),
            "sep" => Ok(CPUFlag::Sep),
            "serialize" => Ok(CPUFlag::Serialize),
            "sgx" => Ok(CPUFlag::Sgx),
            "sgx-aex-notify" => Ok(CPUFlag::Sgxaexnotify),
            "sgx-debug" => Ok(CPUFlag::Sgxdebug),
            "sgx-edeccssa" => Ok(CPUFlag::Sgxedeccssa),
            "sgx-exinfo" => Ok(CPUFlag::Sgxexinfo),
            "sgx-kss" => Ok(CPUFlag::Sgxkss),
            "sgx-mode64" => Ok(CPUFlag::Sgxmode64),
            "sgx-provisionkey" => Ok(CPUFlag::Sgxprovisionkey),
            "sgx-tokenkey" => Ok(CPUFlag::Sgxtokenkey),
            "sgx1" => Ok(CPUFlag::Sgx1),
            "sgx2" => Ok(CPUFlag::Sgx2),
            "sgxlc" => Ok(CPUFlag::Sgxlc),
            "sha-ni" => Ok(CPUFlag::Shani),
            "sha512" => Ok(CPUFlag::Sha512),
            "skinit" => Ok(CPUFlag::Skinit),
            "skip-l1dfl-vmentry" => Ok(CPUFlag::Skipl1dflVmentry),
            "sm3" => Ok(CPUFlag::Sm3),
            "sm4" => Ok(CPUFlag::Sm4),
            "smap" => Ok(CPUFlag::Smap),
            "smep" => Ok(CPUFlag::Smep),
            "smx" => Ok(CPUFlag::Smx),
            "spec-ctrl" => Ok(CPUFlag::Specctrl),
            "split-lock-detect" => Ok(CPUFlag::Splitlockdetect),
            "srso-no" => Ok(CPUFlag::Srsono),
            "srso-user-kernel-no" => Ok(CPUFlag::Srsouserkernelno),
            "ss" => Ok(CPUFlag::Ss),
            "ssb-no" => Ok(CPUFlag::Ssbno),
            "ssbd" => Ok(CPUFlag::Ssbd),
            "sse" => Ok(CPUFlag::Sse),
            "sse2" => Ok(CPUFlag::Sse2),
            "sse4.1" => Ok(CPUFlag::Sse4_1),
            "sse4.2" => Ok(CPUFlag::Sse4_2),
            "sse4a" => Ok(CPUFlag::Sse4a),
            "ssse3" => Ok(CPUFlag::Ssse3),
            "stibp" => Ok(CPUFlag::Stibp),
            "stibp-always-on" => Ok(CPUFlag::Stibpalwayson),
            "succor" => Ok(CPUFlag::Succor),
            "svm" => Ok(CPUFlag::Svm),
            "svm-lock" => Ok(CPUFlag::Svmlock),
            "svme-addr-chk" => Ok(CPUFlag::Svmeaddrchk),
            "syscall" => Ok(CPUFlag::Syscall),
            "taa-no" => Ok(CPUFlag::Taano),
            "tbm" => Ok(CPUFlag::Tbm),
            "tce" => Ok(CPUFlag::Tce),
            "tm" => Ok(CPUFlag::Tm),
            "tm2" => Ok(CPUFlag::Tm2),
            "topoext" => Ok(CPUFlag::Topoext),
            "tsc" => Ok(CPUFlag::Tsc),
            "tsc-adjust" => Ok(CPUFlag::Tscadjust),
            "tsc-deadline" => Ok(CPUFlag::Tscdeadline),
            "tsc-scale" => Ok(CPUFlag::Tscscale),
            "tsx-ctrl" => Ok(CPUFlag::Tsxctrl),
            "tsx-ldtrk" => Ok(CPUFlag::Tsxldtrk),
            "umip" => Ok(CPUFlag::Umip),
            "v-vmsave-vmload" => Ok(CPUFlag::VVmsavevmload),
            "vaes" => Ok(CPUFlag::Vaes),
            "vgif" => Ok(CPUFlag::Vgif),
            "virt-ssbd" => Ok(CPUFlag::Virtssbd),
            "vmcb-clean" => Ok(CPUFlag::Vmcbclean),
            "vme" => Ok(CPUFlag::Vme),
            "vmx" => Ok(CPUFlag::Vmx),
            "vmx-activity-hlt" => Ok(CPUFlag::Vmxactivityhlt),
            "vmx-activity-shutdown" => Ok(CPUFlag::Vmxactivityshutdown),
            "vmx-activity-wait-sipi" => Ok(CPUFlag::Vmxactivitywaitsipi),
            "vmx-any-errcode" => Ok(CPUFlag::Vmxanyerrcode),
            "vmx-apicv-register" => Ok(CPUFlag::Vmxapicvregister),
            "vmx-apicv-vid" => Ok(CPUFlag::VmxapicvVid),
            "vmx-apicv-x2apic" => Ok(CPUFlag::Vmxapicvx2apic),
            "vmx-apicv-xapic" => Ok(CPUFlag::Vmxapicvxapic),
            "vmx-cr3-load-noexit" => Ok(CPUFlag::Vmxcr3loadnoexit),
            "vmx-cr3-store-noexit" => Ok(CPUFlag::Vmxcr3storenoexit),
            "vmx-cr8-load-exit" => Ok(CPUFlag::Vmxcr8loadexit),
            "vmx-cr8-store-exit" => Ok(CPUFlag::Vmxcr8storeexit),
            "vmx-desc-exit" => Ok(CPUFlag::Vmxdescexit),
            "vmx-enable-user-wait-pause" => Ok(CPUFlag::Vmxenableuserwaitpause),
            "vmx-encls-exit" => Ok(CPUFlag::Vmxenclsexit),
            "vmx-entry-ia32e-mode" => Ok(CPUFlag::Vmxentryia32emode),
            "vmx-entry-load-bndcfgs" => Ok(CPUFlag::Vmxentryloadbndcfgs),
            "vmx-entry-load-efer" => Ok(CPUFlag::Vmxentryloadefer),
            "vmx-entry-load-fred" => Ok(CPUFlag::Vmxentryloadfred),
            "vmx-entry-load-pat" => Ok(CPUFlag::Vmxentryloadpat),
            "vmx-entry-load-perf-global-ctrl" => Ok(CPUFlag::Vmxentryloadperfglobalctrl),
            "vmx-entry-load-pkrs" => Ok(CPUFlag::Vmxentryloadpkrs),
            "vmx-entry-load-rtit-ctl" => Ok(CPUFlag::Vmxentryloadrtitctl),
            "vmx-entry-noload-debugctl" => Ok(CPUFlag::Vmxentrynoloaddebugctl),
            "vmx-ept" => Ok(CPUFlag::Vmxept),
            "vmx-ept-1gb" => Ok(CPUFlag::Vmxept1gb),
            "vmx-ept-2mb" => Ok(CPUFlag::Vmxept2mb),
            "vmx-ept-advanced-exitinfo" => Ok(CPUFlag::Vmxeptadvancedexitinfo),
            "vmx-ept-execonly" => Ok(CPUFlag::Vmxeptexeconly),
            "vmx-eptad" => Ok(CPUFlag::Vmxeptad),
            "vmx-eptp-switching" => Ok(CPUFlag::Vmxeptpswitching),
            "vmx-exit-ack-intr" => Ok(CPUFlag::Vmxexitackintr),
            "vmx-exit-clear-bndcfgs" => Ok(CPUFlag::Vmxexitclearbndcfgs),
            "vmx-exit-clear-rtit-ctl" => Ok(CPUFlag::Vmxexitclearrtitctl),
            "vmx-exit-load-efer" => Ok(CPUFlag::Vmxexitloadefer),
            "vmx-exit-load-pat" => Ok(CPUFlag::Vmxexitloadpat),
            "vmx-exit-load-perf-global-ctrl" => Ok(CPUFlag::Vmxexitloadperfglobalctrl),
            "vmx-exit-load-pkrs" => Ok(CPUFlag::Vmxexitloadpkrs),
            "vmx-exit-nosave-debugctl" => Ok(CPUFlag::Vmxexitnosavedebugctl),
            "vmx-exit-save-efer" => Ok(CPUFlag::Vmxexitsaveefer),
            "vmx-exit-save-pat" => Ok(CPUFlag::Vmxexitsavepat),
            "vmx-exit-save-preemption-timer" => Ok(CPUFlag::Vmxexitsavepreemptiontimer),
            "vmx-exit-secondary-ctls" => Ok(CPUFlag::Vmxexitsecondaryctls),
            "vmx-flexpriority" => Ok(CPUFlag::Vmxflexpriority),
            "vmx-hlt-exit" => Ok(CPUFlag::Vmxhltexit),
            "vmx-ins-outs" => Ok(CPUFlag::Vmxinsouts),
            "vmx-intr-exit" => Ok(CPUFlag::Vmxintrexit),
            "vmx-invept" => Ok(CPUFlag::Vmxinvept),
            "vmx-invept-all-context" => Ok(CPUFlag::Vmxinveptallcontext),
            "vmx-invept-single-context" => Ok(CPUFlag::Vmxinveptsinglecontext),
            "vmx-invept-single-context-noglobals" => Ok(CPUFlag::Vmxinveptsinglecontextnoglobals),
            "vmx-invlpg-exit" => Ok(CPUFlag::Vmxinvlpgexit),
            "vmx-invpcid-exit" => Ok(CPUFlag::Vmxinvpcidexit),
            "vmx-invvpid" => Ok(CPUFlag::Vmxinvvpid),
            "vmx-invvpid-all-context" => Ok(CPUFlag::Vmxinvvpidallcontext),
            "vmx-invvpid-single-addr" => Ok(CPUFlag::Vmxinvvpidsingleaddr),
            "vmx-io-bitmap" => Ok(CPUFlag::Vmxiobitmap),
            "vmx-io-exit" => Ok(CPUFlag::Vmxioexit),
            "vmx-monitor-exit" => Ok(CPUFlag::Vmxmonitorexit),
            "vmx-movdr-exit" => Ok(CPUFlag::Vmxmovdrexit),
            "vmx-msr-bitmap" => Ok(CPUFlag::Vmxmsrbitmap),
            "vmx-mtf" => Ok(CPUFlag::Vmxmtf),
            "vmx-mwait-exit" => Ok(CPUFlag::Vmxmwaitexit),
            "vmx-nested-exception" => Ok(CPUFlag::Vmxnestedexception),
            "vmx-nmi-exit" => Ok(CPUFlag::Vmxnmiexit),
            "vmx-page-walk-4" => Ok(CPUFlag::Vmxpagewalk4),
            "vmx-page-walk-5" => Ok(CPUFlag::Vmxpagewalk5),
            "vmx-pause-exit" => Ok(CPUFlag::Vmxpauseexit),
            "vmx-ple" => Ok(CPUFlag::Vmxple),
            "vmx-pml" => Ok(CPUFlag::Vmxpml),
            "vmx-posted-intr" => Ok(CPUFlag::Vmxpostedintr),
            "vmx-preemption-timer" => Ok(CPUFlag::Vmxpreemptiontimer),
            "vmx-rdpmc-exit" => Ok(CPUFlag::Vmxrdpmcexit),
            "vmx-rdrand-exit" => Ok(CPUFlag::Vmxrdrandexit),
            "vmx-rdseed-exit" => Ok(CPUFlag::Vmxrdseedexit),
            "vmx-rdtsc-exit" => Ok(CPUFlag::Vmxrdtscexit),
            "vmx-rdtscp-exit" => Ok(CPUFlag::Vmxrdtscpexit),
            "vmx-secondary-ctls" => Ok(CPUFlag::Vmxsecondaryctls),
            "vmx-shadow-vmcs" => Ok(CPUFlag::VmxshadowVmcs),
            "vmx-store-lma" => Ok(CPUFlag::Vmxstorelma),
            "vmx-true-ctls" => Ok(CPUFlag::Vmxtruectls),
            "vmx-tsc-offset" => Ok(CPUFlag::Vmxtscoffset),
            "vmx-tsc-scaling" => Ok(CPUFlag::Vmxtscscaling),
            "vmx-unrestricted-guest" => Ok(CPUFlag::Vmxunrestrictedguest),
            "vmx-vintr-pending" => Ok(CPUFlag::VmxVintrpending),
            "vmx-vmfunc" => Ok(CPUFlag::VmxVmfunc),
            "vmx-vmwrite-vmexit-fields" => Ok(CPUFlag::VmxVmwritevmexitfields),
            "vmx-vnmi" => Ok(CPUFlag::VmxVnmi),
            "vmx-vnmi-pending" => Ok(CPUFlag::VmxVnmipending),
            "vmx-vpid" => Ok(CPUFlag::VmxVpid),
            "vmx-wbinvd-exit" => Ok(CPUFlag::Vmxwbinvdexit),
            "vmx-xsaves" => Ok(CPUFlag::Vmxxsaves),
            "vmx-zero-len-inject" => Ok(CPUFlag::Vmxzeroleninject),
            "vnmi" => Ok(CPUFlag::Vnmi),
            "vpclmulqdq" => Ok(CPUFlag::Vpclmulqdq),
            "waitpkg" => Ok(CPUFlag::Waitpkg),
            "wbnoinvd" => Ok(CPUFlag::Wbnoinvd),
            "wdt" => Ok(CPUFlag::Wdt),
            "wrmsrns" => Ok(CPUFlag::Wrmsrns),
            "x2apic" => Ok(CPUFlag::X2apic),
            "xcrypt" => Ok(CPUFlag::Xcrypt),
            "xcrypt-en" => Ok(CPUFlag::Xcrypten),
            "xfd" => Ok(CPUFlag::Xfd),
            "xgetbv1" => Ok(CPUFlag::Xgetbv1),
            "xop" => Ok(CPUFlag::Xop),
            "xsave" => Ok(CPUFlag::Xsave),
            "xsavec" => Ok(CPUFlag::Xsavec),
            "xsaveerptr" => Ok(CPUFlag::Xsaveerptr),
            "xsaveopt" => Ok(CPUFlag::Xsaveopt),
            "xsaves" => Ok(CPUFlag::Xsaves),
            "xstore" => Ok(CPUFlag::Xstore),
            "xstore-en" => Ok(CPUFlag::Xstoreen),
            "xtpr" => Ok(CPUFlag::Xtpr),
            "zero-fcs-fds" => Ok(CPUFlag::Zerofcsfds),
            _ => Err(()),
        }
    }
}
