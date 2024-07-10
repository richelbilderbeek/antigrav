use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub fn create_app() -> App {
    let mut app = App::new();

    // Only add this plugin in testing.
    // The main app will assume it to be absent
    if cfg!(test) {
        app.add_plugins(bevy::input::InputPlugin);
    }

    app.add_systems(Startup, add_player);
    app.add_systems(Update, respond_to_keyboard);

    // Do not do update, as this will disallow to do more steps
    // app.update(); //Don't!
    app
}

fn add_player(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                scale: Vec3::new(64.0, 32.0, 0.0),
                ..default()
            },
            ..default()
        },
        Player,
    ));
}

fn respond_to_keyboard(
    mut query: Query<(&mut Transform, &Player)>,
    maybe_input: Option<Res<ButtonInput<KeyCode>>>,
) {
    if maybe_input.is_none() {
        return;
    }
    let input = maybe_input.unwrap();
    let (mut transform, _) = query.single_mut();
    use bevy::input::keyboard::KeyCode;
    if input.pressed(KeyCode::ArrowRight) {
        transform.translation.x += 1.0;
    }
    if input.pressed(KeyCode::ArrowLeft) {
        transform.translation.x -= 1.0;
    }
    if input.pressed(KeyCode::ArrowUp) {
        transform.translation.y += 1.0;
    }
    if input.pressed(KeyCode::ArrowDown) {
        transform.translation.y -= 1.0;
    }
}

#[cfg(test)]
fn count_n_players(app: &mut App) -> usize {
    let mut query = app.world_mut().query::<&Player>();
    query.iter(app.world()).len()
}

#[cfg(test)]
fn get_player_coordinat(app: &mut App) -> Vec2 {
    let mut query = app.world_mut().query::<(&Transform, &Player)>();
    let (transform, _) = query.single(app.world());
    transform.translation.xy()
}

#[cfg(test)]
fn get_player_scale(app: &mut App) -> Vec2 {
    let mut query = app.world_mut().query::<(&Transform, &Player)>();
    let (transform, _) = query.single(app.world());
    transform.scale.xy()
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::input::keyboard::KeyCode;

    #[test]
    fn test_can_create_app() {
        create_app();
    }

    #[test]
    fn test_empty_app_has_no_players() {
        let mut app = App::new();
        assert_eq!(count_n_players(&mut app), 0);
    }

    #[test]
    fn test_create_app_has_a_player() {
        let mut app = create_app();
        app.update();
        assert_eq!(count_n_players(&mut app), 1);
    }

    #[test]
    fn test_player_is_at_origin() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_player_coordinat(&mut app), Vec2::new(0.0, 0.0));
    }

    #[test]
    fn test_player_has_a_custom_scale() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_player_scale(&mut app), Vec2::new(64.0, 32.0));
    }

    #[test]
    fn test_player_responds_to_right_arrow_key() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_player_coordinat(&mut app), Vec2::new(0.0, 0.0));

        // Press the right arrow key, thanks Periwinkle
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::ArrowRight);

        app.update();
        assert_ne!(get_player_coordinat(&mut app), Vec2::new(0.0, 0.0));
    }

    #[test]
    fn test_player_responds_to_left_arrow_key() {
        let mut app = create_app();
        app.update();

        assert_eq!(get_player_coordinat(&mut app), Vec2::new(0.0, 0.0));
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::ArrowLeft);

        app.update();

        assert_ne!(get_player_coordinat(&mut app), Vec2::new(0.0, 0.0));
    }

    #[test]
    fn test_player_responds_to_up_arrow_key() {
        let mut app = create_app();
        app.update();

        assert_eq!(get_player_coordinat(&mut app), Vec2::new(0.0, 0.0));

        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::ArrowUp);

        app.update();

        assert_ne!(get_player_coordinat(&mut app), Vec2::new(0.0, 0.0));
    }

    #[test]
    fn test_player_responds_to_down_arrow_key() {
        let mut app = create_app();
        app.update();

        assert_eq!(get_player_coordinat(&mut app), Vec2::new(0.0, 0.0));

        // Press the key
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::ArrowDown);

        app.update();

        assert_ne!(get_player_coordinat(&mut app), Vec2::new(0.0, 0.0));
    }
}
