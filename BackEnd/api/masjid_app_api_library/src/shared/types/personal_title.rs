use enum_stringify::EnumStringify;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, EnumStringify, PartialEq, Eq)]
pub enum PersonalTitle {
    Mr,
    Mrs,
    Ms,
}
