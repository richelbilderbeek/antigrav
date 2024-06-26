const BOTTOM_WALL: f32 = -300.0;
const GAP_BETWEEN_PADDLE_AND_FLOOR: f32 = 60.0;

//struct InitialLayout {
//
//}

pub fn get_init_paddle_y() -> f32 {
    return BOTTOM_WALL + GAP_BETWEEN_PADDLE_AND_FLOOR;
}

pub fn get_init_bottom_wall_y() -> f32 {
    return BOTTOM_WALL;
}

