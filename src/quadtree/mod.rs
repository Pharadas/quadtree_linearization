use std::collections::HashMap;

use glm::TVec2;
use glm::vec2;

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::{Point, Rect};
use sdl2::pixels::Color;

use bitvec::prelude::*;

const MAX_DEPTH: u32 = 3;

const COLORS: [Color; 4] = [Color::RGB(255, 0, 0), Color::RGB(0, 255, 0), Color::RGB(0, 0, 255), Color::RGB(255, 255, 255)];

pub struct QuadTree {
    pub border_ul: TVec2<i32>,
    pub border_lr: TVec2<i32>,
    depth: u32,
    color: Option<Color>,
    ul: Option<Box<QuadTree>>,
    ur: Option<Box<QuadTree>>,
    ll: Option<Box<QuadTree>>,
    lr: Option<Box<QuadTree>>
}

impl QuadTree {
    pub fn new(b_ul: TVec2<i32>, b_lr: TVec2<i32>) -> QuadTree {
        QuadTree { border_ul: b_ul, border_lr: b_lr, depth: 0, color: None, ul: None, ur: None, ll: None, lr: None}
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        if !self.color.is_none() {
            canvas.set_draw_color(Color::RGB(255, 255, 255));
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
            if self.depth == MAX_DEPTH - 1 {
                let cosa = self.ul.as_ref().unwrap().color.as_ref().unwrap();
                canvas.set_draw_color(*cosa);
                canvas.fill_rect(Rect::new(
                    self.ul.as_ref().unwrap().border_ul.x,
                    self.ul.as_ref().unwrap().border_ul.y,
                    (self.ul.as_ref().unwrap().border_lr.x - self.ul.as_ref().unwrap().border_ul.x) as u32,
                    (self.ul.as_ref().unwrap().border_lr.y - self.ul.as_ref().unwrap().border_ul.y) as u32)).unwrap();
            }
            self.ul.as_ref().unwrap().draw(canvas);
        }

        if !self.ur.is_none() {
            if self.depth == MAX_DEPTH - 1 {
                let cosa = self.ur.as_ref().unwrap().color.as_ref().unwrap();
                canvas.set_draw_color(*cosa);
                canvas.fill_rect(Rect::new(
                    self.ur.as_ref().unwrap().border_ul.x,
                    self.ur.as_ref().unwrap().border_ul.y,
                    (self.ur.as_ref().unwrap().border_lr.x - self.ur.as_ref().unwrap().border_ul.x) as u32,
                    (self.ur.as_ref().unwrap().border_lr.y - self.ur.as_ref().unwrap().border_ul.y) as u32)).unwrap();
            }
            self.ur.as_ref().unwrap().draw(canvas);
        }

        if !self.ll.is_none() {
            if self.depth == MAX_DEPTH - 1 {
                let cosa = self.ll.as_ref().unwrap().color.as_ref().unwrap();
                canvas.set_draw_color(*cosa);
                canvas.fill_rect(Rect::new(
                    self.ll.as_ref().unwrap().border_ul.x,
                    self.ll.as_ref().unwrap().border_ul.y,
                    (self.ll.as_ref().unwrap().border_lr.x - self.ll.as_ref().unwrap().border_ul.x) as u32,
                    (self.ll.as_ref().unwrap().border_lr.y - self.ll.as_ref().unwrap().border_ul.y) as u32)).unwrap();
            }
            self.ll.as_ref().unwrap().draw(canvas);
        }

        if !self.lr.is_none() {
            if self.depth == MAX_DEPTH - 1 {
                let cosa = self.lr.as_ref().unwrap().color.as_ref().unwrap();
                canvas.set_draw_color(*cosa);
                canvas.fill_rect(Rect::new(
                    self.lr.as_ref().unwrap().border_ul.x,
                    self.lr.as_ref().unwrap().border_ul.y,
                    (self.lr.as_ref().unwrap().border_lr.x - self.lr.as_ref().unwrap().border_ul.x) as u32,
                    (self.lr.as_ref().unwrap().border_lr.y - self.lr.as_ref().unwrap().border_ul.y) as u32)).unwrap();
            }
            self.lr.as_ref().unwrap().draw(canvas);
        }
    }

    pub fn linearize_quad_tree(&self, map: &HashMap<Color, &str>, bits_for_feature: usize) -> (String, bool) {
        let mut linearized_quad_string = String::from("");
        let mut all_equal = true;
        let mut last_value: String = "".to_owned();

        // Si ya estamos checando los ultimos quads
        if self.depth == MAX_DEPTH - 1 {
            if !self.get_quad_from_num(1).is_none() {
                last_value = map.get(&self.get_quad_from_num(1).as_ref().unwrap().color.unwrap()).unwrap().to_owned().to_string();
            } else {
                all_equal = false;
            }
            for quad_num in 1..5 {
                let current_quad = self.get_quad_from_num(quad_num);

                if current_quad.is_none() {
                    all_equal = false;
                    linearized_quad_string += "0";

                } else {
                    linearized_quad_string += &("1".to_owned() + map.get(&current_quad.as_ref().unwrap().color.unwrap()).unwrap().to_owned());
                    if last_value != map.get(&current_quad.as_ref().unwrap().color.unwrap()).unwrap().to_owned() {
                        all_equal = false;
                    }
                }
            }

        } else {
            for quad_num in 1..5 {
                let current_quad = self.get_quad_from_num(quad_num);

                if current_quad.is_none() || current_quad.as_ref().unwrap().color.is_none() {
                    all_equal = false;
                    linearized_quad_string += "0";
                } else {
                    let pair = current_quad.as_ref().unwrap().linearize_quad_tree(map, bits_for_feature);
                    let str_value = pair.0.clone();

                    // Un poco feo pero si es el primero, namas guardamos
                    // el primer valor
                    if quad_num == 1 {
                        last_value = str_value;
                    } else if last_value != str_value {
                        all_equal = false;
                    }
                    linearized_quad_string += pair.0.as_str();
                    all_equal &= pair.1;
                }
            }
        }

        if all_equal {
            linearized_quad_string = "11".to_owned() + last_value.drain(last_value.len() - bits_for_feature..last_value.len()).as_str();
        } else {
            linearized_quad_string = "10".to_owned() + &linearized_quad_string;
        }

        (linearized_quad_string, all_equal)
    }

