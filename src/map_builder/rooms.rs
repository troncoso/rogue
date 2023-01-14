use super::MapArchitect;
use crate::prelude::*;

pub struct RoomsArchitect {}

impl MapArchitect for RoomsArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            amulet_pos: Point::zero(),
            theme: super::themes::ForestTheme::new(),
        };
        mb.fill(TileType::Wall);
        mb.build_random_rooms(rng);
        mb.build_corridors(rng);
        mb.build_corridors(rng);

        mb.player_start = mb.rooms[0].center();
        mb.monster_spawns = mb.spawn_monsters(&mb.rooms[0].center(), rng);
        mb.amulet_pos = mb.find_most_distant();
        mb
    }
}
