// The y coordinat is flipped:
// - a positive y coordinat: above the center
// - a negative y coordinat: below the center

pub struct GameLayout {
    pub paddle_y: f32,
    pub left_wall_x: f32,
    pub right_wall_x: f32,
    pub top_wall_y: f32,
    pub bottom_wall_y: f32,
}

impl GameLayout {
    pub fn get_arena_height(&self) -> f32 {
        return self.top_wall_y - self.bottom_wall_y;
    }
    pub fn get_arena_width(&self) -> f32 {
        return self.right_wall_x - self.left_wall_x;
    }
}

pub fn create_initial_layout() -> GameLayout {
    // x coordinates
    const LEFT_WALL: f32 = -450.;
    const RIGHT_WALL: f32 = 450.;
    // y coordinates
    const BOTTOM_WALL: f32 = -300.0;
    const TOP_WALL: f32 = 300.;

    const GAP_BETWEEN_PADDLE_AND_FLOOR: f32 = 60.0;

    let paddle_y = BOTTOM_WALL + GAP_BETWEEN_PADDLE_AND_FLOOR;
    let left_wall_x = LEFT_WALL;
    let right_wall_x = RIGHT_WALL;

    let top_wall_y = TOP_WALL;
    let bottom_wall_y = BOTTOM_WALL;

    let layout = GameLayout {
        paddle_y,
        left_wall_x,
        right_wall_x,
        top_wall_y,
        bottom_wall_y,
    };
    return layout;
}

fn get_init_paddle_y() -> f32 {
    return create_initial_layout().paddle_y;
}

fn get_init_bottom_wall_y() -> f32 {
    return create_initial_layout().bottom_wall_y;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bottom_wall_y() {
        assert_eq!(get_init_bottom_wall_y(), -300.0);
    }

    #[test]
    fn test_initial_layout() {
        let layout = create_initial_layout();
        assert_eq!(get_init_paddle_y(), layout.paddle_y);
        assert!(layout.bottom_wall_y < 0.0);
        assert!(layout.top_wall_y > 0.0);
        assert!(layout.get_arena_height() > 0.0);
        assert!(layout.get_arena_width() > 0.0);
    }
}
