use bevy::app::App;
use bevy::DefaultPlugins;
use bevy::prelude::{Commands, Component, Plugin, Query, Startup, Update, Without};
use strum_macros::Display;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PeoplePlugin))
        .run();
}

pub struct PeoplePlugin;

impl Plugin for PeoplePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (people_ready_for_hire, print_names));
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn(
        (
            Person {
                name: "Alex".to_string(),
            },
            Employed {
                job: Job::Doctor
            }
        ),
    );
    commands.spawn(Person {
        name: "Bobby".to_string(),
    });
    commands.spawn((
        Person {
            name: "Cindy".to_string(),
        },
        Employed {
            job: Job::Lawyer
        })
    );
    commands.spawn(Person {
        name: "Dylan".to_string(),
    });
    commands.spawn(Person {
        name: "Evelyn".to_string(),
    });
}

pub fn people_ready_for_hire(person_query: Query<&Person, Without<Employed>>) {
    for person in person_query.iter() {
        println!("{} is ready for hire!", person.name);
    }
}

pub fn print_names(person_query: Query<(&Person, &Employed)>) {
    for (person, employed) in person_query.iter() {
        println!("Name: {}, job: {}", person.name, employed.job);
    }
}

#[derive(Component)]
pub struct Person {
    pub name: String,
}

#[derive(Component)]
pub struct Employed {
    job: Job,
}

#[derive(Debug, Display)]
pub enum Job {
    Doctor,
    FireFighter,
    Lawyer,
}