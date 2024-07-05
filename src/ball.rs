use bevy::prelude::*;

#[derive(Component)]
pub struct Ball;

pub fn get_ball_color() -> Color {
    Color::rgb(1.0, 0.5, 0.5)
}

pub fn get_ball_diameter() -> f32 {
    30.0
}

pub fn get_ball_starting_position() -> Vec3 {
    Vec3::new(0.0, -50.0, 1.0)
}

pub fn get_ball_speed() -> f32 {
    400.0
}

pub fn get_initial_ball_direction() -> Vec2 {
    Vec2::new(0.5, -0.5)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ball_color() {
        assert_eq!(get_ball_color(), Color::rgb(1.0, 0.5, 0.5));
    }

    #[test]
    fn test_ball_diameter() {
        assert_eq!(get_ball_diameter(), 30.0);
    }

    #[test]
    fn test_ball_starting_position() {
        assert_eq!(get_ball_starting_position(), Vec3::new(0.0, -50.0, 1.0));
    }

    #[test]
    fn test_speed() {
        assert_eq!(get_ball_speed(), 400.0);
    }

    #[test]
    fn test_initial_ball_direction() {
        assert_eq!(get_initial_ball_direction(), Vec2::new(0.5, -0.5));
    }
}
