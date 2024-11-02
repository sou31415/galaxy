use bevy::prelude::*;
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, animate_sprite)
        .run();
}

#[derive(Component)]
struct Wobble;

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
    commands
        .spawn(SpriteBundle {
            texture: wobble_texture,
            transform: Transform::from_scale(Vec3::splat(0.5)), // 前景サイズに合わせて調整
            ..Default::default()
        })
        .insert(Wobble); // Wobbleコンポーネントを追加してアニメーション対象にする
}

fn animate_sprite(time: Res<Time>, mut query: Query<&mut Transform, With<Wobble>>) {
    for mut transform in query.iter_mut() {
        let wobble_speed = 1.5; // 揺れのスピード
        let wobble_amount = 0.1; // 揺れの大きさ
        let rotation_amount = 0.05; // 回転の大きさ

        // x軸方向に滑らかに左右に揺れる
        transform.translation.x = (time.elapsed_seconds() * wobble_speed).sin() * wobble_amount;

        // y軸方向にはゆっくり上下に揺れる
        transform.translation.y =
            (time.elapsed_seconds() * wobble_speed * 0.5).cos() * wobble_amount;

        // ゆっくりと回転も加える
        transform.rotation =
            Quat::from_rotation_z((time.elapsed_seconds() * wobble_speed).sin() * rotation_amount);
    }
}
