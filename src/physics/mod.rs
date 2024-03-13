use wasm_bindgen::prelude::*;
use rapier2d::dynamics::{RigidBodyBuilder, RigidBodySet};
use rapier2d::geometry::{ColliderBuilder, ColliderSet};
use rapier2d::na::Vector2;
use rapier2d::prelude::*;
use super::*;

pub fn initialize_physics() {
    let mut state = BASE_STATE.read().expect("Could not read CURRENT_STATE").clone();
    let mut  colliders = ColliderSet::new();
    let mut rigidbodies = RigidBodySet::new();

    let goal_collider = ColliderBuilder::new(rapier2d::geometry::SharedShape::cuboid(5.0, 25.0)).sensor(true).build();
    let goal_r_handle = rigidbodies.insert(RigidBodyBuilder::new(rapier2d::dynamics::RigidBodyType::Fixed).translation(Vector::new(state.goal_x as f32 - 10.0, state.goal_y as f32 - 25.0)).build());
    let goal_handle = colliders.insert_with_parent(goal_collider, goal_r_handle, &mut rigidbodies);

    let post_shape = rapier2d::geometry::SharedShape::cuboid(5.0, 2.5);
    let goal_post_1 = ColliderBuilder::new(post_shape.clone()).build();
    let goal_r_post_1 = rigidbodies.insert(RigidBodyBuilder::new(rapier2d::dynamics::RigidBodyType::Fixed).translation(Vector::new(state.goal_x as f32 - 10.0, state.goal_y as f32 - 25.0)).build());
    colliders.insert_with_parent(goal_post_1, goal_r_post_1, &mut rigidbodies);

    let goal_post_2 = ColliderBuilder::new(post_shape).build();
    let goal_r_post_2 = rigidbodies.insert(RigidBodyBuilder::new(rapier2d::dynamics::RigidBodyType::Fixed).translation(Vector::new(state.goal_x as f32 - 10.0, state.goal_y as f32 + 25.0)).build());
    colliders.insert_with_parent(goal_post_2, goal_r_post_2, &mut rigidbodies);

    let goal_back = ColliderBuilder::new(rapier2d::geometry::SharedShape::cuboid(2.5, 25.0)).build();
    let goal_r_back = rigidbodies.insert(RigidBodyBuilder::new(rapier2d::dynamics::RigidBodyType::Fixed).translation(Vector::new(state.goal_x as f32 + 5.0, state.goal_y as f32 - 25.0)).build());
    colliders.insert_with_parent(goal_back, goal_r_back, &mut rigidbodies);

    let puck_collider = ColliderBuilder::new(rapier2d::geometry::SharedShape::ball(6.0)).mass(1.0).mass_properties(MassProperties::from_ball(1.0, 6.0)).friction(0.0).restitution(0.7).build();
    let puck_handle = rigidbodies.insert(RigidBodyBuilder::new(rapier2d::dynamics::RigidBodyType::Dynamic).translation(Vector::new(state.puck_x as f32, state.puck_y as f32)).build());
    let puck_collider_handle = colliders.insert_with_parent(puck_collider, puck_handle, &mut rigidbodies);

    state.physics = Some(PhysicsState {
        params: IntegrationParameters::default(),
        island_man: IslandManager::new(),
        broad: BroadPhase::new(),
        narrow: NarrowPhase::new(),
        rigidbodies,
        colliders,
        impulse_joints: ImpulseJointSet::new(),
        multibody_joints: MultibodyJointSet::new(),
        solver: CCDSolver::new(),
        query: QueryPipeline::new(),
        events: PhysEventHandler { 
            goal_handle,
            puck_collider_handle,
            puck_handle
        }
    });

    *CURRENT_STATE.write().expect("Could not write to CURRENT_STATE") = Some(state.clone());
}

#[wasm_bindgen]
pub async fn wasm_calc_physics_step(delta_time: f32) {
    let mut state = if let Some(state) = CURRENT_STATE.read().expect("Could not read CURRENT_STATE").clone() {
        state.clone()
    }
    else {
        return;
    };
    if state.pause {
        return;
    }

    if let Some(ref mut physics) = state.physics {
        physics.params.dt = delta_time; // timestep is deltatime calculated by js
        PHYSICS.write().expect("Could not read Physics Pipeline").step( // commenting the step call out produces the same results as not doing so
            &Vector2::zeros(),
            &physics.params,
            &mut physics.island_man,
            &mut physics.broad,
            &mut physics.narrow,
            &mut physics.rigidbodies,
            &mut physics.colliders,
            &mut physics.impulse_joints,
            &mut physics.multibody_joints,
            &mut physics.solver,
            Some(&mut physics.query),
            &(),
            &physics.events
        );
        physics.query.update(&physics.rigidbodies, &physics.colliders);

        let puck = physics.rigidbodies.get_mut(physics.events.puck_handle).expect("Could not get puck");
        let puck_translation: Vector2<f32> = puck.translation().clone();

        for charge in state.charges {
            if let Some(charge) = charge {
                if charge.is_neg == state.neg_puck { // if same charge
                    puck.add_force(make_force(puck_translation.x, puck_translation.y, charge.x, charge.y), true);
                }
                else {
                    puck.add_force(-make_force(puck_translation.x, puck_translation.y, charge.x, charge.y), true);
                }
            }
        }

        web_sys::console::log_1(&format!("{}, {}: {}, {}", puck_translation.x, puck_translation.y, puck.user_force().x, puck.user_force().y).into());

        state.puck_x = puck_translation.x;
        state.puck_y = puck_translation.y;
    }

    *CURRENT_STATE.write().expect("Could not write to CURRENT_STATE") = Some(state.clone());
}

fn make_force(puck_x: f32, puck_y: f32, charge_x: f32, charge_y: f32) -> Vector2<f32> {
    let dir = Vector2::new(charge_x - puck_x, charge_y - puck_y);
    let distance2 = dir.magnitude().powi(2);
    Vector2::new(dir.x / distance2, dir.y / distance2)
}