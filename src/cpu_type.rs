use crate::to_command::{ToArg, ToCommand};
use std::str::FromStr;
#[derive(Debug, Clone)]
pub struct CpuNotFound;
#[derive(Debug, Clone)]
pub enum CpuTypeAarch64 {
    /// a64fx
    A64fx,
    /// arm1026
    Arm1026,
    /// arm1136
    Arm1136,
    /// arm1136-r2
    Arm1136r2,
    /// arm1176
    Arm1176,
    /// arm11mpcore
    Arm11mpcore,
    /// arm926
    Arm926,
    /// arm946
    Arm946,
    /// cortex-a15
    Cortexa15,
    /// cortex-a35
    Cortexa35,
    /// cortex-a53
    Cortexa53,
    /// cortex-a55
    Cortexa55,
    /// cortex-a57
    Cortexa57,
    /// cortex-a7
    Cortexa7,
    /// cortex-a710
    Cortexa710,
    /// cortex-a72
    Cortexa72,
    /// cortex-a76
    Cortexa76,
    /// cortex-a8
    Cortexa8,
    /// cortex-a9
    Cortexa9,
    /// cortex-m0
    Cortexm0,
    /// cortex-m3
    Cortexm3,
    /// cortex-m33
    Cortexm33,
    /// cortex-m4
    Cortexm4,
    /// cortex-m55
    Cortexm55,
    /// cortex-m7
    Cortexm7,
    /// cortex-r5
    Cortexr5,
    /// cortex-r52
    Cortexr52,
    /// cortex-r5f
    Cortexr5f,
    /// host
    Host,
    /// max
    Max,
    /// neoverse-n1
    Neoversen1,
    /// neoverse-n2
    Neoversen2,
    /// neoverse-v1
    NeoverseV1,
    /// (deprecated)
    Pxa250,
    /// (deprecated)
    Pxa255,
    /// (deprecated)
    Pxa260,
    /// (deprecated)
    Pxa261,
    /// (deprecated)
    Pxa262,
    /// (deprecated)
    Pxa270a0,
    /// (deprecated)
    Pxa270a1,
    /// (deprecated)
    Pxa270,
    /// (deprecated)
    Pxa270b0,
    /// (deprecated)
    Pxa270b1,
    /// (deprecated)
    Pxa270c0,
    /// (deprecated)
    Pxa270c5,
    /// sa1100
    Sa1100,
    /// sa1110
    Sa1110,
    /// ti925t
    Ti925t,
}
impl ToCommand for CpuTypeAarch64 {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![];

        match self {
            CpuTypeAarch64::A64fx => cmd.push("a64fx".to_string()),
            CpuTypeAarch64::Arm1026 => cmd.push("arm1026".to_string()),
            CpuTypeAarch64::Arm1136 => cmd.push("arm1136".to_string()),
            CpuTypeAarch64::Arm1136r2 => cmd.push("arm1136-r2".to_string()),
            CpuTypeAarch64::Arm1176 => cmd.push("arm1176".to_string()),
            CpuTypeAarch64::Arm11mpcore => cmd.push("arm11mpcore".to_string()),
            CpuTypeAarch64::Arm926 => cmd.push("arm926".to_string()),
            CpuTypeAarch64::Arm946 => cmd.push("arm946".to_string()),
            CpuTypeAarch64::Cortexa15 => cmd.push("cortex-a15".to_string()),
            CpuTypeAarch64::Cortexa35 => cmd.push("cortex-a35".to_string()),
            CpuTypeAarch64::Cortexa53 => cmd.push("cortex-a53".to_string()),
            CpuTypeAarch64::Cortexa55 => cmd.push("cortex-a55".to_string()),
            CpuTypeAarch64::Cortexa57 => cmd.push("cortex-a57".to_string()),
            CpuTypeAarch64::Cortexa7 => cmd.push("cortex-a7".to_string()),
            CpuTypeAarch64::Cortexa710 => cmd.push("cortex-a710".to_string()),
            CpuTypeAarch64::Cortexa72 => cmd.push("cortex-a72".to_string()),
            CpuTypeAarch64::Cortexa76 => cmd.push("cortex-a76".to_string()),
            CpuTypeAarch64::Cortexa8 => cmd.push("cortex-a8".to_string()),
            CpuTypeAarch64::Cortexa9 => cmd.push("cortex-a9".to_string()),
            CpuTypeAarch64::Cortexm0 => cmd.push("cortex-m0".to_string()),
            CpuTypeAarch64::Cortexm3 => cmd.push("cortex-m3".to_string()),
            CpuTypeAarch64::Cortexm33 => cmd.push("cortex-m33".to_string()),
            CpuTypeAarch64::Cortexm4 => cmd.push("cortex-m4".to_string()),
            CpuTypeAarch64::Cortexm55 => cmd.push("cortex-m55".to_string()),
            CpuTypeAarch64::Cortexm7 => cmd.push("cortex-m7".to_string()),
            CpuTypeAarch64::Cortexr5 => cmd.push("cortex-r5".to_string()),
            CpuTypeAarch64::Cortexr52 => cmd.push("cortex-r52".to_string()),
            CpuTypeAarch64::Cortexr5f => cmd.push("cortex-r5f".to_string()),
            CpuTypeAarch64::Host => cmd.push("host".to_string()),
            CpuTypeAarch64::Max => cmd.push("max".to_string()),
            CpuTypeAarch64::Neoversen1 => cmd.push("neoverse-n1".to_string()),
            CpuTypeAarch64::Neoversen2 => cmd.push("neoverse-n2".to_string()),
            CpuTypeAarch64::NeoverseV1 => cmd.push("neoverse-v1".to_string()),
            CpuTypeAarch64::Pxa250 => cmd.push("pxa250".to_string()),
            CpuTypeAarch64::Pxa255 => cmd.push("pxa255".to_string()),
            CpuTypeAarch64::Pxa260 => cmd.push("pxa260".to_string()),
            CpuTypeAarch64::Pxa261 => cmd.push("pxa261".to_string()),
            CpuTypeAarch64::Pxa262 => cmd.push("pxa262".to_string()),
            CpuTypeAarch64::Pxa270a0 => cmd.push("pxa270-a0".to_string()),
            CpuTypeAarch64::Pxa270a1 => cmd.push("pxa270-a1".to_string()),
            CpuTypeAarch64::Pxa270 => cmd.push("pxa270".to_string()),
            CpuTypeAarch64::Pxa270b0 => cmd.push("pxa270-b0".to_string()),
            CpuTypeAarch64::Pxa270b1 => cmd.push("pxa270-b1".to_string()),
            CpuTypeAarch64::Pxa270c0 => cmd.push("pxa270-c0".to_string()),
            CpuTypeAarch64::Pxa270c5 => cmd.push("pxa270-c5".to_string()),
            CpuTypeAarch64::Sa1100 => cmd.push("sa1100".to_string()),
            CpuTypeAarch64::Sa1110 => cmd.push("sa1110".to_string()),
            CpuTypeAarch64::Ti925t => cmd.push("ti925t".to_string()),
        }
        cmd
    }
}

impl ToArg for CpuTypeAarch64 {
    fn to_arg(&self) -> &str {
        match self {
            CpuTypeAarch64::A64fx => "a64fx",
            CpuTypeAarch64::Arm1026 => "arm1026",
            CpuTypeAarch64::Arm1136 => "arm1136",
            CpuTypeAarch64::Arm1136r2 => "arm1136-r2",
            CpuTypeAarch64::Arm1176 => "arm1176",
            CpuTypeAarch64::Arm11mpcore => "arm11mpcore",
            CpuTypeAarch64::Arm926 => "arm926",
            CpuTypeAarch64::Arm946 => "arm946",
            CpuTypeAarch64::Cortexa15 => "cortex-a15",
            CpuTypeAarch64::Cortexa35 => "cortex-a35",
            CpuTypeAarch64::Cortexa53 => "cortex-a53",
            CpuTypeAarch64::Cortexa55 => "cortex-a55",
            CpuTypeAarch64::Cortexa57 => "cortex-a57",
            CpuTypeAarch64::Cortexa7 => "cortex-a7",
            CpuTypeAarch64::Cortexa710 => "cortex-a710",
            CpuTypeAarch64::Cortexa72 => "cortex-a72",
            CpuTypeAarch64::Cortexa76 => "cortex-a76",
            CpuTypeAarch64::Cortexa8 => "cortex-a8",
            CpuTypeAarch64::Cortexa9 => "cortex-a9",
            CpuTypeAarch64::Cortexm0 => "cortex-m0",
            CpuTypeAarch64::Cortexm3 => "cortex-m3",
            CpuTypeAarch64::Cortexm33 => "cortex-m33",
            CpuTypeAarch64::Cortexm4 => "cortex-m4",
            CpuTypeAarch64::Cortexm55 => "cortex-m55",
            CpuTypeAarch64::Cortexm7 => "cortex-m7",
            CpuTypeAarch64::Cortexr5 => "cortex-r5",
            CpuTypeAarch64::Cortexr52 => "cortex-r52",
            CpuTypeAarch64::Cortexr5f => "cortex-r5f",
            CpuTypeAarch64::Host => "host",
            CpuTypeAarch64::Max => "max",
            CpuTypeAarch64::Neoversen1 => "neoverse-n1",
            CpuTypeAarch64::Neoversen2 => "neoverse-n2",
            CpuTypeAarch64::NeoverseV1 => "neoverse-v1",
            CpuTypeAarch64::Pxa250 => "pxa250",
            CpuTypeAarch64::Pxa255 => "pxa255",
            CpuTypeAarch64::Pxa260 => "pxa260",
            CpuTypeAarch64::Pxa261 => "pxa261",
            CpuTypeAarch64::Pxa262 => "pxa262",
            CpuTypeAarch64::Pxa270a0 => "pxa270-a0",
            CpuTypeAarch64::Pxa270a1 => "pxa270-a1",
            CpuTypeAarch64::Pxa270 => "pxa270",
            CpuTypeAarch64::Pxa270b0 => "pxa270-b0",
            CpuTypeAarch64::Pxa270b1 => "pxa270-b1",
            CpuTypeAarch64::Pxa270c0 => "pxa270-c0",
            CpuTypeAarch64::Pxa270c5 => "pxa270-c5",
            CpuTypeAarch64::Sa1100 => "sa1100",
            CpuTypeAarch64::Sa1110 => "sa1110",
            CpuTypeAarch64::Ti925t => "ti925t",
        }
    }
}

