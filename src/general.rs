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

