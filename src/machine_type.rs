use crate::to_command::{ToArg, ToCommand};

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum MachineX86_64 {
    /// microvm (i386)
    Microvm,
    /// Standard PC (i440FX + PIIX, 1996)
    Pci440fx9_2,
    /// Standard PC (i440FX + PIIX, 1996)
    Pci440fx9_1,
    /// Standard PC (i440FX + PIIX, 1996)
    Pci440fx9_0,
    /// Standard PC (i440FX + PIIX, 1996)
    Pci440fx8_2,
    /// Standard PC (i440FX + PIIX, 1996)
    Pci440fx8_1,
    /// Standard PC (i440FX + PIIX, 1996)
    Pci440fx8_0,
    /// Standard PC (i440FX + PIIX, 1996)
    Pci440fx7_2,
    /// Standard PC (i440FX + PIIX, 1996)
    Pci440fx7_1,
    /// Standard PC (i440FX + PIIX, 1996) (deprecated)
    Pci440fx7_0,
    /// Standard PC (i440FX + PIIX, 1996) (deprecated)
    Pci440fx6_2,
    /// Standard PC (i440FX + PIIX, 1996) (deprecated)
    Pci440fx6_1,
    /// Standard PC (i440FX + PIIX, 1996) (deprecated)
    Pci440fx6_0,
    /// Standard PC (i440FX + PIIX, 1996) (deprecated)
    Pci440fx5_2,
    /// Standard PC (i440FX + PIIX, 1996) (deprecated)
    Pci440fx5_1,
    /// Standard PC (i440FX + PIIX, 1996) (deprecated)
    Pci440fx5_0,
    /// Standard PC (i440FX + PIIX, 1996) (deprecated)
    Pci440fx4_2,
    /// Standard PC (i440FX + PIIX, 1996) (deprecated)
    Pci440fx4_1,
    /// Standard PC (i440FX + PIIX, 1996) (deprecated)
    Pci440fx4_0,
    /// Standard PC (i440FX + PIIX, 1996) (deprecated)
    Pci440fx3_1,
    /// Standard PC (i440FX + PIIX, 1996) (deprecated)
    Pci440fx3_0,
    /// Standard PC (i440FX + PIIX, 1996) (deprecated)
    Pci440fx2_9,
    /// Standard PC (i440FX + PIIX, 1996) (deprecated)
    Pci440fx2_8,
    /// Standard PC (i440FX + PIIX, 1996) (deprecated)
    Pci440fx2_7,
    /// Standard PC (i440FX + PIIX, 1996) (deprecated)
    Pci440fx2_6,
    /// Standard PC (i440FX + PIIX, 1996) (deprecated)
    Pci440fx2_5,
    /// Standard PC (i440FX + PIIX, 1996) (deprecated)
    Pci440fx2_4,
    /// Standard PC (i440FX + PIIX, 1996) (deprecated)
    Pci440fx2_12,
    /// Standard PC (i440FX + PIIX, 1996) (deprecated)
    Pci440fx2_11,
    /// Standard PC (i440FX + PIIX, 1996) (deprecated)
    Pci440fx2_10,
    /// Standard PC (i440FX + PIIX, 1996) (alias of pc-i440fx-10.0)
    Pc,
    /// Standard PC (i440FX + PIIX, 1996) (default)
    Pci440fx10_0,
    /// Standard PC (Q35 + ICH9, 2009)
    Pcq359_2,
    /// Standard PC (Q35 + ICH9, 2009)
    Pcq359_1,
    /// Standard PC (Q35 + ICH9, 2009)
    Pcq359_0,
    /// Standard PC (Q35 + ICH9, 2009)
    Pcq358_2,
    /// Standard PC (Q35 + ICH9, 2009)
    Pcq358_1,
    /// Standard PC (Q35 + ICH9, 2009)
    Pcq358_0,
    /// Standard PC (Q35 + ICH9, 2009)
    Pcq357_2,
    /// Standard PC (Q35 + ICH9, 2009)
    Pcq357_1,
    /// Standard PC (Q35 + ICH9, 2009) (deprecated)
    Pcq357_0,
    /// Standard PC (Q35 + ICH9, 2009) (deprecated)
    Pcq356_2,
    /// Standard PC (Q35 + ICH9, 2009) (deprecated)
    Pcq356_1,
    /// Standard PC (Q35 + ICH9, 2009) (deprecated)
    Pcq356_0,
    /// Standard PC (Q35 + ICH9, 2009) (deprecated)
    Pcq355_2,
    /// Standard PC (Q35 + ICH9, 2009) (deprecated)
    Pcq355_1,
    /// Standard PC (Q35 + ICH9, 2009) (deprecated)
    Pcq355_0,
    /// Standard PC (Q35 + ICH9, 2009) (deprecated)
    Pcq354_2,
    /// Standard PC (Q35 + ICH9, 2009) (deprecated)
    Pcq354_1,
    /// Standard PC (Q35 + ICH9, 2009) (deprecated)
    Pcq354_0_1,
    /// Standard PC (Q35 + ICH9, 2009) (deprecated)
    Pcq354_0,
    /// Standard PC (Q35 + ICH9, 2009) (deprecated)
    Pcq353_1,
    /// Standard PC (Q35 + ICH9, 2009) (deprecated)
    Pcq353_0,
    /// Standard PC (Q35 + ICH9, 2009) (deprecated)
    Pcq352_9,
    /// Standard PC (Q35 + ICH9, 2009) (deprecated)
    Pcq352_8,
    /// Standard PC (Q35 + ICH9, 2009) (deprecated)
    Pcq352_7,
    /// Standard PC (Q35 + ICH9, 2009) (deprecated)
    Pcq352_6,
    /// Standard PC (Q35 + ICH9, 2009) (deprecated)
    Pcq352_5,
    /// Standard PC (Q35 + ICH9, 2009) (deprecated)
    Pcq352_4,
    /// Standard PC (Q35 + ICH9, 2009) (deprecated)
    Pcq352_12,
    /// Standard PC (Q35 + ICH9, 2009) (deprecated)
    Pcq352_11,
    /// Standard PC (Q35 + ICH9, 2009) (deprecated)
    Pcq352_10,
    /// Standard PC (Q35 + ICH9, 2009) (alias of pc-q35-10.0)
    Q35,
    /// Standard PC (Q35 + ICH9, 2009)
    Pcq3510_0,
    /// ISA-only PC
    Isapc,
    /// empty machine
    None,
}
impl ToCommand for MachineX86_64 {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![];