impl FromStr for CpuTypeAarch64 {
    type Err = CpuNotFound;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a64fx" => Ok(CpuTypeAarch64::A64fx),
            "arm1026" => Ok(CpuTypeAarch64::Arm1026),
            "arm1136" => Ok(CpuTypeAarch64::Arm1136),
            "arm1136-r2" => Ok(CpuTypeAarch64::Arm1136r2),
            "arm1176" => Ok(CpuTypeAarch64::Arm1176),
            "arm11mpcore" => Ok(CpuTypeAarch64::Arm11mpcore),
            "arm926" => Ok(CpuTypeAarch64::Arm926),
            "arm946" => Ok(CpuTypeAarch64::Arm946),
            "cortex-a15" => Ok(CpuTypeAarch64::Cortexa15),
            "cortex-a35" => Ok(CpuTypeAarch64::Cortexa35),
            "cortex-a53" => Ok(CpuTypeAarch64::Cortexa53),
            "cortex-a55" => Ok(CpuTypeAarch64::Cortexa55),
            "cortex-a57" => Ok(CpuTypeAarch64::Cortexa57),
            "cortex-a7" => Ok(CpuTypeAarch64::Cortexa7),
            "cortex-a710" => Ok(CpuTypeAarch64::Cortexa710),
            "cortex-a72" => Ok(CpuTypeAarch64::Cortexa72),
            "cortex-a76" => Ok(CpuTypeAarch64::Cortexa76),
            "cortex-a8" => Ok(CpuTypeAarch64::Cortexa8),
            "cortex-a9" => Ok(CpuTypeAarch64::Cortexa9),
            "cortex-m0" => Ok(CpuTypeAarch64::Cortexm0),
            "cortex-m3" => Ok(CpuTypeAarch64::Cortexm3),
            "cortex-m33" => Ok(CpuTypeAarch64::Cortexm33),
            "cortex-m4" => Ok(CpuTypeAarch64::Cortexm4),
            "cortex-m55" => Ok(CpuTypeAarch64::Cortexm55),
            "cortex-m7" => Ok(CpuTypeAarch64::Cortexm7),
            "cortex-r5" => Ok(CpuTypeAarch64::Cortexr5),
            "cortex-r52" => Ok(CpuTypeAarch64::Cortexr52),
            "cortex-r5f" => Ok(CpuTypeAarch64::Cortexr5f),
            "host" => Ok(CpuTypeAarch64::Host),
            "max" => Ok(CpuTypeAarch64::Max),
            "neoverse-n1" => Ok(CpuTypeAarch64::Neoversen1),
            "neoverse-n2" => Ok(CpuTypeAarch64::Neoversen2),
            "neoverse-v1" => Ok(CpuTypeAarch64::NeoverseV1),
            "pxa250" => Ok(CpuTypeAarch64::Pxa250),
            "pxa255" => Ok(CpuTypeAarch64::Pxa255),
            "pxa260" => Ok(CpuTypeAarch64::Pxa260),
            "pxa261" => Ok(CpuTypeAarch64::Pxa261),
            "pxa262" => Ok(CpuTypeAarch64::Pxa262),
            "pxa270-a0" => Ok(CpuTypeAarch64::Pxa270a0),
            "pxa270-a1" => Ok(CpuTypeAarch64::Pxa270a1),
            "pxa270" => Ok(CpuTypeAarch64::Pxa270),
            "pxa270-b0" => Ok(CpuTypeAarch64::Pxa270b0),
            "pxa270-b1" => Ok(CpuTypeAarch64::Pxa270b1),
            "pxa270-c0" => Ok(CpuTypeAarch64::Pxa270c0),
            "pxa270-c5" => Ok(CpuTypeAarch64::Pxa270c5),
            "sa1100" => Ok(CpuTypeAarch64::Sa1100),
            "sa1110" => Ok(CpuTypeAarch64::Sa1110),
            "ti925t" => Ok(CpuTypeAarch64::Ti925t),
            _ => Err(CpuNotFound),
        }
    }
}

