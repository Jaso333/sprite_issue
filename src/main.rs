use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .add_systems(Update, (update_sprites, update_text).chain())
        .run();
}

#[derive(Component)]
enum Func {
    Sin,
    Cos,
}

#[derive(Component)]
struct SinText;

#[derive(Component)]
struct CosText;

fn startup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 0.0, 0.0),
            custom_size: Some(Vec2::splat(200.0)),
            ..default()
        },
        Func::Sin,
        Transform::from_xyz(-50.0, 0.0, 0.0),
    ));
    commands.spawn((
        Sprite {
            color: Color::srgb(0.0, 1.0, 0.0),
            custom_size: Some(Vec2::splat(200.0)),
            ..default()
        },
        Func::Cos,
        Transform::from_xyz(50.0, 0.0, 0.0),
    ));
    commands.spawn((
        SinText,
        Text2d::new("sin"),
        Transform::from_xyz(-100.0, 150.0, 0.0),
    ));
    commands.spawn((
        CosText,
        Text2d::new("cos"),
        Transform::from_xyz(100.0, 150.0, 0.0),
    ));
}

fn update_sprites(mut query: Query<(&Func, &mut Transform)>, time: Res<Time>) {
    for (func, mut transform) in query.iter_mut() {
        transform.translation.z = match func {
            Func::Sin => time.elapsed_secs().sin(),
            Func::Cos => time.elapsed_secs().cos(),
        };
    }
}

fn update_text(
    mut text_query: Query<(&mut Text2d, Option<&SinText>, Option<&CosText>)>,
    sprite_query: Query<(&Func, &Transform)>,
) {
    for (mut text, sin, cos) in text_query.iter_mut() {
        for (func, transform) in sprite_query {
            match func {
                Func::Sin if sin.is_some() => text.0 = format!("{:.2}", transform.translation.z),
                Func::Cos if cos.is_some() => text.0 = format!("{:.2}", transform.translation.z),
                _ => (),
            }
        }
    }
}