        match self {
            MachineX86_64::Microvm => {
                cmd.push("microvm".to_string());
            }
            MachineX86_64::Pci440fx9_2 => {
                cmd.push("pc-i440fx-9.2".to_string());
            }
            MachineX86_64::Pci440fx9_1 => {
                cmd.push("pc-i440fx-9.1".to_string());
            }
            MachineX86_64::Pci440fx9_0 => {
                cmd.push("pc-i440fx-9.0".to_string());
            }
            MachineX86_64::Pci440fx8_2 => {
                cmd.push("pc-i440fx-8.2".to_string());
            }
            MachineX86_64::Pci440fx8_1 => {
                cmd.push("pc-i440fx-8.1".to_string());
            }
            MachineX86_64::Pci440fx8_0 => {
                cmd.push("pc-i440fx-8.0".to_string());
            }
            MachineX86_64::Pci440fx7_2 => {
                cmd.push("pc-i440fx-7.2".to_string());
            }
            MachineX86_64::Pci440fx7_1 => {
                cmd.push("pc-i440fx-7.1".to_string());
            }
            MachineX86_64::Pci440fx7_0 => {
                cmd.push("pc-i440fx-7.0".to_string());
            }
            MachineX86_64::Pci440fx6_2 => {
                cmd.push("pc-i440fx-6.2".to_string());
            }
            MachineX86_64::Pci440fx6_1 => {
                cmd.push("pc-i440fx-6.1".to_string());
            }
            MachineX86_64::Pci440fx6_0 => {
                cmd.push("pc-i440fx-6.0".to_string());
            }
            MachineX86_64::Pci440fx5_2 => {
                cmd.push("pc-i440fx-5.2".to_string());
            }
            MachineX86_64::Pci440fx5_1 => {
                cmd.push("pc-i440fx-5.1".to_string());
            }
            MachineX86_64::Pci440fx5_0 => {
                cmd.push("pc-i440fx-5.0".to_string());
            }
            MachineX86_64::Pci440fx4_2 => {
                cmd.push("pc-i440fx-4.2".to_string());
            }
            MachineX86_64::Pci440fx4_1 => {
                cmd.push("pc-i440fx-4.1".to_string());
            }
            MachineX86_64::Pci440fx4_0 => {
                cmd.push("pc-i440fx-4.0".to_string());
            }
            MachineX86_64::Pci440fx3_1 => {
                cmd.push("pc-i440fx-3.1".to_string());
            }
            MachineX86_64::Pci440fx3_0 => {
                cmd.push("pc-i440fx-3.0".to_string());
            }
            MachineX86_64::Pci440fx2_9 => {
                cmd.push("pc-i440fx-2.9".to_string());
            }
            MachineX86_64::Pci440fx2_8 => {
                cmd.push("pc-i440fx-2.8".to_string());
            }
            MachineX86_64::Pci440fx2_7 => {
                cmd.push("pc-i440fx-2.7".to_string());
            }
            MachineX86_64::Pci440fx2_6 => {
                cmd.push("pc-i440fx-2.6".to_string());
            }
            MachineX86_64::Pci440fx2_5 => {
                cmd.push("pc-i440fx-2.5".to_string());
            }
            MachineX86_64::Pci440fx2_4 => {
                cmd.push("pc-i440fx-2.4".to_string());
            }
            MachineX86_64::Pci440fx2_12 => {
                cmd.push("pc-i440fx-2.12".to_string());
            }
            MachineX86_64::Pci440fx2_11 => {
                cmd.push("pc-i440fx-2.11".to_string());
            }
            MachineX86_64::Pci440fx2_10 => {
                cmd.push("pc-i440fx-2.10".to_string());
            }
            MachineX86_64::Pc => {
                cmd.push("pc".to_string());
            }
            MachineX86_64::Pci440fx10_0 => {
                cmd.push("pc-i440fx-10.0".to_string());
            }
            MachineX86_64::Pcq359_2 => {
                cmd.push("pc-q35-9.2".to_string());
            }
            MachineX86_64::Pcq359_1 => {
                cmd.push("pc-q35-9.1".to_string());
            }
            MachineX86_64::Pcq359_0 => {
                cmd.push("pc-q35-9.0".to_string());
            }
            MachineX86_64::Pcq358_2 => {
                cmd.push("pc-q35-8.2".to_string());
            }
            MachineX86_64::Pcq358_1 => {
                cmd.push("pc-q35-8.1".to_string());
            }
            MachineX86_64::Pcq358_0 => {
                cmd.push("pc-q35-8.0".to_string());
            }
            MachineX86_64::Pcq357_2 => {
                cmd.push("pc-q35-7.2".to_string());
            }
            MachineX86_64::Pcq357_1 => {
                cmd.push("pc-q35-7.1".to_string());
            }
            MachineX86_64::Pcq357_0 => {
                cmd.push("pc-q35-7.0".to_string());
            }
            MachineX86_64::Pcq356_2 => {
                cmd.push("pc-q35-6.2".to_string());
            }
            MachineX86_64::Pcq356_1 => {
                cmd.push("pc-q35-6.1".to_string());
            }
            MachineX86_64::Pcq356_0 => {
                cmd.push("pc-q35-6.0".to_string());
            }
            MachineX86_64::Pcq355_2 => {
                cmd.push("pc-q35-5.2".to_string());
            }
            MachineX86_64::Pcq355_1 => {
                cmd.push("pc-q35-5.1".to_string());
            }
            MachineX86_64::Pcq355_0 => {
                cmd.push("pc-q35-5.0".to_string());
            }
            MachineX86_64::Pcq354_2 => {
                cmd.push("pc-q35-4.2".to_string());
            }
            MachineX86_64::Pcq354_1 => {
                cmd.push("pc-q35-4.1".to_string());
            }
            MachineX86_64::Pcq354_0_1 => {
                cmd.push("pc-q35-4.0.1".to_string());
            }
            MachineX86_64::Pcq354_0 => {
                cmd.push("pc-q35-4.0".to_string());
            }
            MachineX86_64::Pcq353_1 => {
                cmd.push("pc-q35-3.1".to_string());
            }
            MachineX86_64::Pcq353_0 => {
                cmd.push("pc-q35-3.0".to_string());
            }
            MachineX86_64::Pcq352_9 => {
                cmd.push("pc-q35-2.9".to_string());
            }
            MachineX86_64::Pcq352_8 => {
                cmd.push("pc-q35-2.8".to_string());
            }
            MachineX86_64::Pcq352_7 => {
                cmd.push("pc-q35-2.7".to_string());
            }
            MachineX86_64::Pcq352_6 => {
                cmd.push("pc-q35-2.6".to_string());
            }
            MachineX86_64::Pcq352_5 => {
                cmd.push("pc-q35-2.5".to_string());
            }
            MachineX86_64::Pcq352_4 => {
                cmd.push("pc-q35-2.4".to_string());
            }
            MachineX86_64::Pcq352_12 => {
                cmd.push("pc-q35-2.12".to_string());
            }
            MachineX86_64::Pcq352_11 => {
                cmd.push("pc-q35-2.11".to_string());
            }
            MachineX86_64::Pcq352_10 => {
                cmd.push("pc-q35-2.10".to_string());
            }
            MachineX86_64::Q35 => {
                cmd.push("q35".to_string());
            }
            MachineX86_64::Pcq3510_0 => {
                cmd.push("pc-q35-10.0".to_string());
            }
            MachineX86_64::Isapc => {
                cmd.push("isapc".to_string());
            }
            MachineX86_64::None => {
                cmd.push("none".to_string());
            }
        }
        cmd
    }
}