#[derive(Debug, Clone)]
pub enum CpuTypeX86_64 {
    /// (alias configured by machine type)
    X486,
    /// 486-v1
    X486V1,
    /// (alias configured by machine type)
    Broadwell,
    /// (alias of Broadwell-v3)
    BroadwellIBRS,
    /// (alias of Broadwell-v2)
    BroadwellnoTSX,
    /// (alias of Broadwell-v4)
    BroadwellnoTSXIBRS,
    /// Intel Core Processor (Broadwell)
    BroadwellV1,
    /// Intel Core Processor (Broadwell, no TSX)
    BroadwellV2,
    /// Intel Core Processor (Broadwell, IBRS)
    BroadwellV3,
    /// Intel Core Processor (Broadwell, no TSX, IBRS)
    BroadwellV4,
    /// (alias configured by machine type)
    CascadelakeServer,
    /// (alias of Cascadelake-Server-v3)
    CascadelakeServernoTSX,
    /// Intel Xeon Processor (Cascadelake)
    CascadelakeServerV1,
    /// Intel Xeon Processor (Cascadelake) [ARCH_CAPABILITIES]
    CascadelakeServerV2,
    /// Intel Xeon Processor (Cascadelake) [ARCH_CAPABILITIES, no TSX]
    CascadelakeServerV3,
    /// Intel Xeon Processor (Cascadelake) [ARCH_CAPABILITIES, EPT switching, no TSX]
    CascadelakeServerV4,
    /// Intel Xeon Processor (Cascadelake) [ARCH_CAPABILITIES, EPT switching, XSAVES, no TSX]
    CascadelakeServerV5,
    /// (alias configured by machine type)
    ClearwaterForest,
    /// Intel Xeon Processor (ClearwaterForest)
    ClearwaterForestV1,
    /// (alias configured by machine type)
    Conroe,
    /// Intel Celeron_4x0 (Conroe/Merom Class Core 2)
    ConroeV1,
    /// (alias configured by machine type)
    Cooperlake,
    /// Intel Xeon Processor (Cooperlake)
    CooperlakeV1,
    /// Intel Xeon Processor (Cooperlake) [XSAVES]
    CooperlakeV2,
    /// (alias configured by machine type)
    Denverton,
    /// Intel Atom Processor (Denverton)
    DenvertonV1,
    /// Intel Atom Processor (Denverton) [no MPX, no MONITOR]
    DenvertonV2,
    /// Intel Atom Processor (Denverton) [XSAVES, no MPX, no MONITOR]
    DenvertonV3,
    /// (alias configured by machine type)
    Dhyana,
    /// Hygon Dhyana Processor
    DhyanaV1,
    /// Hygon Dhyana Processor [XSAVES]
    DhyanaV2,
    /// (alias configured by machine type)
    EPYC,
    /// (alias configured by machine type)
    EPYCGenoa,
    /// AMD EPYC-Genoa Processor
    EPYCGenoaV1,
    /// AMD EPYC-Genoa-v2 Processor
    EPYCGenoaV2,
    /// (alias of EPYC-v2)
    EPYCIBPB,
    /// (alias configured by machine type)
    EPYCMilan,
    /// AMD EPYC-Milan Processor
    EPYCMilanV1,
    /// AMD EPYC-Milan-v2 Processor
    EPYCMilanV2,
    /// AMD EPYC-Milan-v3 Processor
    EPYCMilanV3,
    /// (alias configured by machine type)
    EPYCRome,
    /// AMD EPYC-Rome Processor
    EPYCRomeV1,
    /// AMD EPYC-Rome Processor
    EPYCRomeV2,
    /// AMD EPYC-Rome-v3 Processor
    EPYCRomeV3,
    /// AMD EPYC-Rome-v4 Processor (no XSAVES)
    EPYCRomeV4,
    /// AMD EPYC-Rome-v5 Processor
    EPYCRomeV5,
    /// (alias configured by machine type)
    EPYCTurin,
    /// AMD EPYC-Turin Processor
    EPYCTurinV1,
    /// AMD EPYC Processor
    EPYCV1,
    /// AMD EPYC Processor (with IBPB)
    EPYCV2,
    /// AMD EPYC Processor
    EPYCV3,
    /// AMD EPYC-v4 Processor
    EPYCV4,
    /// AMD EPYC-v5 Processor
    EPYCV5,
    /// (alias configured by machine type)
    GraniteRapids,
    /// Intel Xeon Processor (GraniteRapids)
    GraniteRapidsV1,
    /// Intel Xeon Processor (GraniteRapids)
    GraniteRapidsV2,
    /// Intel Xeon Processor (GraniteRapids) [with gnr-sp cache model and 0x1f leaf]
    GraniteRapidsV3,
    /// (alias configured by machine type)
    Haswell,
    /// (alias of Haswell-v3)
    HaswellIBRS,
    /// (alias of Haswell-v2)
    HaswellnoTSX,
    /// (alias of Haswell-v4)
    HaswellnoTSXIBRS,
    /// Intel Core Processor (Haswell)
    HaswellV1,
    /// Intel Core Processor (Haswell, no TSX)
    HaswellV2,
    /// Intel Core Processor (Haswell, IBRS)
    HaswellV3,
    /// Intel Core Processor (Haswell, no TSX, IBRS)
    HaswellV4,
    /// (alias configured by machine type)
    IcelakeServer,
    /// (alias of Icelake-Server-v2)
    IcelakeServernoTSX,
    /// Intel Xeon Processor (Icelake)
    IcelakeServerV1,
    /// Intel Xeon Processor (Icelake) [no TSX]
    IcelakeServerV2,
    /// Intel Xeon Processor (Icelake)
    IcelakeServerV3,
    /// Intel Xeon Processor (Icelake)
    IcelakeServerV4,
    /// Intel Xeon Processor (Icelake) [XSAVES]
    IcelakeServerV5,
    /// Intel Xeon Processor (Icelake) [5-level EPT]
    IcelakeServerV6,
    /// Intel Xeon Processor (Icelake) [TSX, taa-no]
    IcelakeServerV7,
    /// (alias configured by machine type)
    IvyBridge,
    /// (alias of IvyBridge-v2)
    IvyBridgeIBRS,
    /// Intel Xeon E3-12xx v2 (Ivy Bridge)
    IvyBridgeV1,
    /// Intel Xeon E3-12xx v2 (Ivy Bridge, IBRS)
    IvyBridgeV2,
    /// (alias configured by machine type)
    KnightsMill,
    /// Intel Xeon Phi Processor (Knights Mill)
    KnightsMillV1,
    /// (alias configured by machine type)
    Nehalem,
    /// (alias of Nehalem-v2)
    NehalemIBRS,
    /// Intel Core i7 9xx (Nehalem Class Core i7)
    NehalemV1,
    /// Intel Core i7 9xx (Nehalem Core i7, IBRS update)
    NehalemV2,
    /// (alias configured by machine type)
    OpteronG1,
    /// AMD Opteron 240 (Gen 1 Class Opteron)
    OpteronG1V1,
    /// (alias configured by machine type)
    OpteronG2,
    /// AMD Opteron 22xx (Gen 2 Class Opteron)
    OpteronG2V1,
    /// (alias configured by machine type)
    OpteronG3,
    /// AMD Opteron 23xx (Gen 3 Class Opteron)
    OpteronG3V1,
    /// (alias configured by machine type)
    OpteronG4,
    /// AMD Opteron 62xx class CPU
    OpteronG4V1,
    /// (alias configured by machine type)
    OpteronG5,
    /// AMD Opteron 63xx class CPU
    OpteronG5V1,
    /// (alias configured by machine type)
    Penryn,
    /// Intel Core 2 Duo P9xxx (Penryn Class Core 2)
    PenrynV1,
    /// (alias configured by machine type)
    SandyBridge,
    /// (alias of SandyBridge-v2)
    SandyBridgeIBRS,
    /// Intel Xeon E312xx (Sandy Bridge)
    SandyBridgeV1,
    /// Intel Xeon E312xx (Sandy Bridge, IBRS update)
    SandyBridgeV2,
    /// (alias configured by machine type)
    SapphireRapids,
    /// Intel Xeon Processor (SapphireRapids)
    SapphireRapidsV1,
    /// Intel Xeon Processor (SapphireRapids)
    SapphireRapidsV2,
    /// Intel Xeon Processor (SapphireRapids)
    SapphireRapidsV3,
    /// Intel Xeon Processor (SapphireRapids) [with spr-sp cache model and 0x1f leaf]
    SapphireRapidsV4,
    /// (alias configured by machine type)
    SierraForest,
    /// Intel Xeon Processor (SierraForest)
    SierraForestV1,
    /// Intel Xeon Processor (SierraForest)
    SierraForestV2,
    /// Intel Xeon Processor (SierraForest) [with srf-sp cache model and 0x1f leaf]
    SierraForestV3,
    /// (alias configured by machine type)
    SkylakeClient,
    /// (alias of Skylake-Client-v2)
    SkylakeClientIBRS,
    /// (alias of Skylake-Client-v3)
    SkylakeClientnoTSXIBRS,
    /// Intel Core Processor (Skylake)
    SkylakeClientV1,
    /// Intel Core Processor (Skylake, IBRS)
    SkylakeClientV2,
    /// Intel Core Processor (Skylake, IBRS, no TSX)
    SkylakeClientV3,
    /// Intel Core Processor (Skylake, IBRS, no TSX) [IBRS, XSAVES, no TSX]
    SkylakeClientV4,
    /// (alias configured by machine type)
    SkylakeServer,
    /// (alias of Skylake-Server-v2)
    SkylakeServerIBRS,
    /// (alias of Skylake-Server-v3)
    SkylakeServernoTSXIBRS,
    /// Intel Xeon Processor (Skylake)
    SkylakeServerV1,
    /// Intel Xeon Processor (Skylake, IBRS)
    SkylakeServerV2,
    /// Intel Xeon Processor (Skylake, IBRS, no TSX)
    SkylakeServerV3,
    /// Intel Xeon Processor (Skylake, IBRS, no TSX) [IBRS, EPT switching, no TSX]
    SkylakeServerV4,
    /// Intel Xeon Processor (Skylake, IBRS, no TSX) [IBRS, XSAVES, EPT switching, no TSX]
    SkylakeServerV5,
    /// (alias configured by machine type)
    Snowridge,
    /// Intel Atom Processor (SnowRidge)
    SnowridgeV1,
    /// Intel Atom Processor (Snowridge, no MPX)
    SnowridgeV2,
    /// Intel Atom Processor (Snowridge, no MPX) [XSAVES, no MPX]
    SnowridgeV3,
    /// Intel Atom Processor (Snowridge, no MPX) [no split lock detect, no core-capability]
    SnowridgeV4,
    /// (alias configured by machine type)
    Westmere,
    /// (alias of Westmere-v2)
    WestmereIBRS,
    /// Westmere E56xx/L56xx/X56xx (Nehalem-C)
    WestmereV1,
    /// Westmere E56xx/L56xx/X56xx (IBRS update)
    WestmereV2,
    /// (alias configured by machine type)
    YongFeng,
    /// Zhaoxin YongFeng Processor
    YongFengV1,
    /// Zhaoxin YongFeng Processor [with the correct model number]
    YongFengV2,
    /// Zhaoxin YongFeng Processor [with the cache model and 0x1f leaf]
    YongFengV3,
    /// (alias configured by machine type)
    Athlon,
    /// QEMU Virtual CPU version 2.5+
    AthlonV1,
    /// (alias configured by machine type)
    Core2duo,
    /// Intel(R) Core(TM)2 Duo CPU T7700 @ 2.40GHz
    Core2duoV1,
    /// (alias configured by machine type)
    Coreduo,
    /// Genuine Intel(R) CPU T2600 @ 2.16GHz
    CoreduoV1,
    /// (alias configured by machine type)
    Kvm32,
    /// Common 32-bit KVM processor
    Kvm32V1,
    /// (alias configured by machine type)
    Kvm64,
    /// Common KVM processor
    Kvm64V1,
    /// (alias configured by machine type)
    N270,
    /// Intel(R) Atom(TM) CPU N270 @ 1.60GHz
    N270V1,
    /// (alias configured by machine type)
    Pentium,
    /// pentium-v1
    PentiumV1,
    /// (alias configured by machine type)
    Pentium2,
    /// pentium2-v1
    Pentium2V1,
    /// (alias configured by machine type)
    Pentium3,
    /// pentium3-v1
    Pentium3V1,
    /// (alias configured by machine type)
    Phenom,
    /// AMD Phenom(tm) 9550 Quad-Core Processor
    PhenomV1,
    /// (alias configured by machine type)
    Qemu32,
    /// QEMU Virtual CPU version 2.5+
    Qemu32V1,
    /// (alias configured by machine type)
    Qemu64,
    /// QEMU Virtual CPU version 2.5+
    Qemu64V1,
    /// base CPU model type with no features enabled
    Base,
    /// Enables all features supported by the accelerator in the current host
    Max,
    /// Enables all features supported by the accelerator in the current host
    Host,
}
impl ToCommand for CpuTypeX86_64 {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![];

