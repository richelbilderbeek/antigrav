//! A simplified implementation of the classic game "Breakout".

fn main() {
    let mut app: App = create_app();
    app.run();
}

use crate::game_layout::create_initial_layout;
use crate::paddle::Paddle;
use bevy::{
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
    prelude::*,
    sprite::MaterialMesh2dBundle,
};

mod ball;
mod game_layout;
mod paddle;
mod stepping;
mod wall_location;
use ball::*;
use paddle::*;
use wall_location::WallLocation;

fn create_app_without_event_loop() -> App {
    let mut app = App::new();
    const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
    app.insert_resource(ClearColor(BACKGROUND_COLOR));
    app.add_event::<CollisionEvent>();
    app.add_systems(
        Startup,
        (setup_ball, setup_camera, setup_paddle, setup_wall_bundles),
    );
    // Add our gameplay simulation systems to the fixed timestep schedule
    // which runs at 64 Hz by default
    app.add_systems(
        FixedUpdate,
        (apply_velocity, move_paddle, check_for_collisions)
            // `chain`ing systems together runs them in order
            .chain(),
    );
    app.add_systems(Update, bevy::window::close_on_esc);
    return app;
}

fn create_app() -> App {
    let mut app = create_app_without_event_loop();
    app.add_plugins(DefaultPlugins).add_plugins(
        stepping::SteppingPlugin::default()
            .add_schedule(Update)
            .add_schedule(FixedUpdate)
            .at(Val::Percent(35.0), Val::Percent(50.0)),
    );
    return app;
}

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Collider;

#[derive(Event, Default)]
struct CollisionEvent;

#[derive(Component)]
struct Brick;

// This bundle is a collection of the components that define a "wall" in our game
#[derive(Bundle)]
struct WallBundle {
    // You can nest bundles inside of other bundles like this
    // Allowing you to compose their functionality
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

impl WallBundle {
    // This "builder method" allows us to reuse logic across our wall entities,
    // making our code easier to read and less prone to bugs when we change the logic
    fn new(location: WallLocation) -> WallBundle {
        let wall_color: Color = Color::rgb(0.8, 0.8, 0.8);
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    // We need to convert our Vec2 into a Vec3, by giving it a z-coordinate
                    // This is used to determine the order of our sprites
                    translation: location.position().extend(0.0),
                    // The z-scale of 2D objects must always be 1.0,
                    // or their ordering will be affected in surprising ways.
                    // See https://github.com/bevyengine/bevy/issues/4149
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

fn setup_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Ball
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::default()).into(),
            material: materials.add(get_ball_color()),
            transform: Transform::from_translation(get_ball_starting_position())
                .with_scale(Vec2::splat(get_ball_diameter()).extend(1.)),
            ..default()
        },
        Ball,
        Velocity(get_initial_ball_direction().normalize() * get_ball_speed()),
    ));
}

// Add the camera to our world
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

// Add the paddle to our world
fn setup_paddle(mut commands: Commands) {
    // Paddle
    let paddle_y = create_initial_layout().paddle_y;

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, paddle_y, 0.0),
                scale: get_paddle_size(),
                ..default()
            },
            sprite: Sprite {
                color: get_paddle_color(),
                ..default()
            },
            ..default()
        },
        Paddle,
        Collider,
    ));
}

// Add the game's entities to our world
fn setup_wall_bundles(mut commands: Commands) {
    // Walls
    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
    commands.spawn(WallBundle::new(WallLocation::Top));
}

fn move_paddle(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Paddle>>,
    time: Res<Time>,
) {
    let mut paddle_transform = query.single_mut();
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        direction += 1.0;
    }

    // Calculate the new horizontal paddle position based on player input
    let new_paddle_position =
        paddle_transform.translation.x + direction * get_paddle_speed() * time.delta_seconds();

    // Update the paddle position,
    // making sure it doesn't cause the paddle to leave the arena
    let layout = create_initial_layout();
    let left_bound = layout.left_wall_x
        + layout.wall_thickness / 2.0
        + get_paddle_size().x / 2.0
        + get_paddle_padding();
    let right_bound = layout.right_wall_x
        - layout.wall_thickness / 2.0
        - get_paddle_size().x / 2.0
        - get_paddle_padding();

    paddle_transform.translation.x = new_paddle_position.clamp(left_bound, right_bound);
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

fn check_for_collisions(
    _commands: Commands,
    mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
    collider_query: Query<(Entity, &Transform, Option<&Brick>), With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let (mut ball_velocity, ball_transform) = ball_query.single_mut();

    // check collision with walls
    for (_collider_entity, transform, _maybe_brick) in &collider_query {
        let collision = collide_with_side(
            BoundingCircle::new(
                ball_transform.translation.truncate(),
                get_ball_diameter() / 2.,
            ),
            Aabb2d::new(
                transform.translation.truncate(),
                transform.scale.truncate() / 2.,
            ),
        );

        if let Some(collision) = collision {
            // Sends a collision event so that other systems can react to the collision
            collision_events.send_default();

            // reflect the ball when it collides
            let mut reflect_x = false;
            let mut reflect_y = false;

            // only reflect if the ball's velocity is going in the opposite direction of the
            // collision
            match collision {
                Collision::Left => reflect_x = ball_velocity.x > 0.0,
                Collision::Right => reflect_x = ball_velocity.x < 0.0,
                Collision::Top => reflect_y = ball_velocity.y < 0.0,
                Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
            }

            // reflect velocity on the x-axis if we hit something on the x-axis
            if reflect_x {
                ball_velocity.x = -ball_velocity.x;
            }

            // reflect velocity on the y-axis if we hit something on the y-axis
            if reflect_y {
                ball_velocity.y = -ball_velocity.y;
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

// Returns `Some` if `ball` collides with `wall`. The returned `Collision` is the
// side of `wall` that `ball` hit.
fn collide_with_side(ball: BoundingCircle, wall: Aabb2d) -> Option<Collision> {
    if !ball.intersects(&wall) {
        return None;
    }

    let closest = wall.closest_point(ball.center());
    let offset = ball.center() - closest;
    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0. {
            Collision::Left
        } else {
            Collision::Right
        }
    } else if offset.y > 0. {
        Collision::Top
    } else {
        Collision::Bottom
    };

    Some(side)
}

fn count_n_players(app: &App) -> usize {
    return app.world.components().len();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bottom_wall_y() {
        assert_eq!(create_initial_layout().bottom_wall_y, -300.0);
    }

    #[test]
    fn test_number_of_players() {
        let app: App = create_app_without_event_loop();
        //let app = create_empty_app();
        assert_ne!(count_n_players(&app), 0);
    }
}