    pub fn get_quad_from_num(&self, num: i32) -> &Option<Box<QuadTree>> {
        match num {
            1 => {&self.ul}
            2 => {&self.ur}
            3 => {&self.ll}
            4 => {&self.lr}
            _ => {&self.ul}
        }
    }

    pub fn get_quad_at_pos(&self, x: i32, y: i32) -> i32 {
        // Izquierda
        if x < self.border_ul.x + ((self.border_lr.x - self.border_ul.x) / 2) {
            // Arriba
            if y < self.border_ul.y + ((self.border_lr.y - self.border_ul.y) / 2) {
                return 1
            // Abajo
            } else {
                return 3
            }

        // Derecha
        } else {
            // Arriba
            if y < self.border_ul.y + ((self.border_lr.y - self.border_ul.y) / 2) {
                return 2
            // Abajo
            } else {
                return 4
            }
        }
    }

    pub fn divide_at_position(&mut self, x: i32, y: i32, color: Color) {
        if self.depth < MAX_DEPTH {
            let quad = self.get_quad_at_pos(x, y);

            match quad {
                1 => {
                    if !self.ul.is_none() {
                        self.ul.as_deref_mut().unwrap().divide_at_position(x, y, color);
                    } else {
                        self.create_sub_quad(1, color);
                        if self.depth < MAX_DEPTH {
                            self.ul.as_deref_mut().unwrap().divide_at_position(x, y, color);
                        } else {
                            self.ul.as_deref_mut().unwrap().color = Some(color);
                        }
                    }
                }

                2 => {
                    if !self.ur.is_none() {
                        self.ur.as_deref_mut().unwrap().divide_at_position(x, y, color);
                    } else {
                        self.create_sub_quad(2, color);
                        if self.depth < MAX_DEPTH {
                            self.ur.as_deref_mut().unwrap().divide_at_position(x, y, color);
                        } else {
                            self.ur.as_deref_mut().unwrap().color = Some(color)
                        }
                    }
                }

                3 => {
                    // Abajo
                    if !self.ll.is_none() {
                        self.ll.as_deref_mut().unwrap().divide_at_position(x, y, color);
                    } else {
                        self.create_sub_quad(3, color);
                        if self.depth < MAX_DEPTH {
                            self.ll.as_deref_mut().unwrap().divide_at_position(x, y, color);
                        } else {
                            self.ll.as_deref_mut().unwrap().color = Some(color);
                        }
                    }
                }

                4 => {
                    // Abajo
                    if !self.lr.is_none() {
                        self.lr.as_deref_mut().unwrap().divide_at_position(x, y, color);
                    } else {
                        self.create_sub_quad(4, color);
                        if self.depth < MAX_DEPTH {
                            self.lr.as_deref_mut().unwrap().divide_at_position(x, y, color);
                        } else {
                            self.lr.as_deref_mut().unwrap().color = Some(color);
                        }
                    }
                }

                _ => {}
            }

        }
    }

    pub fn create_sub_quad(&mut self, num: i32, color: Color) {
        self.color = Some(color);
        let mut new_quad = QuadTree {
            border_ul: vec2(0, 0),
            border_lr: vec2(0, 0),
            depth: self.depth + 1,
            color: Some(color),
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

// TODO hacer que esta funcion sea compatible con la gpu
// TODO     - No puede usar recursion
// TODO     - Debe ser lo mas branchless posible

pub fn get_quad_tree_from_linear(mut quad: QuadTree, linearized_tree: &Vec<bool>, mut d: u32, c: Color, bloque_lleno: bool, mut index: usize, mut curr_quad: i32) -> () {
    let curr_depth = d + 1;

    while curr_depth > d {
        if linearized_tree[index] { // Significa que no hay nada dentro del quad que estamos checando en este momento
            index += 1;
            curr_quad += 1;

        } else {
            if linearized_tree[index] && !linearized_tree[index + 1] { // Si hay algo pero no esta lleno, bajar un nivel mas
                quad.create_sub_quad(curr_quad, c);
                quad.get_quad_from_num(curr_quad);

            } else if linearized_tree[index] && linearized_tree[index + 1] { // Si esta completamente lleno bajar hasta el punto mas bajo y colorear todo
                index += 2;
                let mut val = 0;
                // Leer los siguientes n bits (por ahora esta hardcoded para 2), convertirlos a un indice y buscar el feature en la lista COLORS
                // TODO hacer esto para n bits
                if linearized_tree[index] {
                    val += 1;
                } 
                if linearized_tree[index + 1] {
                    val += 2;
                }

                quad.create_sub_quad(curr_quad, COLORS[val]);
                let depth_quad = quad.get_quad_from_num(curr_quad).as_deref().unwrap();
                get_quad_tree_from_linear(*depth_quad, linearized_tree, d, c, bloque_lleno, index, curr_quad);
            }
        }
    }
}