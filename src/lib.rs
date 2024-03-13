use lazy_static::lazy_static;
use std::sync::{Arc, RwLock};
use wasm_bindgen::prelude::*;
use nalgebra::base::Vector2;
use rapier2d::pipeline::{EventHandler, PhysicsPipeline, PhysicsHooks, QueryPipeline};
use rapier2d::dynamics::{IntegrationParameters, IslandManager, RigidBodyHandle, RigidBodySet, ImpulseJointSet, MultibodyJointSet, CCDSolver};
use rapier2d::geometry::{BroadPhase, ColliderSet, NarrowPhase, ColliderHandle};

#[derive(Copy, Clone, Debug)]
pub struct Wall {
    x: f64,
    y: f64,
    h: f64,
    w: f64
}

#[derive(Copy, Clone, Debug)]
pub struct Charge {
    x: f32,
    y: f32,
    is_neg: bool // false is positive charge
}

#[derive(Clone)]
pub struct PhysEventHandler {
    goal_handle: ColliderHandle,
    puck_collider_handle: ColliderHandle,
    puck_handle: RigidBodyHandle
    
}
impl EventHandler for PhysEventHandler {
    fn handle_collision_event(
            &self,
            bodies: &RigidBodySet,
            colliders: &ColliderSet,
            event: rapier2d::prelude::CollisionEvent,
            contact_pair: Option<&rapier2d::prelude::ContactPair>,
        ) {
        web_sys::console::log_1(&"collision".into());
        if (event.collider1().0 == self.puck_collider_handle.0 || event.collider2().0 == self.puck_collider_handle.0) && event.sensor() { // the only sensor is the goal sensor
            let mut state = if let Some(state) = CURRENT_STATE.read().expect("could not read CURRENT_STATE").clone() {
                state.clone()
            }
            else {
                return;
            };
            if state.win_state == 0 {
               state.win_state = 1;
               *CURRENT_STATE.write().expect("Could not write to CURRENT_STATE") = Some(state.clone()); 
            }
        }
    }
    fn handle_contact_force_event(
            &self,
            dt: rapier2d::prelude::Real,
            bodies: &RigidBodySet,
            colliders: &ColliderSet,
            contact_pair: &rapier2d::prelude::ContactPair,
            total_force_magnitude: rapier2d::prelude::Real,
        ) {
        
    }
}

#[derive(Clone)]
pub struct PhysicsState {
    params: IntegrationParameters,
    island_man: IslandManager,
    broad: BroadPhase,
    narrow: NarrowPhase,
    rigidbodies: RigidBodySet,
    colliders: ColliderSet,
    impulse_joints: ImpulseJointSet,
    multibody_joints: MultibodyJointSet,
    solver: CCDSolver,
    query: QueryPipeline,
    events: PhysEventHandler
}


#[derive(Clone)]
pub struct State {
    pause: bool,
    neg_puck: bool, // false is positively charged puck
    puck_x: f32,
    puck_y: f32,
    goal_x: f64,
    goal_y: f64,
    attached_charge: i32, // 0: none, 1: positive, 2: negative
    win_state: i32, // 0: ongoing game, 1: win, 2: colision, 3: edit mode
    walls: [Option<Wall>; 128],
    physics: Option<PhysicsState>,
    charges: [Option<Charge>; 128]
}

lazy_static! {
    pub static ref BASE_STATE: Arc<RwLock<State>> = Arc::new(RwLock::new(State { pause: false, neg_puck: false, puck_x: 150.0, puck_y: 300.0, goal_x: 650.0, goal_y: 300.0, attached_charge: 0, win_state: 0, walls: [None; 128], physics: None, charges: [None; 128] }));
    pub static ref CURRENT_STATE: Arc<RwLock<Option<State>>> = Arc::new(RwLock::new(None));
    pub static ref PHYSICS: Arc<RwLock<PhysicsPipeline>> = Arc::new(RwLock::new(PhysicsPipeline::new()));
}

pub mod physics;
pub mod rendering;

#[wasm_bindgen]
pub async fn wasm_handle_mouse_down(mousex: f32, mousey: f32) {
    let sim_stopped = if let None = CURRENT_STATE.read().expect("Could not read CURRENT_STATE").clone() { true } else { false };
    if sim_stopped {
        let mut state = BASE_STATE.read().expect("Could not read BASE_STATE").clone();
        if mousey > 610.0 && mousey < 640.0 {
            if mousex > 730.0 && mousex < 760.0 {
                state.attached_charge = 1;
            }
            else if mousex > 760.0 && mousex < 790.0 {
                state.attached_charge = 2;
            }
        }
        else if mousey < 600.0 {
            for charge_opt in state.charges.iter_mut() {
                if let Some(charge) = charge_opt {
                    if Vector2::new(mousex - charge.x, mousey - charge.y).magnitude() < 6.0 {
                        state.attached_charge = if charge.is_neg { 2 } else { 1 };
                        *charge_opt = None;
                        break;
                    }
                }
            }
        }
        *BASE_STATE.write().expect("could not obtain write") = state.clone();
    }
}

#[wasm_bindgen]
pub async fn wasm_handle_mouse_up(mousex: f32, mousey: f32) {
    let cur_state = CURRENT_STATE.read().expect("could not read CURRENT_STATE").clone();
    let mut state = BASE_STATE.read().expect("could not read CURRENT_STATE").clone();
    if Vector2::new(mousex - 25.0, mousey - 625.0).magnitude() <= 15.0 {
        if let Some(mut s) = cur_state {
            s.pause = !s.pause;
            *CURRENT_STATE.write().expect("could not obtain write") = Some(s.clone());
            return;
        }
        physics::initialize_physics();
    }
    else if Vector2::new(mousex - 65.0, mousey - 625.0).magnitude() <= 15.0 {
        if let Some(_) = cur_state {
            *CURRENT_STATE.write().expect("could not obtain write") = None;
            return;
        }
        state.charges = [None; 128];
        *BASE_STATE.write().expect("could not obtain write") = state.clone();
        return;
    }
    if let None = cur_state {
        if state.attached_charge > 0 {
            for charge_opt in state.charges.iter_mut() {
                if let None = charge_opt {
                    if mousey < 600.0 {
                        *charge_opt = Some(Charge { is_neg: if state.attached_charge == 1 {false} else {true}, x: mousex, y: mousey });
                    }
                    break;
                }
            }
            state.attached_charge = 0;
            *BASE_STATE.write().expect("could not obtain write") = state.clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn it_works() {
    }
}