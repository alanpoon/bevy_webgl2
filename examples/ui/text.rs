use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

/// This example illustrates how to create text and update it in a system. It displays the current FPS in the upper left hand corner.
fn main() {
    App::build()
        .add_plugins(bevy_webgl2::DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup.system())
        .add_system(text_update_system.system())
        .run();
}

// A unit struct to help identify the FPS UI component, since there may be many Text components
struct FpsText;

fn text_update_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.sections[0].value = format!("FPS: {:.2}", average);
            }
        }
    }
}

fn setup(commands: &mut Commands, asset_server: Res<AssetServer>) {
    commands
        // 2d camera
        .spawn(UiCameraBundle::default())
        // texture
        .spawn(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            text: Text::with_section(
                "FPS:".to_string(),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 60.0,
                    color: Color::WHITE,
                    ..Default::default()
                },
                TextAlignment::default(),
            ),
            ..Default::default()
        })
        .with(FpsText);
}
