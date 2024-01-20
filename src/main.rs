use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        let fns = (update_people, greet_people);
        let chained_fn = fns.chain();

        app.add_systems(Startup, add_people)
            .add_systems(Update, (hello_world, chained_fn));
    }
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Test1".to_string())));
    commands.spawn((Person, Name("Test2".to_string())));
    commands.spawn((Person, Name("Test2".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0);
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Test1" {
            name.0 = "Test Updated".to_string();
            break;
        }
    }
}

fn hello_world() {
    println!("hello_world")
}

fn main() {
    App::new().add_plugins((DefaultPlugins, HelloPlugin)).run()
}
