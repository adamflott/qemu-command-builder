use crate::to_command::{ToArg, ToCommand};
use proptest_derive::Arbitrary;
use std::str::FromStr;

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum MachineTypeX86_64 {
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
    /// Standard PC (Q35 + ICH9, 2009) (alias of pc-q35-10.0)
    Q35,
    /// Standard PC (Q35 + ICH9, 2009)
    Pcq3510_0,
    /// ISA-only PC
    Isapc,
    /// empty machine
    None,
}

impl ToCommand for MachineTypeX86_64 {
    fn to_args(&self) -> Vec<String> {
        let mut cmd = vec![];

        match self {
            MachineTypeX86_64::Microvm => {
                cmd.push("microvm".to_string());
            }
            MachineTypeX86_64::Pci440fx9_2 => {
                cmd.push("pc-i440fx-9.2".to_string());
            }
            MachineTypeX86_64::Pci440fx9_1 => {
                cmd.push("pc-i440fx-9.1".to_string());
            }
            MachineTypeX86_64::Pci440fx9_0 => {
                cmd.push("pc-i440fx-9.0".to_string());
            }
            MachineTypeX86_64::Pci440fx8_2 => {
                cmd.push("pc-i440fx-8.2".to_string());
            }
            MachineTypeX86_64::Pci440fx8_1 => {
                cmd.push("pc-i440fx-8.1".to_string());
            }
            MachineTypeX86_64::Pci440fx8_0 => {
                cmd.push("pc-i440fx-8.0".to_string());
            }
            MachineTypeX86_64::Pci440fx7_2 => {
                cmd.push("pc-i440fx-7.2".to_string());
            }
            MachineTypeX86_64::Pci440fx7_1 => {
                cmd.push("pc-i440fx-7.1".to_string());
            }
            MachineTypeX86_64::Pc => {
                cmd.push("pc".to_string());
            }
            MachineTypeX86_64::Pci440fx10_0 => {
                cmd.push("pc-i440fx-10.0".to_string());
            }
            MachineTypeX86_64::Pcq359_2 => {
                cmd.push("pc-q35-9.2".to_string());
            }
            MachineTypeX86_64::Pcq359_1 => {
                cmd.push("pc-q35-9.1".to_string());
            }
            MachineTypeX86_64::Pcq359_0 => {
                cmd.push("pc-q35-9.0".to_string());
            }
            MachineTypeX86_64::Pcq358_2 => {
                cmd.push("pc-q35-8.2".to_string());
            }
            MachineTypeX86_64::Pcq358_1 => {
                cmd.push("pc-q35-8.1".to_string());
            }
            MachineTypeX86_64::Pcq358_0 => {
                cmd.push("pc-q35-8.0".to_string());
            }
            MachineTypeX86_64::Pcq357_2 => {
                cmd.push("pc-q35-7.2".to_string());
            }
            MachineTypeX86_64::Pcq357_1 => {
                cmd.push("pc-q35-7.1".to_string());
            }
            MachineTypeX86_64::Q35 => {
                cmd.push("q35".to_string());
            }
            MachineTypeX86_64::Pcq3510_0 => {
                cmd.push("pc-q35-10.0".to_string());
            }
            MachineTypeX86_64::Isapc => {
                cmd.push("isapc".to_string());
            }
            MachineTypeX86_64::None => {
                cmd.push("none".to_string());
            }
        }
        cmd
    }
}

impl ToArg for MachineTypeX86_64 {
    fn to_arg(&self) -> &str {
        match self {
            MachineTypeX86_64::Microvm => "microvm",
            MachineTypeX86_64::Pci440fx9_2 => "pc-i440fx-9.2",
            MachineTypeX86_64::Pci440fx9_1 => "pc-i440fx-9.1",
            MachineTypeX86_64::Pci440fx9_0 => "pc-i440fx-9.0",
            MachineTypeX86_64::Pci440fx8_2 => "pc-i440fx-8.2",
            MachineTypeX86_64::Pci440fx8_1 => "pc-i440fx-8.1",
            MachineTypeX86_64::Pci440fx8_0 => "pc-i440fx-8.0",
            MachineTypeX86_64::Pci440fx7_2 => "pc-i440fx-7.2",
            MachineTypeX86_64::Pci440fx7_1 => "pc-i440fx-7.1",
            MachineTypeX86_64::Pc => "pc",
            MachineTypeX86_64::Pci440fx10_0 => "pc-i440fx-10.0",
            MachineTypeX86_64::Pcq359_2 => "pc-q35-9.2",
            MachineTypeX86_64::Pcq359_1 => "pc-q35-9.1",
            MachineTypeX86_64::Pcq359_0 => "pc-q35-9.0",
            MachineTypeX86_64::Pcq358_2 => "pc-q35-8.2",
            MachineTypeX86_64::Pcq358_1 => "pc-q35-8.1",
            MachineTypeX86_64::Pcq358_0 => "pc-q35-8.0",
            MachineTypeX86_64::Pcq357_2 => "pc-q35-7.2",
            MachineTypeX86_64::Pcq357_1 => "pc-q35-7.1",
            MachineTypeX86_64::Q35 => "q35",
            MachineTypeX86_64::Pcq3510_0 => "pc-q35-10.0",
            MachineTypeX86_64::Isapc => "isapc",
            MachineTypeX86_64::None => "none",
        }
    }
}

