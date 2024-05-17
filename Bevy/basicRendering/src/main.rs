use bevy::{
    diagnostic::{
        DiagnosticsStore, FrameTimeDiagnosticsPlugin
    }, prelude::*, render::camera::ScalingMode, window::{
        PresentMode::*, WindowResolution
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
        }).set(ImagePlugin::default_nearest()))
        .insert_resource(Msaa::Off)
        .add_plugins(ShowFramesPerSecond)
        .add_systems(Startup, add_basic_camera)
        .add_systems(Startup, add_player_character)
        .add_systems(Update, (animate_character,move_character))
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

#[derive(PartialEq)]
enum Frame{
    Idle,
    Walk
}

#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct Player([Handle<Image>;2], Timer, Frame);

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

fn move_character(
    mut query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let dir = keyboard_input.pressed(KeyCode::ArrowRight) as i32 - keyboard_input.pressed(KeyCode::ArrowLeft) as i32;

    for mut player in &mut query {
        let direction = player.local_x();
        player.translation += direction * 24. * 5. * dir as f32 * time.delta_seconds();
    }
}

fn animate_character(
    mut query: Query<(&mut Sprite, &mut Player, &mut Handle<Image>)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    ) {
    let dir = keyboard_input.pressed(KeyCode::ArrowRight) as i32 - keyboard_input.pressed(KeyCode::ArrowLeft) as i32;
    
    for q in &mut query {
        let mut sprite = q.0;
        let mut player = q.1;
        let mut texture = q.2;

        if dir == 0 {
            *texture = player.0[0].clone();
            return
        } else {
            if player.1.tick(time.delta()).just_finished(){
                if player.2 == Frame::Idle {
                    player.2 = Frame::Walk;
                    *texture = player.0[1].clone();
                } else {
                    player.2 = Frame::Idle;
                    *texture = player.0[0].clone();
                }
            }

            if dir > 0 {
                sprite.flip_x = true;   
            } else if dir < 0 {
                sprite.flip_x = false;   
            }
        }
    }

}

// -----------------------------------------------------------------------------
// Entities Management
// -----------------------------------------------------------------------------
fn add_player_character(
    mut c: Commands,
    asset_server: Res<AssetServer>,
) {
    let player_texture_0 = asset_server.load("Player_0.png");
    let player_texture_1 = asset_server.load("Player_1.png");

    let player: Player = Player ([player_texture_0.clone(), player_texture_1], Timer::from_seconds(1./6., TimerMode::Repeating), Frame::Idle);
    
    c.spawn((
        SpriteBundle {
            texture: player_texture_0,
            //transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        player
    ));
}

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
    let mut camera2d = Camera2dBundle::default();
    camera2d.projection.scaling_mode = ScalingMode::WindowSize(2.0);
    c.spawn(camera2d);
}

