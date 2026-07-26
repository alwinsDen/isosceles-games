use bevy::{
    app::{Plugin, Startup, Update},
    ecs::{
        component::Component,
        query::With,
        resource::Resource,
        schedule::IntoScheduleConfigs,
        system::{Commands, Query, Res, ResMut},
    },
    time::{Time, Timer, TimerMode},
};

pub struct HelloPlugin;

#[derive(Resource)]
struct GreetTimer(Timer);

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("test1".to_string())));
    commands.spawn((Person, Name("test2".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>, time: Res<Time>, mut timer: ResMut<GreetTimer>) {
    if timer.0.tick(time.delta()).just_finished() {
        println!(
            "WE just crossed repeated {:?} second loop.",
            timer.0.duration()
        );
        for element in &query {
            println!("Hello {}", element.0);
        }
    }
}

fn mutable_names(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "test1".to_string() {
            name.0 = "bolt".to_string();
            break;
        }
    }
}

fn sample_parallel_execution() {
    // println!("=> check break");
}

/// initialization for the sample plugin library.
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, add_people);
        app.add_systems(Update, sample_parallel_execution);
        app.add_systems(Update, (greet_people, mutable_names).chain());
    }
}
