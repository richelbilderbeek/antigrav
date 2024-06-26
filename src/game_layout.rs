// Uses the standard mathematics system:
//
//      | positive y
//      |
//      |
// -----+----- positive x
//      |
//      |
//      | negative y
//
// The origin (0,0) is at the center of the screen

pub struct GameLayout {
    pub paddle_y: f32,
    pub left_wall_x: f32,
    pub right_wall_x: f32,
    pub top_wall_y: f32,
    pub bottom_wall_y: f32,
    pub wall_thickness: f32,
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
    //const WALL_THICKNESS: f32 = 10.0;
    const GAP_BETWEEN_PADDLE_AND_FLOOR: f32 = 60.0;

    let left_wall_x = -450.0;
    let right_wall_x = 450.0;

    let top_wall_y = 300.0;
    let bottom_wall_y = -300.0;

    let paddle_y = bottom_wall_y + GAP_BETWEEN_PADDLE_AND_FLOOR;

    let layout = GameLayout {
        paddle_y,
        left_wall_x,
        right_wall_x,
        top_wall_y,
        bottom_wall_y,
        wall_thickness: 10.0,
    };
    return layout;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_layout() {
        let layout = create_initial_layout();
        assert!(layout.bottom_wall_y < 0.0);
        assert!(layout.top_wall_y > 0.0);
        assert!(layout.paddle_y < layout.top_wall_y);
        assert!(layout.paddle_y > layout.bottom_wall_y);
        assert_eq!(layout.wall_thickness, 10.0);
        assert!(layout.get_arena_height() > 0.0);
        assert!(layout.get_arena_width() > 0.0);
    }
}
