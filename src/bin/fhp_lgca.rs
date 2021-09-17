use macroquad::prelude::*;
use macroquad::rand::ChooseRandom;

use rayon::prelude::*;

const CELL_SIZE: f32 = 2.;
const CELL_SIZE_Y: f32 = CELL_SIZE * 1.7321 * 0.5;

const OFFSETS: [[[isize; 2]; 6]; 2] = [
    [[1, 0], [0, -1], [-1, -1], [-1, 0], [-1, 1], [0, 1]],
    [[1, 0], [1, -1], [0, -1], [-1, 0], [0, 1], [1, 1]],
];

#[inline]
fn get_cell(i: usize, node: u8) -> bool {
    ((node >> i) & 0b1) != 0b0
}

#[inline]
fn set_cell(i: usize, node: &mut u8, val: bool) {
    *node = (!(0b1 << i) & *node) | (val as u8) << i;
}

fn init_lattice<T: Clone>(w: usize, h: usize, t: T) -> Vec<Vec<T>> {
    vec![vec![t; w]; h]
}

#[derive(Clone, Copy, Debug)]
enum NodeType {
    Fluid,
    Boundary,
    Inflow([f32; 6]),
    Sink,
}

#[macroquad::main("2D FHP Lattice-Gas Automaton")]
async fn main() {
    let height = 400;
    let width = 400;

    let mut lattice = init_lattice(width, height, 0u8);

    let mut type_lattice = init_lattice(width, height, NodeType::Fluid);

    for j in 0..height {
        type_lattice[j][0] = NodeType::Inflow([1., 0.5, 0., 0., 0., 0.5]);
        type_lattice[j][width - 1] = NodeType::Sink;
    }

    for i in 0..width {
        type_lattice[0][i] = NodeType::Boundary;
        type_lattice[height - 1][i] = NodeType::Boundary;
    }

    let wall_width = 200;

    for j in height / 2 - wall_width / 2..height / 2 + wall_width / 2 {
        for i in width / 4..width / 4 + 3 {
            type_lattice[j][i] = NodeType::Boundary;
        }
    }

    let v: Vec<_> = (0..6i32)
        .map(|i| {
            let alpha = i as f32 / 6 as f32 * 2. * std::f32::consts::PI;
            [alpha.cos(), alpha.sin()]
        })
        .collect();

    let mut time = get_time();

    let mut s = 10;

    loop {
        // choose random collision rules once per frame

        let coll_2_0 = *vec![0b010010, 0b001001].choose().unwrap();
        let coll_2_1 = *vec![0b100100, 0b001001].choose().unwrap();
        let coll_2_2 = *vec![0b010010, 0b100100].choose().unwrap();

        let coll_4_0 = *vec![0b101101, 0b011011].choose().unwrap();
        let coll_4_1 = *vec![0b110110, 0b011011].choose().unwrap();
        let coll_4_2 = *vec![0b101101, 0b110110].choose().unwrap();

        // collision

        lattice.iter_mut().enumerate().for_each(|(j, row)| {
            for i in 0..width {
                match type_lattice[j][i] {
                    NodeType::Fluid => {
                        row[i] = match row[i] {
                            0b010101 => 0b101010,
                            0b101010 => 0b010101,
                            0b100100 => coll_2_0,
                            0b010010 => coll_2_1,
                            0b001001 => coll_2_2,
                            0b110110 => coll_4_0,
                            0b101101 => coll_4_1,
                            0b011011 => coll_4_2,
                            _ => row[i],
                        }
                    }
                    NodeType::Inflow(p) => {
                        row[i] = (row[i] >> 3) & 0b111 | (row[i] << 3 & 0b111000);

                        for d in 0..6 {
                            if rand::gen_range(0., 1.) <= p[d] {
                                row[i] |= 0b1 << d;
                            }
                        }
                    }
                    NodeType::Boundary => row[i] = (row[i] >> 3) & 0b111 | (row[i] << 3 & 0b111000),
                    NodeType::Sink => {
                        row[i] = 0b0;
                    }
                }
            }
        });

        let mut new_lattice = init_lattice(width, height, 0);

        // streaming
        for j in 0..height {
            for i in 0..width {
                for d in 0..6 {
                    let [i_off, j_off] = OFFSETS[j % 2][d];

                    let nj = ((j as isize - j_off + height as isize) % height as isize) as usize;

                    let ni = ((i as isize + i_off + width as isize) % width as isize) as usize;

                    set_cell(d, &mut new_lattice[nj][ni], get_cell(d, lattice[j][i]));
                }
            }
        }

        lattice = new_lattice;

        if get_time() - time > 0.05 {
            let x_off = screen_width() / 2. - (width - 1) as f32 * CELL_SIZE / 2.;
            let y_off = screen_height() / 2. - (height - 1) as f32 * CELL_SIZE_Y / 2.;

            for j in 0..height {
                for i in 0..width {
                    let x = x_off
                        + i as f32 * CELL_SIZE
                        + if j % 2 == 0 { 0. } else { 0.5 * CELL_SIZE };
                    let y = y_off + j as f32 * CELL_SIZE_Y;

                    match type_lattice[j][i] {
                        NodeType::Boundary => draw_circle(x, y, CELL_SIZE_Y, WHITE),
                        _ => {}
                    }
                }
            }

            for j in 1..height / s {
                for i in 1..width / s {
                    let j = s * j;
                    let i = s * i;

                    let (mut mx, mut my) = (0., 0.);

                    for ii in i - s..=(i + s).min(width - 1) {
                        for jj in j - s..=(j + s).min(height - 1) {
                            let n = lattice[jj][ii];
                            for d in 0..6 {
                                let t = ((n >> d) & 0b1) as f32;
                                mx += t * v[d][0];
                                my += t * v[d][1];
                            }
                        }
                    }

                    mx = mx / ((2 * s + 1) as f32).powi(2);
                    my = my / ((2 * s + 1) as f32).powi(2);

                    let x = x_off
                        + i as f32 * CELL_SIZE
                        + if j % 2 == 0 { 0. } else { 0.5 * CELL_SIZE };
                    let y = y_off + j as f32 * CELL_SIZE_Y;

                    draw_line(
                        x,
                        y,
                        x + 0.5 * mx * CELL_SIZE * (s as f32 + 0.5) as f32,
                        y + 0.5 * my * CELL_SIZE * s as f32,
                        1.,
                        WHITE,
                    );
                }
            }

            if is_key_pressed(KeyCode::Period) {
                s += 1;
            }

            if is_key_pressed(KeyCode::Comma) && s > 1 {
                s -= 1;
            }

            time = get_time();

            next_frame().await
        }
    }
}
