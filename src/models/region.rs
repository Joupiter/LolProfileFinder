use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Clone, Debug)]
pub enum Region {
    NA,
    EUW,
    EUN,
    KR,
    JP,
    BR,
    TR,
    TW,
    VN
}

const REGIONS: &'static [Region; 9] = &[
    Region::NA,
    Region::EUW,
    Region::EUN,
    Region::KR,
    Region::JP,
    Region::BR,
    Region::TR,
    Region::TW,
    Region::VN];

impl Display for Region {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Region {
    pub fn get_region_name(&self) -> &'static str {
        match self {
            Region::NA => "North America",
            Region::EUW => "Europe West",
            Region::EUN => "Europe Nordic & East",
            Region::KR => "South Korea",
            Region::JP => "Japan",
            Region::BR => "Brazil",
            Region::TR => "Turkey",
            Region::TW => "Taiwan",
            Region::VN => "Vietnam",
        }
    }

    pub fn get_color(&self) -> &'static str {
        match self {
            Region::NA => "#1E90FF",
            Region::EUW => "#32CD32",
            Region::EUN => "#8A2BE2",
            Region::KR => "#DC143C",
            Region::JP => "#FF69B4",
            Region::BR => "#228B22",
            Region::TR => "#FF8C00",
            Region::TW => "#20B2AA",
            Region::VN => "#FFD700",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "NA" => Some(Region::NA),
            "EUW" => Some(Region::EUW),
            "EUN" => Some(Region::EUN),
            "KR" => Some(Region::KR),
            "JP" => Some(Region::JP),
            "BR" => Some(Region::BR),
            "TR" => Some(Region::TR),
            "TW" => Some(Region::TW),
            "VN" => Some(Region::VN),
            _ => None,
        }
    }

    pub fn get_all() -> &'static [Region; 9] {
        REGIONS
    }
}