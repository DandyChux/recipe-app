use serde::{ Deserialize, Serialize };
use chrono::prelude::*;
use validator::Validate;

#[derive(PartialEq, Eq, Clone, Default, Debug)]
pub struct SelectItem {
    pub label: String,
    pub value: String,
}

impl SelectItem {
    pub fn new(label: &str, value: &str) -> Self {
        Self {
            label: label.to_string(),
            value: value.to_string(),
        }
    }
}