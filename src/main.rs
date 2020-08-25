extern crate rand;

use rand::{thread_rng, Rng};
use macroquad::*;
//use macroquad::megaui::{widgets, Vector2};
use bit_vec::BitVec;
use std::time::{Instant};

fn window_conf() -> Conf {
    Conf {
        window_title: "conways game of life".to_owned(),
        window_width: 710,
        window_height: 500,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
//    let w = screen_width();
//    let h = screen_height();

    let rows: usize = 50;
    let columns: usize = 50;
    
    let mut initial = generator(rows*columns);

    let mut now = Instant::now();

    let mut timestep = 0.4;
    let mut pause = true;

    loop {

        clear_background(BLACK);

        if is_key_pressed(KeyCode::R) {
            initial = generator(rows*columns);
        }
        if is_key_pressed(KeyCode::P) {
            pause = !pause;
        }
        if is_key_pressed(KeyCode::Space) {
            if !pause {   
                tick(rows, columns, &mut initial);
            }
        }
        if is_key_pressed(KeyCode::C) {
            initial.clear();
        }       

        if now.elapsed().as_secs_f32() >= timestep && pause {
            tick(rows, columns, &mut initial);
            now = Instant::now();
        }
        draw(rows, columns, &initial);

        post(&mut initial);

        gui(&mut timestep);

        next_frame().await
    }
}

fn tick(rows: usize, columns: usize, initial: &mut BitVec) {
    let mut next = initial.clone();

    for r in 0..rows {
        for c in 0..columns {
            let idx = get_index(r, c);
            let cell = initial.get(idx).unwrap();
            let neighbors = get_neighbors(r, c, initial);

            match (cell, neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (true, x) if x < 2 => next.set(idx, false),
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (true, 2) | (true, 3) => next.set(idx, true),
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (true, x) if x > 3 => next.set(idx, false),
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (false, 3) => next.set(idx, true),
                    // All other cells remain in the same state.
                    (_, _) => (),
                };
        }
    }
    
    *initial = next;

}

fn post(cells: &mut BitVec) {
    if is_mouse_button_down(MouseButton::Left) {
        let (xo, yo) : (f32, f32) = (20.0, 20.0);
        let size: f32 = 8.0;
        let (x, y) = mouse_position();

        let idx = get_index(
            ((y-yo)/(size + 1.0)).round() as usize,
            ((x-xo)/(size + 1.0)).round() as usize
        );

        cells.set(idx, true);
    }

    if is_mouse_button_down(MouseButton::Right) {
        let (xo, yo) : (f32, f32) = (20.0, 20.0);
        let size: f32 = 8.0;
        let (x, y) = mouse_position();

        let idx = get_index(
            ((y-yo)/(size + 1.0)).round() as usize,
            ((x-xo)/(size + 1.0)).round() as usize
        );

        cells.set(idx, false);
    }
    

}

fn gui(mut timestep: &mut f32) {
    draw_window(
            hash!(),
            Vec2::new(490., 20.),
            Vec2::new(400., 200.),
            None,
            |ui| {
                let (x, y) = mouse_position();
                ui.label(None, &format!("Mouse position: {} {}", x, y));

                ui.label(None, &format!("FPS: {}", get_fps()));

                ui.slider(hash!(), "", 0f32..5f32, &mut timestep);

            },
        );
}

fn draw(rows: usize, columns: usize, initial: &BitVec) {
    let s = 20.0;

    for r in 0..rows {
        for c in 0..columns {
            let idx = get_index(r, c);
            if initial.get(idx).unwrap() {
                draw_rectangle((c * (8 + 1)) as f32 + s, (r * (8 + 1)) as f32 + s, 8.0, 8.0, GREEN)
            } else {
                draw_rectangle((c * (8 + 1)) as f32 + s, (r * (8 + 1)) as f32 + s, 8.0, 8.0, WHITE)
            }
        }
    }
}

fn get_neighbors(row: usize, column: usize, initial: &BitVec) -> usize {
    let mut count = 0;
    for delta_row in [50 - 1, 0, 1].iter().cloned() { //height
        for delta_col in [50 - 1, 0, 1].iter().cloned() { //width
            if delta_row == 0 && delta_col == 0 {
                continue;
            }

            let neighbor_row = (row + delta_row) % 50; //height
            let neighbor_col = (column + delta_col) % 50; //;width
            let idx = get_index(neighbor_row, neighbor_col);
            if initial.get(idx).unwrap() == true {
                count += 1;
            };
        }
    }
    count
}

fn get_index(row: usize, col: usize) -> usize {
    (row * 50 + col) as usize
}

fn generator(total: usize) -> BitVec {
    let mut bv = BitVec::from_elem(total, false);

    // uncomment for glider 
    /*bv.set(2, true);
    bv.set(50, true);
    bv.set(52, true);
    bv.set(101, true);
    bv.set(102, true);*/

    // uncomment for 50% chance of spawns
    
    let mut rng = thread_rng();
    for i in 0..bv.len() {
        if rng.gen_bool(1.0 / 2.0) {
            bv.set(i, true);
        }
    }

    bv
}
