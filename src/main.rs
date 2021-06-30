use macroquad::prelude::*;
use macroquad::rand::{gen_range, ChooseRandom};

#[derive(Copy, Clone, PartialEq, Debug)]
struct Particle {
    position: Vec2,
    velocity: Vec2,
    color: Color,
    radius: f32,
    id: usize,
}

impl Particle {
    fn new(radius: f32, color: Color, id: usize) -> Self {
        Particle {
            position: vec2(
                gen_range(0., screen_width()),
                gen_range(0., screen_height()),
            ),
            velocity: vec2(
                gen_range(2.0, 4.0) * vec![-1., 1.].choose().unwrap(),
                gen_range(2.0, 4.0) * vec![-1., 1.].choose().unwrap(),
            ),
            color,
            radius,
            id,
        }
    }
    fn update(&mut self) {
        self.velocity = self.velocity.clamp_length_max(5.6);
        self.position += self.velocity;
        if self.position.x + self.radius >= screen_width() {
            self.position.x = screen_width() - self.radius;
        } else if self.position.x - self.radius <= 0. {
            self.position.x = 0. + self.radius;
        }
        if self.position.y + self.radius >= screen_height() {
            self.position.y = screen_height() - self.radius;
        } else if self.position.y - self.radius <= 0. {
            self.position.y = 0. + self.radius;
        }
        if self.position.x + self.radius == screen_width() || self.position.x - self.radius == 0. {
            self.velocity.x *= -1.;
        }
        if self.position.y + self.radius == screen_height() || self.position.y - self.radius == 0. {
            self.velocity.y *= -1.;
        }
    }
}

#[macroquad::main("Collision")]
async fn main() {
    let mut particles: Vec<Particle> = Vec::new();
    let colors = vec![
        Color::from_rgba(30, 174, 66, 125),
        Color::from_rgba(153, 53, 46, 125),
        Color::from_rgba(64, 25, 159, 125),
        Color::from_rgba(41, 90, 204, 125),
        Color::from_rgba(113, 32, 193, 125),
    ];
    for id in 0..15 {
        particles.push(Particle::new(
            gen_range(13.0, 20.0),
            *colors.choose().unwrap(),
            id,
        ));
    }
    loop {
        clear_background(Color::from_rgba(14, 5, 34, 255));
        particles.sort_by(|a, b| {
            vec2(a.position.x + a.radius, a.position.y + a.radius)
                .x
                .partial_cmp(&vec2(b.position.x + b.radius, b.position.y + b.radius).x)
                .unwrap()
        });
        let mut active_list: Vec<Particle> = vec![particles[0]];
        let mut possible_collisions: Vec<(usize, usize)> = Vec::new();
        for (pind, particle) in particles.iter().enumerate() {
            let mut particles_to_add: Vec<Particle> = Vec::new();
            for active_particle in active_list.iter() {
                if active_particle.id != particle.id {
                    if particle.position.x - particle.radius
                        <= active_particle.position.x + active_particle.radius
                    {
                        possible_collisions.push((pind, active_particle.id));
                    }
                    particles_to_add.push(*particle);
                }
            }
            active_list.retain(|active_particle| {
                particle.position.x - particle.radius
                    <= active_particle.position.x + active_particle.radius
            });

            for pta in particles_to_add {
                active_list.push(pta);
            }
        }

        for (p_pos, ap_id) in possible_collisions {
            let ap_pos = particles.iter().position(|&p| p.id == ap_id).unwrap();
            let part = particles[p_pos];
            let actpart = particles[ap_pos];

            let dist = part.position.distance(actpart.position);
            if dist <= part.radius + actpart.radius {
                let ndist = (part.position - actpart.position).normalize();
                let touch_distance = dist * (actpart.radius / (actpart.radius + part.radius));
                let contact = actpart.position + (ndist * touch_distance);

                particles[ap_pos].position = contact - (ndist * actpart.radius);
                particles[p_pos].position = contact + (ndist * part.radius);

                let p_vel = part.velocity.length();
                let ap_vel = actpart.velocity.length();
                let p_dir = part.velocity.y.atan2(part.velocity.x);
                let ap_dir = actpart.velocity.y.atan2(actpart.velocity.x);

                let contact_dir = ndist.y.atan2(ndist.x);
                let p_mass = part.radius.powf(3.);
                let ap_mass = actpart.radius.powf(3.);

                let t = (ap_vel * (ap_dir - contact_dir).cos() * (ap_mass - p_mass)
                    + 2. * p_mass * p_vel * (p_dir - contact_dir).cos())
                    / (ap_mass + p_mass);
                particles[ap_pos].velocity.x = t * contact_dir.cos()
                    + ap_vel
                        * (ap_dir - contact_dir).sin()
                        * (contact_dir + std::f32::consts::PI / 2.).cos();
                particles[ap_pos].velocity.y = t * contact_dir.sin()
                    + ap_vel
                        * (ap_dir - contact_dir).sin()
                        * (contact_dir + std::f32::consts::PI / 2.).sin();
                let t = (p_vel * (p_dir - contact_dir).cos() * (p_mass - ap_mass)
                    + 2. * ap_mass * ap_vel * (ap_dir - contact_dir).cos())
                    / (ap_mass + p_mass);
                particles[p_pos].velocity.x = t * -(contact_dir.cos())
                    + p_vel
                        * (p_dir - contact_dir).sin()
                        * -(contact_dir + std::f32::consts::PI / 2.).cos();
                particles[p_pos].velocity.y = t * -(contact_dir.sin())
                    + p_vel
                        * (p_dir - contact_dir).sin()
                        * -(contact_dir + std::f32::consts::PI / 2.).sin();
            }
        }
        for particle in particles.iter_mut() {
            particle.update();
            for i in 0..5 {
                draw_circle_lines(
                    particle.position.x,
                    particle.position.y,
                    particle.radius - (8 - i * 2) as f32,
                    2.0,
                    Color::new(
                        particle.color.r,
                        particle.color.g,
                        particle.color.b,
                        (125. - (i * 20) as f32) / 255.,
                    ),
                );
            }
            draw_circle(
                particle.position.x,
                particle.position.y,
                particle.radius - 8.,
                Color::from_rgba(255, 255, 255, 190),
            );
        }
        next_frame().await;
    }
}
