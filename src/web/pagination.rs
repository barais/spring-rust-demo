use schemars::JsonSchema;
use serde::Deserialize;

fn default_usize() -> usize {
    1 as usize
}
fn default_pageusize() -> usize {
    50 as usize
}


#[derive(Deserialize, Debug,JsonSchema)]
pub struct Pagination {
    #[serde(default = "default_usize")]
    pub page: usize,
    #[serde(default = "default_pageusize")]
    pub per_page: usize,
}

impl Pagination {
    pub fn offset(&self) -> usize {
        (self.page - 1) * self.per_page
    }
    pub fn limit(&self) -> usize {
        self.per_page
    }
    
}
