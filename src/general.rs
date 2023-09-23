pub const CCBUILD_TIME: u64 = 1000;
pub const BARRACKS_BUILD_TIME: u64 = 1000;
pub const SHOW_TIER1_BUILDINGS_INDEX: i32 = 0;
pub const SHOW_TIER2_BUILDINGS_INDEX: i32 = 1;
pub const PLACE_CONSTRUCTIN_INDEX: i32 = 6;
pub const BARRACKS_INDEX: i32 = 4;
pub const COMMAND_CENTRE_INDEX: i32 = 3;
pub const WORKER_INDEX: i32 = 5;
pub const BACK_INDEX: i32 = 2;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Collidable {
    GroundCollidable,
    GroundUncollidable,
    AirCollidable,
    AirUncollidable,
    AbsoluteCollidable,
    World,
    UI,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Faction {
    PlaceholderFaction1,
}

