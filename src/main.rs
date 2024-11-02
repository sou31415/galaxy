use bevy::prelude::*;
use rand::Rng;
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, animate_sprite)
        .run();
}

#[derive(Component)]
struct Wobble {
    timer: f32,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // カメラを追加
    commands.spawn(Camera2dBundle::default());

    // 背景画像の読み込みと配置（固定）
    let background_texture = asset_server.load("path/to/background_image.png");
    commands.spawn(SpriteBundle {
        texture: background_texture,
        transform: Transform::from_scale(Vec3::splat(1.0)), // 背景サイズに合わせて調整
        ..Default::default()
    });

    // 揺らめき画像の読み込みと配置
    let wobble_texture = asset_server.load("path/to/wobble_image.png");

    // ランダムな初期位置を生成
    let mut rng = rand::thread_rng();
    let initial_x = rng.gen_range(-300.0..300.0); // 適切な範囲に調整
    let initial_y = rng.gen_range(-200.0..200.0);

    // スプライトを1/10サイズに縮小して初期位置に配置
    commands
        .spawn(SpriteBundle {
            texture: wobble_texture,
            transform: Transform {
                translation: Vec3::new(initial_x, initial_y, 1.0),
                scale: Vec3::splat(0.1), // 1/10サイズ
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgba(1.0, 1.0, 1.0, 1.0), // 初期透明度
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Wobble { timer: 0.0 });
}

fn animate_sprite(time: Res<Time>, mut query: Query<(&mut Transform, &mut Sprite, &mut Wobble)>) {
    for (mut transform, mut sprite, mut wobble) in query.iter_mut() {
        let delta = time.delta_seconds();
        wobble.timer += delta;

        // 不規則な動きを作成
        let wobble_speed_x = 1.5;
        let wobble_speed_y = 1.0;
        let wobble_amount_x = 0.5;
        let wobble_amount_y = 0.3;
        transform.translation.x += (wobble.timer * wobble_speed_x).sin() * wobble_amount_x;
        transform.translation.y += (wobble.timer * wobble_speed_y).cos() * wobble_amount_y;

        // 徐々にフェードアウトさせる
        let fade_speed = 0.2; // フェードアウトの速さ
        let alpha = (1.0 - wobble.timer * fade_speed).max(0.0); // アルファ値を減少
        sprite.color = Color::rgba(1.0, 1.0, 1.0, alpha); // 新しいアルファ値を設定

        // 透明度が0になったら消える
        if alpha <= 0.0 {
            transform.scale = Vec3::ZERO; // スプライトを消去
        }
    }
}
