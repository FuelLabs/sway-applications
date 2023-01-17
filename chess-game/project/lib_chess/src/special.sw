library special;

pub enum CastleRights {
    NoRights: (),
    KingSide: (),
    QueenSide: (),
    Both: (),
}

impl core::ops::Eq for CastleRights {
    fn eq(self, other: CastleRights) -> bool {
        match (self, other) {
            (CastleRights::NoRights, CastleRights::NoRights) => true,
            (CastleRights::KingSide, CastleRights::KingSide) => true,
            (CastleRights::QueenSide, CastleRights::QueenSide) => true,
            (CastleRights::Both, CastleRights::Both) => true,
            _ => false,
        }
    }
}