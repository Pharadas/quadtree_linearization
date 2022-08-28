extern crate sdl2;
extern crate nalgebra_glm as glm;

use glm::TVec2;
// use sdl2::rec&mut t::{Rect, Point};
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use glm::vec2;

mod quadtree;
use std::collections::HashMap;
use quadtree::QuadTree;
use quadtree::get_quad_tree_from_linear;

pub fn main() {
    // let mut item_to_push: u32 = u32::MAX;
    // let mut and_int: u32 = 0x7FFFFFFF;
    // let fixer_int: u32 = 0x80000000;

    // for _i in 0..3 {
    //     println!("||{:32b}", and_int);
    //     and_int = and_int >> 1;
    //     and_int |= fixer_int;
    // }

    // item_to_push &= and_int;

    // println!("{:32b}", item_to_push);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut mouse_state;

    let window_dimensions: TVec2<u32> = vec2(800, 600);

    let mut outer_quad = QuadTree::new(vec2(0, 0), vec2(window_dimensions.x as i32, window_dimensions.y as i32));

    let test_pos = vec![
        vec2(460, 30),
        vec2(669, 114),
        vec2(667, 171),
        vec2(664, 250),
        vec2(720, 241),
        vec2(720, 198),
        vec2(659, 378),
        vec2(629, 356),
        vec2(690, 344),
        vec2(713, 343),
        vec2(716, 403),
        vec2(733, 519),
        vec2(676, 493),
        vec2(679, 548),
        vec2(724, 541),
        vec2(542, 549),
        vec2(466, 566),
        vec2(448, 483),
        vec2(514, 476),
        vec2(515, 405),
        vec2(462, 428),
        vec2(456, 355),
        vec2(513, 367),
    ];

    let test_positions = vec![
        (vec2(53, 47), Color::RGB(255, 0, 0)),
        (vec2(141, 100), Color::RGB(255, 0, 0)),
        (vec2(237, 106), Color::RGB(255, 0, 0)),
        (vec2(342, 104), Color::RGB(255, 0, 0)),
        (vec2(335, 40), Color::RGB(255, 0, 0)),
        (vec2(301, 43), Color::RGB(255, 0, 0)),
        (vec2(273, 39), Color::RGB(255, 0, 0)),
        (vec2(252, 190), Color::RGB(255, 0, 0)),
        (vec2(336, 193), Color::RGB(255, 0, 0)),
        (vec2(335, 255), Color::RGB(255, 0, 0)),
        (vec2(271, 258), Color::RGB(255, 0, 0)),
        (vec2(456, 241), Color::RGB(255, 0, 0)),
        (vec2(472, 185), Color::RGB(255, 0, 0)),
        (vec2(531, 183), Color::RGB(255, 0, 0)),
        (vec2(539, 236), Color::RGB(255, 0, 0)),
        (vec2(661, 106), Color::RGB(255, 0, 0)),
        (vec2(749, 111), Color::RGB(255, 0, 0)),
        (vec2(743, 44), Color::RGB(255, 0, 0)),
        (vec2(663, 45), Color::RGB(255, 0, 0)),
        (vec2(542, 19), Color::RGB(255, 255, 255)),
        (vec2(552, 96), Color::RGB(255, 255, 255)),
        (vec2(469, 84), Color::RGB(255, 255, 255)),
        (vec2(459, 27), Color::RGB(255, 255, 255)),
        (vec2(644, 178), Color::RGB(255, 255, 255)),
        (vec2(724, 191), Color::RGB(255, 255, 255)),
        (vec2(722, 243), Color::RGB(255, 255, 255)),
        (vec2(656, 250), Color::RGB(255, 255, 255)),
        (vec2(646, 355), Color::RGB(255, 255, 255)),
        (vec2(720, 349), Color::RGB(255, 255, 255)),
        (vec2(729, 403), Color::RGB(255, 255, 255)),
        (vec2(679, 411), Color::RGB(255, 255, 255)),
        (vec2(717, 474), Color::RGB(255, 255, 255)),
        (vec2(660, 474), Color::RGB(255, 255, 255)),
        (vec2(734, 565), Color::RGB(255, 255, 255)),
        (vec2(660, 553), Color::RGB(255, 255, 255)),
        (vec2(550, 552), Color::RGB(255, 255, 255)),
        (vec2(451, 550), Color::RGB(255, 255, 255)),
        (vec2(453, 483), Color::RGB(255, 255, 255)),
        (vec2(539, 489), Color::RGB(255, 255, 255)),
        (vec2(527, 411), Color::RGB(255, 255, 255)),
        (vec2(429, 410), Color::RGB(255, 255, 255)),
        (vec2(439, 364), Color::RGB(255, 255, 255)),
        (vec2(523, 363), Color::RGB(255, 255, 255)),
        (vec2(382, 340), Color::RGB(255, 255, 255)),
        (vec2(263, 341), Color::RGB(255, 255, 255)),
        (vec2(261, 398), Color::RGB(255, 255, 255)),
        (vec2(330, 405), Color::RGB(255, 255, 255)),
        (vec2(333, 514), Color::RGB(0, 255, 0)),
        (vec2(340, 549), Color::RGB(0, 255, 0)),
        (vec2(265, 549), Color::RGB(0, 255, 0)),
        (vec2(245, 487), Color::RGB(0, 255, 0)),
        (vec2(165, 16), Color::RGB(0, 255, 0)),
        (vec2(65, 109), Color::RGB(255, 255, 255)),
        (vec2(67, 177), Color::RGB(255, 255, 255)),
        (vec2(152, 292), Color::RGB(255, 0, 0)),
        (vec2(131, 208), Color::RGB(0, 0, 255)),
        (vec2(58, 286), Color::RGB(0, 0, 255)),
        (vec2(62, 354), Color::RGB(0, 0, 255)),
        (vec2(63, 392), Color::RGB(0, 0, 255)),
        (vec2(66, 481), Color::RGB(0, 0, 255)),
        (vec2(69, 545), Color::RGB(0, 0, 255)),
        (vec2(143, 540), Color::RGB(0, 0, 255)),
        (vec2(136, 469), Color::RGB(0, 0, 255)),
        (vec2(130, 413), Color::RGB(0, 0, 255)),
        (vec2(130, 352), Color::RGB(0, 0, 255)),
    ];

    // let shit = vec![
    //     (vec2(85, 69), Color::RGB(255, 0, 0)),
    //     (vec2(266, 64), Color::RGB(0, 255, 0)),
    //     (vec2(103, 227), Color::RGB(0, 0, 255)),
    //     (vec2(258, 220), Color::RGB(255, 255, 255)),
    //     (vec2(107, 372), Color::RGB(255, 0, 0)),
    //     (vec2(108, 501), Color::RGB(255, 0, 0)),
    //     (vec2(236, 498), Color::RGB(255, 0, 0)),
    //     (vec2(250, 363), Color::RGB(255, 0, 0)),
    // ];

    let mut colors_collection: HashMap<Color, &str> = HashMap::new();

    colors_collection.insert(Color::RGB(255, 0, 0), "00");
    colors_collection.insert(Color::RGB(0, 255, 0), "01");
    colors_collection.insert(Color::RGB(0, 0, 255), "10");
    colors_collection.insert(Color::RGB(255, 255, 255), "11");

    // colors_collection.insert(Color::RGB(125, 125, 0), "000");
    // colors_collection.insert(Color::RGB(0, 255, 0), "001");
    // colors_collection.insert(Color::RGB(0, 0, 100), "010");
    // colors_collection.insert(Color::RGB(50, 50, 150), "011");
    // colors_collection.insert(Color::RGB(255, 192, 203), "100");
    // colors_collection.insert(Color::RGB(150, 150, 0), "101");

    // for vec in test_pos {
    //     outer_quad.divide_at_position(vec.0.x, vec.0.y, vec.1);
    // }

    for vec in test_positions {
        outer_quad.divide_at_position(vec.0.x, vec.0.y, vec.1);
    }

    // for vec in shit {
    //     outer_quad.divide_at_position(vec.0.x, vec.0.y, vec.1);
    // }

    // let cosa = outer_quad.linearize_quad_tree(&colors_collection).0.as_str().to_owned();
    let mut cosa = outer_quad.linearize_quad_tree(&colors_collection, 2).0.as_str().to_owned();
    // let val = "10101010010111110011001011111011010011001011111100110011111011101111111011011111";
    // let mut cosa = "10101010010111110011001011111011010011001011111100110011111011101111111011011111".to_string();
    // let val = "1010100101110111011000";
    // println!("unit test:        {val}");
    // println!("calculated value: {}, {}", cosa, cosa.len());

    let mut buff = String::from("");
    let mut num_array: Vec<u8> = Vec::new();

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

    println!("\n");
    // println!("{:?}, {}", num_array, num_array.len());
    let holy_shit = num_array.clone();
    for i in holy_shit {
        print!("{:b}, ", i);
    }
    print!("\n");

    // print!("\n");
    // // shift_linear_quad_tree(&mut num_array, 10);

    // Ahora que tenemos los bits como booleanos, podemos manipular
    // mas facilmente esto
    let mut linearized_tree_vec: Vec<bool> = Vec::new();
    let mut mask = 0;

    for i in num_array {
        for x in 0..8 {
            mask = u8::pow(2, 7 - x as u32);
            println!("{:b}", mask);
            println!("{:b}", i);
            println!("{:b}", i & mask);
            linearized_tree_vec.push(i & mask != 0);
        }
    }

    // for i in linearized_tree_vec {
    //     if i {
    //         print!("1");
    //     } else {
    //         print!("0");
    //     }
    // }

    let mut quad = QuadTree::new(vec2(0, 0), vec2(window_dimensions.x as i32, window_dimensions.y as i32));
    get_quad_tree_from_linear(&mut quad, &linearized_tree_vec, 0, Color::RGB(0, 0, 0), false, 0, 1);

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

                // Event::KeyDown { keycode: Some(Keycode::L), .. } => {
                //     outer_quad.divide_at_position(mouse_state.x(), mouse_state.y(), Color::RGB(125, 125, 0));
                //     println!("{}, {}, Color::RGB(0, 255, 0)", mouse_state.x(), mouse_state.y())
                // },

                // Event::KeyDown { keycode: Some(Keycode::P), .. } => {
                //     outer_quad.divide_at_position(mouse_state.x(), mouse_state.y(), Color::RGB(0, 255, 0));
                //     println!("{}, {}, Color::RGB(0, 0, 255)", mouse_state.x(), mouse_state.y())
                // }

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
                    // println!("{}, {}, Color::RGB(255, 0, 0)", mouse_state.x(), mouse_state.y());
                    // println!("{}, {}", mouse_state.x(), mouse_state.y());

                    let cosa = outer_quad.linearize_quad_tree(&colors_collection, 2).0.as_str().to_owned();
                    // println!("shitass: {}", "101011010110001110011011110011110110110101100011100101010100111011000".len());
                    println!("{}", cosa);
                }

                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    observer_position.x = mouse_state.x();
                    observer_position.y = mouse_state.y();
                }

                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    ray_position.x = mouse_state.x();
                    ray_position.y = mouse_state.y();
                }

                _ => {}
            }
        }

        // * Dibujar las lineas para la cuadricula
        canvas.set_draw_color(Color::RGB(255, 255, 255));

        // let current_quad_observer_position = outer_quad.get_quad_from_num(outer_quad.get_quad_at_pos(observer_position.x, observer_position.y)).as_ref().unwrap().as_ref();
        // let observer_ray_vector = ray_position - observer_position;
        // let clamped_vec: TVec2<i32>;
        // if observer_ray_vector.x > current_quad_observer_position.border_lr.x {
        //     // clamped_vec = 
        // }
        // // println!("{}", ((observer_ray_vector.y / observer_position.x) as f64).tan());

        // canvas.draw_rect(Rect::new(observer_position.x, observer_position.y, 10, 10)).unwrap();
        // canvas.draw_line(Point::new(observer_position.x, observer_position.y), Point::new(ray_position.x, ray_position.y)).unwrap();

        outer_quad.draw(&mut canvas);

        // * Ahora si dibujar en la pantalla
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}