use macroquad::color::hsl_to_rgb;
use macroquad::{prelude::*, window};
use std::time::Duration;
use std::{cell, thread};

fn update(state: &Vec<(bool, usize)>, rule: i32) -> Vec<(bool, usize)> {
    let n = state.len();

    let mut new_state = vec![(false, 0); n];

    for i in 0i32..n as i32 {
        let mut c = 0;

        for j in -2i32..=2 {
            if state[((i + j + n as i32) % n as i32) as usize].0 {
                c += 1;
            }
        }

        if (rule >> c) & 0b1 == 0b1 {
            new_state[i as usize] = (true, state[i as usize].1 + 1);
        }
    }

    new_state
}

#[macroquad::main("1D Cellular Automaton")]
async fn main() {
    const CELL_SIZE: f32 = 8.;

    clear_background(WHITE);

    let cell_number: usize = (screen_width() / CELL_SIZE) as usize;

    let mut reset = true;

    let mut states = vec![];

    rand::srand(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    );

    let mut rule = 20;
    let mut probability = 0.5;

    loop {
        clear_background(BLACK);

        if reset {
            states = vec![vec![(false, 0usize); cell_number]];

            probability = rand::gen_range(0.0, 1.0);

            for i in 0..states[0].len() {
                if rand::gen_range(0.0, 1.0) <= probability {
                    states[0][i] = (true, 1);
                }
            }

            rule = 20;

            reset = false;
        }

        for (j, state) in states.iter().enumerate() {
            for (i, cell) in state.iter().enumerate() {
                if cell.0 {
                    draw_rectangle(
                        screen_width() / 2.0 - state.len() as f32 / 2. * CELL_SIZE
                            + i as f32 * CELL_SIZE,
                        25. + j as f32 * CELL_SIZE,
                        CELL_SIZE,
                        CELL_SIZE,
                        hsl_to_rgb((j % 128) as f32 / 128. + 0.5, 0.8, 0.8),
                    );
                }
            }
        }

        draw_text(
            &format!("Rule: {} P: {:.2}", rule, probability),
            20.,
            20.,
            30.,
            WHITE,
        );

        if states.len() < screen_height() as usize / CELL_SIZE as usize {
            let next_state = update(states.last().unwrap(), rule);
            states.push(next_state);
        }

        if is_key_pressed(KeyCode::R) {
            reset = true;
        }

        next_frame().await
    }
}
