use crate::to_command::{ToArg, ToCommand};
pub enum CPUFlags {
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
impl ToCommand for CPUFlags {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![];

        match self {
            CPUFlags::X3dnow => cmd.push("3dnow".to_string()),
            CPUFlags::X3dnowext => cmd.push("3dnowext".to_string()),
            CPUFlags::X3dnowprefetch => cmd.push("3dnowprefetch".to_string()),
            CPUFlags::Abm => cmd.push("abm".to_string()),
            CPUFlags::Ace2 => cmd.push("ace2".to_string()),
            CPUFlags::Ace2en => cmd.push("ace2-en".to_string()),
            CPUFlags::Acpi => cmd.push("acpi".to_string()),
            CPUFlags::Adx => cmd.push("adx".to_string()),
            CPUFlags::Aes => cmd.push("aes".to_string()),
            CPUFlags::Amdnossb => cmd.push("amd-no-ssb".to_string()),
            CPUFlags::Amdpsfd => cmd.push("amd-psfd".to_string()),
            CPUFlags::Amdssbd => cmd.push("amd-ssbd".to_string()),
            CPUFlags::Amdstibp => cmd.push("amd-stibp".to_string()),
            CPUFlags::Amxbf16 => cmd.push("amx-bf16".to_string()),
            CPUFlags::Amxcomplex => cmd.push("amx-complex".to_string()),
            CPUFlags::Amxfp16 => cmd.push("amx-fp16".to_string()),
            CPUFlags::Amxint8 => cmd.push("amx-int8".to_string()),
            CPUFlags::Amxtile => cmd.push("amx-tile".to_string()),
            CPUFlags::Apic => cmd.push("apic".to_string()),
            CPUFlags::Arat => cmd.push("arat".to_string()),
            CPUFlags::Archcapabilities => cmd.push("arch-capabilities".to_string()),
            CPUFlags::Archlbr => cmd.push("arch-lbr".to_string()),
            CPUFlags::Autoibrs => cmd.push("auto-ibrs".to_string()),
            CPUFlags::Avic => cmd.push("avic".to_string()),
            CPUFlags::Avx => cmd.push("avx".to_string()),
            CPUFlags::Avxifma => cmd.push("avx-ifma".to_string()),
            CPUFlags::Avxneconvert => cmd.push("avx-ne-convert".to_string()),
            CPUFlags::AvxVnni => cmd.push("avx-vnni".to_string()),
            CPUFlags::AvxVnniint16 => cmd.push("avx-vnni-int16".to_string()),
            CPUFlags::AvxVnniint8 => cmd.push("avx-vnni-int8".to_string()),
            CPUFlags::Avx10 => cmd.push("avx10".to_string()),
            CPUFlags::Avx10128 => cmd.push("avx10-128".to_string()),
            CPUFlags::Avx10256 => cmd.push("avx10-256".to_string()),
            CPUFlags::Avx10512 => cmd.push("avx10-512".to_string()),
            CPUFlags::Avx2 => cmd.push("avx2".to_string()),
            CPUFlags::Avx5124fmaps => cmd.push("avx512-4fmaps".to_string()),
            CPUFlags::Avx5124vnniw => cmd.push("avx512-4vnniw".to_string()),
            CPUFlags::Avx512bf16 => cmd.push("avx512-bf16".to_string()),
            CPUFlags::Avx512fp16 => cmd.push("avx512-fp16".to_string()),
            CPUFlags::Avx512Vp2intersect => cmd.push("avx512-vp2intersect".to_string()),
            CPUFlags::Avx512Vpopcntdq => cmd.push("avx512-vpopcntdq".to_string()),
            CPUFlags::Avx512bitalg => cmd.push("avx512bitalg".to_string()),
            CPUFlags::Avx512bw => cmd.push("avx512bw".to_string()),
            CPUFlags::Avx512cd => cmd.push("avx512cd".to_string()),
            CPUFlags::Avx512dq => cmd.push("avx512dq".to_string()),
            CPUFlags::Avx512er => cmd.push("avx512er".to_string()),
            CPUFlags::Avx512f => cmd.push("avx512f".to_string()),
            CPUFlags::Avx512ifma => cmd.push("avx512ifma".to_string()),
            CPUFlags::Avx512pf => cmd.push("avx512pf".to_string()),
            CPUFlags::Avx512vbmi => cmd.push("avx512vbmi".to_string()),
            CPUFlags::Avx512vbmi2 => cmd.push("avx512vbmi2".to_string()),
            CPUFlags::Avx512vl => cmd.push("avx512vl".to_string()),
            CPUFlags::Avx512vnni => cmd.push("avx512vnni".to_string()),
            CPUFlags::Bhictrl => cmd.push("bhi-ctrl".to_string()),
            CPUFlags::Bhino => cmd.push("bhi-no".to_string()),
            CPUFlags::Bmi1 => cmd.push("bmi1".to_string()),
            CPUFlags::Bmi2 => cmd.push("bmi2".to_string()),
            CPUFlags::Buslockdetect => cmd.push("bus-lock-detect".to_string()),
            CPUFlags::Cid => cmd.push("cid".to_string()),
            CPUFlags::Cldemote => cmd.push("cldemote".to_string()),
            CPUFlags::Clflush => cmd.push("clflush".to_string()),
            CPUFlags::Clflushopt => cmd.push("clflushopt".to_string()),
            CPUFlags::Clwb => cmd.push("clwb".to_string()),
            CPUFlags::Clzero => cmd.push("clzero".to_string()),
            CPUFlags::Cmov => cmd.push("cmov".to_string()),
            CPUFlags::Cmplegacy => cmd.push("cmp-legacy".to_string()),
            CPUFlags::Cmpccxadd => cmd.push("cmpccxadd".to_string()),
            CPUFlags::Corecapability => cmd.push("core-capability".to_string()),
            CPUFlags::Cr8legacy => cmd.push("cr8legacy".to_string()),
            CPUFlags::Cx16 => cmd.push("cx16".to_string()),
            CPUFlags::Cx8 => cmd.push("cx8".to_string()),
            CPUFlags::Dca => cmd.push("dca".to_string()),
            CPUFlags::Ddpdu => cmd.push("ddpd-u".to_string()),
            CPUFlags::De => cmd.push("de".to_string()),
            CPUFlags::Decodeassists => cmd.push("decodeassists".to_string()),
            CPUFlags::Ds => cmd.push("ds".to_string()),
            CPUFlags::Dscpl => cmd.push("ds-cpl".to_string()),
            CPUFlags::Dtes64 => cmd.push("dtes64".to_string()),
            CPUFlags::Eraps => cmd.push("eraps".to_string()),
            CPUFlags::Erms => cmd.push("erms".to_string()),
            CPUFlags::Est => cmd.push("est".to_string()),
            CPUFlags::Extapic => cmd.push("extapic".to_string()),
            CPUFlags::F16c => cmd.push("f16c".to_string()),
            CPUFlags::Fbclear => cmd.push("fb-clear".to_string()),
            CPUFlags::Fbsdpno => cmd.push("fbsdp-no".to_string()),
            CPUFlags::Fdpexcptnonly => cmd.push("fdp-excptn-only".to_string()),
            CPUFlags::Flushl1d => cmd.push("flush-l1d".to_string()),
            CPUFlags::Flushbyasid => cmd.push("flushbyasid".to_string()),
            CPUFlags::Fma => cmd.push("fma".to_string()),
            CPUFlags::Fma4 => cmd.push("fma4".to_string()),
            CPUFlags::Fpu => cmd.push("fpu".to_string()),
            CPUFlags::Fred => cmd.push("fred".to_string()),
            CPUFlags::Fsgsbase => cmd.push("fsgsbase".to_string()),
            CPUFlags::Fsrc => cmd.push("fsrc".to_string()),
            CPUFlags::Fsrm => cmd.push("fsrm".to_string()),
            CPUFlags::Fsrs => cmd.push("fsrs".to_string()),
            CPUFlags::Fullwidthwrite => cmd.push("full-width-write".to_string()),
            CPUFlags::Fxsr => cmd.push("fxsr".to_string()),
            CPUFlags::Fxsropt => cmd.push("fxsr-opt".to_string()),
            CPUFlags::Fzrm => cmd.push("fzrm".to_string()),
            CPUFlags::Gdsno => cmd.push("gds-no".to_string()),
            CPUFlags::Gfni => cmd.push("gfni".to_string()),
            CPUFlags::Hle => cmd.push("hle".to_string()),
            CPUFlags::Ht => cmd.push("ht".to_string()),
            CPUFlags::Hypervisor => cmd.push("hypervisor".to_string()),
            CPUFlags::Ia64 => cmd.push("ia64".to_string()),
            CPUFlags::Ibpb => cmd.push("ibpb".to_string()),
            CPUFlags::Ibpbbrtype => cmd.push("ibpb-brtype".to_string()),
            CPUFlags::Ibrs => cmd.push("ibrs".to_string()),
            CPUFlags::Ibrsall => cmd.push("ibrs-all".to_string()),
            CPUFlags::Ibs => cmd.push("ibs".to_string()),
            CPUFlags::Intelpsfd => cmd.push("intel-psfd".to_string()),
            CPUFlags::Intelpt => cmd.push("intel-pt".to_string()),
            CPUFlags::Intelptlip => cmd.push("intel-pt-lip".to_string()),
            CPUFlags::Invpcid => cmd.push("invpcid".to_string()),
            CPUFlags::Invtsc => cmd.push("invtsc".to_string()),
            CPUFlags::Ipredctrl => cmd.push("ipred-ctrl".to_string()),
            CPUFlags::Kvmasyncpf => cmd.push("kvm-asyncpf".to_string()),
            CPUFlags::Kvmasyncpfint => cmd.push("kvm-asyncpf-int".to_string()),
            CPUFlags::KvmasyncpfVmexit => cmd.push("kvm-asyncpf-vmexit".to_string()),
            CPUFlags::Kvmhintdedicated => cmd.push("kvm-hint-dedicated".to_string()),
            CPUFlags::Kvmmmu => cmd.push("kvm-mmu".to_string()),
            CPUFlags::Kvmmsiextdestid => cmd.push("kvm-msi-ext-dest-id".to_string()),
            CPUFlags::Kvmnopiodelay => cmd.push("kvm-nopiodelay".to_string()),
            CPUFlags::Kvmpollcontrol => cmd.push("kvm-poll-control".to_string()),
            CPUFlags::Kvmpveoi => cmd.push("kvm-pv-eoi".to_string()),
            CPUFlags::Kvmpvipi => cmd.push("kvm-pv-ipi".to_string()),
            CPUFlags::Kvmpvschedyield => cmd.push("kvm-pv-sched-yield".to_string()),
            CPUFlags::Kvmpvtlbflush => cmd.push("kvm-pv-tlb-flush".to_string()),
            CPUFlags::Kvmpvunhalt => cmd.push("kvm-pv-unhalt".to_string()),
            CPUFlags::Kvmstealtime => cmd.push("kvm-steal-time".to_string()),
            CPUFlags::Kvmclock => cmd.push("kvmclock".to_string()),
            CPUFlags::Kvmclockstablebit => cmd.push("kvmclock-stable-bit".to_string()),
            CPUFlags::La57 => cmd.push("la57".to_string()),
            CPUFlags::Lahflm => cmd.push("lahf-lm".to_string()),
            CPUFlags::Lam => cmd.push("lam".to_string()),
            CPUFlags::Lbrv => cmd.push("lbrv".to_string()),
            CPUFlags::Lfencealwaysserializing => cmd.push("lfence-always-serializing".to_string()),
            CPUFlags::Lkgs => cmd.push("lkgs".to_string()),
            CPUFlags::Lm => cmd.push("lm".to_string()),
            CPUFlags::Lwp => cmd.push("lwp".to_string()),
            CPUFlags::Mca => cmd.push("mca".to_string()),
            CPUFlags::Mcdtno => cmd.push("mcdt-no".to_string()),
            CPUFlags::Mce => cmd.push("mce".to_string()),
            CPUFlags::Mdclear => cmd.push("md-clear".to_string()),
            CPUFlags::Mdsno => cmd.push("mds-no".to_string()),
            CPUFlags::Misalignsse => cmd.push("misalignsse".to_string()),
            CPUFlags::Mmx => cmd.push("mmx".to_string()),
            CPUFlags::Mmxext => cmd.push("mmxext".to_string()),
            CPUFlags::Monitor => cmd.push("monitor".to_string()),
            CPUFlags::Movbe => cmd.push("movbe".to_string()),
            CPUFlags::Movdir64b => cmd.push("movdir64b".to_string()),
            CPUFlags::Movdiri => cmd.push("movdiri".to_string()),
            CPUFlags::Mpx => cmd.push("mpx".to_string()),
            CPUFlags::Msr => cmd.push("msr".to_string()),
            CPUFlags::Mtrr => cmd.push("mtrr".to_string()),
            CPUFlags::Nonesteddatabp => cmd.push("no-nested-data-bp".to_string()),
            CPUFlags::Nodeidmsr => cmd.push("nodeid-msr".to_string()),
            CPUFlags::Npt => cmd.push("npt".to_string()),
            CPUFlags::Nripsave => cmd.push("nrip-save".to_string()),
            CPUFlags::Nullselclrbase => cmd.push("null-sel-clr-base".to_string()),
            CPUFlags::Nx => cmd.push("nx".to_string()),
            CPUFlags::Osvw => cmd.push("osvw".to_string()),
            CPUFlags::Overflowrecov => cmd.push("overflow-recov".to_string()),
            CPUFlags::Pae => cmd.push("pae".to_string()),
            CPUFlags::Pat => cmd.push("pat".to_string()),
            CPUFlags::Pausefilter => cmd.push("pause-filter".to_string()),
            CPUFlags::Pbe => cmd.push("pbe".to_string()),
            CPUFlags::Pbrsbno => cmd.push("pbrsb-no".to_string()),
            CPUFlags::Pcid => cmd.push("pcid".to_string()),
            CPUFlags::Pclmulqdq => cmd.push("pclmulqdq".to_string()),
            CPUFlags::Pcommit => cmd.push("pcommit".to_string()),
            CPUFlags::Pdcm => cmd.push("pdcm".to_string()),
            CPUFlags::Pdpe1gb => cmd.push("pdpe1gb".to_string()),
            CPUFlags::Perfctrcore => cmd.push("perfctr-core".to_string()),
            CPUFlags::Perfctrnb => cmd.push("perfctr-nb".to_string()),
            CPUFlags::PerfmonV2 => cmd.push("perfmon-v2".to_string()),
            CPUFlags::Pfthreshold => cmd.push("pfthreshold".to_string()),
            CPUFlags::Pge => cmd.push("pge".to_string()),
            CPUFlags::Phe => cmd.push("phe".to_string()),
            CPUFlags::Pheen => cmd.push("phe-en".to_string()),
            CPUFlags::Pks => cmd.push("pks".to_string()),
            CPUFlags::Pku => cmd.push("pku".to_string()),
            CPUFlags::Pmm => cmd.push("pmm".to_string()),
            CPUFlags::Pmmen => cmd.push("pmm-en".to_string()),
            CPUFlags::Pn => cmd.push("pn".to_string()),
            CPUFlags::Pni => cmd.push("pni".to_string()),
            CPUFlags::Popcnt => cmd.push("popcnt".to_string()),
            CPUFlags::Prefetchiti => cmd.push("prefetchiti".to_string()),
            CPUFlags::Pschangemcno => cmd.push("pschange-mc-no".to_string()),
            CPUFlags::Psdpno => cmd.push("psdp-no".to_string()),
            CPUFlags::Pse => cmd.push("pse".to_string()),
            CPUFlags::Pse36 => cmd.push("pse36".to_string()),
            CPUFlags::Rdctlno => cmd.push("rdctl-no".to_string()),
            CPUFlags::Rdpid => cmd.push("rdpid".to_string()),
            CPUFlags::Rdrand => cmd.push("rdrand".to_string()),
            CPUFlags::Rdseed => cmd.push("rdseed".to_string()),
            CPUFlags::Rdtscp => cmd.push("rdtscp".to_string()),
            CPUFlags::Rfdsclear => cmd.push("rfds-clear".to_string()),
            CPUFlags::Rfdsno => cmd.push("rfds-no".to_string()),
            CPUFlags::Rrsbactrl => cmd.push("rrsba-ctrl".to_string()),
            CPUFlags::Rsba => cmd.push("rsba".to_string()),
            CPUFlags::Rtm => cmd.push("rtm".to_string()),
            CPUFlags::Sbdrssdpno => cmd.push("sbdr-ssdp-no".to_string()),
            CPUFlags::Sbpb => cmd.push("sbpb".to_string()),
            CPUFlags::Sep => cmd.push("sep".to_string()),
            CPUFlags::Serialize => cmd.push("serialize".to_string()),
            CPUFlags::Sgx => cmd.push("sgx".to_string()),
            CPUFlags::Sgxaexnotify => cmd.push("sgx-aex-notify".to_string()),
            CPUFlags::Sgxdebug => cmd.push("sgx-debug".to_string()),
            CPUFlags::Sgxedeccssa => cmd.push("sgx-edeccssa".to_string()),
            CPUFlags::Sgxexinfo => cmd.push("sgx-exinfo".to_string()),
            CPUFlags::Sgxkss => cmd.push("sgx-kss".to_string()),
            CPUFlags::Sgxmode64 => cmd.push("sgx-mode64".to_string()),
            CPUFlags::Sgxprovisionkey => cmd.push("sgx-provisionkey".to_string()),
            CPUFlags::Sgxtokenkey => cmd.push("sgx-tokenkey".to_string()),
            CPUFlags::Sgx1 => cmd.push("sgx1".to_string()),
            CPUFlags::Sgx2 => cmd.push("sgx2".to_string()),
            CPUFlags::Sgxlc => cmd.push("sgxlc".to_string()),
            CPUFlags::Shani => cmd.push("sha-ni".to_string()),
            CPUFlags::Sha512 => cmd.push("sha512".to_string()),
            CPUFlags::Skinit => cmd.push("skinit".to_string()),
            CPUFlags::Skipl1dflVmentry => cmd.push("skip-l1dfl-vmentry".to_string()),
            CPUFlags::Sm3 => cmd.push("sm3".to_string()),
            CPUFlags::Sm4 => cmd.push("sm4".to_string()),
            CPUFlags::Smap => cmd.push("smap".to_string()),
            CPUFlags::Smep => cmd.push("smep".to_string()),
            CPUFlags::Smx => cmd.push("smx".to_string()),
            CPUFlags::Specctrl => cmd.push("spec-ctrl".to_string()),
            CPUFlags::Splitlockdetect => cmd.push("split-lock-detect".to_string()),
            CPUFlags::Srsono => cmd.push("srso-no".to_string()),
            CPUFlags::Srsouserkernelno => cmd.push("srso-user-kernel-no".to_string()),
            CPUFlags::Ss => cmd.push("ss".to_string()),
            CPUFlags::Ssbno => cmd.push("ssb-no".to_string()),
            CPUFlags::Ssbd => cmd.push("ssbd".to_string()),
            CPUFlags::Sse => cmd.push("sse".to_string()),
            CPUFlags::Sse2 => cmd.push("sse2".to_string()),
            CPUFlags::Sse4_1 => cmd.push("sse4.1".to_string()),
            CPUFlags::Sse4_2 => cmd.push("sse4.2".to_string()),
            CPUFlags::Sse4a => cmd.push("sse4a".to_string()),
            CPUFlags::Ssse3 => cmd.push("ssse3".to_string()),
            CPUFlags::Stibp => cmd.push("stibp".to_string()),
            CPUFlags::Stibpalwayson => cmd.push("stibp-always-on".to_string()),
            CPUFlags::Succor => cmd.push("succor".to_string()),
            CPUFlags::Svm => cmd.push("svm".to_string()),
            CPUFlags::Svmlock => cmd.push("svm-lock".to_string()),
            CPUFlags::Svmeaddrchk => cmd.push("svme-addr-chk".to_string()),
            CPUFlags::Syscall => cmd.push("syscall".to_string()),
            CPUFlags::Taano => cmd.push("taa-no".to_string()),
            CPUFlags::Tbm => cmd.push("tbm".to_string()),
            CPUFlags::Tce => cmd.push("tce".to_string()),
            CPUFlags::Tm => cmd.push("tm".to_string()),
            CPUFlags::Tm2 => cmd.push("tm2".to_string()),
            CPUFlags::Topoext => cmd.push("topoext".to_string()),
            CPUFlags::Tsc => cmd.push("tsc".to_string()),
            CPUFlags::Tscadjust => cmd.push("tsc-adjust".to_string()),
            CPUFlags::Tscdeadline => cmd.push("tsc-deadline".to_string()),
            CPUFlags::Tscscale => cmd.push("tsc-scale".to_string()),
            CPUFlags::Tsxctrl => cmd.push("tsx-ctrl".to_string()),
            CPUFlags::Tsxldtrk => cmd.push("tsx-ldtrk".to_string()),
            CPUFlags::Umip => cmd.push("umip".to_string()),
            CPUFlags::VVmsavevmload => cmd.push("v-vmsave-vmload".to_string()),
            CPUFlags::Vaes => cmd.push("vaes".to_string()),
            CPUFlags::Vgif => cmd.push("vgif".to_string()),
            CPUFlags::Virtssbd => cmd.push("virt-ssbd".to_string()),
            CPUFlags::Vmcbclean => cmd.push("vmcb-clean".to_string()),
            CPUFlags::Vme => cmd.push("vme".to_string()),
            CPUFlags::Vmx => cmd.push("vmx".to_string()),
            CPUFlags::Vmxactivityhlt => cmd.push("vmx-activity-hlt".to_string()),
            CPUFlags::Vmxactivityshutdown => cmd.push("vmx-activity-shutdown".to_string()),
            CPUFlags::Vmxactivitywaitsipi => cmd.push("vmx-activity-wait-sipi".to_string()),
            CPUFlags::Vmxanyerrcode => cmd.push("vmx-any-errcode".to_string()),
            CPUFlags::Vmxapicvregister => cmd.push("vmx-apicv-register".to_string()),
            CPUFlags::VmxapicvVid => cmd.push("vmx-apicv-vid".to_string()),
            CPUFlags::Vmxapicvx2apic => cmd.push("vmx-apicv-x2apic".to_string()),
            CPUFlags::Vmxapicvxapic => cmd.push("vmx-apicv-xapic".to_string()),
            CPUFlags::Vmxcr3loadnoexit => cmd.push("vmx-cr3-load-noexit".to_string()),
            CPUFlags::Vmxcr3storenoexit => cmd.push("vmx-cr3-store-noexit".to_string()),
            CPUFlags::Vmxcr8loadexit => cmd.push("vmx-cr8-load-exit".to_string()),
            CPUFlags::Vmxcr8storeexit => cmd.push("vmx-cr8-store-exit".to_string()),
            CPUFlags::Vmxdescexit => cmd.push("vmx-desc-exit".to_string()),
            CPUFlags::Vmxenableuserwaitpause => cmd.push("vmx-enable-user-wait-pause".to_string()),
            CPUFlags::Vmxenclsexit => cmd.push("vmx-encls-exit".to_string()),
            CPUFlags::Vmxentryia32emode => cmd.push("vmx-entry-ia32e-mode".to_string()),
            CPUFlags::Vmxentryloadbndcfgs => cmd.push("vmx-entry-load-bndcfgs".to_string()),
            CPUFlags::Vmxentryloadefer => cmd.push("vmx-entry-load-efer".to_string()),
            CPUFlags::Vmxentryloadfred => cmd.push("vmx-entry-load-fred".to_string()),
            CPUFlags::Vmxentryloadpat => cmd.push("vmx-entry-load-pat".to_string()),
            CPUFlags::Vmxentryloadperfglobalctrl => {
                cmd.push("vmx-entry-load-perf-global-ctrl".to_string())
            }
            CPUFlags::Vmxentryloadpkrs => cmd.push("vmx-entry-load-pkrs".to_string()),
            CPUFlags::Vmxentryloadrtitctl => cmd.push("vmx-entry-load-rtit-ctl".to_string()),
            CPUFlags::Vmxentrynoloaddebugctl => cmd.push("vmx-entry-noload-debugctl".to_string()),
            CPUFlags::Vmxept => cmd.push("vmx-ept".to_string()),
            CPUFlags::Vmxept1gb => cmd.push("vmx-ept-1gb".to_string()),
            CPUFlags::Vmxept2mb => cmd.push("vmx-ept-2mb".to_string()),
            CPUFlags::Vmxeptadvancedexitinfo => cmd.push("vmx-ept-advanced-exitinfo".to_string()),
            CPUFlags::Vmxeptexeconly => cmd.push("vmx-ept-execonly".to_string()),
            CPUFlags::Vmxeptad => cmd.push("vmx-eptad".to_string()),
            CPUFlags::Vmxeptpswitching => cmd.push("vmx-eptp-switching".to_string()),
            CPUFlags::Vmxexitackintr => cmd.push("vmx-exit-ack-intr".to_string()),
            CPUFlags::Vmxexitclearbndcfgs => cmd.push("vmx-exit-clear-bndcfgs".to_string()),
            CPUFlags::Vmxexitclearrtitctl => cmd.push("vmx-exit-clear-rtit-ctl".to_string()),
            CPUFlags::Vmxexitloadefer => cmd.push("vmx-exit-load-efer".to_string()),
            CPUFlags::Vmxexitloadpat => cmd.push("vmx-exit-load-pat".to_string()),
            CPUFlags::Vmxexitloadperfglobalctrl => {
                cmd.push("vmx-exit-load-perf-global-ctrl".to_string())
            }
            CPUFlags::Vmxexitloadpkrs => cmd.push("vmx-exit-load-pkrs".to_string()),
            CPUFlags::Vmxexitnosavedebugctl => cmd.push("vmx-exit-nosave-debugctl".to_string()),
            CPUFlags::Vmxexitsaveefer => cmd.push("vmx-exit-save-efer".to_string()),
            CPUFlags::Vmxexitsavepat => cmd.push("vmx-exit-save-pat".to_string()),
            CPUFlags::Vmxexitsavepreemptiontimer => {
                cmd.push("vmx-exit-save-preemption-timer".to_string())
            }
            CPUFlags::Vmxexitsecondaryctls => cmd.push("vmx-exit-secondary-ctls".to_string()),
            CPUFlags::Vmxflexpriority => cmd.push("vmx-flexpriority".to_string()),
            CPUFlags::Vmxhltexit => cmd.push("vmx-hlt-exit".to_string()),
            CPUFlags::Vmxinsouts => cmd.push("vmx-ins-outs".to_string()),
            CPUFlags::Vmxintrexit => cmd.push("vmx-intr-exit".to_string()),
            CPUFlags::Vmxinvept => cmd.push("vmx-invept".to_string()),
            CPUFlags::Vmxinveptallcontext => cmd.push("vmx-invept-all-context".to_string()),
            CPUFlags::Vmxinveptsinglecontext => cmd.push("vmx-invept-single-context".to_string()),
            CPUFlags::Vmxinveptsinglecontextnoglobals => {
                cmd.push("vmx-invept-single-context-noglobals".to_string())
            }
            CPUFlags::Vmxinvlpgexit => cmd.push("vmx-invlpg-exit".to_string()),
            CPUFlags::Vmxinvpcidexit => cmd.push("vmx-invpcid-exit".to_string()),
            CPUFlags::Vmxinvvpid => cmd.push("vmx-invvpid".to_string()),
            CPUFlags::Vmxinvvpidallcontext => cmd.push("vmx-invvpid-all-context".to_string()),
            CPUFlags::Vmxinvvpidsingleaddr => cmd.push("vmx-invvpid-single-addr".to_string()),
            CPUFlags::Vmxiobitmap => cmd.push("vmx-io-bitmap".to_string()),
            CPUFlags::Vmxioexit => cmd.push("vmx-io-exit".to_string()),
            CPUFlags::Vmxmonitorexit => cmd.push("vmx-monitor-exit".to_string()),
            CPUFlags::Vmxmovdrexit => cmd.push("vmx-movdr-exit".to_string()),
            CPUFlags::Vmxmsrbitmap => cmd.push("vmx-msr-bitmap".to_string()),
            CPUFlags::Vmxmtf => cmd.push("vmx-mtf".to_string()),
            CPUFlags::Vmxmwaitexit => cmd.push("vmx-mwait-exit".to_string()),
            CPUFlags::Vmxnestedexception => cmd.push("vmx-nested-exception".to_string()),
            CPUFlags::Vmxnmiexit => cmd.push("vmx-nmi-exit".to_string()),
            CPUFlags::Vmxpagewalk4 => cmd.push("vmx-page-walk-4".to_string()),
            CPUFlags::Vmxpagewalk5 => cmd.push("vmx-page-walk-5".to_string()),
            CPUFlags::Vmxpauseexit => cmd.push("vmx-pause-exit".to_string()),
            CPUFlags::Vmxple => cmd.push("vmx-ple".to_string()),
            CPUFlags::Vmxpml => cmd.push("vmx-pml".to_string()),
            CPUFlags::Vmxpostedintr => cmd.push("vmx-posted-intr".to_string()),
            CPUFlags::Vmxpreemptiontimer => cmd.push("vmx-preemption-timer".to_string()),
            CPUFlags::Vmxrdpmcexit => cmd.push("vmx-rdpmc-exit".to_string()),
            CPUFlags::Vmxrdrandexit => cmd.push("vmx-rdrand-exit".to_string()),
            CPUFlags::Vmxrdseedexit => cmd.push("vmx-rdseed-exit".to_string()),
            CPUFlags::Vmxrdtscexit => cmd.push("vmx-rdtsc-exit".to_string()),
            CPUFlags::Vmxrdtscpexit => cmd.push("vmx-rdtscp-exit".to_string()),
            CPUFlags::Vmxsecondaryctls => cmd.push("vmx-secondary-ctls".to_string()),
            CPUFlags::VmxshadowVmcs => cmd.push("vmx-shadow-vmcs".to_string()),
            CPUFlags::Vmxstorelma => cmd.push("vmx-store-lma".to_string()),
            CPUFlags::Vmxtruectls => cmd.push("vmx-true-ctls".to_string()),
            CPUFlags::Vmxtscoffset => cmd.push("vmx-tsc-offset".to_string()),
            CPUFlags::Vmxtscscaling => cmd.push("vmx-tsc-scaling".to_string()),
            CPUFlags::Vmxunrestrictedguest => cmd.push("vmx-unrestricted-guest".to_string()),
            CPUFlags::VmxVintrpending => cmd.push("vmx-vintr-pending".to_string()),
            CPUFlags::VmxVmfunc => cmd.push("vmx-vmfunc".to_string()),
            CPUFlags::VmxVmwritevmexitfields => cmd.push("vmx-vmwrite-vmexit-fields".to_string()),
            CPUFlags::VmxVnmi => cmd.push("vmx-vnmi".to_string()),
            CPUFlags::VmxVnmipending => cmd.push("vmx-vnmi-pending".to_string()),
            CPUFlags::VmxVpid => cmd.push("vmx-vpid".to_string()),
            CPUFlags::Vmxwbinvdexit => cmd.push("vmx-wbinvd-exit".to_string()),
            CPUFlags::Vmxxsaves => cmd.push("vmx-xsaves".to_string()),
            CPUFlags::Vmxzeroleninject => cmd.push("vmx-zero-len-inject".to_string()),
            CPUFlags::Vnmi => cmd.push("vnmi".to_string()),
            CPUFlags::Vpclmulqdq => cmd.push("vpclmulqdq".to_string()),
            CPUFlags::Waitpkg => cmd.push("waitpkg".to_string()),
            CPUFlags::Wbnoinvd => cmd.push("wbnoinvd".to_string()),
            CPUFlags::Wdt => cmd.push("wdt".to_string()),
            CPUFlags::Wrmsrns => cmd.push("wrmsrns".to_string()),
            CPUFlags::X2apic => cmd.push("x2apic".to_string()),
            CPUFlags::Xcrypt => cmd.push("xcrypt".to_string()),
            CPUFlags::Xcrypten => cmd.push("xcrypt-en".to_string()),
            CPUFlags::Xfd => cmd.push("xfd".to_string()),
            CPUFlags::Xgetbv1 => cmd.push("xgetbv1".to_string()),
            CPUFlags::Xop => cmd.push("xop".to_string()),
            CPUFlags::Xsave => cmd.push("xsave".to_string()),
            CPUFlags::Xsavec => cmd.push("xsavec".to_string()),
            CPUFlags::Xsaveerptr => cmd.push("xsaveerptr".to_string()),
            CPUFlags::Xsaveopt => cmd.push("xsaveopt".to_string()),
            CPUFlags::Xsaves => cmd.push("xsaves".to_string()),
            CPUFlags::Xstore => cmd.push("xstore".to_string()),
            CPUFlags::Xstoreen => cmd.push("xstore-en".to_string()),
            CPUFlags::Xtpr => cmd.push("xtpr".to_string()),
            CPUFlags::Zerofcsfds => cmd.push("zero-fcs-fds".to_string()),
        }
        cmd
    }
}

