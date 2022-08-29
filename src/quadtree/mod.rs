use std::collections::HashMap;

use glm::TVec2;
use glm::vec2;

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::{Point, Rect};
use sdl2::pixels::Color;

const MAX_DEPTH: u32 = 3;

const COLORS: [Color; 4] = [
    Color::RGB(255, 0, 0),
    Color::RGB(0, 255, 0),
    Color::RGB(0, 0, 255),
    Color::RGB(255, 255, 255)
];

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

        // Disgusting but works for now
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

    // TODO: checar que deberia regresar esta funcion para que se pueda usar en mas situaciones
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

            // TODO: reescribirla
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

    fn fill_quad(&mut self, mask: usize) {
        let color = COLORS[mask];
        println!("filling shit");

        if self.depth < MAX_DEPTH {
            // Si no existe, crear una subdivision y bajar mas
            if self.ul.is_none() {
                self.create_sub_quad(1, color);
            } else {
                for i in 1..5 {
                    self.ul.as_mut().unwrap().create_sub_quad(i, color);
                }
            }
            self.ul.as_mut().unwrap().fill_quad(mask);

            // Si no existe, crear una subdivision y bajar mas
            if self.ur.is_none() {
                self.create_sub_quad(2, color);
            } else {
                for i in 1..5 {
                    self.ur.as_mut().unwrap().create_sub_quad(i, color);
                }
            }
            self.ur.as_mut().unwrap().fill_quad(mask);

            // Si no existe, crear una subdivision y bajar mas
            if self.ll.is_none() {
                self.create_sub_quad(3, color);
            } else {
                for i in 1..5 {
                    self.ll.as_mut().unwrap().create_sub_quad(i, color);
                }
            }
            self.ll.as_mut().unwrap().fill_quad(mask);

            // Si no existe, crear una subdivision y bajar mas
            if self.lr.is_none() {
                self.create_sub_quad(4, color);
            } else {
                for i in 1..5 {
                    self.lr.as_mut().unwrap().create_sub_quad(i, color);
                }
            }
            self.lr.as_mut().unwrap().fill_quad(mask);

        } else {
            // Si ya llegamos al punto mas bajo, solo llenar el quad en el que estamos
            self.color = Some(color);
        }
    }

    fn debug_print_linear_tree_left(tree_vector: &Vec<bool>, index: &usize) {
        for i in *index..tree_vector.len() {
            if tree_vector[i] {
                print!("1");
            } else {
                print!("0");
            }
        }
        print!("\n");
    }

    pub fn get_quadtree_from_linear(&mut self, tree_vector: &Vec<bool>, index: &mut usize) {
        QuadTree::debug_print_linear_tree_left(tree_vector, index);
        println!("{}", self.depth);

        // Si ya estamos en el penultimo quad, solo crear los de abajo
        if self.depth == MAX_DEPTH - 1 {

            // No es bonito, pero tambien tenemos que checar el caso en el que este penultimo
            // quad este lleno
            if tree_vector[*index] && tree_vector[*index + 1] {
                let mut mask: usize = 0;
                if tree_vector[*index + 1] {
                    mask += 1;
                }
                if tree_vector[*index + 2] {
                    mask += 2;
                }
                *index += 4;
                self.fill_quad(mask);

            } else if tree_vector[*index] && !tree_vector[*index + 1] {
                *index += 2;

                for i in 1..5 {
                    // Solo hacemos algo si el siguiente valor no es 0
                    if tree_vector[*index] {
                        // Si si hay algo checamos los siguiente dos valores
                        let mut mask: usize = 0;
                        if tree_vector[*index + 1] {
                            mask += 1;
                        }
                        if tree_vector[*index + 2] {
                            mask += 2;
                        }
                        *index += 3;

                        self.create_sub_quad(i, COLORS[mask]);
                    } else {
                        *index += 1;
                    }
                }
            } else {
                *index += 1;
            }

        } else {
            if !tree_vector[*index] { // Si sabemos que este es el ultimo quad, solo saltar al siguiente index
                *index += 1;

            } else {
                if tree_vector[*index + 1] {
                    // Si el siguiente tambien es un 1 sabemos que esta lleno
                    // Solo checar cual es la feature con la que vamos a llenar todo
                    // TODO: Hacer que este chequeo sea para n bits
                    let mut mask: usize = 0;
                    if tree_vector[*index + 2] {
                        mask += 1;
                    }
                    if tree_vector[*index + 3] {
                        mask += 2;
                    }

                    self.fill_quad(mask);
                    *index += 4;
                } else {
                    // Ahora sabemos que hay algo pero no es homogeneo hasta abajo,
                    // hay que seguir bajando
                    *index += 2;

                    // TODO: refactorizar esto con una macro
                    if tree_vector[*index] {
                        self.create_sub_quad(1, Color::RGB(0, 0, 0));
                        self.ul.as_mut().unwrap().get_quadtree_from_linear(tree_vector, index);
                    } else {
                        *index += 1;
                    }

                    if tree_vector[*index] {
                        self.create_sub_quad(2, Color::RGB(0, 0, 0));
                        self.ur.as_mut().unwrap().get_quadtree_from_linear(tree_vector, index);
                    } else {
                        *index += 1;
                    }

                    if tree_vector[*index] {
                        self.create_sub_quad(3, Color::RGB(0, 0, 0));
                        self.ll.as_mut().unwrap().get_quadtree_from_linear(tree_vector, index);
                    } else {
                        *index += 1;
                    }

                    if tree_vector[*index] {
                        self.create_sub_quad(4, Color::RGB(0, 0, 0));
                        self.lr.as_mut().unwrap().get_quadtree_from_linear(tree_vector, index);
                    } else {
                        *index += 1;
                    }

                }
            }
        }
    }
}

// TODO hacer que esta funcion sea compatible con la gpu
// TODO     - No puede usar recursion
// TODO     - Debe ser lo mas branchless posible
pub fn traverse_linear_quad_tree() {}