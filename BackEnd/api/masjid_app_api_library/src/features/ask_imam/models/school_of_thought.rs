use enum_stringify::EnumStringify;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, EnumStringify, Clone, Copy, Eq, PartialEq)]
pub enum SchoolOfThought {
    Hanafi,
    Shaafi,
    Maliki,
    Hanbali,
}