impl ToArg for CPUFlags {
    fn to_arg(&self) -> &str {
        match self {
            CPUFlags::X3dnow => "3dnow",
            CPUFlags::X3dnowext => "3dnowext",
            CPUFlags::X3dnowprefetch => "3dnowprefetch",
            CPUFlags::Abm => "abm",
            CPUFlags::Ace2 => "ace2",
            CPUFlags::Ace2en => "ace2-en",
            CPUFlags::Acpi => "acpi",
            CPUFlags::Adx => "adx",
            CPUFlags::Aes => "aes",
            CPUFlags::Amdnossb => "amd-no-ssb",
            CPUFlags::Amdpsfd => "amd-psfd",
            CPUFlags::Amdssbd => "amd-ssbd",
            CPUFlags::Amdstibp => "amd-stibp",
            CPUFlags::Amxbf16 => "amx-bf16",
            CPUFlags::Amxcomplex => "amx-complex",
            CPUFlags::Amxfp16 => "amx-fp16",
            CPUFlags::Amxint8 => "amx-int8",
            CPUFlags::Amxtile => "amx-tile",
            CPUFlags::Apic => "apic",
            CPUFlags::Arat => "arat",
            CPUFlags::Archcapabilities => "arch-capabilities",
            CPUFlags::Archlbr => "arch-lbr",
            CPUFlags::Autoibrs => "auto-ibrs",
            CPUFlags::Avic => "avic",
            CPUFlags::Avx => "avx",
            CPUFlags::Avxifma => "avx-ifma",
            CPUFlags::Avxneconvert => "avx-ne-convert",
            CPUFlags::AvxVnni => "avx-vnni",
            CPUFlags::AvxVnniint16 => "avx-vnni-int16",
            CPUFlags::AvxVnniint8 => "avx-vnni-int8",
            CPUFlags::Avx10 => "avx10",
            CPUFlags::Avx10128 => "avx10-128",
            CPUFlags::Avx10256 => "avx10-256",
            CPUFlags::Avx10512 => "avx10-512",
            CPUFlags::Avx2 => "avx2",
            CPUFlags::Avx5124fmaps => "avx512-4fmaps",
            CPUFlags::Avx5124vnniw => "avx512-4vnniw",
            CPUFlags::Avx512bf16 => "avx512-bf16",
            CPUFlags::Avx512fp16 => "avx512-fp16",
            CPUFlags::Avx512Vp2intersect => "avx512-vp2intersect",
            CPUFlags::Avx512Vpopcntdq => "avx512-vpopcntdq",
            CPUFlags::Avx512bitalg => "avx512bitalg",
            CPUFlags::Avx512bw => "avx512bw",
            CPUFlags::Avx512cd => "avx512cd",
            CPUFlags::Avx512dq => "avx512dq",
            CPUFlags::Avx512er => "avx512er",
            CPUFlags::Avx512f => "avx512f",
            CPUFlags::Avx512ifma => "avx512ifma",
            CPUFlags::Avx512pf => "avx512pf",
            CPUFlags::Avx512vbmi => "avx512vbmi",
            CPUFlags::Avx512vbmi2 => "avx512vbmi2",
            CPUFlags::Avx512vl => "avx512vl",
            CPUFlags::Avx512vnni => "avx512vnni",
            CPUFlags::Bhictrl => "bhi-ctrl",
            CPUFlags::Bhino => "bhi-no",
            CPUFlags::Bmi1 => "bmi1",
            CPUFlags::Bmi2 => "bmi2",
            CPUFlags::Buslockdetect => "bus-lock-detect",
            CPUFlags::Cid => "cid",
            CPUFlags::Cldemote => "cldemote",
            CPUFlags::Clflush => "clflush",
            CPUFlags::Clflushopt => "clflushopt",
            CPUFlags::Clwb => "clwb",
            CPUFlags::Clzero => "clzero",
            CPUFlags::Cmov => "cmov",
            CPUFlags::Cmplegacy => "cmp-legacy",
            CPUFlags::Cmpccxadd => "cmpccxadd",
            CPUFlags::Corecapability => "core-capability",
            CPUFlags::Cr8legacy => "cr8legacy",
            CPUFlags::Cx16 => "cx16",
            CPUFlags::Cx8 => "cx8",
            CPUFlags::Dca => "dca",
            CPUFlags::Ddpdu => "ddpd-u",
            CPUFlags::De => "de",
            CPUFlags::Decodeassists => "decodeassists",
            CPUFlags::Ds => "ds",
            CPUFlags::Dscpl => "ds-cpl",
            CPUFlags::Dtes64 => "dtes64",
            CPUFlags::Eraps => "eraps",
            CPUFlags::Erms => "erms",
            CPUFlags::Est => "est",
            CPUFlags::Extapic => "extapic",
            CPUFlags::F16c => "f16c",
            CPUFlags::Fbclear => "fb-clear",
            CPUFlags::Fbsdpno => "fbsdp-no",
            CPUFlags::Fdpexcptnonly => "fdp-excptn-only",
            CPUFlags::Flushl1d => "flush-l1d",
            CPUFlags::Flushbyasid => "flushbyasid",
            CPUFlags::Fma => "fma",
            CPUFlags::Fma4 => "fma4",
            CPUFlags::Fpu => "fpu",
            CPUFlags::Fred => "fred",
            CPUFlags::Fsgsbase => "fsgsbase",
            CPUFlags::Fsrc => "fsrc",
            CPUFlags::Fsrm => "fsrm",
            CPUFlags::Fsrs => "fsrs",
            CPUFlags::Fullwidthwrite => "full-width-write",
            CPUFlags::Fxsr => "fxsr",
            CPUFlags::Fxsropt => "fxsr-opt",
            CPUFlags::Fzrm => "fzrm",
            CPUFlags::Gdsno => "gds-no",
            CPUFlags::Gfni => "gfni",
            CPUFlags::Hle => "hle",
            CPUFlags::Ht => "ht",
            CPUFlags::Hypervisor => "hypervisor",
            CPUFlags::Ia64 => "ia64",
            CPUFlags::Ibpb => "ibpb",
            CPUFlags::Ibpbbrtype => "ibpb-brtype",
            CPUFlags::Ibrs => "ibrs",
            CPUFlags::Ibrsall => "ibrs-all",
            CPUFlags::Ibs => "ibs",
            CPUFlags::Intelpsfd => "intel-psfd",
            CPUFlags::Intelpt => "intel-pt",
            CPUFlags::Intelptlip => "intel-pt-lip",
            CPUFlags::Invpcid => "invpcid",
            CPUFlags::Invtsc => "invtsc",
            CPUFlags::Ipredctrl => "ipred-ctrl",
            CPUFlags::Kvmasyncpf => "kvm-asyncpf",
            CPUFlags::Kvmasyncpfint => "kvm-asyncpf-int",
            CPUFlags::KvmasyncpfVmexit => "kvm-asyncpf-vmexit",
            CPUFlags::Kvmhintdedicated => "kvm-hint-dedicated",
            CPUFlags::Kvmmmu => "kvm-mmu",
            CPUFlags::Kvmmsiextdestid => "kvm-msi-ext-dest-id",
            CPUFlags::Kvmnopiodelay => "kvm-nopiodelay",
            CPUFlags::Kvmpollcontrol => "kvm-poll-control",
            CPUFlags::Kvmpveoi => "kvm-pv-eoi",
            CPUFlags::Kvmpvipi => "kvm-pv-ipi",
            CPUFlags::Kvmpvschedyield => "kvm-pv-sched-yield",
            CPUFlags::Kvmpvtlbflush => "kvm-pv-tlb-flush",
            CPUFlags::Kvmpvunhalt => "kvm-pv-unhalt",
            CPUFlags::Kvmstealtime => "kvm-steal-time",
            CPUFlags::Kvmclock => "kvmclock",
            CPUFlags::Kvmclockstablebit => "kvmclock-stable-bit",
            CPUFlags::La57 => "la57",
            CPUFlags::Lahflm => "lahf-lm",
            CPUFlags::Lam => "lam",
            CPUFlags::Lbrv => "lbrv",
            CPUFlags::Lfencealwaysserializing => "lfence-always-serializing",
            CPUFlags::Lkgs => "lkgs",
            CPUFlags::Lm => "lm",
            CPUFlags::Lwp => "lwp",
            CPUFlags::Mca => "mca",
            CPUFlags::Mcdtno => "mcdt-no",
            CPUFlags::Mce => "mce",
            CPUFlags::Mdclear => "md-clear",
            CPUFlags::Mdsno => "mds-no",
            CPUFlags::Misalignsse => "misalignsse",
            CPUFlags::Mmx => "mmx",
            CPUFlags::Mmxext => "mmxext",
            CPUFlags::Monitor => "monitor",
            CPUFlags::Movbe => "movbe",
            CPUFlags::Movdir64b => "movdir64b",
            CPUFlags::Movdiri => "movdiri",
            CPUFlags::Mpx => "mpx",
            CPUFlags::Msr => "msr",
            CPUFlags::Mtrr => "mtrr",
            CPUFlags::Nonesteddatabp => "no-nested-data-bp",
            CPUFlags::Nodeidmsr => "nodeid-msr",
            CPUFlags::Npt => "npt",
            CPUFlags::Nripsave => "nrip-save",
            CPUFlags::Nullselclrbase => "null-sel-clr-base",
            CPUFlags::Nx => "nx",
            CPUFlags::Osvw => "osvw",
            CPUFlags::Overflowrecov => "overflow-recov",
            CPUFlags::Pae => "pae",
            CPUFlags::Pat => "pat",
            CPUFlags::Pausefilter => "pause-filter",
            CPUFlags::Pbe => "pbe",
            CPUFlags::Pbrsbno => "pbrsb-no",
            CPUFlags::Pcid => "pcid",
            CPUFlags::Pclmulqdq => "pclmulqdq",
            CPUFlags::Pcommit => "pcommit",
            CPUFlags::Pdcm => "pdcm",
            CPUFlags::Pdpe1gb => "pdpe1gb",
            CPUFlags::Perfctrcore => "perfctr-core",
            CPUFlags::Perfctrnb => "perfctr-nb",
            CPUFlags::PerfmonV2 => "perfmon-v2",
            CPUFlags::Pfthreshold => "pfthreshold",
            CPUFlags::Pge => "pge",
            CPUFlags::Phe => "phe",
            CPUFlags::Pheen => "phe-en",
            CPUFlags::Pks => "pks",
            CPUFlags::Pku => "pku",
            CPUFlags::Pmm => "pmm",
            CPUFlags::Pmmen => "pmm-en",
            CPUFlags::Pn => "pn",
            CPUFlags::Pni => "pni",
            CPUFlags::Popcnt => "popcnt",
            CPUFlags::Prefetchiti => "prefetchiti",
            CPUFlags::Pschangemcno => "pschange-mc-no",
            CPUFlags::Psdpno => "psdp-no",
            CPUFlags::Pse => "pse",
            CPUFlags::Pse36 => "pse36",
            CPUFlags::Rdctlno => "rdctl-no",
            CPUFlags::Rdpid => "rdpid",
            CPUFlags::Rdrand => "rdrand",
            CPUFlags::Rdseed => "rdseed",
            CPUFlags::Rdtscp => "rdtscp",
            CPUFlags::Rfdsclear => "rfds-clear",
            CPUFlags::Rfdsno => "rfds-no",
            CPUFlags::Rrsbactrl => "rrsba-ctrl",
            CPUFlags::Rsba => "rsba",
            CPUFlags::Rtm => "rtm",
            CPUFlags::Sbdrssdpno => "sbdr-ssdp-no",
            CPUFlags::Sbpb => "sbpb",
            CPUFlags::Sep => "sep",
            CPUFlags::Serialize => "serialize",
            CPUFlags::Sgx => "sgx",
            CPUFlags::Sgxaexnotify => "sgx-aex-notify",
            CPUFlags::Sgxdebug => "sgx-debug",
            CPUFlags::Sgxedeccssa => "sgx-edeccssa",
            CPUFlags::Sgxexinfo => "sgx-exinfo",
            CPUFlags::Sgxkss => "sgx-kss",
            CPUFlags::Sgxmode64 => "sgx-mode64",
            CPUFlags::Sgxprovisionkey => "sgx-provisionkey",
            CPUFlags::Sgxtokenkey => "sgx-tokenkey",
            CPUFlags::Sgx1 => "sgx1",
            CPUFlags::Sgx2 => "sgx2",
            CPUFlags::Sgxlc => "sgxlc",
            CPUFlags::Shani => "sha-ni",
            CPUFlags::Sha512 => "sha512",
            CPUFlags::Skinit => "skinit",
            CPUFlags::Skipl1dflVmentry => "skip-l1dfl-vmentry",
            CPUFlags::Sm3 => "sm3",
            CPUFlags::Sm4 => "sm4",
            CPUFlags::Smap => "smap",
            CPUFlags::Smep => "smep",
            CPUFlags::Smx => "smx",
            CPUFlags::Specctrl => "spec-ctrl",
            CPUFlags::Splitlockdetect => "split-lock-detect",
            CPUFlags::Srsono => "srso-no",
            CPUFlags::Srsouserkernelno => "srso-user-kernel-no",
            CPUFlags::Ss => "ss",
            CPUFlags::Ssbno => "ssb-no",
            CPUFlags::Ssbd => "ssbd",
            CPUFlags::Sse => "sse",
            CPUFlags::Sse2 => "sse2",
            CPUFlags::Sse4_1 => "sse4.1",
            CPUFlags::Sse4_2 => "sse4.2",
            CPUFlags::Sse4a => "sse4a",
            CPUFlags::Ssse3 => "ssse3",
            CPUFlags::Stibp => "stibp",
            CPUFlags::Stibpalwayson => "stibp-always-on",
            CPUFlags::Succor => "succor",
            CPUFlags::Svm => "svm",
            CPUFlags::Svmlock => "svm-lock",
            CPUFlags::Svmeaddrchk => "svme-addr-chk",
            CPUFlags::Syscall => "syscall",
            CPUFlags::Taano => "taa-no",
            CPUFlags::Tbm => "tbm",
            CPUFlags::Tce => "tce",
            CPUFlags::Tm => "tm",
            CPUFlags::Tm2 => "tm2",
            CPUFlags::Topoext => "topoext",
            CPUFlags::Tsc => "tsc",
            CPUFlags::Tscadjust => "tsc-adjust",
            CPUFlags::Tscdeadline => "tsc-deadline",
            CPUFlags::Tscscale => "tsc-scale",
            CPUFlags::Tsxctrl => "tsx-ctrl",
            CPUFlags::Tsxldtrk => "tsx-ldtrk",
            CPUFlags::Umip => "umip",
            CPUFlags::VVmsavevmload => "v-vmsave-vmload",
            CPUFlags::Vaes => "vaes",
            CPUFlags::Vgif => "vgif",
            CPUFlags::Virtssbd => "virt-ssbd",
            CPUFlags::Vmcbclean => "vmcb-clean",
            CPUFlags::Vme => "vme",
            CPUFlags::Vmx => "vmx",
            CPUFlags::Vmxactivityhlt => "vmx-activity-hlt",
            CPUFlags::Vmxactivityshutdown => "vmx-activity-shutdown",
            CPUFlags::Vmxactivitywaitsipi => "vmx-activity-wait-sipi",
            CPUFlags::Vmxanyerrcode => "vmx-any-errcode",
            CPUFlags::Vmxapicvregister => "vmx-apicv-register",
            CPUFlags::VmxapicvVid => "vmx-apicv-vid",
            CPUFlags::Vmxapicvx2apic => "vmx-apicv-x2apic",
            CPUFlags::Vmxapicvxapic => "vmx-apicv-xapic",
            CPUFlags::Vmxcr3loadnoexit => "vmx-cr3-load-noexit",
            CPUFlags::Vmxcr3storenoexit => "vmx-cr3-store-noexit",
            CPUFlags::Vmxcr8loadexit => "vmx-cr8-load-exit",
            CPUFlags::Vmxcr8storeexit => "vmx-cr8-store-exit",
            CPUFlags::Vmxdescexit => "vmx-desc-exit",
            CPUFlags::Vmxenableuserwaitpause => "vmx-enable-user-wait-pause",
            CPUFlags::Vmxenclsexit => "vmx-encls-exit",
            CPUFlags::Vmxentryia32emode => "vmx-entry-ia32e-mode",
            CPUFlags::Vmxentryloadbndcfgs => "vmx-entry-load-bndcfgs",
            CPUFlags::Vmxentryloadefer => "vmx-entry-load-efer",
            CPUFlags::Vmxentryloadfred => "vmx-entry-load-fred",
            CPUFlags::Vmxentryloadpat => "vmx-entry-load-pat",
            CPUFlags::Vmxentryloadperfglobalctrl => "vmx-entry-load-perf-global-ctrl",
            CPUFlags::Vmxentryloadpkrs => "vmx-entry-load-pkrs",
            CPUFlags::Vmxentryloadrtitctl => "vmx-entry-load-rtit-ctl",
            CPUFlags::Vmxentrynoloaddebugctl => "vmx-entry-noload-debugctl",
            CPUFlags::Vmxept => "vmx-ept",
            CPUFlags::Vmxept1gb => "vmx-ept-1gb",
            CPUFlags::Vmxept2mb => "vmx-ept-2mb",
            CPUFlags::Vmxeptadvancedexitinfo => "vmx-ept-advanced-exitinfo",
            CPUFlags::Vmxeptexeconly => "vmx-ept-execonly",
            CPUFlags::Vmxeptad => "vmx-eptad",
            CPUFlags::Vmxeptpswitching => "vmx-eptp-switching",
            CPUFlags::Vmxexitackintr => "vmx-exit-ack-intr",
            CPUFlags::Vmxexitclearbndcfgs => "vmx-exit-clear-bndcfgs",
            CPUFlags::Vmxexitclearrtitctl => "vmx-exit-clear-rtit-ctl",
            CPUFlags::Vmxexitloadefer => "vmx-exit-load-efer",
            CPUFlags::Vmxexitloadpat => "vmx-exit-load-pat",
            CPUFlags::Vmxexitloadperfglobalctrl => "vmx-exit-load-perf-global-ctrl",
            CPUFlags::Vmxexitloadpkrs => "vmx-exit-load-pkrs",
            CPUFlags::Vmxexitnosavedebugctl => "vmx-exit-nosave-debugctl",
            CPUFlags::Vmxexitsaveefer => "vmx-exit-save-efer",
            CPUFlags::Vmxexitsavepat => "vmx-exit-save-pat",
            CPUFlags::Vmxexitsavepreemptiontimer => "vmx-exit-save-preemption-timer",
            CPUFlags::Vmxexitsecondaryctls => "vmx-exit-secondary-ctls",
            CPUFlags::Vmxflexpriority => "vmx-flexpriority",
            CPUFlags::Vmxhltexit => "vmx-hlt-exit",
            CPUFlags::Vmxinsouts => "vmx-ins-outs",
            CPUFlags::Vmxintrexit => "vmx-intr-exit",
            CPUFlags::Vmxinvept => "vmx-invept",
            CPUFlags::Vmxinveptallcontext => "vmx-invept-all-context",
            CPUFlags::Vmxinveptsinglecontext => "vmx-invept-single-context",
            CPUFlags::Vmxinveptsinglecontextnoglobals => "vmx-invept-single-context-noglobals",
            CPUFlags::Vmxinvlpgexit => "vmx-invlpg-exit",
            CPUFlags::Vmxinvpcidexit => "vmx-invpcid-exit",
            CPUFlags::Vmxinvvpid => "vmx-invvpid",
            CPUFlags::Vmxinvvpidallcontext => "vmx-invvpid-all-context",
            CPUFlags::Vmxinvvpidsingleaddr => "vmx-invvpid-single-addr",
            CPUFlags::Vmxiobitmap => "vmx-io-bitmap",
            CPUFlags::Vmxioexit => "vmx-io-exit",
            CPUFlags::Vmxmonitorexit => "vmx-monitor-exit",
            CPUFlags::Vmxmovdrexit => "vmx-movdr-exit",
            CPUFlags::Vmxmsrbitmap => "vmx-msr-bitmap",
            CPUFlags::Vmxmtf => "vmx-mtf",
            CPUFlags::Vmxmwaitexit => "vmx-mwait-exit",
            CPUFlags::Vmxnestedexception => "vmx-nested-exception",
            CPUFlags::Vmxnmiexit => "vmx-nmi-exit",
            CPUFlags::Vmxpagewalk4 => "vmx-page-walk-4",
            CPUFlags::Vmxpagewalk5 => "vmx-page-walk-5",
            CPUFlags::Vmxpauseexit => "vmx-pause-exit",
            CPUFlags::Vmxple => "vmx-ple",
            CPUFlags::Vmxpml => "vmx-pml",
            CPUFlags::Vmxpostedintr => "vmx-posted-intr",
            CPUFlags::Vmxpreemptiontimer => "vmx-preemption-timer",
            CPUFlags::Vmxrdpmcexit => "vmx-rdpmc-exit",
            CPUFlags::Vmxrdrandexit => "vmx-rdrand-exit",
            CPUFlags::Vmxrdseedexit => "vmx-rdseed-exit",
            CPUFlags::Vmxrdtscexit => "vmx-rdtsc-exit",
            CPUFlags::Vmxrdtscpexit => "vmx-rdtscp-exit",
            CPUFlags::Vmxsecondaryctls => "vmx-secondary-ctls",
            CPUFlags::VmxshadowVmcs => "vmx-shadow-vmcs",
            CPUFlags::Vmxstorelma => "vmx-store-lma",
            CPUFlags::Vmxtruectls => "vmx-true-ctls",
            CPUFlags::Vmxtscoffset => "vmx-tsc-offset",
            CPUFlags::Vmxtscscaling => "vmx-tsc-scaling",
            CPUFlags::Vmxunrestrictedguest => "vmx-unrestricted-guest",
            CPUFlags::VmxVintrpending => "vmx-vintr-pending",
            CPUFlags::VmxVmfunc => "vmx-vmfunc",
            CPUFlags::VmxVmwritevmexitfields => "vmx-vmwrite-vmexit-fields",
            CPUFlags::VmxVnmi => "vmx-vnmi",
            CPUFlags::VmxVnmipending => "vmx-vnmi-pending",
            CPUFlags::VmxVpid => "vmx-vpid",
            CPUFlags::Vmxwbinvdexit => "vmx-wbinvd-exit",
            CPUFlags::Vmxxsaves => "vmx-xsaves",
            CPUFlags::Vmxzeroleninject => "vmx-zero-len-inject",
            CPUFlags::Vnmi => "vnmi",
            CPUFlags::Vpclmulqdq => "vpclmulqdq",
            CPUFlags::Waitpkg => "waitpkg",
            CPUFlags::Wbnoinvd => "wbnoinvd",
            CPUFlags::Wdt => "wdt",
            CPUFlags::Wrmsrns => "wrmsrns",
            CPUFlags::X2apic => "x2apic",
            CPUFlags::Xcrypt => "xcrypt",
            CPUFlags::Xcrypten => "xcrypt-en",
            CPUFlags::Xfd => "xfd",
            CPUFlags::Xgetbv1 => "xgetbv1",
            CPUFlags::Xop => "xop",
            CPUFlags::Xsave => "xsave",
            CPUFlags::Xsavec => "xsavec",
            CPUFlags::Xsaveerptr => "xsaveerptr",
            CPUFlags::Xsaveopt => "xsaveopt",
            CPUFlags::Xsaves => "xsaves",
            CPUFlags::Xstore => "xstore",
            CPUFlags::Xstoreen => "xstore-en",
            CPUFlags::Xtpr => "xtpr",
            CPUFlags::Zerofcsfds => "zero-fcs-fds",
        }
    }
}
