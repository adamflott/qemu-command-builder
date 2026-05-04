use crate::parsers::ARG_SMBIOS;
use bon::Builder;
use proptest_derive::Arbitrary;
use std::path::PathBuf;
use std::str::FromStr;

use crate::common::OnOff;
use crate::parsers::DELIM_COMMA;
use crate::to_command::{ToArg, ToCommand};

/// Load SMBIOS entry from binary file.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct SmbiosFile {
    path: PathBuf,
}

impl ToCommand for SmbiosFile {
    fn to_args(&self) -> Vec<String> {
        let mut args = vec![];
        args.push(format!("file={}", self.path.display()));
        args
    }
}

impl FromStr for SmbiosFile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let path = s.strip_prefix("file=").ok_or_else(|| format!("unsupported smbios file form: {s}"))?;
        Ok(Self { path: PathBuf::from(path) })
    }
}
/// Specify SMBIOS type 0 fields
///
/// Corresponds to QEMU `-smbios type=0[,vendor=str][,version=str][,date=str][,release=%d.%d][,uefi=on|off]`.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct SmbiosType0 {
    vendor: Option<String>,
    version: Option<String>,
    date: Option<String>,
    release: Option<(usize, usize)>,
    uefi: Option<OnOff>,
}

impl ToCommand for SmbiosType0 {
    fn to_args(&self) -> Vec<String> {
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
        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for SmbiosType0 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut value = Self::default();
        for part in s.split(',') {
            let (key, raw) = part.split_once('=').ok_or_else(|| format!("invalid smbios type 0 option: {part}"))?;
            match key {
                "type" if raw == "0" => {}
                "vendor" => value.vendor = Some(raw.to_string()),
                "version" => value.version = Some(raw.to_string()),
                "date" => value.date = Some(raw.to_string()),
                "release" => {
                    let (major, minor) = raw.split_once('.').ok_or_else(|| format!("invalid release value: {raw}"))?;
                    value.release = Some((major.parse::<usize>().map_err(|e| e.to_string())?, minor.parse::<usize>().map_err(|e| e.to_string())?));
                }
                "uefi" => value.uefi = Some(raw.parse::<OnOff>().map_err(|_| format!("invalid uefi value: {raw}"))?),
                other => return Err(format!("unsupported smbios type 0 option: {other}")),
            }
        }
        Ok(value)
    }
}
/// Specify SMBIOS type 1 fields
///
/// Corresponds to QEMU `-smbios type=1[,manufacturer=str][,product=str][,version=str][,serial=str][,uuid=uuid][,sku=str][,family=str]`.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
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
    fn to_args(&self) -> Vec<String> {
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
        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for SmbiosType1 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut value = Self::default();
        for part in s.split(',') {
            let (key, raw) = part.split_once('=').ok_or_else(|| format!("invalid smbios type 1 option: {part}"))?;
            match key {
                "type" if raw == "1" => {}
                "manufacturer" => value.manufacturer = Some(raw.to_string()),
                "product" => value.product = Some(raw.to_string()),
                "version" => value.version = Some(raw.to_string()),
                "serial" => value.serial = Some(raw.to_string()),
                "uuid" => value.uuid = Some(raw.to_string()),
                "sku" => value.sku = Some(raw.to_string()),
                "family" => value.family = Some(raw.to_string()),
                other => return Err(format!("unsupported smbios type 1 option: {other}")),
            }
        }
        Ok(value)
    }
}

/// Specify SMBIOS type 2 fields
///
/// Corresponds to QEMU `-smbios type=2[,manufacturer=str][,product=str][,version=str][,serial=str][,asset=str][,location=str]`.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct SmbiosType2 {
    manufacturer: Option<String>,
    product: Option<String>,
    version: Option<String>,
    serial: Option<String>,
    asset: Option<String>,
    location: Option<String>,
}

impl ToCommand for SmbiosType2 {
    fn to_args(&self) -> Vec<String> {
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
        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for SmbiosType2 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut value = Self::default();
        for part in s.split(',') {
            let (key, raw) = part.split_once('=').ok_or_else(|| format!("invalid smbios type 2 option: {part}"))?;
            match key {
                "type" if raw == "2" => {}
                "manufacturer" => value.manufacturer = Some(raw.to_string()),
                "product" => value.product = Some(raw.to_string()),
                "version" => value.version = Some(raw.to_string()),
                "serial" => value.serial = Some(raw.to_string()),
                "asset" => value.asset = Some(raw.to_string()),
                "location" => value.location = Some(raw.to_string()),
                other => return Err(format!("unsupported smbios type 2 option: {other}")),
            }
        }
        Ok(value)
    }
}

