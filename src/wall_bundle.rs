use bevy::prelude::*;

use crate::collider::Collider;
use crate::wall_location::WallLocation;
//mod collider;

// This bundle is a collection of the components that define a "wall" in our game
#[derive(Bundle)]
pub struct WallBundle {
    // You can nest bundles inside of other bundles like this
    // Allowing you to compose their functionality
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

impl WallBundle {
    // This "builder method" allows us to reuse logic across our wall entities,
    // making our code easier to read and less prone to bugs when we change the logic
    pub fn new(location: WallLocation) -> WallBundle {
        let wall_color: Color = Color::rgb(0.8, 0.8, 0.8);
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: location.position().extend(0.0),
                    scale: location.size().extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: wall_color,
                    ..default()
                },
                ..default()
            },
            collider: Collider,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_walls_have_a_z_scale_of_one() {
        // The z-scale of 2D objects must always be 1.0,
        // or their ordering will be affected in surprising ways.
        // See https://github.com/bevyengine/bevy/issues/4149
        let wall = WallBundle::new(WallLocation::Top);

        assert_eq!(wall.sprite_bundle.transform.scale.z, 1.0);
    }

    #[test]
    fn test_walls_have_a_z_order_of_zero() {
        // We need to convert our Vec2 into a Vec3, by giving it a z-coordinate
        // This is used to determine the order of our sprites
        let wall = WallBundle::new(WallLocation::Top);
        assert_eq!(wall.sprite_bundle.transform.translation.z, 0.0);
    }
}
