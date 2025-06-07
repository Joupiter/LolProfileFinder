use std::fmt::{Display, Formatter};
use crate::app;

#[derive(Debug, PartialEq, Clone)]
pub enum Rank {
    CHALLENGER,
    GRANDMASTER,
    MASTER,
    DIAMOND,
    PLATINUM,
    GOLD,
    SILVER,
    BRONZE,
    IRON,
    UNRANKED,
}

impl Display for Rank {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Rank {
    pub fn get_icon_path(&self) -> String {
        format!("{}/Emblem_{}.png", app::RANK_ICON_FOLDER, self)
    }

    pub fn get_color(&self) -> &'static str {
        match self {
            Rank::CHALLENGER => "#FFD700",
            Rank::GRANDMASTER => "#FF3030",
            Rank::MASTER => "#BA55D3",
            Rank::DIAMOND => "#00BFFF",
            Rank::PLATINUM => "#32CD32",
            Rank::GOLD => "#FFA500",
            Rank::SILVER => "#C0C0C0",
            Rank::BRONZE => "#CD7F32",
            Rank::IRON => "#696969",
            Rank::UNRANKED => "#808080",
        }
    }
}