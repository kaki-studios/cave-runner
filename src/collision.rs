use bevy::prelude::*;
use bevy_rapier2d::{pipeline::CollisionEvent, rapier::geometry::CollisionEventFlags};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_collisions);
    }
}

fn handle_collisions(mut collision_events: EventReader<CollisionEvent>) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(_, _, CollisionEventFlags::SENSOR) = event {
            println!("event! {:?}, f", event);
            println!("killing program because you won the level!");
            std::process::abort();
        }
    }
}
