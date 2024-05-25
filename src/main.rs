use std::time;

use tiny_game_framework::{rand_betw, Circle, Quad, Renderer};

use tiny_game_framework::EventLoop;
use tiny_game_framework::glam::{vec2, vec3, vec4, Vec3};

use tiny_game_framework::gl::*;
use tiny_game_framework::glfw::Key;

fn main() {
    let mut renderer = Renderer::new();
    let resolution = vec2(800., 800.);
    let mut el = EventLoop::new(resolution.x as u32, resolution.y as u32);
    
    let ship = tiny_game_framework::Circle::new(16, 0.05, vec4(1., 0., 0., 1.));
    ship.add_to_renderer("ship", &mut renderer);
    let speed = 300.;
    let hp_max = 100.;
    let mut hp = hp_max;
    let mut dead = false;

    let mut asteroids: Vec<Asteroid> = vec![];
    let mut asteroid_count = 0;

    // setting up the health bar ------------------------------------------------
    let health_pos = vec3(-350., 300., 0.);

    let health_back = Quad::new(vec3(0.3, 1./10., 0.), vec4(1., 0., 0., 1.));
    health_back.add_to_renderer("health_back", &mut renderer);
    let health_fill = Quad::new(vec3(0.3, 1./10., 0.), vec4(0., 1., 0., 1.));
    health_fill.add_to_renderer("health_fill", &mut renderer);

    renderer.get_mesh_mut("health_back").unwrap().position = vec3(health_pos.x, health_pos.y - 5., 0.);
    renderer.get_mesh_mut("health_fill").unwrap().position = health_pos;

    let mut cooldown = 0.;
    // --------------------------------------------------------------------------

    let mut dt = 0.;
    let mut time = 0.;
    while !el.window.should_close() {
        let now = time::Instant::now();

        el.update();

        if !dead{
            handle_keys(speed, dt, &mut el, &mut renderer);

            // update each asteroid and check if it's collided
            let pos = renderer.get_mesh("ship").unwrap().position;
            for asteroid in asteroids.iter_mut(){
                asteroid.follow(&mut renderer, pos, dt);
                // check if asteroid collided AND if last collision was at lest 0.7 seconds ago
                if asteroid.check_collision(pos, &mut renderer) && time - cooldown > 0.7{
                    hp -= 10.;
                    println!("Took a hit! - {}", hp);
                    if hp <= 0. { hp = 0.; dead = true; }
                    else{
                        cooldown = time;

                        let mesh = renderer.get_mesh_mut("health_fill").unwrap();
                        mesh.scale = vec3(hp/hp_max, 1., 1.);
                    }
                }
            }

            // 1% chance of generating an asteroid per game tick
            if rand_betw(0., 1.) > 0.99{
                asteroid_count += 1;
                asteroids.push(Asteroid::new(asteroid_count, rand_betw(0.02, 0.07), &mut renderer));
            }
        }
        else{
            // complete disgrace and caos
            renderer.get_mesh_mut("ship").unwrap().scale = vec3(rand_betw(0., 5.), rand_betw(0., 5.), 1.);
        }

        unsafe{
            Clear(COLOR_BUFFER_BIT);
            
            renderer.draw(&el);
        }
        dt = now.elapsed().as_secs_f32();
        time += dt;
    }
}

pub fn handle_keys(speed: f32, dt: f32, el: &mut EventLoop, renderer: &mut Renderer){
    let s = renderer.get_mesh_mut("ship").unwrap();
        if el.is_key_down(Key::W){
            s.position += vec3(0., 1., 0.)*speed*dt;
        }
        if el.is_key_down(Key::S){
            s.position -= vec3(0., 1., 0.)*speed*dt;
        }
        if el.is_key_down(Key::D){
            s.position += vec3(1., 0., 0.)*speed*dt;
        }
        if el.is_key_down(Key::A){
            s.position -= vec3(1., 0., 0.)*speed*dt;
        }
}

struct Asteroid{
    i: i32,
    radius: f32,
    dead: bool,
}

impl Asteroid{
    pub fn new(i: i32, radius: f32, renderer: &mut Renderer) -> Self{
        let c = Circle::new(16, radius, vec4(rand_betw(0., 1.), rand_betw(0., 1.), rand_betw(0., 1.), 1.));
        c.add_to_renderer(&format!("{}", i), renderer);

        // create asteroid mesh
        let a = renderer.get_mesh_mut(&format!("{}", i)).unwrap();

            let n = rand_betw(0., std::f32::consts::PI*2.);

            a.position = vec3(n.cos(), n.sin(), 0.)*800.;

        Self{
            i,
            radius,
            dead: false,
        }
    }

    pub fn check_collision(&mut self, position: Vec3, renderer: &mut Renderer) -> bool{
        // for some reason the radius is VERY messed up
        if position.distance(renderer.get_mesh(&format!("{}", self.i)).unwrap().position) < self.radius*600.{
            self.dead = true;
            true
        }
        else{
            false
        }
    }

    // drive asteroid into the ship's position
    pub fn follow(&self, renderer: &mut Renderer, position: Vec3, dt: f32){
        if !self.dead{
            let mesh = renderer.get_mesh_mut(&format!("{}", self.i)).unwrap();
        mesh.position += (position-mesh.position)*dt;
        }
    }
}

// TODO: shooting to kill asteroids