impl FromStr for MachineTypeX86_64 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "microvm" => Ok(MachineTypeX86_64::Microvm),
            "pc-i440fx-9.2" => Ok(MachineTypeX86_64::Pci440fx9_2),
            "pc-i440fx-9.1" => Ok(MachineTypeX86_64::Pci440fx9_1),
            "pc-i440fx-9.0" => Ok(MachineTypeX86_64::Pci440fx9_0),
            "pc-i440fx-8.2" => Ok(MachineTypeX86_64::Pci440fx8_2),
            "pc-i440fx-8.1" => Ok(MachineTypeX86_64::Pci440fx8_1),
            "pc-i440fx-8.0" => Ok(MachineTypeX86_64::Pci440fx8_0),
            "pc-i440fx-7.2" => Ok(MachineTypeX86_64::Pci440fx7_2),
            "pc-i440fx-7.1" => Ok(MachineTypeX86_64::Pci440fx7_1),
            "pc" => Ok(MachineTypeX86_64::Pc),
            "pc-i440fx-10.0" => Ok(MachineTypeX86_64::Pci440fx10_0),
            "pc-q35-9.2" => Ok(MachineTypeX86_64::Pcq359_2),
            "pc-q35-9.1" => Ok(MachineTypeX86_64::Pcq359_1),
            "pc-q35-9.0" => Ok(MachineTypeX86_64::Pcq359_0),
            "pc-q35-8.2" => Ok(MachineTypeX86_64::Pcq358_2),
            "pc-q35-8.1" => Ok(MachineTypeX86_64::Pcq358_1),
            "pc-q35-8.0" => Ok(MachineTypeX86_64::Pcq358_0),
            "pc-q35-7.2" => Ok(MachineTypeX86_64::Pcq357_2),
            "pc-q35-7.1" => Ok(MachineTypeX86_64::Pcq357_1),
            "q35" => Ok(MachineTypeX86_64::Q35),
            "pc-q35-10.0" => Ok(MachineTypeX86_64::Pcq3510_0),
            "isapc" => Ok(MachineTypeX86_64::Isapc),
            "none" => Ok(MachineTypeX86_64::None),
            other => Err(format!("{} is not a supported machine type", other)),
        }
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum MachineTypeAarch64 {
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

impl ToCommand for MachineTypeAarch64 {
    fn to_args(&self) -> Vec<String> {
        let mut cmd = vec![];

        match self {
            MachineTypeAarch64::Ast1030evb => {
                cmd.push("ast1030-evb".to_string());
            }
            MachineTypeAarch64::Ast2500evb => {
                cmd.push("ast2500-evb".to_string());
            }
            MachineTypeAarch64::Ast2600evb => {
                cmd.push("ast2600-evb".to_string());
            }
            MachineTypeAarch64::Ast2700evb => {
                cmd.push("ast2700-evb".to_string());
            }
            MachineTypeAarch64::Ast2700a0evb => {
                cmd.push("ast2700a0-evb".to_string());
            }
            MachineTypeAarch64::Ast2700a1evb => {
                cmd.push("ast2700a1-evb".to_string());
            }
            MachineTypeAarch64::Bl475eiot01a => {
                cmd.push("b-l475e-iot01a".to_string());
            }
            MachineTypeAarch64::Bletchleybmc => {
                cmd.push("bletchley-bmc".to_string());
            }
            MachineTypeAarch64::Bpim2u => {
                cmd.push("bpim2u".to_string());
            }
            MachineTypeAarch64::Canona1100 => {
                cmd.push("canon-a1100".to_string());
            }
            MachineTypeAarch64::Collie => {
                cmd.push("collie".to_string());
            }
            MachineTypeAarch64::Cubieboard => {
                cmd.push("cubieboard".to_string());
            }
            MachineTypeAarch64::Emcraftsf2 => {
                cmd.push("emcraft-sf2".to_string());
            }
            MachineTypeAarch64::Fby35bmc => {
                cmd.push("fby35-bmc".to_string());
            }
            MachineTypeAarch64::Fby35 => {
                cmd.push("fby35".to_string());
            }
            MachineTypeAarch64::Fp5280g2bmc => {
                cmd.push("fp5280g2-bmc".to_string());
            }
            MachineTypeAarch64::Fujibmc => {
                cmd.push("fuji-bmc".to_string());
            }
            MachineTypeAarch64::G220abmc => {
                cmd.push("g220a-bmc".to_string());
            }
            MachineTypeAarch64::Highbank => {
                cmd.push("highbank".to_string());
            }
            MachineTypeAarch64::Imx25pdk => {
                cmd.push("imx25-pdk".to_string());
            }
            MachineTypeAarch64::Imx8mpevk => {
                cmd.push("imx8mp-evk".to_string());
            }
            MachineTypeAarch64::Integratorcp => {
                cmd.push("integratorcp".to_string());
            }
            MachineTypeAarch64::Kudobmc => {
                cmd.push("kudo-bmc".to_string());
            }
            MachineTypeAarch64::Kzm => {
                cmd.push("kzm".to_string());
            }
            MachineTypeAarch64::Lm3s6965evb => {
                cmd.push("lm3s6965evb".to_string());
            }
            MachineTypeAarch64::Lm3s811evb => {
                cmd.push("lm3s811evb".to_string());
            }
            MachineTypeAarch64::Mcimx6ulevk => {
                cmd.push("mcimx6ul-evk".to_string());
            }
            MachineTypeAarch64::Mcimx7dsabre => {
                cmd.push("mcimx7d-sabre".to_string());
            }
            MachineTypeAarch64::Microbit => {
                cmd.push("microbit".to_string());
            }
            MachineTypeAarch64::Midway => {
                cmd.push("midway".to_string());
            }
            MachineTypeAarch64::Moribmc => {
                cmd.push("mori-bmc".to_string());
            }
            MachineTypeAarch64::Mps2an385 => {
                cmd.push("mps2-an385".to_string());
            }
            MachineTypeAarch64::Mps2an386 => {
                cmd.push("mps2-an386".to_string());
            }
            MachineTypeAarch64::Mps2an500 => {
                cmd.push("mps2-an500".to_string());
            }
            MachineTypeAarch64::Mps2an505 => {
                cmd.push("mps2-an505".to_string());
            }
            MachineTypeAarch64::Mps2an511 => {
                cmd.push("mps2-an511".to_string());
            }
            MachineTypeAarch64::Mps2an521 => {
                cmd.push("mps2-an521".to_string());
            }
            MachineTypeAarch64::Mps3an524 => {
                cmd.push("mps3-an524".to_string());
            }
            MachineTypeAarch64::Mps3an536 => {
                cmd.push("mps3-an536".to_string());
            }
            MachineTypeAarch64::Mps3an547 => {
                cmd.push("mps3-an547".to_string());
            }
            MachineTypeAarch64::Muscaa => {
                cmd.push("musca-a".to_string());
            }
            MachineTypeAarch64::Muscab1 => {
                cmd.push("musca-b1".to_string());
            }
            MachineTypeAarch64::Musicpal => {
                cmd.push("musicpal".to_string());
            }
            MachineTypeAarch64::Netduino2 => {
                cmd.push("netduino2".to_string());
            }
            MachineTypeAarch64::Netduinoplus2 => {
                cmd.push("netduinoplus2".to_string());
            }
            MachineTypeAarch64::None => {
                cmd.push("none".to_string());
            }
            MachineTypeAarch64::Npcm750evb => {
                cmd.push("npcm750-evb".to_string());
            }
            MachineTypeAarch64::Npcm845evb => {
                cmd.push("npcm845-evb".to_string());
            }
            MachineTypeAarch64::Nuri => {
                cmd.push("nuri".to_string());
            }
            MachineTypeAarch64::Olimexstm32h405 => {
                cmd.push("olimex-stm32-h405".to_string());
            }
            MachineTypeAarch64::Orangepipc => {
                cmd.push("orangepi-pc".to_string());
            }
            MachineTypeAarch64::Palmettobmc => {
                cmd.push("palmetto-bmc".to_string());
            }
            MachineTypeAarch64::QcomdcscmV1bmc => {
                cmd.push("qcom-dc-scm-v1-bmc".to_string());
            }
            MachineTypeAarch64::Qcomfireworkbmc => {
                cmd.push("qcom-firework-bmc".to_string());
            }
            MachineTypeAarch64::Quantagbsbmc => {
                cmd.push("quanta-gbs-bmc".to_string());
            }
            MachineTypeAarch64::Quantagsj => {
                cmd.push("quanta-gsj".to_string());
            }
            MachineTypeAarch64::Quantaq71lbmc => {
                cmd.push("quanta-q71l-bmc".to_string());
            }
            MachineTypeAarch64::Rainierbmc => {
                cmd.push("rainier-bmc".to_string());
            }
            MachineTypeAarch64::Raspi0 => {
                cmd.push("raspi0".to_string());
            }
            MachineTypeAarch64::Raspi1ap => {
                cmd.push("raspi1ap".to_string());
            }
            MachineTypeAarch64::Raspi2b => {
                cmd.push("raspi2b".to_string());
            }
            MachineTypeAarch64::Raspi3ap => {
                cmd.push("raspi3ap".to_string());
            }
            MachineTypeAarch64::Raspi3b => {
                cmd.push("raspi3b".to_string());
            }
            MachineTypeAarch64::Raspi4b => {
                cmd.push("raspi4b".to_string());
            }
            MachineTypeAarch64::Realvieweb => {
                cmd.push("realview-eb".to_string());
            }
            MachineTypeAarch64::Realviewebmpcore => {
                cmd.push("realview-eb-mpcore".to_string());
            }
            MachineTypeAarch64::Realviewpba8 => {
                cmd.push("realview-pb-a8".to_string());
            }
            MachineTypeAarch64::Realviewpbxa9 => {
                cmd.push("realview-pbx-a9".to_string());
            }
            MachineTypeAarch64::Romulusbmc => {
                cmd.push("romulus-bmc".to_string());
            }
            MachineTypeAarch64::Sabrelite => {
                cmd.push("sabrelite".to_string());
            }
            MachineTypeAarch64::Sbsaref => {
                cmd.push("sbsa-ref".to_string());
            }
            MachineTypeAarch64::Smdkc210 => {
                cmd.push("smdkc210".to_string());
            }
            MachineTypeAarch64::Sonorapassbmc => {
                cmd.push("sonorapass-bmc".to_string());
            }
            MachineTypeAarch64::Stm32vldiscovery => {
                cmd.push("stm32vldiscovery".to_string());
            }
            MachineTypeAarch64::Supermicrox11spibmc => {
                cmd.push("supermicro-x11spi-bmc".to_string());
            }
            MachineTypeAarch64::Supermicrox11bmc => {
                cmd.push("supermicrox11-bmc".to_string());
            }
            MachineTypeAarch64::Sx1 => {
                cmd.push("sx1".to_string());
            }
            MachineTypeAarch64::Sx1V1 => {
                cmd.push("sx1-v1".to_string());
            }
            MachineTypeAarch64::Tiogapassbmc => {
                cmd.push("tiogapass-bmc".to_string());
            }
            MachineTypeAarch64::Versatileab => {
                cmd.push("versatileab".to_string());
            }
            MachineTypeAarch64::Versatilepb => {
                cmd.push("versatilepb".to_string());
            }
            MachineTypeAarch64::Vexpressa15 => {
                cmd.push("vexpress-a15".to_string());
            }
            MachineTypeAarch64::Vexpressa9 => {
                cmd.push("vexpress-a9".to_string());
            }
            MachineTypeAarch64::Virt => {
                cmd.push("virt".to_string());
            }
            MachineTypeAarch64::Virt10_0 => {
                cmd.push("virt-10.0".to_string());
            }
            MachineTypeAarch64::Virt7_1 => {
                cmd.push("virt-7.1".to_string());
            }
            MachineTypeAarch64::Virt7_2 => {
                cmd.push("virt-7.2".to_string());
            }
            MachineTypeAarch64::Virt8_0 => {
                cmd.push("virt-8.0".to_string());
            }
            MachineTypeAarch64::Virt8_1 => {
                cmd.push("virt-8.1".to_string());
            }
            MachineTypeAarch64::Virt8_2 => {
                cmd.push("virt-8.2".to_string());
            }
            MachineTypeAarch64::Virt9_0 => {
                cmd.push("virt-9.0".to_string());
            }
            MachineTypeAarch64::Virt9_1 => {
                cmd.push("virt-9.1".to_string());
            }
            MachineTypeAarch64::Virt9_2 => {
                cmd.push("virt-9.2".to_string());
            }
            MachineTypeAarch64::Witherspoonbmc => {
                cmd.push("witherspoon-bmc".to_string());
            }
            MachineTypeAarch64::Xilinxzynqa9 => {
                cmd.push("xilinx-zynq-a9".to_string());
            }
            MachineTypeAarch64::XlnxVersalvirt => {
                cmd.push("xlnx-versal-virt".to_string());
            }
            MachineTypeAarch64::Xlnxzcu102 => {
                cmd.push("xlnx-zcu102".to_string());
            }
            MachineTypeAarch64::Yosemitev2bmc => {
                cmd.push("yosemitev2-bmc".to_string());
            }
        }
        cmd
    }
}

impl ToArg for MachineTypeAarch64 {
    fn to_arg(&self) -> &str {
        match self {
            MachineTypeAarch64::Ast1030evb => "ast1030-evb",
            MachineTypeAarch64::Ast2500evb => "ast2500-evb",
            MachineTypeAarch64::Ast2600evb => "ast2600-evb",
            MachineTypeAarch64::Ast2700evb => "ast2700-evb",
            MachineTypeAarch64::Ast2700a0evb => "ast2700a0-evb",
            MachineTypeAarch64::Ast2700a1evb => "ast2700a1-evb",
            MachineTypeAarch64::Bl475eiot01a => "b-l475e-iot01a",
            MachineTypeAarch64::Bletchleybmc => "bletchley-bmc",
            MachineTypeAarch64::Bpim2u => "bpim2u",
            MachineTypeAarch64::Canona1100 => "canon-a1100",
            MachineTypeAarch64::Collie => "collie",
            MachineTypeAarch64::Cubieboard => "cubieboard",
            MachineTypeAarch64::Emcraftsf2 => "emcraft-sf2",
            MachineTypeAarch64::Fby35bmc => "fby35-bmc",
            MachineTypeAarch64::Fby35 => "fby35",
            MachineTypeAarch64::Fp5280g2bmc => "fp5280g2-bmc",
            MachineTypeAarch64::Fujibmc => "fuji-bmc",
            MachineTypeAarch64::G220abmc => "g220a-bmc",
            MachineTypeAarch64::Highbank => "highbank",
            MachineTypeAarch64::Imx25pdk => "imx25-pdk",
            MachineTypeAarch64::Imx8mpevk => "imx8mp-evk",
            MachineTypeAarch64::Integratorcp => "integratorcp",
            MachineTypeAarch64::Kudobmc => "kudo-bmc",
            MachineTypeAarch64::Kzm => "kzm",
            MachineTypeAarch64::Lm3s6965evb => "lm3s6965evb",
            MachineTypeAarch64::Lm3s811evb => "lm3s811evb",
            MachineTypeAarch64::Mcimx6ulevk => "mcimx6ul-evk",
            MachineTypeAarch64::Mcimx7dsabre => "mcimx7d-sabre",
            MachineTypeAarch64::Microbit => "microbit",
            MachineTypeAarch64::Midway => "midway",
            MachineTypeAarch64::Moribmc => "mori-bmc",
            MachineTypeAarch64::Mps2an385 => "mps2-an385",
            MachineTypeAarch64::Mps2an386 => "mps2-an386",
            MachineTypeAarch64::Mps2an500 => "mps2-an500",
            MachineTypeAarch64::Mps2an505 => "mps2-an505",
            MachineTypeAarch64::Mps2an511 => "mps2-an511",
            MachineTypeAarch64::Mps2an521 => "mps2-an521",
            MachineTypeAarch64::Mps3an524 => "mps3-an524",
            MachineTypeAarch64::Mps3an536 => "mps3-an536",
            MachineTypeAarch64::Mps3an547 => "mps3-an547",
            MachineTypeAarch64::Muscaa => "musca-a",
            MachineTypeAarch64::Muscab1 => "musca-b1",
            MachineTypeAarch64::Musicpal => "musicpal",
            MachineTypeAarch64::Netduino2 => "netduino2",
            MachineTypeAarch64::Netduinoplus2 => "netduinoplus2",
            MachineTypeAarch64::None => "none",
            MachineTypeAarch64::Npcm750evb => "npcm750-evb",
            MachineTypeAarch64::Npcm845evb => "npcm845-evb",
            MachineTypeAarch64::Nuri => "nuri",
            MachineTypeAarch64::Olimexstm32h405 => "olimex-stm32-h405",
            MachineTypeAarch64::Orangepipc => "orangepi-pc",
            MachineTypeAarch64::Palmettobmc => "palmetto-bmc",
            MachineTypeAarch64::QcomdcscmV1bmc => "qcom-dc-scm-v1-bmc",
            MachineTypeAarch64::Qcomfireworkbmc => "qcom-firework-bmc",
            MachineTypeAarch64::Quantagbsbmc => "quanta-gbs-bmc",
            MachineTypeAarch64::Quantagsj => "quanta-gsj",
            MachineTypeAarch64::Quantaq71lbmc => "quanta-q71l-bmc",
            MachineTypeAarch64::Rainierbmc => "rainier-bmc",
            MachineTypeAarch64::Raspi0 => "raspi0",
            MachineTypeAarch64::Raspi1ap => "raspi1ap",
            MachineTypeAarch64::Raspi2b => "raspi2b",
            MachineTypeAarch64::Raspi3ap => "raspi3ap",
            MachineTypeAarch64::Raspi3b => "raspi3b",
            MachineTypeAarch64::Raspi4b => "raspi4b",
            MachineTypeAarch64::Realvieweb => "realview-eb",
            MachineTypeAarch64::Realviewebmpcore => "realview-eb-mpcore",
            MachineTypeAarch64::Realviewpba8 => "realview-pb-a8",
            MachineTypeAarch64::Realviewpbxa9 => "realview-pbx-a9",
            MachineTypeAarch64::Romulusbmc => "romulus-bmc",
            MachineTypeAarch64::Sabrelite => "sabrelite",
            MachineTypeAarch64::Sbsaref => "sbsa-ref",
            MachineTypeAarch64::Smdkc210 => "smdkc210",
            MachineTypeAarch64::Sonorapassbmc => "sonorapass-bmc",
            MachineTypeAarch64::Stm32vldiscovery => "stm32vldiscovery",
            MachineTypeAarch64::Supermicrox11spibmc => "supermicro-x11spi-bmc",
            MachineTypeAarch64::Supermicrox11bmc => "supermicrox11-bmc",
            MachineTypeAarch64::Sx1 => "sx1",
            MachineTypeAarch64::Sx1V1 => "sx1-v1",
            MachineTypeAarch64::Tiogapassbmc => "tiogapass-bmc",
            MachineTypeAarch64::Versatileab => "versatileab",
            MachineTypeAarch64::Versatilepb => "versatilepb",
            MachineTypeAarch64::Vexpressa15 => "vexpress-a15",
            MachineTypeAarch64::Vexpressa9 => "vexpress-a9",
            MachineTypeAarch64::Virt => "virt",
            MachineTypeAarch64::Virt10_0 => "virt-10.0",
            MachineTypeAarch64::Virt7_1 => "virt-7.1",
            MachineTypeAarch64::Virt7_2 => "virt-7.2",
            MachineTypeAarch64::Virt8_0 => "virt-8.0",
            MachineTypeAarch64::Virt8_1 => "virt-8.1",
            MachineTypeAarch64::Virt8_2 => "virt-8.2",
            MachineTypeAarch64::Virt9_0 => "virt-9.0",
            MachineTypeAarch64::Virt9_1 => "virt-9.1",
            MachineTypeAarch64::Virt9_2 => "virt-9.2",
            MachineTypeAarch64::Witherspoonbmc => "witherspoon-bmc",
            MachineTypeAarch64::Xilinxzynqa9 => "xilinx-zynq-a9",
            MachineTypeAarch64::XlnxVersalvirt => "xlnx-versal-virt",
            MachineTypeAarch64::Xlnxzcu102 => "xlnx-zcu102",
            MachineTypeAarch64::Yosemitev2bmc => "yosemitev2-bmc",
        }
    }
}

impl FromStr for MachineTypeAarch64 {
    type Err = ();

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
