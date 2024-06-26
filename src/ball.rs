use bevy::prelude::*;

// We set the z-value of the ball to 1 so it renders on top in the case of overlapping sprites.
//const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
//const BALL_DIAMETER: f32 = 30.;
//const BALL_SPEED: f32 = 400.0;
//const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);

#[derive(Component)]
pub struct Ball;

pub fn get_ball_starting_position() -> Vec3 {
    return Vec3::new(0.0, -50.0, 1.0);
}

pub fn get_ball_diameter() -> f32 {
    return 30.0;
}

pub fn get_ball_speed() -> f32 {
    return 400.0;
}

pub fn get_initial_ball_direction() -> Vec2 {
    return Vec2::new(0.5, -0.5);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ball_starting_position() {
        assert_eq!(get_ball_starting_position(), Vec3::new(0.0, -50.0, 1.0));
    }

    #[test]
    fn test_ball_diameter() {
        assert_eq!(get_ball_diameter(), 30.0);
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
