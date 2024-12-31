use crate::{expressions::LiteralValue, token::Token};
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct Environment {
    values: HashMap<String, LiteralValue>,
}

impl Environment {
    pub fn define(&mut self, name: String, value: LiteralValue) {
        self.values.insert(name, value);
    }
    pub fn get(&self, name: &Token) -> Result<&LiteralValue, &'static str> {
        self.values.get(&name.lexeme).ok_or("undefined variabel")
    }
}
