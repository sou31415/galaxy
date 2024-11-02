use bevy::prelude::*;
use rand::Rng;
use std::f32::consts::PI;

const NUM_SPRITES: usize = 5; // 動かす画像の数

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, animate_sprites)
        .run();
}

#[derive(Component)]
struct Wobble {
    timer: f32,
    speed_x: f32,
    speed_y: f32,
    amplitude_x: f32,
    amplitude_y: f32,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // カメラを追加
    commands.spawn(Camera2dBundle::default());

    // 背景画像の読み込みと配置（固定）
    let background_texture = asset_server.load("path/to/background_image.png");
    let background_scale = 1.5;
    commands.spawn(SpriteBundle {
        texture: background_texture,
        transform: Transform::from_scale(Vec3::splat(background_scale)),
        ..Default::default()
    });

    // 動かすスプライトの設定
    let wobble_texture = asset_server.load("path/to/wobble_image.png");

    // 複数のスプライトを生成
    let mut rng = rand::thread_rng();
    for _ in 0..NUM_SPRITES {
        // ランダムな初期位置と動きの設定
        let initial_x = rng.gen_range(-300.0..300.0);
        let initial_y = rng.gen_range(-200.0..200.0);
        let speed_x = rng.gen_range(1.0..3.0);
        let speed_y = rng.gen_range(1.0..3.0);
        let amplitude_x = rng.gen_range(0.5..2.0);
        let amplitude_y = rng.gen_range(0.5..2.0);

        commands
            .spawn(SpriteBundle {
                texture: wobble_texture.clone(),
                transform: Transform {
                    translation: Vec3::new(initial_x, initial_y, 1.0),
                    scale: Vec3::splat(0.1), // 1/10サイズ
                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::rgba(1.0, 1.0, 1.0, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Wobble {
                timer: 0.0,
                speed_x,
                speed_y,
                amplitude_x,
                amplitude_y,
            });
    }
}

fn animate_sprites(time: Res<Time>, mut query: Query<(&mut Transform, &mut Sprite, &mut Wobble)>) {
    for (mut transform, mut sprite, mut wobble) in query.iter_mut() {
        let delta = time.delta_seconds();
        wobble.timer += delta;

        // 不規則な動きを生成
        transform.translation.x += (wobble.timer * wobble.speed_x).sin() * wobble.amplitude_x;
        transform.translation.y += (wobble.timer * wobble.speed_y).cos() * wobble.amplitude_y;

        // 徐々にフェードアウト
        let fade_speed = 0.1;
        let alpha = (1.0 - wobble.timer * fade_speed).max(0.0);
        sprite.color = Color::rgba(1.0, 1.0, 1.0, alpha);

        // 透明度が0になったらスプライトを消去
        if alpha <= 0.0 {
            transform.scale = Vec3::ZERO;
        }
    }
}
