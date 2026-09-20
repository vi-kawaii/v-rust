mod editor;
mod loading;
mod mode_switch;
mod playback;
mod scene;
mod states;
mod types;

use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};
use bevy::prelude::*;
use bevy::window::{MonitorSelection, WindowMode};

use scene::make_triangle;
use states::AppState;
use types::VtuberDocument;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "VTuber".into(),
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextFont {
                    font_size: FontSize::Px(24.0),
                    ..default()
                },
                text_color: Color::srgb(0.0, 1.0, 0.0),
                ..default()
            },
        })
        .insert_resource(ClearColor(Color::BLACK))
        .init_state::<AppState>()
        // Заготовка документа до старта сцены
        .add_systems(PreStartup, seed_document)
        // Плагины в стиле проекта
        .add_plugins((
            loading::LoadingPlugin,
            scene::ScenePlugin,
            editor::EditorPlugin,
            playback::PlaybackPlugin,
            mode_switch::ModeSwitchPlugin,
        ))
        .run();
}

/// Кладём в документ стартовый треугольник до старта сцены.
fn seed_document(mut doc: ResMut<VtuberDocument>) {
    if doc.objects.is_empty() {
        let mut tri = make_triangle("Triangle", 100.0);
        tri.playback.spin_speed = 1.5;
        doc.add_object(tri);
    }
}
