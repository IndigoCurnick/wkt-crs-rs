use chrono::{Date, NaiveDateTime};

use crate::ast::WktNode;

pub enum DateOrString {
    Date(NaiveDateTime),
    String(String),
}

pub struct TemporalExtent {
    pub from: DateOrString,
    pub to: DateOrString,
}

impl TryFrom<&WktNode> for TemporalExtent {
    type Error;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        todo!()
    }
}