        match self {
            CpuTypeX86_64::X486 => cmd.push("486".to_string()),
            CpuTypeX86_64::X486V1 => cmd.push("486-v1".to_string()),
            CpuTypeX86_64::Broadwell => cmd.push("broadwell".to_string()),
            CpuTypeX86_64::BroadwellIBRS => cmd.push("broadwell-ibrs".to_string()),
            CpuTypeX86_64::BroadwellnoTSX => cmd.push("broadwell-notsx".to_string()),
            CpuTypeX86_64::BroadwellnoTSXIBRS => cmd.push("broadwell-notsx-ibrs".to_string()),
            CpuTypeX86_64::BroadwellV1 => cmd.push("broadwell-v1".to_string()),
            CpuTypeX86_64::BroadwellV2 => cmd.push("broadwell-v2".to_string()),
            CpuTypeX86_64::BroadwellV3 => cmd.push("broadwell-v3".to_string()),
            CpuTypeX86_64::BroadwellV4 => cmd.push("broadwell-v4".to_string()),
            CpuTypeX86_64::CascadelakeServer => cmd.push("cascadelake-server".to_string()),
            CpuTypeX86_64::CascadelakeServernoTSX => {
                cmd.push("cascadelake-server-notsx".to_string())
            }
            CpuTypeX86_64::CascadelakeServerV1 => cmd.push("cascadelake-server-v1".to_string()),
            CpuTypeX86_64::CascadelakeServerV2 => cmd.push("cascadelake-server-v2".to_string()),
            CpuTypeX86_64::CascadelakeServerV3 => cmd.push("cascadelake-server-v3".to_string()),
            CpuTypeX86_64::CascadelakeServerV4 => cmd.push("cascadelake-server-v4".to_string()),
            CpuTypeX86_64::CascadelakeServerV5 => cmd.push("cascadelake-server-v5".to_string()),
            CpuTypeX86_64::ClearwaterForest => cmd.push("clearwaterforest".to_string()),
            CpuTypeX86_64::ClearwaterForestV1 => cmd.push("clearwaterforest-v1".to_string()),
            CpuTypeX86_64::Conroe => cmd.push("conroe".to_string()),
            CpuTypeX86_64::ConroeV1 => cmd.push("conroe-v1".to_string()),
            CpuTypeX86_64::Cooperlake => cmd.push("cooperlake".to_string()),
            CpuTypeX86_64::CooperlakeV1 => cmd.push("cooperlake-v1".to_string()),
            CpuTypeX86_64::CooperlakeV2 => cmd.push("cooperlake-v2".to_string()),
            CpuTypeX86_64::Denverton => cmd.push("denverton".to_string()),
            CpuTypeX86_64::DenvertonV1 => cmd.push("denverton-v1".to_string()),
            CpuTypeX86_64::DenvertonV2 => cmd.push("denverton-v2".to_string()),
            CpuTypeX86_64::DenvertonV3 => cmd.push("denverton-v3".to_string()),
            CpuTypeX86_64::Dhyana => cmd.push("dhyana".to_string()),
            CpuTypeX86_64::DhyanaV1 => cmd.push("dhyana-v1".to_string()),
            CpuTypeX86_64::DhyanaV2 => cmd.push("dhyana-v2".to_string()),
            CpuTypeX86_64::EPYC => cmd.push("epyc".to_string()),
            CpuTypeX86_64::EPYCGenoa => cmd.push("epyc-genoa".to_string()),
            CpuTypeX86_64::EPYCGenoaV1 => cmd.push("epyc-genoa-v1".to_string()),
            CpuTypeX86_64::EPYCGenoaV2 => cmd.push("epyc-genoa-v2".to_string()),
            CpuTypeX86_64::EPYCIBPB => cmd.push("epyc-ibpb".to_string()),
            CpuTypeX86_64::EPYCMilan => cmd.push("epyc-milan".to_string()),
            CpuTypeX86_64::EPYCMilanV1 => cmd.push("epyc-milan-v1".to_string()),
            CpuTypeX86_64::EPYCMilanV2 => cmd.push("epyc-milan-v2".to_string()),
            CpuTypeX86_64::EPYCMilanV3 => cmd.push("epyc-milan-v3".to_string()),
            CpuTypeX86_64::EPYCRome => cmd.push("epyc-rome".to_string()),
            CpuTypeX86_64::EPYCRomeV1 => cmd.push("epyc-rome-v1".to_string()),
            CpuTypeX86_64::EPYCRomeV2 => cmd.push("epyc-rome-v2".to_string()),
            CpuTypeX86_64::EPYCRomeV3 => cmd.push("epyc-rome-v3".to_string()),
            CpuTypeX86_64::EPYCRomeV4 => cmd.push("epyc-rome-v4".to_string()),
            CpuTypeX86_64::EPYCRomeV5 => cmd.push("epyc-rome-v5".to_string()),
            CpuTypeX86_64::EPYCTurin => cmd.push("epyc-turin".to_string()),
            CpuTypeX86_64::EPYCTurinV1 => cmd.push("epyc-turin-v1".to_string()),
            CpuTypeX86_64::EPYCV1 => cmd.push("epyc-v1".to_string()),
            CpuTypeX86_64::EPYCV2 => cmd.push("epyc-v2".to_string()),
            CpuTypeX86_64::EPYCV3 => cmd.push("epyc-v3".to_string()),
            CpuTypeX86_64::EPYCV4 => cmd.push("epyc-v4".to_string()),
            CpuTypeX86_64::EPYCV5 => cmd.push("epyc-v5".to_string()),
            CpuTypeX86_64::GraniteRapids => cmd.push("graniterapids".to_string()),
            CpuTypeX86_64::GraniteRapidsV1 => cmd.push("graniterapids-v1".to_string()),
            CpuTypeX86_64::GraniteRapidsV2 => cmd.push("graniterapids-v2".to_string()),
            CpuTypeX86_64::GraniteRapidsV3 => cmd.push("graniterapids-v3".to_string()),
            CpuTypeX86_64::Haswell => cmd.push("haswell".to_string()),
            CpuTypeX86_64::HaswellIBRS => cmd.push("haswell-ibrs".to_string()),
            CpuTypeX86_64::HaswellnoTSX => cmd.push("haswell-notsx".to_string()),
            CpuTypeX86_64::HaswellnoTSXIBRS => cmd.push("haswell-notsx-ibrs".to_string()),
            CpuTypeX86_64::HaswellV1 => cmd.push("haswell-v1".to_string()),
            CpuTypeX86_64::HaswellV2 => cmd.push("haswell-v2".to_string()),
            CpuTypeX86_64::HaswellV3 => cmd.push("haswell-v3".to_string()),
            CpuTypeX86_64::HaswellV4 => cmd.push("haswell-v4".to_string()),
            CpuTypeX86_64::IcelakeServer => cmd.push("icelake-server".to_string()),
            CpuTypeX86_64::IcelakeServernoTSX => cmd.push("icelake-server-notsx".to_string()),
            CpuTypeX86_64::IcelakeServerV1 => cmd.push("icelake-server-v1".to_string()),
            CpuTypeX86_64::IcelakeServerV2 => cmd.push("icelake-server-v2".to_string()),
            CpuTypeX86_64::IcelakeServerV3 => cmd.push("icelake-server-v3".to_string()),
            CpuTypeX86_64::IcelakeServerV4 => cmd.push("icelake-server-v4".to_string()),
            CpuTypeX86_64::IcelakeServerV5 => cmd.push("icelake-server-v5".to_string()),
            CpuTypeX86_64::IcelakeServerV6 => cmd.push("icelake-server-v6".to_string()),
            CpuTypeX86_64::IcelakeServerV7 => cmd.push("icelake-server-v7".to_string()),
            CpuTypeX86_64::IvyBridge => cmd.push("ivybridge".to_string()),
            CpuTypeX86_64::IvyBridgeIBRS => cmd.push("ivybridge-ibrs".to_string()),
            CpuTypeX86_64::IvyBridgeV1 => cmd.push("ivybridge-v1".to_string()),
            CpuTypeX86_64::IvyBridgeV2 => cmd.push("ivybridge-v2".to_string()),
            CpuTypeX86_64::KnightsMill => cmd.push("knightsmill".to_string()),
            CpuTypeX86_64::KnightsMillV1 => cmd.push("knightsmill-v1".to_string()),
            CpuTypeX86_64::Nehalem => cmd.push("nehalem".to_string()),
            CpuTypeX86_64::NehalemIBRS => cmd.push("nehalem-ibrs".to_string()),
            CpuTypeX86_64::NehalemV1 => cmd.push("nehalem-v1".to_string()),
            CpuTypeX86_64::NehalemV2 => cmd.push("nehalem-v2".to_string()),
            CpuTypeX86_64::OpteronG1 => cmd.push("opteron_g1".to_string()),
            CpuTypeX86_64::OpteronG1V1 => cmd.push("opteron_g1-v1".to_string()),
            CpuTypeX86_64::OpteronG2 => cmd.push("opteron_g2".to_string()),
            CpuTypeX86_64::OpteronG2V1 => cmd.push("opteron_g2-v1".to_string()),
            CpuTypeX86_64::OpteronG3 => cmd.push("opteron_g3".to_string()),
            CpuTypeX86_64::OpteronG3V1 => cmd.push("opteron_g3-v1".to_string()),
            CpuTypeX86_64::OpteronG4 => cmd.push("opteron_g4".to_string()),
            CpuTypeX86_64::OpteronG4V1 => cmd.push("opteron_g4-v1".to_string()),
            CpuTypeX86_64::OpteronG5 => cmd.push("opteron_g5".to_string()),
            CpuTypeX86_64::OpteronG5V1 => cmd.push("opteron_g5-v1".to_string()),
            CpuTypeX86_64::Penryn => cmd.push("penryn".to_string()),
            CpuTypeX86_64::PenrynV1 => cmd.push("penryn-v1".to_string()),
            CpuTypeX86_64::SandyBridge => cmd.push("sandybridge".to_string()),
            CpuTypeX86_64::SandyBridgeIBRS => cmd.push("sandybridge-ibrs".to_string()),
            CpuTypeX86_64::SandyBridgeV1 => cmd.push("sandybridge-v1".to_string()),
            CpuTypeX86_64::SandyBridgeV2 => cmd.push("sandybridge-v2".to_string()),
            CpuTypeX86_64::SapphireRapids => cmd.push("sapphirerapids".to_string()),
            CpuTypeX86_64::SapphireRapidsV1 => cmd.push("sapphirerapids-v1".to_string()),
            CpuTypeX86_64::SapphireRapidsV2 => cmd.push("sapphirerapids-v2".to_string()),
            CpuTypeX86_64::SapphireRapidsV3 => cmd.push("sapphirerapids-v3".to_string()),
            CpuTypeX86_64::SapphireRapidsV4 => cmd.push("sapphirerapids-v4".to_string()),
            CpuTypeX86_64::SierraForest => cmd.push("sierraforest".to_string()),
            CpuTypeX86_64::SierraForestV1 => cmd.push("sierraforest-v1".to_string()),
            CpuTypeX86_64::SierraForestV2 => cmd.push("sierraforest-v2".to_string()),
            CpuTypeX86_64::SierraForestV3 => cmd.push("sierraforest-v3".to_string()),
            CpuTypeX86_64::SkylakeClient => cmd.push("skylake-client".to_string()),
            CpuTypeX86_64::SkylakeClientIBRS => cmd.push("skylake-client-ibrs".to_string()),
            CpuTypeX86_64::SkylakeClientnoTSXIBRS => {
                cmd.push("skylake-client-notsx-ibrs".to_string())
            }
            CpuTypeX86_64::SkylakeClientV1 => cmd.push("skylake-client-v1".to_string()),
            CpuTypeX86_64::SkylakeClientV2 => cmd.push("skylake-client-v2".to_string()),
            CpuTypeX86_64::SkylakeClientV3 => cmd.push("skylake-client-v3".to_string()),
            CpuTypeX86_64::SkylakeClientV4 => cmd.push("skylake-client-v4".to_string()),
            CpuTypeX86_64::SkylakeServer => cmd.push("skylake-server".to_string()),
            CpuTypeX86_64::SkylakeServerIBRS => cmd.push("skylake-server-ibrs".to_string()),
            CpuTypeX86_64::SkylakeServernoTSXIBRS => {
                cmd.push("skylake-server-notsx-ibrs".to_string())
            }
            CpuTypeX86_64::SkylakeServerV1 => cmd.push("skylake-server-v1".to_string()),
            CpuTypeX86_64::SkylakeServerV2 => cmd.push("skylake-server-v2".to_string()),
            CpuTypeX86_64::SkylakeServerV3 => cmd.push("skylake-server-v3".to_string()),
            CpuTypeX86_64::SkylakeServerV4 => cmd.push("skylake-server-v4".to_string()),
            CpuTypeX86_64::SkylakeServerV5 => cmd.push("skylake-server-v5".to_string()),
            CpuTypeX86_64::Snowridge => cmd.push("snowridge".to_string()),
            CpuTypeX86_64::SnowridgeV1 => cmd.push("snowridge-v1".to_string()),
            CpuTypeX86_64::SnowridgeV2 => cmd.push("snowridge-v2".to_string()),
            CpuTypeX86_64::SnowridgeV3 => cmd.push("snowridge-v3".to_string()),
            CpuTypeX86_64::SnowridgeV4 => cmd.push("snowridge-v4".to_string()),
            CpuTypeX86_64::Westmere => cmd.push("westmere".to_string()),
            CpuTypeX86_64::WestmereIBRS => cmd.push("westmere-ibrs".to_string()),
            CpuTypeX86_64::WestmereV1 => cmd.push("westmere-v1".to_string()),
            CpuTypeX86_64::WestmereV2 => cmd.push("westmere-v2".to_string()),
            CpuTypeX86_64::YongFeng => cmd.push("yongfeng".to_string()),
            CpuTypeX86_64::YongFengV1 => cmd.push("yongfeng-v1".to_string()),
            CpuTypeX86_64::YongFengV2 => cmd.push("yongfeng-v2".to_string()),
            CpuTypeX86_64::YongFengV3 => cmd.push("yongfeng-v3".to_string()),
            CpuTypeX86_64::Athlon => cmd.push("athlon".to_string()),
            CpuTypeX86_64::AthlonV1 => cmd.push("athlon-v1".to_string()),
            CpuTypeX86_64::Core2duo => cmd.push("core2duo".to_string()),
            CpuTypeX86_64::Core2duoV1 => cmd.push("core2duo-v1".to_string()),
            CpuTypeX86_64::Coreduo => cmd.push("coreduo".to_string()),
            CpuTypeX86_64::CoreduoV1 => cmd.push("coreduo-v1".to_string()),
            CpuTypeX86_64::Kvm32 => cmd.push("kvm32".to_string()),
            CpuTypeX86_64::Kvm32V1 => cmd.push("kvm32-v1".to_string()),
            CpuTypeX86_64::Kvm64 => cmd.push("kvm64".to_string()),
            CpuTypeX86_64::Kvm64V1 => cmd.push("kvm64-v1".to_string()),
            CpuTypeX86_64::N270 => cmd.push("n270".to_string()),
            CpuTypeX86_64::N270V1 => cmd.push("n270-v1".to_string()),
            CpuTypeX86_64::Pentium => cmd.push("pentium".to_string()),
            CpuTypeX86_64::PentiumV1 => cmd.push("pentium-v1".to_string()),
            CpuTypeX86_64::Pentium2 => cmd.push("pentium2".to_string()),
            CpuTypeX86_64::Pentium2V1 => cmd.push("pentium2-v1".to_string()),
            CpuTypeX86_64::Pentium3 => cmd.push("pentium3".to_string()),
            CpuTypeX86_64::Pentium3V1 => cmd.push("pentium3-v1".to_string()),
            CpuTypeX86_64::Phenom => cmd.push("phenom".to_string()),
            CpuTypeX86_64::PhenomV1 => cmd.push("phenom-v1".to_string()),
            CpuTypeX86_64::Qemu32 => cmd.push("qemu32".to_string()),
            CpuTypeX86_64::Qemu32V1 => cmd.push("qemu32-v1".to_string()),
            CpuTypeX86_64::Qemu64 => cmd.push("qemu64".to_string()),
            CpuTypeX86_64::Qemu64V1 => cmd.push("qemu64-v1".to_string()),
            CpuTypeX86_64::Base => cmd.push("base".to_string()),
            CpuTypeX86_64::Max => cmd.push("max".to_string()),
            CpuTypeX86_64::Host => cmd.push("host".to_string()),
        }
        cmd
    }
}

