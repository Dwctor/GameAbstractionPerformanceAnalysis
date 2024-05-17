use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, 
    prelude::*, 
    window::{
        WindowResolution,
        PresentMode::*,
    }
};

fn main(){
    App::new()
        // Sets up the main window without VSync (Unlock FPS)
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            primary_window: Some(Window {
                resolution: WindowResolution::new(1200., 720.),
                present_mode: AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(ShowFramesPerSecond)
        .add_systems(Startup, add_basic_camera)
        //.add_plugins(User_Plugins)
        //.add_resources(User_Resources)
        //.add_systems(User_Systems)
        .run();
}

// -----------------------------------------------------------------------------
// Plugins
// -----------------------------------------------------------------------------
pub struct ShowFramesPerSecond;

impl Plugin for ShowFramesPerSecond{
    fn build(&self, app: &mut App){
        app
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_systems(Startup, add_frame_counter)
        .add_systems(Update, update_fps_counter);
    }
}

// -----------------------------------------------------------------------------
// Components & Resources
// -----------------------------------------------------------------------------

#[derive(Component)]
struct FpsText;

/*
#[derive(Component)]
struct Person;
*/

// -----------------------------------------------------------------------------
// Systems
// -----------------------------------------------------------------------------

fn update_fps_counter(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update FPS value
                text.sections[0].value = format!("Fps: {value:.2}");
            }
        }
    }
}

/*fn count_frames(fps: Res<FPS>, time: Res<Time>){
    
}*/

// -----------------------------------------------------------------------------
// Entities Management
// -----------------------------------------------------------------------------

fn add_frame_counter(mut c: Commands){
    c.spawn((
        TextBundle::from_section(
            "FPS: ",
            TextStyle{
                font_size: 24.,
                ..default()
            }
        ),
        FpsText,
    ));
}

fn add_basic_camera(mut c: Commands){
    c.spawn(Camera2dBundle::default());
}

/*
fn add_people(mut c: Commands){
    c.spawn((Person, Name("Kael Augusto".to_string())));
    c.spawn((Person, Name("Dr. Zobligok".to_string())));
    c.spawn((Person, Name("Skinwalker S".to_string())));
}

*/
