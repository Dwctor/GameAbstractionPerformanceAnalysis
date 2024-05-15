use bevy::prelude::*;

fn main(){
    App::new()
        .add_plugins(DefaultPlugins)
        //.add_plugins(User_Plugins)
        //.add_resources(User_Resources)
        //.add_systems(User_Systems)
        .run();
}

// -----------------------------------------------------------------------------
// Plugins
// -----------------------------------------------------------------------------

/*
pub struct HelloPlugin;

impl Plugin for HelloPlugin{
    fn build(&self, app: &mut App){
        app
        .insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .add_systems(Startup, (add_people, hello_world))
        .add_systems(Update, /*hello_world, */(rm_skinwalker, greet_people).chain());
    }
}
*/

// -----------------------------------------------------------------------------
// Components & Resources
// -----------------------------------------------------------------------------

/*
#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct GreetTimer(Timer);
*/

// -----------------------------------------------------------------------------
// Systems
// -----------------------------------------------------------------------------

/*
fn hello_world(){
    println!("hello world!");
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, q: Query<&Name, With<Person>>){
    if timer.0.tick(time.delta()).just_finished(){
        for name in &q {
            println!("hello {}!", name.0);
        }
    }
}

fn rm_skinwalker(mut q: Query<&mut Name, With<Person>>){
    for mut name in &mut q {
        if name.0 == "Skinwalker S" {
            name.0 = "Saldomer Sin".to_string();
            break;
        }
    }
}
*/

// -----------------------------------------------------------------------------
// Entities Management
// -----------------------------------------------------------------------------

/*
fn add_people(mut c: Commands){
    c.spawn((Person, Name("Kael Augusto".to_string())));
    c.spawn((Person, Name("Dr. Zobligok".to_string())));
    c.spawn((Person, Name("Skinwalker S".to_string())));
}

*/
