use macroquad::prelude::*;
use macroquad::prelude::scene::Node;
use macroquad::rand::ChooseRandom;

const CELL_SIZE: f32 = 5.;
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
    vec![vec![t; w];h]
}

#[derive(Clone, Copy, Debug)]
enum NodeType {
    Fluid,
    Boundary,
    Inflow(u8),
}

#[macroquad::main("2D FHP Lattice-Gas Automaton")]
async fn main() {
    let height = (screen_height() / CELL_SIZE_Y) as usize - 2;
    let width = height;

    let mut lattice= init_lattice(width, height, 0u8);

    let mut type_lattice = init_lattice(width, height, NodeType::Fluid);

    for j in 0..height {
        type_lattice[j][width/8] = NodeType::Boundary;
        type_lattice[j][width * 7 / 8] = NodeType::Boundary;
    }

    for j in height*7/16..height*9/16 {
        type_lattice[j][0] = NodeType::Inflow(0b000001);
        type_lattice[j][width / 8] = NodeType::Fluid;
    }

    for i in width/8..=width * 7 / 8 { 
        type_lattice[0][i] = NodeType::Boundary;
        type_lattice[height-1][i] = NodeType::Boundary;
    }

    let x_off = screen_width() / 2. - (width - 1) as f32 * CELL_SIZE / 2.;
    let y_off = screen_height() / 2. - (height - 1) as f32 * CELL_SIZE_Y / 2.;

    let mut time = get_time();

    loop {

        let x_off = screen_width() / 2. - (width - 1) as f32 * CELL_SIZE / 2.;
        let y_off = screen_height() / 2. - (height - 1) as f32 * CELL_SIZE_Y / 2.;

        for j in 0..height {
            for i in 0..width {
                let x = x_off + i as f32 * CELL_SIZE + if j % 2 == 0 { 0. } else { 0.5 * CELL_SIZE };
                let y = y_off + j as f32 * CELL_SIZE_Y;

                match type_lattice[j][i] {
                    NodeType::Fluid | NodeType::Inflow(_)=> {
                        let n = ((lattice[j][i].count_ones() as f32 / 6.) * 255.) as u8;

                        draw_circle(x, y, CELL_SIZE_Y / 2., Color::from_rgba(n, n, n, 255))
                    },
                    NodeType::Boundary => {
                        draw_circle(x, y, CELL_SIZE_Y / 2., Color::from_rgba(255, 128, 128, 255))
                    },
                }

             
            }
        }

        if get_time() - time >= 0.005 {
            time = get_time();

            let a = *vec![0b010010,0b001001].choose().unwrap();
            let b = *vec![0b100100,0b001001].choose().unwrap();
            let c = *vec![0b010010,0b100100].choose().unwrap();

            // collision


            for j in 0..height {
                for i in 0..width {

                    match type_lattice[j][i] {
                        NodeType::Fluid => {
                            lattice[j][i] = match lattice[j][i] {
                                0b010101 => 0b101010,
                                0b101010 => 0b010101,
                                0b100100 => a,
                                0b010010 => b,
                                0b001001 => c,
        
                                _ => lattice[j][i],
                            }
                        },
                        NodeType::Inflow(d) => {
                            lattice[j][i] = (lattice[j][i] >> 3) & 0b111 | (lattice[j][i] << 3 & 0b111000);
                            lattice[j][i] |= d;
                        }
                        NodeType::Boundary   => {
                            lattice[j][i] = (lattice[j][i] >> 3) & 0b111 | (lattice[j][i] << 3 & 0b111000)
                        },
                    }

               
                }
            }

            let mut new_lattice = init_lattice(width, height, 0);
            

            // streaming
            for j in 0..height {
                for i in 0..width {
                    for d in 0..6 {                   
                        let [i_off, j_off] = OFFSETS[j % 2][d];

                        let nj =
                            ((j as isize - j_off + height as isize) % height as isize) as usize;

                        let ni = ((i as isize + i_off + width as isize) % width as isize) as usize;

                        set_cell(d, &mut new_lattice[nj][ni], get_cell(d, lattice[j][i]));
                    }
                }
            }

            lattice = new_lattice;
        }

        next_frame().await
    }
}
