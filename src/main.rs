use macroquad::prelude::*;

const SCALE: f32 = 50.0; // pixels per world unit

#[macroquad::main("Math World")]
async fn main() {
    let mut player = Vec2::new(2.0, 1.0);

    loop {
        clear_background(BLACK);

        // ===== Camera =====
        let camera = Camera2D {
            target: player, // follow player
            zoom: vec2(
                SCALE / screen_width(),
                -SCALE / screen_height(),
            ),
            ..Default::default()
        };
        set_camera(&camera);

        // ===== Input =====
        let speed = 5.0 * get_frame_time();
        if is_key_down(KeyCode::W) {
            player.y += speed;
        }
        if is_key_down(KeyCode::S) {
            player.y -= speed;
        }
        if is_key_down(KeyCode::A) {
            player.x -= speed;
        }
        if is_key_down(KeyCode::D) {
            player.x += speed;
        }

        // ===== Mouse in world space =====
        let (mx, my) = mouse_position();

        println!("Mouse Screen: ({:.2}, {:.2})", mx, my);

        let mouse_world = camera.screen_to_world(vec2(mx, my));

        // ===== Math =====
        let delta = mouse_world - player;
        let dx = delta.x;
        let dy = delta.y;
        let distance = delta.length();
        let angle = dy.atan2(dx);

        // ===== Draw world =====
        draw_grid(20);

        // Unit circle at origin
        draw_circle_lines(0.0, 0.0, 1.0, 0.05, GREEN);

        // Player
        draw_circle(player.x, player.y, 0.12, BLUE);

        // Mouse
        draw_circle(mouse_world.x, mouse_world.y, 0.08, RED);

        // draw line from x axis to player
        draw_line(
            player.x,
            0.0,
            player.x,
            player.y,
            0.03,
            ORANGE,
        );

        // draw y axis to player
        draw_line(
            0.0,
            player.y,
            player.x,
            player.y,
            0.03,
            ORANGE,
        );

        // draw line of player unit vector
        let player_unit_vec = calculate_unit_vector(player.x, player.y);
        draw_line(
            0.0,
            0.0,
            player_unit_vec.x,
            player_unit_vec.y,
            0.03,
            PURPLE,
        );


        // unit vector from player to mouse
        let to_mouse_unit_vec = calculate_unit_vector(dx, dy);
        draw_line(
            player.x,
            player.y,
            player.x + to_mouse_unit_vec.x,
            player.y + to_mouse_unit_vec.y,
            0.03,
            PINK,
        );


        // draw line 45 degrees left of player->mouse
        let angle_45_left = angle + std::f32::consts::FRAC_PI_4;
        let vec_45_left = vec2(angle_45_left.cos(), angle_45_left.sin());
        draw_line(
            player.x,
            player.y,
            player.x + vec_45_left.x,
            player.y + vec_45_left.y,
            0.03,
            YELLOW,
        );

        // draw 3.5 units along that 45 degree line
        draw_line(
            player.x,
            player.y,
            player.x + vec_45_left.x * 3.5,
            player.y + vec_45_left.y * 3.5,
            0.02,
            YELLOW,
        );



        // ===== UI (screen space) =====
        set_default_camera();
        draw_text("WASD to move • Mouse shows vector math", 20.0, 30.0, 30.0, WHITE);


        // Origin label
        draw_anchored_text(
            &format!("Origin (0,0)"),
            -2.0,
            -2.0,
            &camera,
        );

        // LookAtPlayer Unit Vector label
        draw_anchored_text(
            &format!(
                "lookAtPlayer Unit Vec: ({:.2}, {:.2})",
                player_unit_vec.x, player_unit_vec.y
            ),
            3.0,
            0.0,
            &camera,
        );



        // Player look at mouse unit vector label
        draw_text(
            &format!(
                "To Mouse Unit Vec: ({:.2}, {:.2})",
                to_mouse_unit_vec.x, to_mouse_unit_vec.y
            ),
            20.0,
            90.0,
            20.0,
            WHITE,
        );

        // draw label at cursor
        draw_text(
            &format!("Mouse World: ({:.2}, {:.2})", mouse_world.x, mouse_world.y),
            mx + 15.0,
            my + 15.0,
            20.0,
            WHITE,
        );

        // Mouse look at vector
        draw_line(
            mouse_world.x,
            mouse_world.y,
            mouse_world.x + player_unit_vec.x,
            mouse_world.y + player_unit_vec.y,
            0.03,
            PURPLE,
        );


        next_frame().await;
    }
}

// ===================== Helpers =====================

fn draw_grid(half_size: i32) {
    let hs = half_size as f32;

    for i in -half_size..=half_size {
        let i = i as f32;
        draw_line(i, -hs, i, hs, 0.02, DARKGRAY);
        draw_line(-hs, i, hs, i, 0.02, DARKGRAY);
    }

    // Axes
    draw_line(-hs, 0.0, hs, 0.0, 0.05, GRAY);
    draw_line(0.0, -hs, 0.0, hs, 0.05, GRAY);
}

fn draw_text_world_to_screen(text: &str, camera_pos: Vec2, world_pos: Vec2) {
    let screen_pos = camera_pos + world_pos * vec2(SCALE, -SCALE);
    draw_text(text, screen_pos.x, screen_pos.y, 20.0, WHITE);
}
fn calculate_unit_vector(x: f32, y: f32) -> Vec2 {
    let length = (x * x + y * y).sqrt();
    if length == 0.0 {
        Vec2::ZERO
    } else {
        Vec2::new(x / length, y / length)
    }
}

fn draw_anchored_text(
    text: &str,
    world_x: f32,
    world_y: f32,
    camera: &Camera2D,
) {
    let screen_pos = camera.world_to_screen(vec2(world_x, world_y));
    draw_text(text, screen_pos.x, screen_pos.y, 20.0, WHITE);
}