/// Specify SMBIOS type 3 fields
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct SmbiosType3 {
    manufacturer: Option<String>,
    version: Option<String>,
    serial: Option<String>,
    asset: Option<String>,
    sku: Option<String>,
}

impl ToCommand for SmbiosType3 {
    fn to_args(&self) -> Vec<String> {
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
        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for SmbiosType3 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut value = Self::default();
        for part in s.split(',') {
            let (key, raw) = part.split_once('=').ok_or_else(|| format!("invalid smbios type 3 option: {part}"))?;
            match key {
                "type" if raw == "3" => {}
                "manufacturer" => value.manufacturer = Some(raw.to_string()),
                "version" => value.version = Some(raw.to_string()),
                "serial" => value.serial = Some(raw.to_string()),
                "asset" => value.asset = Some(raw.to_string()),
                "sku" => value.sku = Some(raw.to_string()),
                other => return Err(format!("unsupported smbios type 3 option: {other}")),
            }
        }
        Ok(value)
    }
}

/// Specify SMBIOS type 4 fields
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
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
    fn to_args(&self) -> Vec<String> {
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
        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for SmbiosType4 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut value = Self::default();
        for part in s.split(',') {
            let (key, raw) = part.split_once('=').ok_or_else(|| format!("invalid smbios type 4 option: {part}"))?;
            match key {
                "type" if raw == "4" => {}
                "sock_pfx" => value.sock_pfx = Some(raw.to_string()),
                "manufacturer" => value.manufacturer = Some(raw.to_string()),
                "version" => value.version = Some(raw.to_string()),
                "serial" => value.serial = Some(raw.to_string()),
                "asset" => value.asset = Some(raw.to_string()),
                "part" => value.part = Some(raw.to_string()),
                "max-speed" => value.max_speed = Some(raw.parse::<usize>().map_err(|e| e.to_string())?),
                "current-speed" => value.current_speed = Some(raw.parse::<usize>().map_err(|e| e.to_string())?),
                "processor-family" => value.processor_family = Some(raw.parse::<usize>().map_err(|e| e.to_string())?),
                "processor-id" => value.processor_id = Some(raw.parse::<usize>().map_err(|e| e.to_string())?),
                other => return Err(format!("unsupported smbios type 4 option: {other}")),
            }
        }
        Ok(value)
    }
}

/// Specify SMBIOS type 8 fields
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct SmbiosType8 {
    external_reference: Option<String>,
    internal_reference: Option<String>,
    connector_type: Option<usize>,
    port_type: Option<usize>,
}

impl ToCommand for SmbiosType8 {
    fn to_args(&self) -> Vec<String> {
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
        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for SmbiosType8 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut value = Self::default();
        for part in s.split(',') {
            let (key, raw) = part.split_once('=').ok_or_else(|| format!("invalid smbios type 8 option: {part}"))?;
            match key {
                "type" if raw == "8" => {}
                "external_reference" => value.external_reference = Some(raw.to_string()),
                "internal_reference" => value.internal_reference = Some(raw.to_string()),
                "connector_type" => value.connector_type = Some(raw.parse::<usize>().map_err(|e| e.to_string())?),
                "port_type" => value.port_type = Some(raw.parse::<usize>().map_err(|e| e.to_string())?),
                other => return Err(format!("unsupported smbios type 8 option: {other}")),
            }
        }
        Ok(value)
    }
}

/// Specify SMBIOS type 11 fields
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct SmbiosType11 {
    value: Option<String>,
    path: Option<String>,
}

impl ToCommand for SmbiosType11 {
    fn to_args(&self) -> Vec<String> {
        let mut args = vec!["type=11".to_string()];
        if let Some(value) = &self.value {
            args.push(format!("value={}", value));
        }
        if let Some(path) = &self.path {
            args.push(format!("path={}", path));
        }
        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for SmbiosType11 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut value = Self::default();
        for part in s.split(',') {
            let (key, raw) = part.split_once('=').ok_or_else(|| format!("invalid smbios type 11 option: {part}"))?;
            match key {
                "type" if raw == "11" => {}
                "value" => value.value = Some(raw.to_string()),
                "path" => value.path = Some(raw.to_string()),
                other => return Err(format!("unsupported smbios type 11 option: {other}")),
            }
        }
        Ok(value)
    }
}

