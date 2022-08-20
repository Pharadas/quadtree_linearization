extern crate sdl2;
extern crate nalgebra_glm as glm;

use glm::TVec2;
use sdl2::rect::Point;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use glm::vec2;
use sdl2::render::Canvas;
use sdl2::video::Window;

const DEPTH: u32 = 4;

struct QuadTree {
    border_ul: TVec2<i32>,
    border_lr: TVec2<i32>,
    empty: bool,
    ul: Option<Box<QuadTree>>,
    ur: Option<Box<QuadTree>>,
    ll: Option<Box<QuadTree>>,
    lr: Option<Box<QuadTree>>
}

impl QuadTree {
    fn draw(&self, canvas: &mut Canvas<Window>) {
        if !self.empty {
            canvas.draw_line(
                Point::new(self.border_ul.x + ((self.border_lr.x - self.border_ul.x) / 2) as i32, self.border_ul.y as i32),
                Point::new(self.border_ul.x + ((self.border_lr.x - self.border_ul.x) / 2) as i32, self.border_lr.y as i32)
            ).unwrap();

            canvas.draw_line(
                Point::new(self.border_ul.x, self.border_ul.y + ((self.border_lr.y - self.border_ul.y) / 2) as i32), 
                Point::new(self.border_lr.x as i32, self.border_ul.y + ((self.border_lr.y - self.border_ul.y) / 2) as i32)
            ).unwrap();
        }

        if !self.ul.is_none() {
            self.ul.as_ref().unwrap().draw(canvas);
        }

        if !self.ur.is_none() {
            self.ur.as_ref().unwrap().draw(canvas);
        }

        if !self.ll.is_none() {
            self.ll.as_ref().unwrap().draw(canvas);
        }

        if !self.lr.is_none() {
            self.lr.as_ref().unwrap().draw(canvas);
        }
    }

    fn divide_at_position(&mut self, x: i32, y: i32) {
        // Izquierda
        if x < self.border_ul.x + ((self.border_lr.x - self.border_ul.x) / 2) {
            // Arriba
            if y < self.border_ul.y + ((self.border_lr.y - self.border_ul.y) / 2) {
                if !self.ul.is_none() {
                    self.ul.as_deref_mut().unwrap().divide_at_position(x, y);
                } else {
                    self.create_sub_quad(1);
                }
            } else {
                // Abajo
                if !self.ll.is_none() {
                    self.ll.as_deref_mut().unwrap().divide_at_position(x, y);
                } else {
                    self.create_sub_quad(3);
                }
            }

        // Derecha
        } else {
            // Arriba
            if y < self.border_ul.y + ((self.border_lr.y - self.border_ul.y) / 2) {
                if !self.ur.is_none() {
                    self.ur.as_deref_mut().unwrap().divide_at_position(x, y);
                } else {
                    self.create_sub_quad(2);
                }
            } else {
                // Abajo
                if !self.lr.is_none() {
                    self.lr.as_deref_mut().unwrap().divide_at_position(x, y);
                } else {
                    self.create_sub_quad(4);
                }
            }
        }
    }

    fn create_sub_quad(&mut self, num: i32) {
        self.empty = false;
        let mut new_quad = QuadTree {
            border_ul: vec2(0, 0),
            border_lr: vec2(0, 0),
            empty: true,
            ul: None,
            ur: None,
            ll: None,
            lr: None
        };

        match num {
            1 => { // Upper left
                new_quad.border_ul = self.border_ul;
                new_quad.border_lr.x = self.border_ul.x + ((self.border_lr.x - self.border_ul.x) / 2);
                new_quad.border_lr.y = self.border_ul.y + ((self.border_lr.y - self.border_ul.y) / 2);

                self.ul = Some(Box::new(new_quad));
            }

            2 => { // Upper right
                new_quad.border_ul.x = self.border_ul.x + ((self.border_lr.x - self.border_ul.x) / 2);
                new_quad.border_ul.y = self.border_ul.y;
                new_quad.border_lr.x = self.border_lr.x;
                new_quad.border_lr.y = self.border_ul.y + ((self.border_lr.y - self.border_ul.y) / 2);

                self.ur = Some(Box::new(new_quad));
            }

            3 => { // Lower Left
                new_quad.border_ul.x = self.border_ul.x;
                new_quad.border_ul.y = self.border_ul.y + ((self.border_lr.y - self.border_ul.y) / 2);
                new_quad.border_lr.x = self.border_ul.x + ((self.border_lr.x - self.border_ul.x) / 2);
                new_quad.border_lr.y = self.border_lr.y;

                self.ll = Some(Box::new(new_quad));
            }

            4 => { // Lower Right
                new_quad.border_ul.x = self.border_ul.x + ((self.border_lr.x - self.border_ul.x) / 2);
                new_quad.border_ul.y = self.border_ul.y + ((self.border_lr.y - self.border_ul.y) / 2);
                new_quad.border_lr = self.border_lr;

                self.lr = Some(Box::new(new_quad));
            }

            _ => {}
        }
    }
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut mouse_state;

    let window_dimensions: TVec2<u32> = vec2(800, 600);

    let mut outer_quad = QuadTree {
        border_ul: vec2(0, 0),
        border_lr: vec2(window_dimensions.x as i32, window_dimensions.y as i32),
        empty: true,
        ul: None,
        ur: None,
        ll: None,
        lr: None
    };

    let window = video_subsystem.window("gaming time", window_dimensions.x, window_dimensions.y)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

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

                Event::KeyDown { keycode: Some(Keycode::K), .. } => {
                    outer_quad.divide_at_position(mouse_state.x(), mouse_state.y())
                }

                _ => {}
            }
        }

        // * Dibujar las lineas para la cuadricula
        canvas.set_draw_color(Color::RGB(255, 255, 255));

        outer_quad.draw(&mut canvas);

        // * Ahora si dibujar en la pantalla
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}