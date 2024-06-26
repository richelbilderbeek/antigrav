use crate::game_layout::create_initial_layout;
use bevy::prelude::*;

/// Which side of the arena is this wall located on?
pub enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

impl WallLocation {
    pub fn position(&self) -> Vec2 {
        let layout = create_initial_layout();
        match self {
            WallLocation::Left => Vec2::new(layout.left_wall_x, 0.),
            WallLocation::Right => Vec2::new(layout.right_wall_x, 0.),
            WallLocation::Bottom => Vec2::new(0., layout.bottom_wall_y),
            WallLocation::Top => Vec2::new(0., layout.top_wall_y),
        }
    }

    pub fn size(&self) -> Vec2 {
        let layout = create_initial_layout();
        let arena_height = layout.get_arena_height();
        let arena_width = layout.get_arena_width();

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(layout.wall_thickness, arena_height + layout.wall_thickness)
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(arena_width + layout.wall_thickness, layout.wall_thickness)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sizes() {
        assert_eq!(WallLocation::Left.size(), WallLocation::Right.size());
        assert_eq!(WallLocation::Top.size(), WallLocation::Bottom.size());
    }

    #[test]
    fn test_positions() {
        assert!(WallLocation::Left.position().x < WallLocation::Right.position().x);
        assert!(WallLocation::Top.position().y > WallLocation::Bottom.position().y);
    }
}
