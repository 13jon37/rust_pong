extern crate sdl2;

use rand::Rng;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::rect::{Rect};
use std::time::Duration;

const WIND_WIDTH: u32 = 800;
const WIND_HEIGHT: u32 = 600;

const PADDLE_WIDTH: u32 = 128;
const PADDLE_HEIGHT: u32 = 32;

const PONG_SIZE: u32 = 16;

enum EntityType {
    Player,
    Pong,
    Bot,
}

enum Movement {
    Left,
    Right,
    Idle,
}

struct Entity {
    entity_type: EntityType,
    movement: Movement,
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    speed: i32,
}

impl Entity {
    fn new(entity_type: EntityType, x: i32, y: i32, w: u32, h: u32, speed: i32) -> Self {
        Self {
            entity_type: entity_type,
            movement: Movement::Idle,
            x: x,
            y: y,
            w: w,
            h: h,
            speed: speed,
        }
    }

    fn update(&mut self, pong_pos_x: &i32) {
        // Movment
        match self.entity_type {
            EntityType::Player => {
                match self.movement {
                    Movement::Left => {
                        self.x -= self.speed;
                    },
                    Movement::Right => {
                        self.x += self.speed;
                    },
                    _ => {}
                }
            },
            EntityType::Pong => {
                // self.x -= self.speed;
                self.y += self.speed;
            },
            EntityType::Bot => {
                self.x = *pong_pos_x;
            }
        }
    }

    fn render(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.fill_rect(Rect::new(self.x, self.y, self.w, self.h))?;
        Ok(())
    }
}

fn collision(entities: &mut Vec<Entity>, player: usize, pong: usize, bot: usize) {
    /* Collision */
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(4..7);
    // Player and Pong Collision
    if entities[player].x < entities[pong].x + PONG_SIZE as i32 &&
        entities[player].x + entities[player].w as i32 > entities[pong].x &&
        entities[player].y < entities[pong].y + PONG_SIZE as i32 &&
        entities[player].y + entities[player].h as i32 > entities[pong].y
    {
        println!("Player and Pong collision");
        entities[pong].speed *= -1;
        if entities[pong].speed < 0 {
            entities[pong].speed = -random_number;
        }
        if entities[pong].speed > 0 {
            entities[pong].speed = random_number;
        }
    }

    // Bot and Pong Collision
    if entities[bot].x < entities[bot].x + PONG_SIZE as i32 &&
        entities[bot].x + entities[bot].w as i32 > entities[pong].x &&
        entities[bot].y < entities[pong].y + PONG_SIZE as i32 &&
        entities[bot].y + entities[bot].h as i32 > entities[pong].y
    {
        println!("Bot and Pong collision");
        entities[pong].speed *= -1;
        if entities[pong].speed < 0 {
            entities[pong].speed = -random_number;
        }
        if entities[pong].speed > 0 {
            entities[pong].speed = random_number;
        }
    }

    // Player Window Collision
    if entities[player].x <= 0 {
        entities[player].x = 0;
    }
    if entities[player].x >= (WIND_WIDTH - entities[player].w) as i32 {
        entities[player].x = (WIND_WIDTH - entities[player].w) as i32;
    }

    // Pong Window Collision
    if entities[pong].x <= 0 {
        entities[pong].speed *= -1;
    }
    if entities[pong].x + entities[pong].w as i32 >= WIND_WIDTH as i32 {
        entities[pong].speed *= -1;
    }
    if entities[pong].y <= 0 {
        entities[pong].speed *= -1;
    }
    if entities[pong].y + entities[pong].h as i32 >= WIND_HEIGHT as i32 {
        entities[pong].speed *= -1;
    }

    // Bot Window Collision
    if entities[bot].x <= 0 {
        entities[bot].x = 0;
    }
    if entities[bot].x >= (WIND_WIDTH - entities[bot].w) as i32 {
        entities[bot].x = (WIND_WIDTH - entities[bot].w) as i32;
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("Pong", WIND_WIDTH, WIND_HEIGHT)
        .position_centered()
        .build()
        .expect("Could not init video subsystem.");

    let mut canvas = window.into_canvas().build()
        .expect("Could not create window canvas.");

    let mut event_pump = sdl_context.event_pump()?;

    let mut entities: Vec<Entity> = Vec::new();

    let player: Entity = Entity::new(EntityType::Player,
                                     (WIND_WIDTH / 2) as i32 - (PADDLE_WIDTH / 2) as i32,
                                     WIND_HEIGHT as i32 - (PADDLE_WIDTH / 2) as i32,
                                     PADDLE_WIDTH,
                                     PADDLE_HEIGHT,
                                     5);

    let pong: Entity = Entity::new(EntityType::Pong,
                                   (WIND_WIDTH as i32 / 2) - PONG_SIZE as i32,
                                   (WIND_HEIGHT as i32 / 2) - PONG_SIZE as i32,
                                   PONG_SIZE,
                                   PONG_SIZE,
                                   7);

    let bot: Entity = Entity::new(EntityType::Bot,
                                  (WIND_WIDTH / 8) as i32 - (PADDLE_WIDTH / 2) as i32,
                                  (WIND_HEIGHT / 8) as i32 - (PADDLE_HEIGHT) as i32,
                                  PADDLE_WIDTH,
                                  PADDLE_HEIGHT,
                                  5);

    entities.push(player);
    entities.push(pong);
    entities.push(bot);

    const PLAYER: usize = 0;
    const PONG: usize = 1;
    const BOT: usize = 2;

    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
        match event {
            Event::Quit {..} =>  {break 'running},
                    Event::KeyDown { keycode: Some(Keycode::Left), repeat: false, .. } => {
                    entities[PLAYER].movement = Movement::Left;
                },
                Event::KeyDown { keycode: Some(Keycode::Right), repeat: false, .. } => {
                    entities[PLAYER].movement = Movement::Right;
                },
                Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, .. } |
                    Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, .. } => {
                    entities[PLAYER].movement = Movement::Idle;
                },
                _ => {}
            }
        }

        // Update
        let pong_x = entities[PONG].x;
        for i in 0..entities.len() {
            entities[i].update(&pong_x);
        }
        collision(&mut entities, PLAYER, PONG, BOT);

        // Render
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        for i in 0..entities.len() {
            entities[i].render(&mut canvas)?;
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