/// Specify SMBIOS type 17 fields
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
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
    fn to_args(&self) -> Vec<String> {
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
        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for SmbiosType17 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut value = Self::default();
        for part in s.split(',') {
            let (key, raw) = part.split_once('=').ok_or_else(|| format!("invalid smbios type 17 option: {part}"))?;
            match key {
                "type" if raw == "17" => {}
                "loc_pfx" => value.loc_pfx = Some(raw.to_string()),
                "bank" => value.bank = Some(raw.to_string()),
                "manufacturer" => value.manufacturer = Some(raw.to_string()),
                "serial" => value.serial = Some(raw.to_string()),
                "asset" => value.asset = Some(raw.to_string()),
                "part" => value.part = Some(raw.to_string()),
                "speed" => value.speed = Some(raw.parse::<usize>().map_err(|e| e.to_string())?),
                other => return Err(format!("unsupported smbios type 17 option: {other}")),
            }
        }
        Ok(value)
    }
}

/// Specify SMBIOS type 41 fields
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct SmbiosType41 {
    designation: Option<String>,
    kind: Option<String>,
    instance: Option<usize>,
    pcidev: Option<String>,
}

impl ToCommand for SmbiosType41 {
    fn to_args(&self) -> Vec<String> {
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
        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for SmbiosType41 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut value = Self::default();
        for part in s.split(',') {
            let (key, raw) = part.split_once('=').ok_or_else(|| format!("invalid smbios type 41 option: {part}"))?;
            match key {
                "type" if raw == "41" => {}
                "designation" => value.designation = Some(raw.to_string()),
                "kind" => value.kind = Some(raw.to_string()),
                "instance" => value.instance = Some(raw.parse::<usize>().map_err(|e| e.to_string())?),
                "pcidev" => value.pcidev = Some(raw.to_string()),
                other => return Err(format!("unsupported smbios type 41 option: {other}")),
            }
        }
        Ok(value)
    }
}

/// A supported `-smbios` payload.
///
/// The formatter emits the canonical QEMU comma-separated form for the
/// selected SMBIOS record type or `file=` source.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum Smbios {
    File(SmbiosFile),
    Type0(SmbiosType0),
    Type1(SmbiosType1),
    Type2(SmbiosType2),
    Type3(SmbiosType3),
    Type4(SmbiosType4),
    Type8(SmbiosType8),
    Type11(SmbiosType11),
    Type17(SmbiosType17),
    Type41(SmbiosType41),
}

impl ToCommand for Smbios {
    fn command(&self) -> String {
        ARG_SMBIOS.to_string()
    }
    fn to_args(&self) -> Vec<String> {
        match self {
            Smbios::File(file) => file.to_args(),
            Smbios::Type0(type0) => type0.to_args(),
            Smbios::Type1(type1) => type1.to_args(),
            Smbios::Type2(type2) => type2.to_args(),
            Smbios::Type3(type3) => type3.to_args(),
            Smbios::Type4(type4) => type4.to_args(),
            Smbios::Type8(type8) => type8.to_args(),
            Smbios::Type11(type11) => type11.to_args(),
            Smbios::Type17(type17) => type17.to_args(),
            Smbios::Type41(type41) => type41.to_args(),
        }
    }
}

impl FromStr for Smbios {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("file=") {
            return Ok(Self::File(s.parse::<SmbiosFile>()?));
        }
        if s.starts_with("type=41") {
            return Ok(Self::Type41(s.parse::<SmbiosType41>()?));
        }
        if s.starts_with("type=17") {
            return Ok(Self::Type17(s.parse::<SmbiosType17>()?));
        }
        if s.starts_with("type=11") {
            return Ok(Self::Type11(s.parse::<SmbiosType11>()?));
        }
        if s.starts_with("type=0") {
            return Ok(Self::Type0(s.parse::<SmbiosType0>()?));
        }
        if s.starts_with("type=1") {
            return Ok(Self::Type1(s.parse::<SmbiosType1>()?));
        }
        if s.starts_with("type=2") {
            return Ok(Self::Type2(s.parse::<SmbiosType2>()?));
        }
        if s.starts_with("type=3") {
            return Ok(Self::Type3(s.parse::<SmbiosType3>()?));
        }
        if s.starts_with("type=4") {
            return Ok(Self::Type4(s.parse::<SmbiosType4>()?));
        }
        if s.starts_with("type=8") {
            return Ok(Self::Type8(s.parse::<SmbiosType8>()?));
        }

        Err(format!("unsupported smbios argument: {s}"))
    }
}