impl ToArg for CpuTypeX86_64 {
    fn to_arg(&self) -> &str {
        match self {
            CpuTypeX86_64::X486 => "486",
            CpuTypeX86_64::X486V1 => "486-v1",
            CpuTypeX86_64::Broadwell => "Broadwell",
            CpuTypeX86_64::BroadwellIBRS => "Broadwell-IBRS",
            CpuTypeX86_64::BroadwellnoTSX => "Broadwell-noTSX",
            CpuTypeX86_64::BroadwellnoTSXIBRS => "Broadwell-noTSX-IBRS",
            CpuTypeX86_64::BroadwellV1 => "Broadwell-v1",
            CpuTypeX86_64::BroadwellV2 => "Broadwell-v2",
            CpuTypeX86_64::BroadwellV3 => "Broadwell-v3",
            CpuTypeX86_64::BroadwellV4 => "Broadwell-v4",
            CpuTypeX86_64::CascadelakeServer => "Cascadelake-Server",
            CpuTypeX86_64::CascadelakeServernoTSX => "Cascadelake-Server-noTSX",
            CpuTypeX86_64::CascadelakeServerV1 => "Cascadelake-Server-v1",
            CpuTypeX86_64::CascadelakeServerV2 => "Cascadelake-Server-v2",
            CpuTypeX86_64::CascadelakeServerV3 => "Cascadelake-Server-v3",
            CpuTypeX86_64::CascadelakeServerV4 => "Cascadelake-Server-v4",
            CpuTypeX86_64::CascadelakeServerV5 => "Cascadelake-Server-v5",
            CpuTypeX86_64::ClearwaterForest => "ClearwaterForest",
            CpuTypeX86_64::ClearwaterForestV1 => "ClearwaterForest-v1",
            CpuTypeX86_64::Conroe => "Conroe",
            CpuTypeX86_64::ConroeV1 => "Conroe-v1",
            CpuTypeX86_64::Cooperlake => "Cooperlake",
            CpuTypeX86_64::CooperlakeV1 => "Cooperlake-v1",
            CpuTypeX86_64::CooperlakeV2 => "Cooperlake-v2",
            CpuTypeX86_64::Denverton => "Denverton",
            CpuTypeX86_64::DenvertonV1 => "Denverton-v1",
            CpuTypeX86_64::DenvertonV2 => "Denverton-v2",
            CpuTypeX86_64::DenvertonV3 => "Denverton-v3",
            CpuTypeX86_64::Dhyana => "Dhyana",
            CpuTypeX86_64::DhyanaV1 => "Dhyana-v1",
            CpuTypeX86_64::DhyanaV2 => "Dhyana-v2",
            CpuTypeX86_64::EPYC => "EPYC",
            CpuTypeX86_64::EPYCGenoa => "EPYC-Genoa",
            CpuTypeX86_64::EPYCGenoaV1 => "EPYC-Genoa-v1",
            CpuTypeX86_64::EPYCGenoaV2 => "EPYC-Genoa-v2",
            CpuTypeX86_64::EPYCIBPB => "EPYC-IBPB",
            CpuTypeX86_64::EPYCMilan => "EPYC-Milan",
            CpuTypeX86_64::EPYCMilanV1 => "EPYC-Milan-v1",
            CpuTypeX86_64::EPYCMilanV2 => "EPYC-Milan-v2",
            CpuTypeX86_64::EPYCMilanV3 => "EPYC-Milan-v3",
            CpuTypeX86_64::EPYCRome => "EPYC-Rome",
            CpuTypeX86_64::EPYCRomeV1 => "EPYC-Rome-v1",
            CpuTypeX86_64::EPYCRomeV2 => "EPYC-Rome-v2",
            CpuTypeX86_64::EPYCRomeV3 => "EPYC-Rome-v3",
            CpuTypeX86_64::EPYCRomeV4 => "EPYC-Rome-v4",
            CpuTypeX86_64::EPYCRomeV5 => "EPYC-Rome-v5",
            CpuTypeX86_64::EPYCTurin => "EPYC-Turin",
            CpuTypeX86_64::EPYCTurinV1 => "EPYC-Turin-v1",
            CpuTypeX86_64::EPYCV1 => "EPYC-v1",
            CpuTypeX86_64::EPYCV2 => "EPYC-v2",
            CpuTypeX86_64::EPYCV3 => "EPYC-v3",
            CpuTypeX86_64::EPYCV4 => "EPYC-v4",
            CpuTypeX86_64::EPYCV5 => "EPYC-v5",
            CpuTypeX86_64::GraniteRapids => "GraniteRapids",
            CpuTypeX86_64::GraniteRapidsV1 => "GraniteRapids-v1",
            CpuTypeX86_64::GraniteRapidsV2 => "GraniteRapids-v2",
            CpuTypeX86_64::GraniteRapidsV3 => "GraniteRapids-v3",
            CpuTypeX86_64::Haswell => "Haswell",
            CpuTypeX86_64::HaswellIBRS => "Haswell-IBRS",
            CpuTypeX86_64::HaswellnoTSX => "Haswell-noTSX",
            CpuTypeX86_64::HaswellnoTSXIBRS => "Haswell-noTSX-IBRS",
            CpuTypeX86_64::HaswellV1 => "Haswell-v1",
            CpuTypeX86_64::HaswellV2 => "Haswell-v2",
            CpuTypeX86_64::HaswellV3 => "Haswell-v3",
            CpuTypeX86_64::HaswellV4 => "Haswell-v4",
            CpuTypeX86_64::IcelakeServer => "Icelake-Server",
            CpuTypeX86_64::IcelakeServernoTSX => "Icelake-Server-noTSX",
            CpuTypeX86_64::IcelakeServerV1 => "Icelake-Server-v1",
            CpuTypeX86_64::IcelakeServerV2 => "Icelake-Server-v2",
            CpuTypeX86_64::IcelakeServerV3 => "Icelake-Server-v3",
            CpuTypeX86_64::IcelakeServerV4 => "Icelake-Server-v4",
            CpuTypeX86_64::IcelakeServerV5 => "Icelake-Server-v5",
            CpuTypeX86_64::IcelakeServerV6 => "Icelake-Server-v6",
            CpuTypeX86_64::IcelakeServerV7 => "Icelake-Server-v7",
            CpuTypeX86_64::IvyBridge => "IvyBridge",
            CpuTypeX86_64::IvyBridgeIBRS => "IvyBridge-IBRS",
            CpuTypeX86_64::IvyBridgeV1 => "IvyBridge-v1",
            CpuTypeX86_64::IvyBridgeV2 => "IvyBridge-v2",
            CpuTypeX86_64::KnightsMill => "KnightsMill",
            CpuTypeX86_64::KnightsMillV1 => "KnightsMill-v1",
            CpuTypeX86_64::Nehalem => "Nehalem",
            CpuTypeX86_64::NehalemIBRS => "Nehalem-IBRS",
            CpuTypeX86_64::NehalemV1 => "Nehalem-v1",
            CpuTypeX86_64::NehalemV2 => "Nehalem-v2",
            CpuTypeX86_64::OpteronG1 => "Opteron_G1",
            CpuTypeX86_64::OpteronG1V1 => "Opteron_G1-v1",
            CpuTypeX86_64::OpteronG2 => "Opteron_G2",
            CpuTypeX86_64::OpteronG2V1 => "Opteron_G2-v1",
            CpuTypeX86_64::OpteronG3 => "Opteron_G3",
            CpuTypeX86_64::OpteronG3V1 => "Opteron_G3-v1",
            CpuTypeX86_64::OpteronG4 => "Opteron_G4",
            CpuTypeX86_64::OpteronG4V1 => "Opteron_G4-v1",
            CpuTypeX86_64::OpteronG5 => "Opteron_G5",
            CpuTypeX86_64::OpteronG5V1 => "Opteron_G5-v1",
            CpuTypeX86_64::Penryn => "Penryn",
            CpuTypeX86_64::PenrynV1 => "Penryn-v1",
            CpuTypeX86_64::SandyBridge => "SandyBridge",
            CpuTypeX86_64::SandyBridgeIBRS => "SandyBridge-IBRS",
            CpuTypeX86_64::SandyBridgeV1 => "SandyBridge-v1",
            CpuTypeX86_64::SandyBridgeV2 => "SandyBridge-v2",
            CpuTypeX86_64::SapphireRapids => "SapphireRapids",
            CpuTypeX86_64::SapphireRapidsV1 => "SapphireRapids-v1",
            CpuTypeX86_64::SapphireRapidsV2 => "SapphireRapids-v2",
            CpuTypeX86_64::SapphireRapidsV3 => "SapphireRapids-v3",
            CpuTypeX86_64::SapphireRapidsV4 => "SapphireRapids-v4",
            CpuTypeX86_64::SierraForest => "SierraForest",
            CpuTypeX86_64::SierraForestV1 => "SierraForest-v1",
            CpuTypeX86_64::SierraForestV2 => "SierraForest-v2",
            CpuTypeX86_64::SierraForestV3 => "SierraForest-v3",
            CpuTypeX86_64::SkylakeClient => "Skylake-Client",
            CpuTypeX86_64::SkylakeClientIBRS => "Skylake-Client-IBRS",
            CpuTypeX86_64::SkylakeClientnoTSXIBRS => "Skylake-Client-noTSX-IBRS",
            CpuTypeX86_64::SkylakeClientV1 => "Skylake-Client-v1",
            CpuTypeX86_64::SkylakeClientV2 => "Skylake-Client-v2",
            CpuTypeX86_64::SkylakeClientV3 => "Skylake-Client-v3",
            CpuTypeX86_64::SkylakeClientV4 => "Skylake-Client-v4",
            CpuTypeX86_64::SkylakeServer => "Skylake-Server",
            CpuTypeX86_64::SkylakeServerIBRS => "Skylake-Server-IBRS",
            CpuTypeX86_64::SkylakeServernoTSXIBRS => "Skylake-Server-noTSX-IBRS",
            CpuTypeX86_64::SkylakeServerV1 => "Skylake-Server-v1",
            CpuTypeX86_64::SkylakeServerV2 => "Skylake-Server-v2",
            CpuTypeX86_64::SkylakeServerV3 => "Skylake-Server-v3",
            CpuTypeX86_64::SkylakeServerV4 => "Skylake-Server-v4",
            CpuTypeX86_64::SkylakeServerV5 => "Skylake-Server-v5",
            CpuTypeX86_64::Snowridge => "Snowridge",
            CpuTypeX86_64::SnowridgeV1 => "Snowridge-v1",
            CpuTypeX86_64::SnowridgeV2 => "Snowridge-v2",
            CpuTypeX86_64::SnowridgeV3 => "Snowridge-v3",
            CpuTypeX86_64::SnowridgeV4 => "Snowridge-v4",
            CpuTypeX86_64::Westmere => "Westmere",
            CpuTypeX86_64::WestmereIBRS => "Westmere-IBRS",
            CpuTypeX86_64::WestmereV1 => "Westmere-v1",
            CpuTypeX86_64::WestmereV2 => "Westmere-v2",
            CpuTypeX86_64::YongFeng => "YongFeng",
            CpuTypeX86_64::YongFengV1 => "YongFeng-v1",
            CpuTypeX86_64::YongFengV2 => "YongFeng-v2",
            CpuTypeX86_64::YongFengV3 => "YongFeng-v3",
            CpuTypeX86_64::Athlon => "athlon",
            CpuTypeX86_64::AthlonV1 => "athlon-v1",
            CpuTypeX86_64::Core2duo => "core2duo",
            CpuTypeX86_64::Core2duoV1 => "core2duo-v1",
            CpuTypeX86_64::Coreduo => "coreduo",
            CpuTypeX86_64::CoreduoV1 => "coreduo-v1",
            CpuTypeX86_64::Kvm32 => "kvm32",
            CpuTypeX86_64::Kvm32V1 => "kvm32-v1",
            CpuTypeX86_64::Kvm64 => "kvm64",
            CpuTypeX86_64::Kvm64V1 => "kvm64-v1",
            CpuTypeX86_64::N270 => "n270",
            CpuTypeX86_64::N270V1 => "n270-v1",
            CpuTypeX86_64::Pentium => "pentium",
            CpuTypeX86_64::PentiumV1 => "pentium-v1",
            CpuTypeX86_64::Pentium2 => "pentium2",
            CpuTypeX86_64::Pentium2V1 => "pentium2-v1",
            CpuTypeX86_64::Pentium3 => "pentium3",
            CpuTypeX86_64::Pentium3V1 => "pentium3-v1",
            CpuTypeX86_64::Phenom => "phenom",
            CpuTypeX86_64::PhenomV1 => "phenom-v1",
            CpuTypeX86_64::Qemu32 => "qemu32",
            CpuTypeX86_64::Qemu32V1 => "qemu32-v1",
            CpuTypeX86_64::Qemu64 => "qemu64",
            CpuTypeX86_64::Qemu64V1 => "qemu64-v1",
            CpuTypeX86_64::Base => "base",
            CpuTypeX86_64::Max => "max",
            CpuTypeX86_64::Host => "host",
        }
    }
}