impl ToArg for MachineX86_64 {
    fn to_arg(&self) -> &str {
        match self {
            MachineX86_64::Microvm => "microvm",
            MachineX86_64::Pci440fx9_2 => "pc-i440fx-9.2",
            MachineX86_64::Pci440fx9_1 => "pc-i440fx-9.1",
            MachineX86_64::Pci440fx9_0 => "pc-i440fx-9.0",
            MachineX86_64::Pci440fx8_2 => "pc-i440fx-8.2",
            MachineX86_64::Pci440fx8_1 => "pc-i440fx-8.1",
            MachineX86_64::Pci440fx8_0 => "pc-i440fx-8.0",
            MachineX86_64::Pci440fx7_2 => "pc-i440fx-7.2",
            MachineX86_64::Pci440fx7_1 => "pc-i440fx-7.1",
            MachineX86_64::Pci440fx7_0 => "pc-i440fx-7.0",
            MachineX86_64::Pci440fx6_2 => "pc-i440fx-6.2",
            MachineX86_64::Pci440fx6_1 => "pc-i440fx-6.1",
            MachineX86_64::Pci440fx6_0 => "pc-i440fx-6.0",
            MachineX86_64::Pci440fx5_2 => "pc-i440fx-5.2",
            MachineX86_64::Pci440fx5_1 => "pc-i440fx-5.1",
            MachineX86_64::Pci440fx5_0 => "pc-i440fx-5.0",
            MachineX86_64::Pci440fx4_2 => "pc-i440fx-4.2",
            MachineX86_64::Pci440fx4_1 => "pc-i440fx-4.1",
            MachineX86_64::Pci440fx4_0 => "pc-i440fx-4.0",
            MachineX86_64::Pci440fx3_1 => "pc-i440fx-3.1",
            MachineX86_64::Pci440fx3_0 => "pc-i440fx-3.0",
            MachineX86_64::Pci440fx2_9 => "pc-i440fx-2.9",
            MachineX86_64::Pci440fx2_8 => "pc-i440fx-2.8",
            MachineX86_64::Pci440fx2_7 => "pc-i440fx-2.7",
            MachineX86_64::Pci440fx2_6 => "pc-i440fx-2.6",
            MachineX86_64::Pci440fx2_5 => "pc-i440fx-2.5",
            MachineX86_64::Pci440fx2_4 => "pc-i440fx-2.4",
            MachineX86_64::Pci440fx2_12 => "pc-i440fx-2.12",
            MachineX86_64::Pci440fx2_11 => "pc-i440fx-2.11",
            MachineX86_64::Pci440fx2_10 => "pc-i440fx-2.10",
            MachineX86_64::Pc => "pc",
            MachineX86_64::Pci440fx10_0 => "pc-i440fx-10.0",
            MachineX86_64::Pcq359_2 => "pc-q35-9.2",
            MachineX86_64::Pcq359_1 => "pc-q35-9.1",
            MachineX86_64::Pcq359_0 => "pc-q35-9.0",
            MachineX86_64::Pcq358_2 => "pc-q35-8.2",
            MachineX86_64::Pcq358_1 => "pc-q35-8.1",
            MachineX86_64::Pcq358_0 => "pc-q35-8.0",
            MachineX86_64::Pcq357_2 => "pc-q35-7.2",
            MachineX86_64::Pcq357_1 => "pc-q35-7.1",
            MachineX86_64::Pcq357_0 => "pc-q35-7.0",
            MachineX86_64::Pcq356_2 => "pc-q35-6.2",
            MachineX86_64::Pcq356_1 => "pc-q35-6.1",
            MachineX86_64::Pcq356_0 => "pc-q35-6.0",
            MachineX86_64::Pcq355_2 => "pc-q35-5.2",
            MachineX86_64::Pcq355_1 => "pc-q35-5.1",
            MachineX86_64::Pcq355_0 => "pc-q35-5.0",
            MachineX86_64::Pcq354_2 => "pc-q35-4.2",
            MachineX86_64::Pcq354_1 => "pc-q35-4.1",
            MachineX86_64::Pcq354_0_1 => "pc-q35-4.0.1",
            MachineX86_64::Pcq354_0 => "pc-q35-4.0",
            MachineX86_64::Pcq353_1 => "pc-q35-3.1",
            MachineX86_64::Pcq353_0 => "pc-q35-3.0",
            MachineX86_64::Pcq352_9 => "pc-q35-2.9",
            MachineX86_64::Pcq352_8 => "pc-q35-2.8",
            MachineX86_64::Pcq352_7 => "pc-q35-2.7",
            MachineX86_64::Pcq352_6 => "pc-q35-2.6",
            MachineX86_64::Pcq352_5 => "pc-q35-2.5",
            MachineX86_64::Pcq352_4 => "pc-q35-2.4",
            MachineX86_64::Pcq352_12 => "pc-q35-2.12",
            MachineX86_64::Pcq352_11 => "pc-q35-2.11",
            MachineX86_64::Pcq352_10 => "pc-q35-2.10",
            MachineX86_64::Q35 => "q35",
            MachineX86_64::Pcq3510_0 => "pc-q35-10.0",
            MachineX86_64::Isapc => "isapc",
            MachineX86_64::None => "none",
        }
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum MachineAarch64 {
    /// Aspeed AST1030 MiniBMC (Cortex-M4)
    Ast1030evb,
    /// Aspeed AST2500 EVB (ARM1176)
    Ast2500evb,
    /// Aspeed AST2600 EVB (Cortex-A7)
    Ast2600evb,
    /// Aspeed AST2700 A0 EVB (Cortex-A35) (alias of ast2700a0-evb)
    Ast2700evb,
    /// Aspeed AST2700 A0 EVB (Cortex-A35)
    Ast2700a0evb,
    /// Aspeed AST2700 A1 EVB (Cortex-A35)
    Ast2700a1evb,
    /// B-L475E-IOT01A Discovery Kit (Cortex-M4)
    Bl475eiot01a,
    /// Facebook Bletchley BMC (Cortex-A7)
    Bletchleybmc,
    /// Bananapi M2U (Cortex-A7)
    Bpim2u,
    /// Canon PowerShot A1100 IS (ARM946)
    Canona1100,
    /// Sharp SL-5500 (Collie) PDA (SA-1110)
    Collie,
    /// cubietech cubieboard (Cortex-A8)
    Cubieboard,
    /// SmartFusion2 SOM kit from Emcraft (M2S010)
    Emcraftsf2,
    /// Facebook fby35 BMC (Cortex-A7)
    Fby35bmc,
    /// Meta Platforms fby35
    Fby35,
    /// Inspur FP5280G2 BMC (ARM1176)
    Fp5280g2bmc,
    /// Facebook Fuji BMC (Cortex-A7)
    Fujibmc,
    /// Bytedance G220A BMC (ARM1176)
    G220abmc,
    /// Calxeda Highbank (ECX-1000)
    Highbank,
    /// ARM i.MX25 PDK board (ARM926)
    Imx25pdk,
    /// NXP i.MX 8M Plus EVK Board
    Imx8mpevk,
    /// ARM Integrator/CP (ARM926EJ-S)
    Integratorcp,
    /// Kudo BMC (Cortex-A9)
    Kudobmc,
    /// ARM KZM Emulation Baseboard (ARM1136)
    Kzm,
    /// Stellaris LM3S6965EVB (Cortex-M3)
    Lm3s6965evb,
    /// Stellaris LM3S811EVB (Cortex-M3)
    Lm3s811evb,
    /// Freescale i.MX6UL Evaluation Kit (Cortex-A7)
    Mcimx6ulevk,
    /// Freescale i.MX7 DUAL SABRE (Cortex-A7)
    Mcimx7dsabre,
    /// BBC micro:bit (Cortex-M0)
    Microbit,
    /// Calxeda Midway (ECX-2000)
    Midway,
    /// Mori BMC (Cortex-A9)
    Moribmc,
    /// ARM MPS2 with AN385 FPGA image for Cortex-M3
    Mps2an385,
    /// ARM MPS2 with AN386 FPGA image for Cortex-M4
    Mps2an386,
    /// ARM MPS2 with AN500 FPGA image for Cortex-M7
    Mps2an500,
    /// ARM MPS2 with AN505 FPGA image for Cortex-M33
    Mps2an505,
    /// ARM MPS2 with AN511 DesignStart FPGA image for Cortex-M3
    Mps2an511,
    /// ARM MPS2 with AN521 FPGA image for dual Cortex-M33
    Mps2an521,
    /// ARM MPS3 with AN524 FPGA image for dual Cortex-M33
    Mps3an524,
    /// ARM MPS3 with AN536 FPGA image for Cortex-R52
    Mps3an536,
    /// ARM MPS3 with AN547 FPGA image for Cortex-M55
    Mps3an547,
    /// ARM Musca-A board (dual Cortex-M33)
    Muscaa,
    /// ARM Musca-B1 board (dual Cortex-M33)
    Muscab1,
    /// Marvell 88w8618 / MusicPal (ARM926EJ-S)
    Musicpal,
    /// Netduino 2 Machine (Cortex-M3)
    Netduino2,
    /// Netduino Plus 2 Machine (Cortex-M4)
    Netduinoplus2,
    /// empty machine
    None,
    /// Nuvoton NPCM750 Evaluation Board (Cortex-A9)
    Npcm750evb,
    /// Nuvoton NPCM845 Evaluation Board (Cortex-A35)
    Npcm845evb,
    /// Samsung NURI board (Exynos4210)
    Nuri,
    /// Olimex STM32-H405 (Cortex-M4)
    Olimexstm32h405,
    /// Orange Pi PC (Cortex-A7)
    Orangepipc,
    /// OpenPOWER Palmetto BMC (ARM926EJ-S)
    Palmettobmc,
    /// Qualcomm DC-SCM V1 BMC (Cortex A7)
    QcomdcscmV1bmc,
    /// Qualcomm DC-SCM V1/Firework BMC (Cortex A7)
    Qcomfireworkbmc,
    /// Quanta GBS (Cortex-A9)
    Quantagbsbmc,
    /// Quanta GSJ (Cortex-A9)
    Quantagsj,
    /// Quanta-Q71l BMC (ARM926EJ-S)
    Quantaq71lbmc,
    /// IBM Rainier BMC (Cortex-A7)
    Rainierbmc,
    /// Raspberry Pi Zero (revision 1.2)
    Raspi0,
    /// Raspberry Pi A+ (revision 1.1)
    Raspi1ap,
    /// Raspberry Pi 2B (revision 1.1)
    Raspi2b,
    /// Raspberry Pi 3A+ (revision 1.0)
    Raspi3ap,
    /// Raspberry Pi 3B (revision 1.2)
    Raspi3b,
    /// Raspberry Pi 4B (revision 1.5)
    Raspi4b,
    /// ARM RealView Emulation Baseboard (ARM926EJ-S)
    Realvieweb,
    /// ARM RealView Emulation Baseboard (ARM11MPCore)
    Realviewebmpcore,
    /// ARM RealView Platform Baseboard for Cortex-A8
    Realviewpba8,
    /// ARM RealView Platform Baseboard Explore for Cortex-A9
    Realviewpbxa9,
    /// OpenPOWER Romulus BMC (ARM1176)
    Romulusbmc,
    /// Freescale i.MX6 Quad SABRE Lite Board (Cortex-A9)
    Sabrelite,
    /// QEMU 'SBSA Reference' ARM Virtual Machine
    Sbsaref,
    /// Samsung SMDKC210 board (Exynos4210)
    Smdkc210,
    /// OCP SonoraPass BMC (ARM1176)
    Sonorapassbmc,
    /// ST STM32VLDISCOVERY (Cortex-M3)
    Stm32vldiscovery,
    /// Supermicro X11 SPI BMC (ARM1176)
    Supermicrox11spibmc,
    /// Supermicro X11 BMC (ARM926EJ-S)
    Supermicrox11bmc,
    /// Siemens SX1 (OMAP310) V2
    Sx1,
    /// Siemens SX1 (OMAP310) V1
    Sx1V1,
    /// Facebook Tiogapass BMC (ARM1176)
    Tiogapassbmc,
    /// ARM Versatile/AB (ARM926EJ-S)
    Versatileab,
    /// ARM Versatile/PB (ARM926EJ-S)
    Versatilepb,
    /// ARM Versatile Express for Cortex-A15
    Vexpressa15,
    /// ARM Versatile Express for Cortex-A9
    Vexpressa9,
    /// QEMU 10.0 ARM Virtual Machine (alias of virt-10.0)
    Virt,
    /// QEMU 10.0 ARM Virtual Machine
    Virt10_0,
    /// QEMU 2.10 ARM Virtual Machine (deprecated)
    Virt2_10,
    /// QEMU 2.11 ARM Virtual Machine (deprecated)
    Virt2_11,
    /// QEMU 2.12 ARM Virtual Machine (deprecated)
    Virt2_12,
    /// QEMU 2.6 ARM Virtual Machine (deprecated)
    Virt2_6,
    /// QEMU 2.7 ARM Virtual Machine (deprecated)
    Virt2_7,
    /// QEMU 2.8 ARM Virtual Machine (deprecated)
    Virt2_8,
    /// QEMU 2.9 ARM Virtual Machine (deprecated)
    Virt2_9,
    /// QEMU 3.0 ARM Virtual Machine (deprecated)
    Virt3_0,
    /// QEMU 3.1 ARM Virtual Machine (deprecated)
    Virt3_1,
    /// QEMU 4.0 ARM Virtual Machine (deprecated)
    Virt4_0,
    /// QEMU 4.1 ARM Virtual Machine (deprecated)
    Virt4_1,
    /// QEMU 4.2 ARM Virtual Machine (deprecated)
    Virt4_2,
    /// QEMU 5.0 ARM Virtual Machine (deprecated)
    Virt5_0,
    /// QEMU 5.1 ARM Virtual Machine (deprecated)
    Virt5_1,
    /// QEMU 5.2 ARM Virtual Machine (deprecated)
    Virt5_2,
    /// QEMU 6.0 ARM Virtual Machine (deprecated)
    Virt6_0,
    /// QEMU 6.1 ARM Virtual Machine (deprecated)
    Virt6_1,
    /// QEMU 6.2 ARM Virtual Machine (deprecated)
    Virt6_2,
    /// QEMU 7.0 ARM Virtual Machine (deprecated)
    Virt7_0,
    /// QEMU 7.1 ARM Virtual Machine
    Virt7_1,
    /// QEMU 7.2 ARM Virtual Machine
    Virt7_2,
    /// QEMU 8.0 ARM Virtual Machine
    Virt8_0,
    /// QEMU 8.1 ARM Virtual Machine
    Virt8_1,
    /// QEMU 8.2 ARM Virtual Machine
    Virt8_2,
    /// QEMU 9.0 ARM Virtual Machine
    Virt9_0,
    /// QEMU 9.1 ARM Virtual Machine
    Virt9_1,
    /// QEMU 9.2 ARM Virtual Machine
    Virt9_2,
    /// OpenPOWER Witherspoon BMC (ARM1176)
    Witherspoonbmc,
    /// Xilinx Zynq 7000 Platform Baseboard for Cortex-A9
    Xilinxzynqa9,
    /// Xilinx Versal Virtual development board
    XlnxVersalvirt,
    /// Xilinx ZynqMP ZCU102 board with 4xA53s and 2xR5Fs based on the value of smp
    Xlnxzcu102,
    /// Facebook YosemiteV2 BMC (ARM1176)
    Yosemitev2bmc,
}
impl ToCommand for MachineAarch64 {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![];

        match self {
            MachineAarch64::Ast1030evb => {
                cmd.push("ast1030-evb".to_string());
            }
            MachineAarch64::Ast2500evb => {
                cmd.push("ast2500-evb".to_string());
            }
            MachineAarch64::Ast2600evb => {
                cmd.push("ast2600-evb".to_string());
            }
            MachineAarch64::Ast2700evb => {
                cmd.push("ast2700-evb".to_string());
            }
            MachineAarch64::Ast2700a0evb => {
                cmd.push("ast2700a0-evb".to_string());
            }
            MachineAarch64::Ast2700a1evb => {
                cmd.push("ast2700a1-evb".to_string());
            }
            MachineAarch64::Bl475eiot01a => {
                cmd.push("b-l475e-iot01a".to_string());
            }
            MachineAarch64::Bletchleybmc => {
                cmd.push("bletchley-bmc".to_string());
            }
            MachineAarch64::Bpim2u => {
                cmd.push("bpim2u".to_string());
            }
            MachineAarch64::Canona1100 => {
                cmd.push("canon-a1100".to_string());
            }
            MachineAarch64::Collie => {
                cmd.push("collie".to_string());
            }
            MachineAarch64::Cubieboard => {
                cmd.push("cubieboard".to_string());
            }
            MachineAarch64::Emcraftsf2 => {
                cmd.push("emcraft-sf2".to_string());
            }
            MachineAarch64::Fby35bmc => {
                cmd.push("fby35-bmc".to_string());
            }
            MachineAarch64::Fby35 => {
                cmd.push("fby35".to_string());
            }
            MachineAarch64::Fp5280g2bmc => {
                cmd.push("fp5280g2-bmc".to_string());
            }
            MachineAarch64::Fujibmc => {
                cmd.push("fuji-bmc".to_string());
            }
            MachineAarch64::G220abmc => {
                cmd.push("g220a-bmc".to_string());
            }
            MachineAarch64::Highbank => {
                cmd.push("highbank".to_string());
            }
            MachineAarch64::Imx25pdk => {
                cmd.push("imx25-pdk".to_string());
            }
            MachineAarch64::Imx8mpevk => {
                cmd.push("imx8mp-evk".to_string());
            }
            MachineAarch64::Integratorcp => {
                cmd.push("integratorcp".to_string());
            }
            MachineAarch64::Kudobmc => {
                cmd.push("kudo-bmc".to_string());
            }
            MachineAarch64::Kzm => {
                cmd.push("kzm".to_string());
            }
            MachineAarch64::Lm3s6965evb => {
                cmd.push("lm3s6965evb".to_string());
            }
            MachineAarch64::Lm3s811evb => {
                cmd.push("lm3s811evb".to_string());
            }
            MachineAarch64::Mcimx6ulevk => {
                cmd.push("mcimx6ul-evk".to_string());
            }
            MachineAarch64::Mcimx7dsabre => {
                cmd.push("mcimx7d-sabre".to_string());
            }
            MachineAarch64::Microbit => {
                cmd.push("microbit".to_string());
            }
            MachineAarch64::Midway => {
                cmd.push("midway".to_string());
            }
            MachineAarch64::Moribmc => {
                cmd.push("mori-bmc".to_string());
            }
            MachineAarch64::Mps2an385 => {
                cmd.push("mps2-an385".to_string());
            }
            MachineAarch64::Mps2an386 => {
                cmd.push("mps2-an386".to_string());
            }
            MachineAarch64::Mps2an500 => {
                cmd.push("mps2-an500".to_string());
            }
            MachineAarch64::Mps2an505 => {
                cmd.push("mps2-an505".to_string());
            }
            MachineAarch64::Mps2an511 => {
                cmd.push("mps2-an511".to_string());
            }
            MachineAarch64::Mps2an521 => {
                cmd.push("mps2-an521".to_string());
            }
            MachineAarch64::Mps3an524 => {
                cmd.push("mps3-an524".to_string());
            }
            MachineAarch64::Mps3an536 => {
                cmd.push("mps3-an536".to_string());
            }
            MachineAarch64::Mps3an547 => {
                cmd.push("mps3-an547".to_string());
            }
            MachineAarch64::Muscaa => {
                cmd.push("musca-a".to_string());
            }
            MachineAarch64::Muscab1 => {
                cmd.push("musca-b1".to_string());
            }
            MachineAarch64::Musicpal => {
                cmd.push("musicpal".to_string());
            }
            MachineAarch64::Netduino2 => {
                cmd.push("netduino2".to_string());
            }
            MachineAarch64::Netduinoplus2 => {
                cmd.push("netduinoplus2".to_string());
            }
            MachineAarch64::None => {
                cmd.push("none".to_string());
            }
            MachineAarch64::Npcm750evb => {
                cmd.push("npcm750-evb".to_string());
            }
            MachineAarch64::Npcm845evb => {
                cmd.push("npcm845-evb".to_string());
            }
            MachineAarch64::Nuri => {
                cmd.push("nuri".to_string());
            }
            MachineAarch64::Olimexstm32h405 => {
                cmd.push("olimex-stm32-h405".to_string());
            }
            MachineAarch64::Orangepipc => {
                cmd.push("orangepi-pc".to_string());
            }
            MachineAarch64::Palmettobmc => {
                cmd.push("palmetto-bmc".to_string());
            }
            MachineAarch64::QcomdcscmV1bmc => {
                cmd.push("qcom-dc-scm-v1-bmc".to_string());
            }
            MachineAarch64::Qcomfireworkbmc => {
                cmd.push("qcom-firework-bmc".to_string());
            }
            MachineAarch64::Quantagbsbmc => {
                cmd.push("quanta-gbs-bmc".to_string());
            }
            MachineAarch64::Quantagsj => {
                cmd.push("quanta-gsj".to_string());
            }
            MachineAarch64::Quantaq71lbmc => {
                cmd.push("quanta-q71l-bmc".to_string());
            }
            MachineAarch64::Rainierbmc => {
                cmd.push("rainier-bmc".to_string());
            }
            MachineAarch64::Raspi0 => {
                cmd.push("raspi0".to_string());
            }
            MachineAarch64::Raspi1ap => {
                cmd.push("raspi1ap".to_string());
            }
            MachineAarch64::Raspi2b => {
                cmd.push("raspi2b".to_string());
            }
            MachineAarch64::Raspi3ap => {
                cmd.push("raspi3ap".to_string());
            }
            MachineAarch64::Raspi3b => {
                cmd.push("raspi3b".to_string());
            }
            MachineAarch64::Raspi4b => {
                cmd.push("raspi4b".to_string());
            }
            MachineAarch64::Realvieweb => {
                cmd.push("realview-eb".to_string());
            }
            MachineAarch64::Realviewebmpcore => {
                cmd.push("realview-eb-mpcore".to_string());
            }
            MachineAarch64::Realviewpba8 => {
                cmd.push("realview-pb-a8".to_string());
            }
            MachineAarch64::Realviewpbxa9 => {
                cmd.push("realview-pbx-a9".to_string());
            }
            MachineAarch64::Romulusbmc => {
                cmd.push("romulus-bmc".to_string());
            }
            MachineAarch64::Sabrelite => {
                cmd.push("sabrelite".to_string());
            }
            MachineAarch64::Sbsaref => {
                cmd.push("sbsa-ref".to_string());
            }
            MachineAarch64::Smdkc210 => {
                cmd.push("smdkc210".to_string());
            }
            MachineAarch64::Sonorapassbmc => {
                cmd.push("sonorapass-bmc".to_string());
            }
            MachineAarch64::Stm32vldiscovery => {
                cmd.push("stm32vldiscovery".to_string());
            }
            MachineAarch64::Supermicrox11spibmc => {
                cmd.push("supermicro-x11spi-bmc".to_string());
            }
            MachineAarch64::Supermicrox11bmc => {
                cmd.push("supermicrox11-bmc".to_string());
            }
            MachineAarch64::Sx1 => {
                cmd.push("sx1".to_string());
            }
            MachineAarch64::Sx1V1 => {
                cmd.push("sx1-v1".to_string());
            }
            MachineAarch64::Tiogapassbmc => {
                cmd.push("tiogapass-bmc".to_string());
            }
            MachineAarch64::Versatileab => {
                cmd.push("versatileab".to_string());
            }
            MachineAarch64::Versatilepb => {
                cmd.push("versatilepb".to_string());
            }
            MachineAarch64::Vexpressa15 => {
                cmd.push("vexpress-a15".to_string());
            }
            MachineAarch64::Vexpressa9 => {
                cmd.push("vexpress-a9".to_string());
            }
            MachineAarch64::Virt => {
                cmd.push("virt".to_string());
            }
            MachineAarch64::Virt10_0 => {
                cmd.push("virt-10.0".to_string());
            }
            MachineAarch64::Virt2_10 => {
                cmd.push("virt-2.10".to_string());
            }
            MachineAarch64::Virt2_11 => {
                cmd.push("virt-2.11".to_string());
            }
            MachineAarch64::Virt2_12 => {
                cmd.push("virt-2.12".to_string());
            }
            MachineAarch64::Virt2_6 => {
                cmd.push("virt-2.6".to_string());
            }
            MachineAarch64::Virt2_7 => {
                cmd.push("virt-2.7".to_string());
            }
            MachineAarch64::Virt2_8 => {
                cmd.push("virt-2.8".to_string());
            }
            MachineAarch64::Virt2_9 => {
                cmd.push("virt-2.9".to_string());
            }
            MachineAarch64::Virt3_0 => {
                cmd.push("virt-3.0".to_string());
            }
            MachineAarch64::Virt3_1 => {
                cmd.push("virt-3.1".to_string());
            }
            MachineAarch64::Virt4_0 => {
                cmd.push("virt-4.0".to_string());
            }
            MachineAarch64::Virt4_1 => {
                cmd.push("virt-4.1".to_string());
            }
            MachineAarch64::Virt4_2 => {
                cmd.push("virt-4.2".to_string());
            }
            MachineAarch64::Virt5_0 => {
                cmd.push("virt-5.0".to_string());
            }
            MachineAarch64::Virt5_1 => {
                cmd.push("virt-5.1".to_string());
            }
            MachineAarch64::Virt5_2 => {
                cmd.push("virt-5.2".to_string());
            }
            MachineAarch64::Virt6_0 => {
                cmd.push("virt-6.0".to_string());
            }
            MachineAarch64::Virt6_1 => {
                cmd.push("virt-6.1".to_string());
            }
            MachineAarch64::Virt6_2 => {
                cmd.push("virt-6.2".to_string());
            }
            MachineAarch64::Virt7_0 => {
                cmd.push("virt-7.0".to_string());
            }
            MachineAarch64::Virt7_1 => {
                cmd.push("virt-7.1".to_string());
            }
            MachineAarch64::Virt7_2 => {
                cmd.push("virt-7.2".to_string());
            }
            MachineAarch64::Virt8_0 => {
                cmd.push("virt-8.0".to_string());
            }
            MachineAarch64::Virt8_1 => {
                cmd.push("virt-8.1".to_string());
            }
            MachineAarch64::Virt8_2 => {
                cmd.push("virt-8.2".to_string());
            }
            MachineAarch64::Virt9_0 => {
                cmd.push("virt-9.0".to_string());
            }
            MachineAarch64::Virt9_1 => {
                cmd.push("virt-9.1".to_string());
            }
            MachineAarch64::Virt9_2 => {
                cmd.push("virt-9.2".to_string());
            }
            MachineAarch64::Witherspoonbmc => {
                cmd.push("witherspoon-bmc".to_string());
            }
            MachineAarch64::Xilinxzynqa9 => {
                cmd.push("xilinx-zynq-a9".to_string());
            }
            MachineAarch64::XlnxVersalvirt => {
                cmd.push("xlnx-versal-virt".to_string());
            }
            MachineAarch64::Xlnxzcu102 => {
                cmd.push("xlnx-zcu102".to_string());
            }
            MachineAarch64::Yosemitev2bmc => {
                cmd.push("yosemitev2-bmc".to_string());
            }
        }
        cmd
    }
}

impl ToArg for MachineAarch64 {
    fn to_arg(&self) -> &str {
        match self {
            MachineAarch64::Ast1030evb => "ast1030-evb",
            MachineAarch64::Ast2500evb => "ast2500-evb",
            MachineAarch64::Ast2600evb => "ast2600-evb",
            MachineAarch64::Ast2700evb => "ast2700-evb",
            MachineAarch64::Ast2700a0evb => "ast2700a0-evb",
            MachineAarch64::Ast2700a1evb => "ast2700a1-evb",
            MachineAarch64::Bl475eiot01a => "b-l475e-iot01a",
            MachineAarch64::Bletchleybmc => "bletchley-bmc",
            MachineAarch64::Bpim2u => "bpim2u",
            MachineAarch64::Canona1100 => "canon-a1100",
            MachineAarch64::Collie => "collie",
            MachineAarch64::Cubieboard => "cubieboard",
            MachineAarch64::Emcraftsf2 => "emcraft-sf2",
            MachineAarch64::Fby35bmc => "fby35-bmc",
            MachineAarch64::Fby35 => "fby35",
            MachineAarch64::Fp5280g2bmc => "fp5280g2-bmc",
            MachineAarch64::Fujibmc => "fuji-bmc",
            MachineAarch64::G220abmc => "g220a-bmc",
            MachineAarch64::Highbank => "highbank",
            MachineAarch64::Imx25pdk => "imx25-pdk",
            MachineAarch64::Imx8mpevk => "imx8mp-evk",
            MachineAarch64::Integratorcp => "integratorcp",
            MachineAarch64::Kudobmc => "kudo-bmc",
            MachineAarch64::Kzm => "kzm",
            MachineAarch64::Lm3s6965evb => "lm3s6965evb",
            MachineAarch64::Lm3s811evb => "lm3s811evb",
            MachineAarch64::Mcimx6ulevk => "mcimx6ul-evk",
            MachineAarch64::Mcimx7dsabre => "mcimx7d-sabre",
            MachineAarch64::Microbit => "microbit",
            MachineAarch64::Midway => "midway",
            MachineAarch64::Moribmc => "mori-bmc",
            MachineAarch64::Mps2an385 => "mps2-an385",
            MachineAarch64::Mps2an386 => "mps2-an386",
            MachineAarch64::Mps2an500 => "mps2-an500",
            MachineAarch64::Mps2an505 => "mps2-an505",
            MachineAarch64::Mps2an511 => "mps2-an511",
            MachineAarch64::Mps2an521 => "mps2-an521",
            MachineAarch64::Mps3an524 => "mps3-an524",
            MachineAarch64::Mps3an536 => "mps3-an536",
            MachineAarch64::Mps3an547 => "mps3-an547",
            MachineAarch64::Muscaa => "musca-a",
            MachineAarch64::Muscab1 => "musca-b1",
            MachineAarch64::Musicpal => "musicpal",
            MachineAarch64::Netduino2 => "netduino2",
            MachineAarch64::Netduinoplus2 => "netduinoplus2",
            MachineAarch64::None => "none",
            MachineAarch64::Npcm750evb => "npcm750-evb",
            MachineAarch64::Npcm845evb => "npcm845-evb",
            MachineAarch64::Nuri => "nuri",
            MachineAarch64::Olimexstm32h405 => "olimex-stm32-h405",
            MachineAarch64::Orangepipc => "orangepi-pc",
            MachineAarch64::Palmettobmc => "palmetto-bmc",
            MachineAarch64::QcomdcscmV1bmc => "qcom-dc-scm-v1-bmc",
            MachineAarch64::Qcomfireworkbmc => "qcom-firework-bmc",
            MachineAarch64::Quantagbsbmc => "quanta-gbs-bmc",
            MachineAarch64::Quantagsj => "quanta-gsj",
            MachineAarch64::Quantaq71lbmc => "quanta-q71l-bmc",
            MachineAarch64::Rainierbmc => "rainier-bmc",
            MachineAarch64::Raspi0 => "raspi0",
            MachineAarch64::Raspi1ap => "raspi1ap",
            MachineAarch64::Raspi2b => "raspi2b",
            MachineAarch64::Raspi3ap => "raspi3ap",
            MachineAarch64::Raspi3b => "raspi3b",
            MachineAarch64::Raspi4b => "raspi4b",
            MachineAarch64::Realvieweb => "realview-eb",
            MachineAarch64::Realviewebmpcore => "realview-eb-mpcore",
            MachineAarch64::Realviewpba8 => "realview-pb-a8",
            MachineAarch64::Realviewpbxa9 => "realview-pbx-a9",
            MachineAarch64::Romulusbmc => "romulus-bmc",
            MachineAarch64::Sabrelite => "sabrelite",
            MachineAarch64::Sbsaref => "sbsa-ref",
            MachineAarch64::Smdkc210 => "smdkc210",
            MachineAarch64::Sonorapassbmc => "sonorapass-bmc",
            MachineAarch64::Stm32vldiscovery => "stm32vldiscovery",
            MachineAarch64::Supermicrox11spibmc => "supermicro-x11spi-bmc",
            MachineAarch64::Supermicrox11bmc => "supermicrox11-bmc",
            MachineAarch64::Sx1 => "sx1",
            MachineAarch64::Sx1V1 => "sx1-v1",
            MachineAarch64::Tiogapassbmc => "tiogapass-bmc",
            MachineAarch64::Versatileab => "versatileab",
            MachineAarch64::Versatilepb => "versatilepb",
            MachineAarch64::Vexpressa15 => "vexpress-a15",
            MachineAarch64::Vexpressa9 => "vexpress-a9",
            MachineAarch64::Virt => "virt",
            MachineAarch64::Virt10_0 => "virt-10.0",
            MachineAarch64::Virt2_10 => "virt-2.10",
            MachineAarch64::Virt2_11 => "virt-2.11",
            MachineAarch64::Virt2_12 => "virt-2.12",
            MachineAarch64::Virt2_6 => "virt-2.6",
            MachineAarch64::Virt2_7 => "virt-2.7",
            MachineAarch64::Virt2_8 => "virt-2.8",
            MachineAarch64::Virt2_9 => "virt-2.9",
            MachineAarch64::Virt3_0 => "virt-3.0",
            MachineAarch64::Virt3_1 => "virt-3.1",
            MachineAarch64::Virt4_0 => "virt-4.0",
            MachineAarch64::Virt4_1 => "virt-4.1",
            MachineAarch64::Virt4_2 => "virt-4.2",
            MachineAarch64::Virt5_0 => "virt-5.0",
            MachineAarch64::Virt5_1 => "virt-5.1",
            MachineAarch64::Virt5_2 => "virt-5.2",
            MachineAarch64::Virt6_0 => "virt-6.0",
            MachineAarch64::Virt6_1 => "virt-6.1",
            MachineAarch64::Virt6_2 => "virt-6.2",
            MachineAarch64::Virt7_0 => "virt-7.0",
            MachineAarch64::Virt7_1 => "virt-7.1",
            MachineAarch64::Virt7_2 => "virt-7.2",
            MachineAarch64::Virt8_0 => "virt-8.0",
            MachineAarch64::Virt8_1 => "virt-8.1",
            MachineAarch64::Virt8_2 => "virt-8.2",
            MachineAarch64::Virt9_0 => "virt-9.0",
            MachineAarch64::Virt9_1 => "virt-9.1",
            MachineAarch64::Virt9_2 => "virt-9.2",
            MachineAarch64::Witherspoonbmc => "witherspoon-bmc",
            MachineAarch64::Xilinxzynqa9 => "xilinx-zynq-a9",
            MachineAarch64::XlnxVersalvirt => "xlnx-versal-virt",
            MachineAarch64::Xlnxzcu102 => "xlnx-zcu102",
            MachineAarch64::Yosemitev2bmc => "yosemitev2-bmc",
        }
    }
}
