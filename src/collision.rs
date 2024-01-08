use bevy::prelude::*;
use bevy_rapier2d::{pipeline::CollisionEvent, rapier::geometry::CollisionEventFlags};

use crate::Difficulty;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_collisions);
    }
}

fn handle_collisions(
    mut collision_events: EventReader<CollisionEvent>,
    mut difficulty: ResMut<Difficulty>,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(_, _, CollisionEventFlags::SENSOR) = event {
            println!("event! {:?}, f", event);
            // println!("killing program because you won the level!");
            // TODO: one collision triggers multiple events so it will instatly abort
            let new_difficulty = match difficulty.clone() {
                Difficulty::Normal(200) => Difficulty::Normal(300),
                Difficulty::Normal(300) => Difficulty::Hardest,
                Difficulty::Hardest => {
                    std::process::abort();
                }
                _ => Difficulty::Normal(200),
            };
            *difficulty = new_difficulty;
        }
    }
}