impl FromStr for CpuTypeX86_64 {
    type Err = CpuNotFound;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "486" => Ok(CpuTypeX86_64::X486),
            "486-v1" => Ok(CpuTypeX86_64::X486V1),
            "Broadwell" => Ok(CpuTypeX86_64::Broadwell),
            "Broadwell-IBRS" => Ok(CpuTypeX86_64::BroadwellIBRS),
            "Broadwell-noTSX" => Ok(CpuTypeX86_64::BroadwellnoTSX),
            "Broadwell-noTSX-IBRS" => Ok(CpuTypeX86_64::BroadwellnoTSXIBRS),
            "Broadwell-v1" => Ok(CpuTypeX86_64::BroadwellV1),
            "Broadwell-v2" => Ok(CpuTypeX86_64::BroadwellV2),
            "Broadwell-v3" => Ok(CpuTypeX86_64::BroadwellV3),
            "Broadwell-v4" => Ok(CpuTypeX86_64::BroadwellV4),
            "Cascadelake-Server" => Ok(CpuTypeX86_64::CascadelakeServer),
            "Cascadelake-Server-noTSX" => Ok(CpuTypeX86_64::CascadelakeServernoTSX),
            "Cascadelake-Server-v1" => Ok(CpuTypeX86_64::CascadelakeServerV1),
            "Cascadelake-Server-v2" => Ok(CpuTypeX86_64::CascadelakeServerV2),
            "Cascadelake-Server-v3" => Ok(CpuTypeX86_64::CascadelakeServerV3),
            "Cascadelake-Server-v4" => Ok(CpuTypeX86_64::CascadelakeServerV4),
            "Cascadelake-Server-v5" => Ok(CpuTypeX86_64::CascadelakeServerV5),
            "ClearwaterForest" => Ok(CpuTypeX86_64::ClearwaterForest),
            "ClearwaterForest-v1" => Ok(CpuTypeX86_64::ClearwaterForestV1),
            "Conroe" => Ok(CpuTypeX86_64::Conroe),
            "Conroe-v1" => Ok(CpuTypeX86_64::ConroeV1),
            "Cooperlake" => Ok(CpuTypeX86_64::Cooperlake),
            "Cooperlake-v1" => Ok(CpuTypeX86_64::CooperlakeV1),
            "Cooperlake-v2" => Ok(CpuTypeX86_64::CooperlakeV2),
            "Denverton" => Ok(CpuTypeX86_64::Denverton),
            "Denverton-v1" => Ok(CpuTypeX86_64::DenvertonV1),
            "Denverton-v2" => Ok(CpuTypeX86_64::DenvertonV2),
            "Denverton-v3" => Ok(CpuTypeX86_64::DenvertonV3),
            "Dhyana" => Ok(CpuTypeX86_64::Dhyana),
            "Dhyana-v1" => Ok(CpuTypeX86_64::DhyanaV1),
            "Dhyana-v2" => Ok(CpuTypeX86_64::DhyanaV2),
            "EPYC" => Ok(CpuTypeX86_64::EPYC),
            "EPYC-Genoa" => Ok(CpuTypeX86_64::EPYCGenoa),
            "EPYC-Genoa-v1" => Ok(CpuTypeX86_64::EPYCGenoaV1),
            "EPYC-Genoa-v2" => Ok(CpuTypeX86_64::EPYCGenoaV2),
            "EPYC-IBPB" => Ok(CpuTypeX86_64::EPYCIBPB),
            "EPYC-Milan" => Ok(CpuTypeX86_64::EPYCMilan),
            "EPYC-Milan-v1" => Ok(CpuTypeX86_64::EPYCMilanV1),
            "EPYC-Milan-v2" => Ok(CpuTypeX86_64::EPYCMilanV2),
            "EPYC-Milan-v3" => Ok(CpuTypeX86_64::EPYCMilanV3),
            "EPYC-Rome" => Ok(CpuTypeX86_64::EPYCRome),
            "EPYC-Rome-v1" => Ok(CpuTypeX86_64::EPYCRomeV1),
            "EPYC-Rome-v2" => Ok(CpuTypeX86_64::EPYCRomeV2),
            "EPYC-Rome-v3" => Ok(CpuTypeX86_64::EPYCRomeV3),
            "EPYC-Rome-v4" => Ok(CpuTypeX86_64::EPYCRomeV4),
            "EPYC-Rome-v5" => Ok(CpuTypeX86_64::EPYCRomeV5),
            "EPYC-Turin" => Ok(CpuTypeX86_64::EPYCTurin),
            "EPYC-Turin-v1" => Ok(CpuTypeX86_64::EPYCTurinV1),
            "EPYC-v1" => Ok(CpuTypeX86_64::EPYCV1),
            "EPYC-v2" => Ok(CpuTypeX86_64::EPYCV2),
            "EPYC-v3" => Ok(CpuTypeX86_64::EPYCV3),
            "EPYC-v4" => Ok(CpuTypeX86_64::EPYCV4),
            "EPYC-v5" => Ok(CpuTypeX86_64::EPYCV5),
            "GraniteRapids" => Ok(CpuTypeX86_64::GraniteRapids),
            "GraniteRapids-v1" => Ok(CpuTypeX86_64::GraniteRapidsV1),
            "GraniteRapids-v2" => Ok(CpuTypeX86_64::GraniteRapidsV2),
            "GraniteRapids-v3" => Ok(CpuTypeX86_64::GraniteRapidsV3),
            "Haswell" => Ok(CpuTypeX86_64::Haswell),
            "Haswell-IBRS" => Ok(CpuTypeX86_64::HaswellIBRS),
            "Haswell-noTSX" => Ok(CpuTypeX86_64::HaswellnoTSX),
            "Haswell-noTSX-IBRS" => Ok(CpuTypeX86_64::HaswellnoTSXIBRS),
            "Haswell-v1" => Ok(CpuTypeX86_64::HaswellV1),
            "Haswell-v2" => Ok(CpuTypeX86_64::HaswellV2),
            "Haswell-v3" => Ok(CpuTypeX86_64::HaswellV3),
            "Haswell-v4" => Ok(CpuTypeX86_64::HaswellV4),
            "Icelake-Server" => Ok(CpuTypeX86_64::IcelakeServer),
            "Icelake-Server-noTSX" => Ok(CpuTypeX86_64::IcelakeServernoTSX),
            "Icelake-Server-v1" => Ok(CpuTypeX86_64::IcelakeServerV1),
            "Icelake-Server-v2" => Ok(CpuTypeX86_64::IcelakeServerV2),
            "Icelake-Server-v3" => Ok(CpuTypeX86_64::IcelakeServerV3),
            "Icelake-Server-v4" => Ok(CpuTypeX86_64::IcelakeServerV4),
            "Icelake-Server-v5" => Ok(CpuTypeX86_64::IcelakeServerV5),
            "Icelake-Server-v6" => Ok(CpuTypeX86_64::IcelakeServerV6),
            "Icelake-Server-v7" => Ok(CpuTypeX86_64::IcelakeServerV7),
            "IvyBridge" => Ok(CpuTypeX86_64::IvyBridge),
            "IvyBridge-IBRS" => Ok(CpuTypeX86_64::IvyBridgeIBRS),
            "IvyBridge-v1" => Ok(CpuTypeX86_64::IvyBridgeV1),
            "IvyBridge-v2" => Ok(CpuTypeX86_64::IvyBridgeV2),
            "KnightsMill" => Ok(CpuTypeX86_64::KnightsMill),
            "KnightsMill-v1" => Ok(CpuTypeX86_64::KnightsMillV1),
            "Nehalem" => Ok(CpuTypeX86_64::Nehalem),
            "Nehalem-IBRS" => Ok(CpuTypeX86_64::NehalemIBRS),
            "Nehalem-v1" => Ok(CpuTypeX86_64::NehalemV1),
            "Nehalem-v2" => Ok(CpuTypeX86_64::NehalemV2),
            "Opteron_G1" => Ok(CpuTypeX86_64::OpteronG1),
            "Opteron_G1-v1" => Ok(CpuTypeX86_64::OpteronG1V1),
            "Opteron_G2" => Ok(CpuTypeX86_64::OpteronG2),
            "Opteron_G2-v1" => Ok(CpuTypeX86_64::OpteronG2V1),
            "Opteron_G3" => Ok(CpuTypeX86_64::OpteronG3),
            "Opteron_G3-v1" => Ok(CpuTypeX86_64::OpteronG3V1),
            "Opteron_G4" => Ok(CpuTypeX86_64::OpteronG4),
            "Opteron_G4-v1" => Ok(CpuTypeX86_64::OpteronG4V1),
            "Opteron_G5" => Ok(CpuTypeX86_64::OpteronG5),
            "Opteron_G5-v1" => Ok(CpuTypeX86_64::OpteronG5V1),
            "Penryn" => Ok(CpuTypeX86_64::Penryn),
            "Penryn-v1" => Ok(CpuTypeX86_64::PenrynV1),
            "SandyBridge" => Ok(CpuTypeX86_64::SandyBridge),
            "SandyBridge-IBRS" => Ok(CpuTypeX86_64::SandyBridgeIBRS),
            "SandyBridge-v1" => Ok(CpuTypeX86_64::SandyBridgeV1),
            "SandyBridge-v2" => Ok(CpuTypeX86_64::SandyBridgeV2),
            "SapphireRapids" => Ok(CpuTypeX86_64::SapphireRapids),
            "SapphireRapids-v1" => Ok(CpuTypeX86_64::SapphireRapidsV1),
            "SapphireRapids-v2" => Ok(CpuTypeX86_64::SapphireRapidsV2),
            "SapphireRapids-v3" => Ok(CpuTypeX86_64::SapphireRapidsV3),
            "SapphireRapids-v4" => Ok(CpuTypeX86_64::SapphireRapidsV4),
            "SierraForest" => Ok(CpuTypeX86_64::SierraForest),
            "SierraForest-v1" => Ok(CpuTypeX86_64::SierraForestV1),
            "SierraForest-v2" => Ok(CpuTypeX86_64::SierraForestV2),
            "SierraForest-v3" => Ok(CpuTypeX86_64::SierraForestV3),
            "Skylake-Client" => Ok(CpuTypeX86_64::SkylakeClient),
            "Skylake-Client-IBRS" => Ok(CpuTypeX86_64::SkylakeClientIBRS),
            "Skylake-Client-noTSX-IBRS" => Ok(CpuTypeX86_64::SkylakeClientnoTSXIBRS),
            "Skylake-Client-v1" => Ok(CpuTypeX86_64::SkylakeClientV1),
            "Skylake-Client-v2" => Ok(CpuTypeX86_64::SkylakeClientV2),
            "Skylake-Client-v3" => Ok(CpuTypeX86_64::SkylakeClientV3),
            "Skylake-Client-v4" => Ok(CpuTypeX86_64::SkylakeClientV4),
            "Skylake-Server" => Ok(CpuTypeX86_64::SkylakeServer),
            "Skylake-Server-IBRS" => Ok(CpuTypeX86_64::SkylakeServerIBRS),
            "Skylake-Server-noTSX-IBRS" => Ok(CpuTypeX86_64::SkylakeServernoTSXIBRS),
            "Skylake-Server-v1" => Ok(CpuTypeX86_64::SkylakeServerV1),
            "Skylake-Server-v2" => Ok(CpuTypeX86_64::SkylakeServerV2),
            "Skylake-Server-v3" => Ok(CpuTypeX86_64::SkylakeServerV3),
            "Skylake-Server-v4" => Ok(CpuTypeX86_64::SkylakeServerV4),
            "Skylake-Server-v5" => Ok(CpuTypeX86_64::SkylakeServerV5),
            "Snowridge" => Ok(CpuTypeX86_64::Snowridge),
            "Snowridge-v1" => Ok(CpuTypeX86_64::SnowridgeV1),
            "Snowridge-v2" => Ok(CpuTypeX86_64::SnowridgeV2),
            "Snowridge-v3" => Ok(CpuTypeX86_64::SnowridgeV3),
            "Snowridge-v4" => Ok(CpuTypeX86_64::SnowridgeV4),
            "Westmere" => Ok(CpuTypeX86_64::Westmere),
            "Westmere-IBRS" => Ok(CpuTypeX86_64::WestmereIBRS),
            "Westmere-v1" => Ok(CpuTypeX86_64::WestmereV1),
            "Westmere-v2" => Ok(CpuTypeX86_64::WestmereV2),
            "YongFeng" => Ok(CpuTypeX86_64::YongFeng),
            "YongFeng-v1" => Ok(CpuTypeX86_64::YongFengV1),
            "YongFeng-v2" => Ok(CpuTypeX86_64::YongFengV2),
            "YongFeng-v3" => Ok(CpuTypeX86_64::YongFengV3),
            "athlon" => Ok(CpuTypeX86_64::Athlon),
            "athlon-v1" => Ok(CpuTypeX86_64::AthlonV1),
            "core2duo" => Ok(CpuTypeX86_64::Core2duo),
            "core2duo-v1" => Ok(CpuTypeX86_64::Core2duoV1),
            "coreduo" => Ok(CpuTypeX86_64::Coreduo),
            "coreduo-v1" => Ok(CpuTypeX86_64::CoreduoV1),
            "kvm32" => Ok(CpuTypeX86_64::Kvm32),
            "kvm32-v1" => Ok(CpuTypeX86_64::Kvm32V1),
            "kvm64" => Ok(CpuTypeX86_64::Kvm64),
            "kvm64-v1" => Ok(CpuTypeX86_64::Kvm64V1),
            "n270" => Ok(CpuTypeX86_64::N270),
            "n270-v1" => Ok(CpuTypeX86_64::N270V1),
            "pentium" => Ok(CpuTypeX86_64::Pentium),
            "pentium-v1" => Ok(CpuTypeX86_64::PentiumV1),
            "pentium2" => Ok(CpuTypeX86_64::Pentium2),
            "pentium2-v1" => Ok(CpuTypeX86_64::Pentium2V1),
            "pentium3" => Ok(CpuTypeX86_64::Pentium3),
            "pentium3-v1" => Ok(CpuTypeX86_64::Pentium3V1),
            "phenom" => Ok(CpuTypeX86_64::Phenom),
            "phenom-v1" => Ok(CpuTypeX86_64::PhenomV1),
            "qemu32" => Ok(CpuTypeX86_64::Qemu32),
            "qemu32-v1" => Ok(CpuTypeX86_64::Qemu32V1),
            "qemu64" => Ok(CpuTypeX86_64::Qemu64),
            "qemu64-v1" => Ok(CpuTypeX86_64::Qemu64V1),
            "base" => Ok(CpuTypeX86_64::Base),
            "max" => Ok(CpuTypeX86_64::Max),
            "host" => Ok(CpuTypeX86_64::Host),
            _ => Err(CpuNotFound),
        }
    }
}
