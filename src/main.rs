// Based on https://github.com/bevyengine/bevy/blob/main/examples/games/breakout.rs

fn main() {
    let mut app: App = create_app();
    app.run();
}

use crate::ball::Ball;
use crate::collider::Collider;
use crate::collision::Collision;
use crate::collision_event::CollisionEvent;
use crate::game_layout::create_initial_layout;
use crate::paddle::Paddle;
use crate::velocity::Velocity;
use crate::wall::Wall;
use bevy::{
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
    prelude::*,
    sprite::MaterialMesh2dBundle,
};
use iyes_perf_ui::prelude::*;
use std::any::Any;

mod ball;
mod collider;
mod collision;
mod collision_event;
mod game_layout;
mod paddle;
mod player;
mod stepping;
mod velocity;
mod wall;
mod wall_location;
use ball::*;
use paddle::*;
use wall_location::WallLocation;

fn create_app_without_event_loop() -> App {
    let mut app = App::new();
    app.add_event::<CollisionEvent>();
    app.add_systems(
        Startup,
        (
            setup_ball,
            setup_camera,
            setup_debug,
            setup_paddle,
            setup_walls,
        ),
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
    app
}

fn create_app() -> App {
    let mut app = create_app_without_event_loop();
    app.add_plugins(DefaultPlugins).add_plugins(
        stepping::SteppingPlugin::default()
            .add_schedule(Update)
            .add_schedule(FixedUpdate)
            .at(Val::Percent(35.0), Val::Percent(50.0)),
    );
    // we want Bevy to measure these values for us:
    app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
        .add_plugins(PerfUiPlugin);
    app
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

fn setup_debug(mut commands: Commands) {
    commands.spawn(PerfUiCompleteBundle::default());
}

// Add the paddle to our world
fn setup_paddle(mut commands: Commands) {
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
fn setup_walls(mut commands: Commands) {
    // Walls
    commands.spawn(Wall::new(WallLocation::Left));
    commands.spawn(Wall::new(WallLocation::Right));
    commands.spawn(Wall::new(WallLocation::Bottom));
    commands.spawn(Wall::new(WallLocation::Top));
}

fn move_paddle(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Paddle>>,
    time: Res<Time>,
) {
    let mut paddle_transform = query.single_mut();
    let mut delta_x = 0.0;
    let mut delta_y = 0.0;

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        delta_x -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        delta_x += 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        delta_y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        delta_y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::Space) {
        paddle_transform.scale *= 1.1;
    }

    // Calculate the new horizontal paddle position based on player input
    let new_paddle_x =
        paddle_transform.translation.x + (delta_x * get_paddle_speed() * time.delta_seconds());
    let new_paddle_y =
        paddle_transform.translation.y + (delta_y * get_paddle_speed() * time.delta_seconds());

    // Update the paddle position,
    // making sure it doesn't cause the paddle to leave the arena
    let layout = create_initial_layout();
    let minx = layout.left_wall_x
        + layout.wall_thickness / 2.0
        + get_paddle_size().x / 2.0
        + get_paddle_padding();
    let maxx = layout.right_wall_x
        - layout.wall_thickness / 2.0
        - get_paddle_size().x / 2.0
        - get_paddle_padding();

    paddle_transform.translation.x = new_paddle_x.clamp(minx, maxx);
    paddle_transform.translation.y = new_paddle_y.clamp(-1000.0, 1000.0);
}

// Moves everything that has a velocity
fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

fn check_for_collisions(
    _commands: Commands,
    mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
    collider_query: Query<(Entity, &Transform), With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let (mut ball_velocity, ball_transform) = ball_query.single_mut();

    // check collision with walls
    for (_collider_entity, transform) in &collider_query {
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

fn count_n_balls(app: &App) -> usize {
    let a_ball = Ball;
    let ball_id = a_ball.type_id();
    let mut n = 0;

    for entity in app.world.components().iter() {
        if entity.type_id().unwrap() == ball_id {
            n += 1;
        }
    }
    n
}

fn count_n_paddles(app: &App) -> usize {
    let a_paddle = Paddle;
    let paddle_id = a_paddle.type_id();
    let mut n = 0;

    for entity in app.world.components().iter() {
        if entity.type_id().unwrap() == paddle_id {
            n += 1;
        }
    }
    n
}

/*
fn count_n_players(app: &App) -> usize {
    let a_player = Player;
    let player_id = a_player.type_id();
    let mut n = 0;

    for entity in app.world.components().iter() {
        if entity.type_id().unwrap() == player_id {
            n += 1;
        }
    }
    return n;
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bottom_wall_y() {
        assert_eq!(create_initial_layout().bottom_wall_y, -300.0);
    }

    #[test]
    fn test_no_balls() {
        let mut app = App::new();
        app.update();
        assert_eq!(count_n_balls(&app), 0);
    }

    #[test]
    fn test_no_paddles() {
        let mut app = App::new();
        app.update();
        assert_eq!(count_n_paddles(&app), 0);
    }

    /*
    #[test]
    fn test_no_players() {
        let mut app = App::new();
        app.update();
        assert_eq!(count_n_players(&app), 0);
    }
    */

    /*
    #[test]
    fn test_setup_ball() {
        let mut app = App::new();
        assert_eq!(count_n_balls(&app), 0);
        app.add_systems(Startup, setup_ball);
        app.update();
        assert_eq!(count_n_balls(&app), 1);
    }
    */

    #[test]
    fn test_setup_paddle() {
        let mut app = App::new();
        assert_eq!(count_n_paddles(&app), 0);
        app.add_systems(Startup, setup_paddle);
        app.update();
        assert_eq!(count_n_paddles(&app), 1);
    }
}
