#[derive(Clone, Copy)]
pub enum Collidable {
    GroundCollidable,
    GroundUncollidable,
    AirCollidable,
    AirUncollidable,
    AbsoluteCollidable,
    World,
    UI,
}

#[derive(Clone, Copy)]
pub enum Faction {
    PlaceholderFaction1,
}

