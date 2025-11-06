use std::path::PathBuf;

use bon::Builder;

use crate::common::OnOff;
use crate::to_command::{ToArg, ToCommand};

/// Load SMBIOS entry from binary file.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder)]
pub struct SmbiosFile {
    path: PathBuf,
}

impl ToCommand for SmbiosFile {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![];
        cmd.push("-smbios".to_string());
        cmd.push(format!("file={}", self.path.display()));
        cmd
    }
}

/// Specify SMBIOS type 0 fields
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder)]
pub struct SmbiosType0 {
    vendor: Option<String>,
    version: Option<String>,
    date: Option<String>,
    release: Option<(usize, usize)>,
    uefi: Option<OnOff>,
}

impl ToCommand for SmbiosType0 {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![];
        cmd.push("-smbios".to_string());
        let mut args = vec!["type=0".to_string()];
        if let Some(vendor) = &self.vendor {
            args.push(format!("vendor={}", vendor));
        }
        if let Some(version) = &self.version {
            args.push(format!("version={}", version));
        }
        if let Some(date) = &self.date {
            args.push(format!("date={}", date));
        }
        if let Some(release) = &self.release {
            args.push(format!("release={}.{}", release.0, release.1));
        }
        if let Some(uefi) = &self.uefi {
            args.push(format!("uefi={}", uefi.to_arg()));
        }
        cmd.push(args.join(","));
        cmd
    }
}

/// Specify SMBIOS type 1 fields
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder)]
pub struct SmbiosType1 {
    manufacturer: Option<String>,
    product: Option<String>,
    version: Option<String>,
    serial: Option<String>,
    uuid: Option<String>,
    sku: Option<String>,
    family: Option<String>,
}

impl ToCommand for SmbiosType1 {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![];
        cmd.push("-smbios".to_string());
        let mut args = vec!["type=1".to_string()];
        if let Some(manufacturer) = &self.manufacturer {
            args.push(format!("manufacturer={}", manufacturer));
        }
        if let Some(product) = &self.product {
            args.push(format!("product={}", product));
        }
        if let Some(version) = &self.version {
            args.push(format!("version={}", version));
        }
        if let Some(serial) = &self.serial {
            args.push(format!("serial={}", serial));
        }
        if let Some(uuid) = &self.uuid {
            args.push(format!("uuid={}", uuid));
        }
        if let Some(sku) = &self.sku {
            args.push(format!("sku={}", sku));
        }
        if let Some(family) = &self.family {
            args.push(format!("family={}", family));
        }
        cmd.push(args.join(","));
        cmd
    }
}

/// Specify SMBIOS type 2 fields
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder)]
pub struct SmbiosType2 {
    manufacturer: Option<String>,
    product: Option<String>,
    version: Option<String>,
    serial: Option<String>,
    asset: Option<String>,
    location: Option<String>,
}

impl ToCommand for SmbiosType2 {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![];
        cmd.push("-smbios".to_string());
        let mut args = vec!["type=2".to_string()];
        if let Some(manufacturer) = &self.manufacturer {
            args.push(format!("manufacturer={}", manufacturer));
        }
        if let Some(product) = &self.product {
            args.push(format!("product={}", product));
        }
        if let Some(version) = &self.version {
            args.push(format!("version={}", version));
        }
        if let Some(serial) = &self.serial {
            args.push(format!("serial={}", serial));
        }
        if let Some(asset) = &self.asset {
            args.push(format!("asset={}", asset));
        }
        if let Some(location) = &self.location {
            args.push(format!("location={}", location));
        }
        cmd.push(args.join(","));
        cmd
    }
}

/// Specify SMBIOS type 3 fields
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder)]
pub struct SmbiosType3 {
    manufacturer: Option<String>,
    version: Option<String>,
    serial: Option<String>,
    asset: Option<String>,
    sku: Option<String>,
}

impl ToCommand for SmbiosType3 {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![];
        cmd.push("-smbios".to_string());
        let mut args = vec!["type=3".to_string()];
        if let Some(manufacturer) = &self.manufacturer {
            args.push(format!("manufacturer={}", manufacturer));
        }
        if let Some(version) = &self.version {
            args.push(format!("version={}", version));
        }
        if let Some(serial) = &self.serial {
            args.push(format!("serial={}", serial));
        }
        if let Some(asset) = &self.asset {
            args.push(format!("asset={}", asset));
        }
        if let Some(sku) = &self.sku {
            args.push(format!("sku={}", sku));
        }
        cmd.push(args.join(","));
        cmd
    }
}
/// Specify SMBIOS type 4 fields
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder)]
pub struct SmbiosType4 {
    sock_pfx: Option<String>,
    manufacturer: Option<String>,
    version: Option<String>,
    serial: Option<String>,
    asset: Option<String>,
    part: Option<String>,
    max_speed: Option<usize>,
    current_speed: Option<usize>,
    processor_family: Option<usize>,
    processor_id: Option<usize>,
}

