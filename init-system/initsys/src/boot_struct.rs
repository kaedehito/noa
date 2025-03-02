use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize, Default)]
pub struct Noaboot {
    pub name: String,
    pub description: String,
    pub after: Option<String>,
    pub service: Service,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize, Default)]
pub struct Service {
    pub execstart: String,
    pub restart: bool,
}
