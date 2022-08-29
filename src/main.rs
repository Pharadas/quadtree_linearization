extern crate sdl2;
extern crate nalgebra_glm as glm;

use std::collections::HashMap;

use glm::TVec2;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use glm::vec2;

mod quadtree;
use quadtree::QuadTree;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut mouse_state;

    let window_dimensions: TVec2<u32> = vec2(800, 600);

    let mut outer_quad = QuadTree::new(vec2(0, 0), vec2(window_dimensions.x as i32, window_dimensions.y as i32));
    let mut colors_collection: HashMap<Color, &str> = HashMap::new();

    colors_collection.insert(Color::RGB(255, 0, 0), "00");
    colors_collection.insert(Color::RGB(0, 255, 0), "01");
    colors_collection.insert(Color::RGB(0, 0, 255), "10");
    colors_collection.insert(Color::RGB(255, 255, 255), "11");

    let mut linearized_tree = "10100001100100101111011001110101000000100001010111101000".to_string();
    println!("{}", linearized_tree.len());

    let mut buff = String::from("");
    let mut num_array: Vec<u8> = Vec::new();

    // Agregar el padding necesario para que este bien alineado
    linearized_tree += &"0".repeat(8 - (linearized_tree.len() % 8));

    for i in 0..linearized_tree.len() {
        if (i % 8 == 0 && i != 0) || (i == linearized_tree.len() - 1) {
            num_array.push(u8::from_str_radix(&buff, 2).unwrap().try_into().unwrap());
            buff = String::from(linearized_tree.as_bytes()[i] as char);
        } else {
            buff += &String::from(linearized_tree.as_bytes()[i] as char);
        }
    }

    // Ahora que tenemos los bits como booleanos, podemos manipular
    // mas facilmente esto
    let mut linearized_tree_vec: Vec<bool> = Vec::new();

    for i in num_array {
        for x in 0..8 {
            let mask = u8::pow(2, 7 - x as u32);
            linearized_tree_vec.push(i & mask != 0);
        }
    }

    let mut quad = QuadTree::new(vec2(0, 0), vec2(window_dimensions.x as i32, window_dimensions.y as i32));
    quad.get_quadtree_from_linear(&linearized_tree_vec, &mut 0);

    let window = video_subsystem.window("gaming time", window_dimensions.x, window_dimensions.y)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    // recovered_quad.get_quad_tree_from_linear(num_array, vec2(0, 0), vec2(window_dimensions.x as i32, window_dimensions.y as i32), 0, Color::RGB(0, 0, 0));

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut observer_position: TVec2<i32> = vec2(10, 10);
    let mut ray_position: TVec2<i32> = vec2(200, 100);

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        mouse_state = event_pump.mouse_state();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |

                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },

                Event::KeyDown { keycode: Some(Keycode::O), .. } => {
                    outer_quad.divide_at_position(mouse_state.x(), mouse_state.y(), Color::RGB(255, 255, 255));
                    println!("{}, {}, Color::RGB(255, 255, 255)", mouse_state.x(), mouse_state.y())
                }

                Event::KeyDown { keycode: Some(Keycode::U), .. } => {
                    outer_quad.divide_at_position(mouse_state.x(), mouse_state.y(), Color::RGB(0, 0, 255));
                    println!("{}, {}, Color::RGB(255, 255, 255)", mouse_state.x(), mouse_state.y())
                }

                Event::KeyDown { keycode: Some(Keycode::J), .. } => {
                    outer_quad.divide_at_position(mouse_state.x(), mouse_state.y(), Color::RGB(0, 255, 0));
                    println!("{}, {}, Color::RGB(255, 255, 255)", mouse_state.x(), mouse_state.y())
                }

                Event::KeyDown { keycode: Some(Keycode::K), .. } => {
                    outer_quad.divide_at_position(mouse_state.x(), mouse_state.y(), Color::RGB(255, 0, 0));

                    let mut cosa = outer_quad.linearize_quad_tree(&colors_collection, 2).0.as_str().to_owned();
                    println!("{}", cosa);

                    let mut num_array: Vec<u8> = Vec::new();
                    let mut buff = String::new();
                    // Agregar el padding necesario para que este bien alineado
                    cosa += &"0".repeat(8 - (cosa.len() % 8));
                    println!("{cosa}");

                    for i in 0..cosa.len() {
                        if (i % 8 == 0 && i != 0) || (i == cosa.len() - 1) {
                            num_array.push(u8::from_str_radix(&buff, 2).unwrap().try_into().unwrap());
                            print!("{buff}, ");
                            buff = String::from(cosa.as_bytes()[i] as char);
                        } else {
                            buff += &String::from(cosa.as_bytes()[i] as char);
                        }
                    }
                }

                _ => {}
            }
        }

        // * Dibujar las lineas para la cuadricula
        canvas.set_draw_color(Color::RGB(255, 255, 255));

        // outer_quad.draw(&mut canvas);
        quad.draw(&mut canvas);

        // * Ahora si dibujar en la pantalla
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}