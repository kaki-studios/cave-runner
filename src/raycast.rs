use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct RaycastPlugin;

impl Plugin for RaycastPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, cast_ray);
    }
}

fn cast_ray(
    rapier_context: Res<RapierContext>,

    buttons: Res<Input<MouseButton>>,
   

) {



    if buttons.just_pressed(MouseButton::Left) {
        
        let ray_pos = Vec2::new(0.0, -200.0);
        let ray_dir = Vec2::new(0.0, 1.0);
        let max_toi = 400000.0;
        let solid = true;
        let filter = QueryFilter::default();

        if let Some((entity, _toi)) = rapier_context.cast_ray(ray_pos, ray_dir, max_toi, solid, filter) {
            // The first collider hit has the entity `entity` and it hit after
            // the ray travelled a distance equal to `ray_dir * toi`.
            //let hit_point = ray_pos + ray_dir * toi;
            //println!("Entity {:?} hit at point {}", entity, hit_point);
            
            println!("raycast id {}", entity.index())
        }


    

    }
}