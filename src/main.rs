use std::{cell::RefCell, time::Instant};

use sfml::{
    graphics::{
        CircleShape, Color, RenderStates, RenderTarget, RenderTexture, RenderWindow, Shader,
        ShaderType, Sprite, Texture,
    },
    system::Vector2f,
    window::{Event, Key, Style},
};
use world::World;

mod entity;
mod world;

fn main() {
    // Set up window handle and render states
    const WINDOW_WIDTH: u32 = 900;
    const WINDOW_HEIGHT: u32 = 900;
    const SIM_OFFSET: f32 = 35.0;
    let mut window = RenderWindow::new(
        (WINDOW_WIDTH, WINDOW_HEIGHT),
        "Spire",
        Style::TITLEBAR,
        &Default::default(),
    );
    let mut main_render_states = RenderStates::default();
    window.set_vertical_sync_enabled(true);

    // Set up pre-fx render texture
    let mut pre_render_texture = RenderTexture::new(WINDOW_WIDTH, WINDOW_HEIGHT).unwrap();

    // Load shaders
    let mut fx_blur_frag_shader: RefCell<Shader> =
        Shader::from_file("./res/shaders/blur_frag.glsl", ShaderType::Fragment)
            .unwrap()
            .into();
    let mut fx_blur_vert_shader: Shader =
        Shader::from_file("./res/shaders/blur_vert.glsl", ShaderType::Vertex).unwrap();

    // Set up tick system
    const UPDATE_TICK_TIME_MS: u128 = 25;
    const RENDER_TICK_TIME_MS: u128 = 15;
    let mut last_update_tick = Instant::now();
    let mut last_render_tick = Instant::now();

    const CLEAR_INTERVAL: u32 = 0;
    let mut ticks_not_cleared = CLEAR_INTERVAL;

    // Initiate sim logic
    let mut world = World::new(
        40000,
        (0.005, 0.07),
        (0.75, 5.5),
        (
            SIM_OFFSET,
            SIM_OFFSET,
            window.size().x as f32 - SIM_OFFSET,
            window.size().y as f32 - SIM_OFFSET,
        ),
    );

    let mut follow_mouse_pos = false;
    let mut focus_point_inversed = false;

    // Main sim loop
    loop {
        // Poll events
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed
                | Event::KeyPressed {
                    code: Key::Escape, ..
                } => return,
                Event::MouseButtonPressed { button, x, y } => {
                    follow_mouse_pos = !follow_mouse_pos;
                }
                Event::KeyPressed { code: Key::E, .. } => {
                    focus_point_inversed = !focus_point_inversed;
                }
                _ => {}
            }
        }

        // Update sim every UPDATE_TICK_TIME
        if last_update_tick.elapsed().as_millis() >= UPDATE_TICK_TIME_MS {
            if follow_mouse_pos {
                world.set_entity_focus_point(
                    Vector2f::new(
                        window.mouse_position().x as f32,
                        window.mouse_position().y as f32,
                    ),
                    focus_point_inversed,
                );
            }
            world.update();
            last_update_tick = Instant::now();
        }

        // Render sim every RENDER_TICK_TIME
        if last_render_tick.elapsed().as_millis() >= RENDER_TICK_TIME_MS {
            if ticks_not_cleared >= CLEAR_INTERVAL {
                pre_render_texture.clear(Color::rgb(0, 0, 0));
                ticks_not_cleared = 0;
            } else {
                ticks_not_cleared += 1;
            }

            world.draw(&mut pre_render_texture);
            window.draw_with_renderstates(
                &Sprite::with_texture(&pre_render_texture.texture()),
                &main_render_states,
            );

            window.display();

            last_render_tick = Instant::now();
        }
    }
}
