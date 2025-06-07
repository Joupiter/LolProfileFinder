use crate::models::{
    region::Region,
    rank::Rank,
};

#[derive(PartialEq, Clone)]
pub struct ProfileAccount {
    pub name: String,
    pub tag: String,
    pub region: Region,
    pub rank: Rank,
    pub level: u16,
}

impl ProfileAccount {
    pub fn get_full_name(&self) -> String {
        format!("{}#{}", self.name, self.tag)
    }
}