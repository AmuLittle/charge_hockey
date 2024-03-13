use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;
use std::f64::consts::PI;
use super::*;

#[wasm_bindgen]
pub async fn wasm_render(ctx: &CanvasRenderingContext2d, mousex: f64, mousey: f64, draw_debug: bool) {
    ctx.clear_rect(0.0, 0.0, 800.0, 650.0);

    // UI

    ctx.set_stroke_style(&"black".into());
    ctx.set_line_width(5.0);
    ctx.stroke_rect(0.0, 0.0, 800.0, 600.0);
    ctx.stroke_rect(0.0, 0.0, 800.0, 650.0);

    let base_state_lock = BASE_STATE.read().expect("could not read RwLock").clone();
    let mut state = base_state_lock;

    // buttons
    // the background for the first button is always visible
    ctx.set_fill_style(&"orange".into());
    ctx.begin_path();
    ctx.arc(25.0, 625.0, 15.0, 0.0, 2.0*PI).expect("could not draw UI");
    ctx.fill();
    ctx.begin_path();
    ctx.arc(65.0, 625.0, 15.0, 0.0, 2.0*PI).expect("could not draw UI");
    ctx.fill();
    {
        let current_state_lock = CURRENT_STATE.read().expect("could not read RwLock");
        match current_state_lock.as_ref() {
            Some(s) => { // sim running, render pause, play, reset, collision text, and goal text
                // pause or play
                if s.pause {
                    ctx.set_fill_style(&"black".into());
                    ctx.begin_path();
                    ctx.move_to(16.340, 616.340);
                    ctx.line_to(16.340, 633.660);
                    ctx.line_to(35.0, 625.0);
                    ctx.fill();
                }
                else {
                    ctx.set_fill_style(&"black".into());
                    ctx.fill_rect(16.340, 616.340, 5.0, 17.32);
                    ctx.fill_rect(28.66, 616.340, 5.0, 17.32)
                }
                // the second button is always reset
                ctx.set_line_width(3.0);
                ctx.set_stroke_style(&"black".into());
                ctx.begin_path();
                ctx.arc(65.0, 625.0, 9.0, 0.0, 1.5*PI).expect("could not draw UI");
                ctx.stroke();
                ctx.set_fill_style(&"black".into());
                ctx.begin_path();
                ctx.move_to(62.0, 612.0);
                ctx.line_to(62.0, 620.0);
                ctx.line_to(73.0, 616.0);
                ctx.fill();

            }
            None => { // setup, render start, difficulty selector, and charge basket
                // clear charges / walls
                ctx.set_line_width(2.0);
                ctx.begin_path();
                ctx.move_to(65.0, 620.0);
                ctx.line_to(65.0, 615.0);
                ctx.move_to(68.536, 628.538);
                ctx.line_to(72.071, 632.071);
                ctx.move_to(70.0, 625.0);
                ctx.line_to(75.0, 625.0);
                ctx.move_to(61.464, 628.536);
                ctx.line_to(57.929, 632.071);
                ctx.move_to(65.0, 630.0);
                ctx.line_to(65.0, 635.0);
                ctx.move_to(61.464, 621.464);
                ctx.line_to(57.929, 617.929);
                ctx.move_to(60.0, 625.0);
                ctx.line_to(55.0, 625.0);
                ctx.move_to(68.536, 621.464);
                ctx.line_to(72.071, 617.929);
                ctx.stroke();

                // charge / wall basket
                ctx.set_stroke_style(&"black".into());
                ctx.set_line_width(2.0);
                ctx.stroke_rect(730.0, 610.0, 60.0, 30.0);

                if state.win_state == 0 {
                    // play button
                    ctx.set_fill_style(&"black".into());
                    ctx.begin_path();
                    ctx.move_to(16.340, 616.340);
                    ctx.line_to(16.340, 633.660);
                    ctx.line_to(35.0, 625.0);
                    ctx.fill();
                    // charge basket
                    ctx.begin_path();
                    ctx.move_to(760.0, 610.0);
                    ctx.line_to(760.0, 640.0);
                    ctx.stroke();
                    // pos charge puck in basket
                    ctx.set_fill_style(&"red".into());
                    ctx.begin_path();
                    ctx.arc(745.0, 625.0, 6.0, 0.0, 2.0*PI).expect("could not draw basket");
                    ctx.fill();
                    ctx.set_stroke_style(&"white".into());
                    ctx.set_line_width(2.0);
                    ctx.begin_path();
                    ctx.move_to(745.5, 620.5);
                    ctx.line_to(745.5, 630.5);
                    ctx.move_to(740.5, 625.5);
                    ctx.line_to(750.5, 625.5);
                    ctx.stroke();
                    // neg charge puck in basket
                    ctx.set_fill_style(&"blue".into());
                    ctx.begin_path();
                    ctx.arc(775.0, 625.0, 6.0, 0.0, 2.0*PI).expect("could not draw basket");
                    ctx.fill();
                    ctx.set_stroke_style(&"white".into());
                    ctx.set_line_width(2.0);
                    ctx.begin_path();
                    ctx.move_to(770.5, 625.5);
                    ctx.line_to(780.5, 625.5);
                    ctx.stroke();
                }
                else if state.win_state == 3 {
                    // exit door

                    // wall basket

                    // save button

                    // load button
                }
            }
        }
    }

    // GAME

    {
        let current_state_lock = CURRENT_STATE.read().expect("could not read RwLock").clone();
        if let Some(current_state) = current_state_lock {
            state = current_state;
        }
    }

    // puck
    ctx.set_fill_style(&"black".into());
    ctx.begin_path();
    ctx.arc(state.puck_x as f64, state.puck_y as f64, 6.0, 0.0, 2.0*PI).expect("could not draw puck");
    ctx.fill();
    ctx.set_stroke_style(&"white".into());
    ctx.set_line_width(2.0);
    ctx.begin_path();
    if !state.neg_puck { // a minus sign is just a plus without a vertical line so dont draw it if neg puck
        ctx.move_to(state.puck_x as f64 + 0.5, state.puck_y as f64 - 4.5);
        ctx.line_to(state.puck_x as f64 + 0.5, state.puck_y as f64 + 5.5);
    }
    ctx.move_to(state.puck_x as f64 - 4.5, state.puck_y as f64 + 0.5);
    ctx.line_to(state.puck_x as f64 + 5.5, state.puck_y as f64 + 0.5);
    ctx.stroke();

    // goal
    ctx.set_stroke_style(&"blue".into());
    ctx.set_line_width(5.0);
    ctx.begin_path();
    ctx.move_to(state.goal_x - 10.0, state.goal_y + 25.0);
    ctx.line_to(state.goal_x, state.goal_y + 25.0);
    ctx.line_to(state.goal_x, state.goal_y - 25.0);
    ctx.line_to(state.goal_x - 10.0, state.goal_y - 25.0);
    ctx.stroke();

    // attached charge
    if state.attached_charge > 0 {
        ctx.set_fill_style(&"red".into());
        if state.attached_charge == 2 {
            ctx.set_fill_style(&"blue".into());
        }
        ctx.begin_path();
        ctx.arc(mousex, mousey, 6.0, 0.0, 2.0*PI).expect("could not draw charges");
        ctx.fill();
        ctx.set_stroke_style(&"white".into());
        ctx.set_line_width(2.0);
        ctx.begin_path();
        if state.attached_charge == 1 { // a minus sign is just a plus without a vertical line so dont draw it if neg charge
            ctx.move_to(mousex + 0.5, mousey - 4.5);
            ctx.line_to(mousex + 0.5, mousey + 5.5);
        }
        ctx.move_to(mousex - 4.5, mousey + 0.5);
        ctx.line_to(mousex + 5.5, mousey + 0.5);
        ctx.stroke();
    }

    // charges
    for charge in state.charges {
        if let Some(charge) = charge {
            ctx.set_fill_style(&"red".into());
            if charge.is_neg {
                ctx.set_fill_style(&"blue".into());
            }
            ctx.begin_path();
            ctx.arc(charge.x as f64, charge.y as f64, 6.0, 0.0, 2.0*PI).expect("could not draw charges");
            ctx.fill();
            ctx.set_stroke_style(&"white".into());
            ctx.set_line_width(2.0);
            ctx.begin_path();
            if !charge.is_neg { // a minus sign is just a plus without a vertical line so dont draw it if neg charge
                ctx.move_to(charge.x as f64 + 0.5, charge.y as f64 - 4.5);
                ctx.line_to(charge.x as f64 + 0.5, charge.y as f64 + 5.5);
            }
            ctx.move_to(charge.x as f64 - 4.5, charge.y as f64 + 0.5);
            ctx.line_to(charge.x as f64 + 5.5, charge.y as f64 + 0.5);
            ctx.stroke();
        }
    }

    // walls

    // win_state
    if state.win_state == 1 {
        ctx.set_fill_style(&"green".into());
        ctx.set_font("128px serif".into());
        ctx.fill_text("GOAL!".into(), 200.0, 150.0).expect("could not draw goal");
    }
    else if state.win_state == 2 {
        ctx.set_fill_style(&"red".into());
        ctx.set_font("128px serif".into());
        ctx.fill_text("Collision!".into(), 150.0, 150.0).expect("could not draw collision");
    }

    // debug
    if draw_debug {
        
    }
}