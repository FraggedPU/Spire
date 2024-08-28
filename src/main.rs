use std::{cell::RefCell, fmt::format, time::Instant};

use sfml::{
    graphics::{
        CircleShape, Color, Drawable, Font, RenderStates, RenderTarget, RenderTexture,
        RenderWindow, Shader, ShaderType, Shape, Sprite, Text, Texture, Transformable,
    },
    system::Vector2f,
    window::{Event, Key, Style},
};
use world::World;

mod entity;
mod world;

fn main() {
    // Set up window handle and render states
    const WINDOW_WIDTH: u32 = 1400;
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
    window.set_mouse_cursor_visible(false);

    // Set up pre-fx render texture - WIP
    let mut pre_render_texture = RenderTexture::new(WINDOW_WIDTH, WINDOW_HEIGHT).unwrap();

    // Load resources
    let mut fx_blur_frag_shader: RefCell<Shader> =
        Shader::from_file("./res/shaders/blur_frag.glsl", ShaderType::Fragment)
            .unwrap()
            .into();
    let mut fx_blur_vert_shader: Shader =
        Shader::from_file("./res/shaders/blur_vert.glsl", ShaderType::Vertex).unwrap();

    let default_font = Font::from_file("./res/fonts/Matemasie-Regular.ttf").unwrap();

    // Set up tick system
    const UPDATE_TICK_TIME_MS: u128 = 25;
    const RENDER_TICK_TIME_MS: u128 = 15;
    let mut last_update_tick = Instant::now();
    let mut last_render_tick = Instant::now();

    const CLEAR_INTERVAL: u32 = 0;
    let mut ticks_not_cleared = CLEAR_INTERVAL;

    let mut fps = 0;
    let mut frames: i32 = 1;
    let mut last_frame_count = Instant::now();

    // Initiate sim logic
    let mut world = World::new(
        35000,
        (0.35, 0.5),
        (5., 12.5),
        (
            SIM_OFFSET,
            SIM_OFFSET,
            window.size().x as f32 - SIM_OFFSET,
            window.size().y as f32 - SIM_OFFSET,
        ),
    );

    const FOCAL_STRENGHT_MOD: f32 = 0.005;

    let mut follow_mouse_pos = false;
    let mut focus_point_inversed = false;
    let mut draw_cursor = true;
    let mut mapped_cursor_pos = Vector2f::default();
    let mut user_has_clicked_anywhere = false;
    let mut map_color = true;
    let mut current_focal_strength = 0.055;

    // UI
    const UI_INFO_DISPLAY_FONT_SIZE: u32 = 25;
    const UI_INFO_DISPLAY_OFFSET: f32 = 15.0;

    let mut cursor_shape = CircleShape::new(4.0, 20);
    cursor_shape.set_fill_color(Color::rgba(33, 33, 33, 150));
    cursor_shape.set_outline_color(Color::rgb(200, 200, 210));
    cursor_shape.set_outline_thickness(2.0);

    let mut focal_strength_label = Text::new("..", &default_font, UI_INFO_DISPLAY_FONT_SIZE);
    focal_strength_label.set_position(Vector2f::new(
        UI_INFO_DISPLAY_OFFSET,
        UI_INFO_DISPLAY_OFFSET,
    ));

    let mut fps_label = Text::new("..", &default_font, UI_INFO_DISPLAY_FONT_SIZE);
    fps_label.set_position(Vector2f::new(
        UI_INFO_DISPLAY_OFFSET,
        UI_INFO_DISPLAY_OFFSET * 3.0,
    ));

    // Main sim loop
    loop {
        // Poll events
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed
                | Event::KeyPressed {
                    code: Key::Escape, ..
                } => return,
                Event::MouseButtonPressed { .. } => {
                    follow_mouse_pos = !follow_mouse_pos;
                    user_has_clicked_anywhere = true;
                }
                Event::KeyPressed { code: Key::E, .. } => {
                    focus_point_inversed = !focus_point_inversed;
                }
                Event::KeyPressed { code: Key::R, .. } => {
                    draw_cursor = !draw_cursor;
                    window.set_mouse_cursor_visible(!draw_cursor);
                }
                Event::KeyPressed { code: Key::T, .. } => {
                    map_color = !map_color;
                }
                Event::KeyPressed {
                    code: Key::Subtract,
                    ..
                } => {
                    current_focal_strength -= FOCAL_STRENGHT_MOD;
                }
                Event::KeyPressed { code: Key::Add, .. } => {
                    current_focal_strength += FOCAL_STRENGHT_MOD;
                }
                _ => {}
            }
        }

        // Update sim every UPDATE_TICK_TIME
        if last_update_tick.elapsed().as_millis() >= UPDATE_TICK_TIME_MS {
            mapped_cursor_pos = Vector2f::new(
                window.mouse_position().x as f32,
                World::map(
                    window.mouse_position().y as f32,
                    WINDOW_HEIGHT as f32,
                    0.0,
                    0.0,
                    WINDOW_HEIGHT as f32,
                ),
            ); // Quick and dirty fix for a quick and dirty project..

            // Logic updates
            if follow_mouse_pos {
                world.set_entity_focus_point(
                    mapped_cursor_pos,
                    focus_point_inversed,
                    current_focal_strength,
                );
            }
            world.update(map_color);
            last_update_tick = Instant::now();

            // Update UI
            focal_strength_label.set_string(&format!("force: {:.1$}", current_focal_strength, 3));
            fps_label.set_string(&format!("fps: {}", fps));

            if last_frame_count.elapsed().as_secs() >= 60 {
                frames = 0;
                last_frame_count = Instant::now();
            }
        }

        // Render sim every RENDER_TICK_TIME
        if last_render_tick.elapsed().as_millis() >= RENDER_TICK_TIME_MS {
            if ticks_not_cleared >= CLEAR_INTERVAL {
                pre_render_texture.clear(Color::rgb(0, 0, 0));
                ticks_not_cleared = 0;
            } else {
                ticks_not_cleared += 1;
            }

            // Draw particles and mapped cursor to render texture
            world.draw(&mut pre_render_texture);

            if draw_cursor {
                let draw_cursor_pos = Vector2f::new(
                    mapped_cursor_pos.x - cursor_shape.radius() / 2.0,
                    mapped_cursor_pos.y - cursor_shape.radius() / 2.0,
                );
                cursor_shape.set_position(draw_cursor_pos);
                pre_render_texture.draw(&cursor_shape);
            }

            let mut render_texture_sprite = Sprite::with_texture(&pre_render_texture.texture());
            let render_texture_origin = Vector2f::new(0.0, 0.0);
            render_texture_sprite.set_origin(render_texture_origin);
            render_texture_sprite.rotate(0.0);

            // Draw render texture to window with shader effects (WIP)
            window.draw_with_renderstates(&render_texture_sprite, &main_render_states);

            // Draw UI to window over particle texture
            if !user_has_clicked_anywhere {
                let mut hint_text = Text::new("click anywhere", &default_font, 22);
                hint_text.set_position(Vector2f::new(
                    window.size().x as f32 / 2.0 - hint_text.global_bounds().width / 2.0,
                    window.size().y as f32 / 2.0 - hint_text.global_bounds().height / 2.0,
                ));

                window.draw(&hint_text);
            }

            window.draw(&focal_strength_label);
            window.draw(&fps_label);

            window.display();

            last_render_tick = Instant::now();
            frames += 1;
            match frames.checked_div(last_frame_count.elapsed().as_secs() as i32) {
                Some(f) => fps = f,
                None => {}
            }
        }
    }
}
