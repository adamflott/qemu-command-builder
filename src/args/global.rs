use std::str::FromStr;

use bon::Builder;
use proptest_derive::Arbitrary;

use crate::parsers::{ARG_GLOBAL, DELIM_COMMA};
use crate::to_command::ToCommand;

/// Set default value of driver's property prop to value
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct Global {
    /// The QOM device or driver name.
    driver: String,
    /// The property name.
    property: String,
    /// The assigned value.
    value: String,
}

impl ToCommand for Global {
    fn command(&self) -> String {
        ARG_GLOBAL.to_string()
    }
    fn to_args(&self) -> Vec<String> {
        vec![format!("driver={},property={},value={}", self.driver, self.property, self.value)]
    }
}

impl FromStr for Global {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((left, value)) = s.rsplit_once('=')
            && !left.contains(',')
            && let Some((driver, property)) = left.rsplit_once('.')
        {
            return Ok(Self {
                driver: driver.to_string(),
                property: property.to_string(),
                value: value.to_string(),
            });
        }

        let mut driver = None;
        let mut property = None;
        let mut value_field = None;
        for part in s.split(DELIM_COMMA) {
            let (key, value) = part.split_once('=').ok_or_else(|| format!("invalid -global option: {part}"))?;
            match key {
                "driver" => driver = Some(value.to_string()),
                "property" => property = Some(value.to_string()),
                "value" => value_field = Some(value.to_string()),
                other => return Err(format!("unsupported -global option: {other}")),
            }
        }
        Ok(Self {
            driver: driver.ok_or_else(|| "global requires driver=".to_string())?,
            property: property.ok_or_else(|| "global requires property=".to_string())?,
            value: value_field.ok_or_else(|| "global requires value=".to_string())?,
        })
    }
}
