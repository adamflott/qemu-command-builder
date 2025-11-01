use bon::Builder;

use crate::to_command::ToCommand;

#[derive(Default, Builder)]
pub struct Object {
    typename: String,
    properties: Vec<(String, String)>,
}

impl Object {
    pub fn new<S: AsRef<str>>(typename: S) -> Self {
        Object {
            typename: typename.as_ref().to_string(),
            properties: Default::default(),
        }
    }
    pub fn add_prop<S: AsRef<str>>(&mut self, key: S, value: S) -> &mut Self {
        self.properties
            .push((key.as_ref().to_string(), value.as_ref().to_string()));
        self
    }
}
impl ToCommand for Object {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![];

        cmd.push("-object".to_string());
        let mut args = vec![self.typename.clone()];

        for (prop_key, prop_value) in &self.properties {
            args.push(format!("{}={}", prop_key, prop_value));
        }
        cmd.push(args.join(","));

        cmd
    }
}