impl ToCommand for SmbiosType4 {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![];
        cmd.push("-smbios".to_string());
        let mut args = vec!["type=4".to_string()];
        if let Some(sock_pfx) = &self.sock_pfx {
            args.push(format!("sock_pfx={}", sock_pfx));
        }
        if let Some(manufacturer) = &self.manufacturer {
            args.push(format!("manufacturer={}", manufacturer));
        }
        if let Some(version) = &self.version {
            args.push(format!("version={}", version));
        }
        if let Some(serial) = &self.serial {
            args.push(format!("serial={}", serial));
        }
        if let Some(asset) = &self.asset {
            args.push(format!("asset={}", asset));
        }
        if let Some(part) = &self.part {
            args.push(format!("part={}", part));
        }
        if let Some(max_speed) = &self.max_speed {
            args.push(format!("max-speed={}", max_speed));
        }
        if let Some(current_speed) = &self.current_speed {
            args.push(format!("current-speed={}", current_speed));
        }
        if let Some(processor_family) = &self.processor_family {
            args.push(format!("processor-family={}", processor_family));
        }
        if let Some(processor_id) = &self.processor_id {
            args.push(format!("processor-id={}", processor_id));
        }
        cmd.push(args.join(","));
        cmd
    }
}

/// Specify SMBIOS type 8 fields
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder)]
pub struct SmbiosType8 {
    external_reference: Option<String>,
    internal_reference: Option<String>,
    connector_type: Option<usize>,
    port_type: Option<usize>,
}

impl ToCommand for SmbiosType8 {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![];
        cmd.push("-smbios".to_string());
        let mut args = vec!["type=8".to_string()];
        if let Some(external_reference) = &self.external_reference {
            args.push(format!("external_reference={}", external_reference));
        }
        if let Some(internal_reference) = &self.internal_reference {
            args.push(format!("internal_reference={}", internal_reference));
        }
        if let Some(connector_type) = &self.connector_type {
            args.push(format!("connector_type={}", connector_type));
        }
        if let Some(port_type) = &self.port_type {
            args.push(format!("port_type={}", port_type));
        }
        cmd.push(args.join(","));
        cmd
    }
}

/// Specify SMBIOS type 11 fields
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder)]
pub struct SmbiosType11 {
    value: Option<String>,
    path: Option<String>,
}

impl ToCommand for SmbiosType11 {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![];
        cmd.push("-smbios".to_string());
        let mut args = vec!["type=11".to_string()];
        if let Some(value) = &self.value {
            args.push(format!("value={}", value));
        }
        if let Some(path) = &self.path {
            args.push(format!("path={}", path));
        }
        cmd.push(args.join(","));
        cmd
    }
}

/// Specify SMBIOS type 17 fields
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder)]
pub struct SmbiosType17 {
    loc_pfx: Option<String>,
    bank: Option<String>,
    manufacturer: Option<String>,
    serial: Option<String>,
    asset: Option<String>,
    part: Option<String>,
    speed: Option<usize>,
}

impl ToCommand for SmbiosType17 {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![];
        cmd.push("-smbios".to_string());
        let mut args = vec!["type=17".to_string()];

        if let Some(loc_pfx) = &self.loc_pfx {
            args.push(format!("loc_pfx={}", loc_pfx));
        }
        if let Some(bank) = &self.bank {
            args.push(format!("bank={}", bank));
        }
        if let Some(manufacturer) = &self.manufacturer {
            args.push(format!("manufacturer={}", manufacturer));
        }
        if let Some(serial) = &self.serial {
            args.push(format!("serial={}", serial));
        }
        if let Some(asset) = &self.asset {
            args.push(format!("asset={}", asset));
        }
        if let Some(part) = &self.part {
            args.push(format!("part={}", part));
        }
        if let Some(speed) = &self.speed {
            args.push(format!("speed={}", speed));
        }
        cmd.push(args.join(","));
        cmd
    }
}

/// Specify SMBIOS type 41 fields
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder)]
pub struct SmbiosType41 {
    designation: Option<String>,
    kind: Option<String>,
    instance: Option<usize>,
    pcidev: Option<String>,
}

impl ToCommand for SmbiosType41 {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![];
        cmd.push("-smbios".to_string());
        let mut args = vec!["type=41".to_string()];

        if let Some(designation) = &self.designation {
            args.push(format!("designation={}", designation));
        }
        if let Some(kind) = &self.kind {
            args.push(format!("kind={}", kind));
        }
        if let Some(instance) = self.instance {
            args.push(format!("instance={}", instance));
        }
        if let Some(pcidev) = &self.pcidev {
            args.push(format!("pcidev={}", pcidev));
        }
        cmd.push(args.join(","));
        cmd
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum Smbios {
    File(SmbiosFile),
    Type0(SmbiosType0),
    Type1(SmbiosType1),
    Type2(SmbiosType2),
    Type3(SmbiosType3),
    Type4(SmbiosType4),
    Type8(SmbiosType8),
    Type11(SmbiosType11),
    Type41(SmbiosType41),
}

impl ToCommand for Smbios {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![];
        match self {
            Smbios::File(file) => {
                cmd.append(file.to_command().as_mut());
            }
            Smbios::Type0(type0) => {
                cmd.append(type0.to_command().as_mut());
            }
            Smbios::Type1(type1) => {
                cmd.append(type1.to_command().as_mut());
            }
            Smbios::Type2(type2) => {
                cmd.append(type2.to_command().as_mut());
            }
            Smbios::Type3(type3) => {
                cmd.append(type3.to_command().as_mut());
            }
            Smbios::Type4(type4) => {
                cmd.append(type4.to_command().as_mut());
            }
            Smbios::Type8(type8) => {
                cmd.append(type8.to_command().as_mut());
            }
            Smbios::Type11(type11) => {
                cmd.append(type11.to_command().as_mut());
            }
            Smbios::Type41(type41) => {
                cmd.append(type41.to_command().as_mut());
            }
        }
        cmd
    }
}
