use bevy::prelude::*;

#[derive(Bundle)]
pub struct Player {
    sprite: SpriteBundle,
}

impl Player {
    // Thanks to Hikasu
    fn get_x(&self) -> f32 {
        return self.sprite.transform.translation.x;
    }
}

pub fn create_player() -> Player {
    return Player {
        sprite: SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_at_origin() {
        assert_eq!(create_player().get_x(), 0.0);
    }
}
