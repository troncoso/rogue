use crate::prelude::*;

pub struct Player {
    pub pos: Point,
}
impl Player {
    pub fn new(position: Point) -> Self {
        Self { pos: position }
    }
    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        ctx.set_active_console(ENTITY_LAYER);
        ctx.set(
            self.pos.x - camera.left_x,
            self.pos.y - camera.top_y,
            WHITE,
            BLACK,
            to_cp437('@'),
        )
    }
    pub fn update(&mut self, ctx: &mut BTerm, map: &Map, camera: &mut Camera) {
        if let Some(key) = ctx.key {
            let delta = match key {
                VirtualKeyCode::A => Point::new(-1, 0),
                VirtualKeyCode::D => Point::new(1, 0),
                VirtualKeyCode::W => Point::new(0, -1),
                VirtualKeyCode::S => Point::new(0, 1),
                _ => Point::zero(),
            };
            let new_pos = self.pos + delta;
            if map.can_enter_tile(new_pos) {
                self.pos = new_pos;
                camera.on_player_move(new_pos);
            }
        }
    }
}
