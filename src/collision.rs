use bevy::prelude::*;
use bevy_rapier2d::{pipeline::CollisionEvent, rapier::geometry::CollisionEventFlags};

use crate::{Difficulty, DifficultyText};

pub struct CollisionPlugin;

#[derive(Resource)]
struct CollisionTimer(Timer);

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_collisions)
            .insert_resource(CollisionTimer(Timer::from_seconds(
                0.5,
                TimerMode::Repeating,
            )));
    }
}

fn handle_collisions(
    mut collision_events: EventReader<CollisionEvent>,
    mut difficulty: ResMut<Difficulty>,
    mut timer: ResMut<CollisionTimer>,
    mut text: Query<&mut Text, With<DifficultyText>>,
    time: Res<Time>,
) {
    if !timer.0.just_finished() {
        timer.0.tick(time.delta());
    }
    for event in collision_events.read() {
        if let CollisionEvent::Started(_, _, CollisionEventFlags::SENSOR) = event {
            // println!("test {}", timer.0.percent());
            if timer.0.finished() {
                timer.0.reset();

                let new_difficulty = match difficulty.clone() {
                    Difficulty::Normal(200) => Difficulty::Normal(300),
                    Difficulty::Normal(300) => Difficulty::Hardest,
                    Difficulty::Hardest => {
                        //TODO: better ending
                        println!("killing program because you won the game");
                        std::process::abort();
                    }
                    _ => Difficulty::Normal(200),
                };
                *difficulty = new_difficulty.clone();
                dbg!(&new_difficulty);
                for mut element in &mut text {
                    println!("riestnrisn");
                    element.sections[0].value = match new_difficulty {
                        Difficulty::Normal(200) => "Difficulty: Normal".into(),
                        Difficulty::Normal(300) => "Difficulty: Hard".into(),
                        Difficulty::Hardest => "Difficulty: Very Hard".into(),
                        _ => "Bug!".into(),
                    }
                }
            }
        }
    }
}
