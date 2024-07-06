use raylib::{drawing::RaylibDrawHandle, math::Vector2};

#[derive(Debug)]
struct Player<'a> {
    position: &'a mut Vector2,
    state: PlayerState,
    movement_speed: f32,
}

#[derive(Debug)]
struct PlayerState {
    destroyed: bool,
    damaged: bool,
    locking_down: bool,
    moving: bool,
}

pub fn render_player(d: &mut RaylibDrawHandle, player: &mut Player) {
    todo!();
}
