use macroquad::prelude::*;

const SCALE: f32 = 50.0; // pixels per world unit
const CYAN: Color = Color::new(0.0, 1.0, 1.0, 1.0);

#[macroquad::main("Math World")]
async fn main() {
    let mut player = Vec2::new(2.0, 1.0);

    loop {
        clear_background(BLACK);

        // ===== Camera =====
        let camera = Camera2D {
            target: player, // follow player
            zoom: vec2(SCALE / screen_width(), -SCALE / screen_height()),
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
        draw_line(player.x, 0.0, player.x, player.y, 0.03, ORANGE);

        // draw y axis to player
        draw_line(0.0, player.y, player.x, player.y, 0.03, ORANGE);

        // draw line of player unit vector
        let player_unit_vec = calculate_unit_vector(player.x, player.y);
        draw_line(0.0, 0.0, player_unit_vec.x, player_unit_vec.y, 0.03, PURPLE);

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

        // ===== ATAN2 Demo Area =====
        let atan2_center = vec2(-8.0, 5.0); // Position of the atan2 demo
        let atan2_radius = 2.0;

        // Draw the demo circle
        draw_circle_lines(atan2_center.x, atan2_center.y, atan2_radius, 0.05, CYAN);

        // Calculate angle from demo center to mouse
        let atan2_delta = mouse_world - atan2_center;
        let atan2_angle = atan2_delta.y.atan2(atan2_delta.x);
        let atan2_angle_deg = atan2_angle.to_degrees();

        // Draw the angle arc (from 0 to current angle)
        draw_arc(atan2_center, atan2_radius * 0.5, atan2_angle, CYAN);

        // Draw center point
        draw_circle(atan2_center.x, atan2_center.y, 0.1, CYAN);

        // Draw line from center to mouse direction (on the circle edge)
        let atan2_dir = vec2(atan2_angle.cos(), atan2_angle.sin());
        draw_line(
            atan2_center.x,
            atan2_center.y,
            atan2_center.x + atan2_dir.x * atan2_radius,
            atan2_center.y + atan2_dir.y * atan2_radius,
            0.05,
            CYAN,
        );

        // Draw reference line (0 degrees / positive X axis)
        draw_line(
            atan2_center.x,
            atan2_center.y,
            atan2_center.x + atan2_radius,
            atan2_center.y,
            0.03,
            DARKGRAY,
        );

        // Draw component lines (dx, dy visualization)
        let mouse_on_circle = atan2_center + atan2_dir * atan2_radius;
        // Vertical line (dy component)
        draw_line(
            mouse_on_circle.x,
            atan2_center.y,
            mouse_on_circle.x,
            mouse_on_circle.y,
            0.03,
            LIME,
        );
        // Horizontal line (dx component)
        draw_line(
            atan2_center.x,
            atan2_center.y,
            mouse_on_circle.x,
            atan2_center.y,
            0.03,
            MAGENTA,
        );

        // ===== 2D ROTATION Demo Area =====
        let rot_center = vec2(8.0, 5.0);

        // Calculate rotation angle based on mouse position relative to demo center
        let rot_delta = mouse_world - rot_center;
        let rot_angle = rot_delta.y.atan2(rot_delta.x);
        let rot_angle_deg = rot_angle.to_degrees();

        // Rotation matrix components
        let cos_a = rot_angle.cos();
        let sin_a = rot_angle.sin();

        // Draw reference circle
        draw_circle_lines(rot_center.x, rot_center.y, 2.5, 0.03, DARKGRAY);
        draw_circle(rot_center.x, rot_center.y, 0.1, GOLD);

        // Original triangle vertices (before rotation) - relative to center
        let tri_original = [
            vec2(1.5, 0.0),   // right point
            vec2(-0.8, 1.0),  // top-left
            vec2(-0.8, -1.0), // bottom-left
        ];

        // Draw original triangle (faded)
        for i in 0..3 {
            let p1 = rot_center + tri_original[i];
            let p2 = rot_center + tri_original[(i + 1) % 3];
            draw_line(p1.x, p1.y, p2.x, p2.y, 0.02, Color::new(0.3, 0.3, 0.3, 1.0));
        }

        // Apply rotation matrix: [cos -sin] [x]   [x*cos - y*sin]
        //                        [sin  cos] [y] = [x*sin + y*cos]
        let tri_rotated: Vec<Vec2> = tri_original
            .iter()
            .map(|p| vec2(p.x * cos_a - p.y * sin_a, p.x * sin_a + p.y * cos_a))
            .collect();

        // Draw rotated triangle
        for i in 0..3 {
            let p1 = rot_center + tri_rotated[i];
            let p2 = rot_center + tri_rotated[(i + 1) % 3];
            draw_line(p1.x, p1.y, p2.x, p2.y, 0.05, GOLD);
        }

        // Draw vertices
        for (i, p) in tri_rotated.iter().enumerate() {
            let world_p = rot_center + *p;
            let colors = [RED, GREEN, BLUE];
            draw_circle(world_p.x, world_p.y, 0.12, colors[i]);
        }

        // Draw rotation arc
        draw_arc(rot_center, 1.0, rot_angle, GOLD);

        // Draw reference axis (0 degrees)
        draw_line(
            rot_center.x,
            rot_center.y,
            rot_center.x + 2.5,
            rot_center.y,
            0.02,
            DARKGRAY,
        );

        // Draw current angle line
        draw_line(
            rot_center.x,
            rot_center.y,
            rot_center.x + rot_angle.cos() * 2.5,
            rot_center.y + rot_angle.sin() * 2.5,
            0.03,
            GOLD,
        );

        // ===== DOT PRODUCT Demo Area =====
        let dot_center = vec2(-8.0, -5.0);

        // Fixed vector A (pointing right-ish)
        let vec_a = vec2(2.0, 0.5).normalize() * 2.0;

        // Vector B points toward mouse
        let dot_delta = mouse_world - dot_center;
        let vec_b = dot_delta.normalize_or_zero() * 2.0;

        // Calculate dot product
        let dot_product = vec_a.dot(vec_b);
        // Dot product of unit vectors = cos(angle between them)
        let dot_unit = vec_a.normalize().dot(vec_b.normalize());
        let angle_between = dot_unit.acos().to_degrees();

        // Draw reference circle
        draw_circle_lines(dot_center.x, dot_center.y, 2.0, 0.02, DARKGRAY);
        draw_circle(dot_center.x, dot_center.y, 0.1, SKYBLUE);

        // Draw vector A (fixed) - Orange
        draw_line(
            dot_center.x,
            dot_center.y,
            dot_center.x + vec_a.x,
            dot_center.y + vec_a.y,
            0.06,
            ORANGE,
        );
        // Arrowhead for A
        draw_circle(dot_center.x + vec_a.x, dot_center.y + vec_a.y, 0.12, ORANGE);

        // Draw vector B (follows mouse) - Skyblue
        draw_line(
            dot_center.x,
            dot_center.y,
            dot_center.x + vec_b.x,
            dot_center.y + vec_b.y,
            0.06,
            SKYBLUE,
        );
        // Arrowhead for B
        draw_circle(
            dot_center.x + vec_b.x,
            dot_center.y + vec_b.y,
            0.12,
            SKYBLUE,
        );

        // Draw arc showing angle between vectors
        let angle_a = vec_a.y.atan2(vec_a.x);
        let angle_b = vec_b.y.atan2(vec_b.x);
        draw_arc_between(dot_center, 0.8, angle_a, angle_b, WHITE);

        // Color indicator based on dot product sign
        let dot_color = if dot_product > 0.1 {
            GREEN // Same direction
        } else if dot_product < -0.1 {
            RED // Opposite direction
        } else {
            YELLOW // Perpendicular
        };
        draw_circle(dot_center.x, dot_center.y - 2.8, 0.15, dot_color);

        // ===== LERP Demo Area =====
        let lerp_center = vec2(8.0, -5.0);

        // Two fixed points to lerp between
        let lerp_a = lerp_center + vec2(-2.5, -1.0);
        let lerp_b = lerp_center + vec2(2.5, 1.0);

        // Calculate t based on mouse X position relative to demo
        let mouse_rel_x = mouse_world.x - lerp_center.x;
        let t = ((mouse_rel_x + 3.0) / 6.0).clamp(0.0, 1.0);

        // Lerp formula: result = a + t * (b - a)  OR  result = a * (1-t) + b * t
        let lerp_result = lerp_a + t * (lerp_b - lerp_a);

        // Draw the line between A and B
        draw_line(lerp_a.x, lerp_a.y, lerp_b.x, lerp_b.y, 0.04, DARKGRAY);

        // Draw endpoint A
        draw_circle(lerp_a.x, lerp_a.y, 0.15, RED);

        // Draw endpoint B
        draw_circle(lerp_b.x, lerp_b.y, 0.15, GREEN);

        // Draw the interpolated point
        draw_circle(lerp_result.x, lerp_result.y, 0.18, YELLOW);

        // Draw t-value bar
        let bar_y = lerp_center.y - 2.0;
        let bar_left = lerp_center.x - 2.5;
        let bar_right = lerp_center.x + 2.5;
        // Bar background
        draw_line(bar_left, bar_y, bar_right, bar_y, 0.08, DARKGRAY);
        // Bar fill
        draw_line(bar_left, bar_y, bar_left + t * 5.0, bar_y, 0.08, YELLOW);
        // Bar endpoints
        draw_circle(bar_left, bar_y, 0.1, RED);
        draw_circle(bar_right, bar_y, 0.1, GREEN);

        // ===== POINT FROM ANGLE + DISTANCE Demo Area =====
        let offset_center = vec2(0.0, -14.0);
        let offset_distance = 3.0; // fixed distance

        // Base angle points toward mouse, then we offset from it
        let offset_delta = mouse_world - offset_center;
        let base_angle = offset_delta.y.atan2(offset_delta.x);

        // Three example offsets: -45 deg, 0 deg (straight), +45 deg
        let offsets_deg = [-45.0_f32, 0.0, 45.0];
        let offset_colors = [ORANGE, LIME, SKYBLUE];

        // Draw reference circle showing the distance
        draw_circle_lines(
            offset_center.x,
            offset_center.y,
            offset_distance,
            0.02,
            DARKGRAY,
        );

        // Draw center point
        draw_circle(offset_center.x, offset_center.y, 0.12, WHITE);

        // Draw base direction line (toward mouse)
        let base_dir = vec2(base_angle.cos(), base_angle.sin());
        draw_line(
            offset_center.x,
            offset_center.y,
            offset_center.x + base_dir.x * offset_distance,
            offset_center.y + base_dir.y * offset_distance,
            0.03,
            GRAY,
        );

        // Draw three offset points
        let mut offset_points = Vec::new();
        for (i, &deg) in offsets_deg.iter().enumerate() {
            let rad = deg.to_radians();
            let final_angle = base_angle + rad;

            // THE KEY FORMULA: point = origin + (cos(angle), sin(angle)) * distance
            let point = vec2(
                offset_center.x + final_angle.cos() * offset_distance,
                offset_center.y + final_angle.sin() * offset_distance,
            );
            offset_points.push((point, final_angle, deg));

            // Draw line from center to offset point
            draw_line(
                offset_center.x,
                offset_center.y,
                point.x,
                point.y,
                0.05,
                offset_colors[i],
            );

            // Draw the point
            draw_circle(point.x, point.y, 0.15, offset_colors[i]);

            // Draw arc from base angle to this angle
            if deg != 0.0 {
                draw_arc_between(
                    offset_center,
                    1.0,
                    base_angle,
                    final_angle,
                    offset_colors[i],
                );
            }
        }

        // ===== UI (screen space) =====
        set_default_camera();
        draw_text(
            "WASD to move, adjust angle with mouse",
            20.0,
            30.0,
            30.0,
            WHITE,
        );

        // Origin label
        draw_anchored_text(&format!("Origin (0,0)"), -2.0, -2.0, &camera);

        // ATAN2 Demo labels (anchored to the ground)
        draw_anchored_text(
            "ATAN2",
            atan2_center.x,
            atan2_center.y + atan2_radius + 0.8,
            &camera,
        );
        draw_anchored_text(
            &format!("atan2(dy, dx) = {:.2}°", atan2_angle_deg),
            atan2_center.x,
            atan2_center.y - atan2_radius - 0.5,
            &camera,
        );
        draw_anchored_text(
            &format!("radians: {:.3}", atan2_angle),
            atan2_center.x,
            atan2_center.y - atan2_radius - 1.0,
            &camera,
        );
        // dx label (magenta)
        draw_anchored_text_color(
            &format!("dx: {:.2}", atan2_dir.x),
            atan2_center.x + atan2_dir.x * 0.5,
            atan2_center.y - 0.4,
            &camera,
            MAGENTA,
        );
        // dy label (lime)
        draw_anchored_text_color(
            &format!("dy: {:.2}", atan2_dir.y),
            mouse_on_circle.x + 0.3,
            atan2_center.y + atan2_dir.y * 0.5,
            &camera,
            LIME,
        );

        // 2D ROTATION Demo labels
        draw_anchored_text("2D ROTATION", rot_center.x, rot_center.y + 3.3, &camera);
        draw_anchored_text(
            &format!("angle = {:.1} deg", rot_angle_deg),
            rot_center.x,
            rot_center.y - 3.0,
            &camera,
        );
        // Rotation matrix display (moved further right to avoid overlap)
        draw_anchored_text_color(
            "R(a) =",
            rot_center.x + 4.0,
            rot_center.y + 1.0,
            &camera,
            GOLD,
        );
        draw_anchored_text_color(
            &format!("[{:+.2}  {:+.2}]", cos_a, -sin_a),
            rot_center.x + 4.0,
            rot_center.y + 0.5,
            &camera,
            GOLD,
        );
        draw_anchored_text_color(
            &format!("[{:+.2}  {:+.2}]", sin_a, cos_a),
            rot_center.x + 4.0,
            rot_center.y + 0.0,
            &camera,
            GOLD,
        );
        // cos/sin labels
        draw_anchored_text_color(
            &format!("cos(a) = {:.3}", cos_a),
            rot_center.x + 4.0,
            rot_center.y - 0.5,
            &camera,
            WHITE,
        );
        draw_anchored_text_color(
            &format!("sin(a) = {:.3}", sin_a),
            rot_center.x + 4.0,
            rot_center.y - 1.0,
            &camera,
            WHITE,
        );

        // Per-vertex matrix multiplication results
        let vertex_colors = [RED, GREEN, BLUE];
        let vertex_names = ["R", "G", "B"];
        for (i, (orig, rotated)) in tri_original.iter().zip(tri_rotated.iter()).enumerate() {
            // Position labels near each vertex
            let label_offset = rotated.normalize_or_zero() * 0.5;
            let label_pos = rot_center + *rotated + label_offset;

            // Show the transformation: (x,y) -> (x',y')
            draw_anchored_text_color(
                &format!(
                    "{}: ({:.1},{:.1})->({:.1},{:.1})",
                    vertex_names[i], orig.x, orig.y, rotated.x, rotated.y
                ),
                label_pos.x,
                label_pos.y,
                &camera,
                vertex_colors[i],
            );
        }

        // Show the matrix equation below
        draw_anchored_text_color(
            "x' = x*cos - y*sin",
            rot_center.x,
            rot_center.y - 3.5,
            &camera,
            GOLD,
        );
        draw_anchored_text_color(
            "y' = x*sin + y*cos",
            rot_center.x,
            rot_center.y - 4.0,
            &camera,
            GOLD,
        );

        // DOT PRODUCT Demo labels
        draw_anchored_text("DOT PRODUCT", dot_center.x, dot_center.y + 2.8, &camera);
        draw_anchored_text_color(
            "A (fixed)",
            dot_center.x + vec_a.x + 0.3,
            dot_center.y + vec_a.y,
            &camera,
            ORANGE,
        );
        draw_anchored_text_color(
            "B (mouse)",
            dot_center.x + vec_b.x + 0.3,
            dot_center.y + vec_b.y,
            &camera,
            SKYBLUE,
        );
        draw_anchored_text(
            &format!("A dot B = {:.2}", dot_product),
            dot_center.x,
            dot_center.y - 3.2,
            &camera,
        );
        draw_anchored_text(
            &format!("angle = {:.1} deg", angle_between),
            dot_center.x,
            dot_center.y - 3.7,
            &camera,
        );
        // Interpretation
        let dot_meaning = if dot_product > 0.1 {
            "same dir (>0)"
        } else if dot_product < -0.1 {
            "opposite (<0)"
        } else {
            "perpendicular (~0)"
        };
        draw_anchored_text_color(
            dot_meaning,
            dot_center.x,
            dot_center.y - 4.2,
            &camera,
            dot_color,
        );

        // LERP Demo labels
        draw_anchored_text("LERP", lerp_center.x, lerp_center.y + 2.5, &camera);
        draw_anchored_text_color("A", lerp_a.x - 0.4, lerp_a.y, &camera, RED);
        draw_anchored_text_color("B", lerp_b.x + 0.4, lerp_b.y, &camera, GREEN);
        draw_anchored_text(
            &format!("t = {:.2}", t),
            lerp_center.x,
            lerp_center.y - 2.6,
            &camera,
        );
        draw_anchored_text(
            &format!(
                "lerp(A,B,t) = ({:.1}, {:.1})",
                lerp_result.x - lerp_center.x,
                lerp_result.y - lerp_center.y
            ),
            lerp_center.x,
            lerp_center.y - 3.1,
            &camera,
        );
        draw_anchored_text_color(
            "result = A + t*(B-A)",
            lerp_center.x,
            lerp_center.y - 3.6,
            &camera,
            YELLOW,
        );

        // POINT FROM ANGLE + DISTANCE Demo labels
        draw_anchored_text(
            "POINT AT ANGLE + DISTANCE",
            offset_center.x,
            offset_center.y + 4.0,
            &camera,
        );
        draw_anchored_text(
            &format!("distance = {:.1}", offset_distance),
            offset_center.x,
            offset_center.y + 3.5,
            &camera,
        );

        // Label for each offset point
        for (i, &(point, final_angle, deg)) in offset_points.iter().enumerate() {
            let label_dir = vec2(final_angle.cos(), final_angle.sin());
            draw_anchored_text_color(
                &format!("{:+.0} deg", deg),
                point.x + label_dir.x * 0.5,
                point.y + label_dir.y * 0.5,
                &camera,
                offset_colors[i],
            );
        }

        // Show the formula
        draw_anchored_text_color(
            "x = origin.x + cos(angle) * dist",
            offset_center.x,
            offset_center.y - 4.0,
            &camera,
            WHITE,
        );
        draw_anchored_text_color(
            "y = origin.y + sin(angle) * dist",
            offset_center.x,
            offset_center.y - 4.5,
            &camera,
            WHITE,
        );

        // Show calculated coordinates for the green (0 deg) point
        let green_point = offset_points[1].0;
        draw_anchored_text_color(
            &format!("result: ({:.2}, {:.2})", green_point.x, green_point.y),
            offset_center.x,
            offset_center.y - 5.0,
            &camera,
            LIME,
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

fn draw_anchored_text(text: &str, world_x: f32, world_y: f32, camera: &Camera2D) {
    let screen_pos = camera.world_to_screen(vec2(world_x, world_y));
    draw_text(text, screen_pos.x, screen_pos.y, 20.0, WHITE);
}

fn draw_anchored_text_color(
    text: &str,
    world_x: f32,
    world_y: f32,
    camera: &Camera2D,
    color: Color,
) {
    let screen_pos = camera.world_to_screen(vec2(world_x, world_y));
    draw_text(text, screen_pos.x, screen_pos.y, 18.0, color);
}

/// Draws an arc from angle 0 to the specified angle
fn draw_arc(center: Vec2, radius: f32, angle: f32, color: Color) {
    let segments = 32;
    let step = angle / segments as f32;

    for i in 0..segments {
        let a1 = step * i as f32;
        let a2 = step * (i + 1) as f32;

        let p1 = center + vec2(a1.cos(), a1.sin()) * radius;
        let p2 = center + vec2(a2.cos(), a2.sin()) * radius;

        draw_line(p1.x, p1.y, p2.x, p2.y, 0.04, color);
    }
}

/// Draws an arc between two angles (takes the shorter path)
fn draw_arc_between(center: Vec2, radius: f32, angle_a: f32, angle_b: f32, color: Color) {
    let segments = 24;

    // Normalize angles and find shorter arc
    let mut start = angle_a;
    let mut end = angle_b;

    // Ensure we take the shorter path
    let mut diff = end - start;
    if diff > std::f32::consts::PI {
        diff -= 2.0 * std::f32::consts::PI;
    } else if diff < -std::f32::consts::PI {
        diff += 2.0 * std::f32::consts::PI;
    }
    end = start + diff;

    let step = (end - start) / segments as f32;

    for i in 0..segments {
        let a1 = start + step * i as f32;
        let a2 = start + step * (i + 1) as f32;

        let p1 = center + vec2(a1.cos(), a1.sin()) * radius;
        let p2 = center + vec2(a2.cos(), a2.sin()) * radius;

        draw_line(p1.x, p1.y, p2.x, p2.y, 0.03, color);
    }
}
