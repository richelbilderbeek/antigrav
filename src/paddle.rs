use bevy::prelude::*;

#[derive(Component)]
pub struct Paddle;

pub fn get_paddle_color() -> Color {
    return Color::rgb(0.3, 0.3, 0.7);
}

// How close can the paddle get to the wall
pub fn get_paddle_padding() -> f32 {
    return 10.0;
}

pub fn get_paddle_size() -> Vec3 {
    return Vec3::new(120.0, 20.0, 0.0);
}

pub fn get_paddle_speed() -> f32 {
    return 500.0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paddle_color() {
        assert_eq!(get_paddle_color(), Color::rgb(0.3, 0.3, 0.7));
    }

    #[test]
    fn test_paddle_padding() {
        assert_eq!(get_paddle_padding(), 10.0);
    }

    #[test]
    fn test_paddle_size() {
        assert_eq!(get_paddle_size(), Vec3::new(120.0, 20.0, 0.0));
    }

    #[test]
    fn test_paddle_speed() {
        assert_eq!(get_paddle_speed(), 500.0);
    }
}
