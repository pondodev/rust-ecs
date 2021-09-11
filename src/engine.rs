extern crate sdl2;

use sdl2::EventPump;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;

use rand::Rng;

use crate::entity_manager::*;
use crate::component_manager::ComponentManager;

#[allow(dead_code, unused_variables)]
pub struct Engine {
    window_width: u32,
    window_height: u32,
    canvas: Canvas<Window>,
    event_pump: EventPump,
    entities: Vec<Entity>,
    em: EntityManager,
    cm: ComponentManager,
}

impl Engine {
    pub fn new(window_width: u32, window_height: u32) -> Result<Self, ()> {
        let sdl_context = sdl2::init().expect("failed to init sdl");

        let video_subsystem = sdl_context
            .video()
            .expect("failed to init video subsystem");

        let window = video_subsystem
            .window("rust ecs", window_width, window_height)
            .position_centered()
            .build()
            .expect("failed to init window");

        let canvas = window.into_canvas()
            .accelerated()
            .present_vsync()
            .build()
            .expect("failed to init canvas");

        let event_pump = sdl_context.event_pump().unwrap();

        let mut em = EntityManager::new();
        let mut cm = ComponentManager::new();
        let mut entities: Vec<Entity> = Vec::new();

        let mut rng = rand::thread_rng();
        for _ in 0..MAX_ENTITIES {
            match em.create_entity() {
                Some(entity) => {
                    entities.push(entity);
                    cm.register_position(
                        entity,
                        rng.gen_range(0..window_width) as f32,
                        rng.gen_range(0..window_height) as f32);

                    cm.register_velocity(
                        entity,
                        rng.gen_range(-5.0..5.0),
                        rng.gen_range(-5.0..5.0));

                    cm.register_color(
                        entity,
                        rng.gen(),
                        rng.gen(),
                        rng.gen());
                },
                None => (),
            };
        }

        Ok(Engine {
            window_width,
            window_height,
            canvas,
            event_pump,
            entities,
            em,
            cm,
        })
    }

    pub fn run(&mut self) {
        'running: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } |
                    Event::KeyDown { keycode : Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    _ => { }
                }
            }

            self.update();
            self.draw();
        }
    }

    fn update(&mut self) {
        let w = self.window_width as f32;
        let h = self.window_height as f32;

        for i in 0..self.entities.len() {
            let mut pos = self.cm.get_position(self.entities[i]);
            let vel = self.cm.get_velocity(self.entities[i]);

            pos.x += vel.x;
            pos.y += vel.y;

            // loop around edges of screen
            if pos.x < 0.0 { pos.x += w }
            if pos.x > w { pos.x -= w }
            if pos.y < 0.0 { pos.y += h }
            if pos.y > h { pos.y -= h }

            self.cm.set_position(self.entities[i], pos);
        }
    }

    fn draw(&mut self) {
        self.canvas.set_draw_color(Color::RGB(30, 30, 30));
        self.canvas.clear();

        for i in 0..self.entities.len() {
            let pos = self.cm.get_position(self.entities[i]);
            let col = self.cm.get_color(self.entities[i]);

            let r = Rect::new(pos.x as i32, pos.y as i32, 10, 10);
            self.canvas.set_draw_color(Color::RGB(col.r, col.g, col.b));
            self.canvas.fill_rect(r).expect("failed to draw rect");
        }

        self.canvas.present();
    }
}
