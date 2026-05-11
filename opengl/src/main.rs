use glfw::{Action, Context as _, Key, WindowEvent};
use nalgebra_glm as glm;
use std::{ffi::c_void, ptr};

const VERTEX_SHADER: &str = r#"
#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;

uniform mat4 mvp_left;   // Left eye camera MVP
uniform mat4 mvp_right;  // Right eye camera MVP
uniform mat4 model;      // Model matrix for lighting
uniform mat4 normal_matrix; // Normal matrix (transpose of inverse of model)
uniform int blend_mode;   // 0-7: different blending/distortion modes

out vec3 FragPos;
out vec3 Normal;
out vec4 LeftEyeClip;
out vec4 RightEyeClip;

// Apply barrel or pincushion distortion
vec2 applyBarrelDistortion(vec2 coord, float strength) {
    vec2 cc = coord - 0.5;
    float dist = dot(cc, cc);
    return coord + cc * dist * strength;
}

void main() {
    // Transform to world space for lighting
    FragPos = vec3(model * vec4(aPos, 1.0));
    Normal = mat3(normal_matrix) * aNormal;

    // Transform vertex from both camera perspectives
    vec4 pos_left = mvp_left * vec4(aPos, 1.0);
    vec4 pos_right = mvp_right * vec4(aPos, 1.0);

    // Store for fragment shader (for anaglyph)
    LeftEyeClip = pos_left;
    RightEyeClip = pos_right;

    vec4 finalPos;

    if (blend_mode == 0) {
        // Average (default)
        finalPos = (pos_left + pos_right) * 0.5;
    }
    else if (blend_mode == 1) {
        // Min/Max mixing - creates interesting stretching
        finalPos = vec4(
            max(abs(pos_left.x), abs(pos_right.x)),
            max(abs(pos_left.y), abs(pos_right.y)),
            (pos_left.z + pos_right.z) * 0.5,
            (pos_left.w + pos_right.w) * 0.5
        );
    }
    else if (blend_mode == 2) {
        // Component swap - X from left, Y from right
        finalPos = vec4(
            pos_left.x,
            pos_right.y,
            (pos_left.z + pos_right.z) * 0.5,
            (pos_left.w + pos_right.w) * 0.5
        );
    }
    else if (blend_mode == 3) {
        // Exaggerate parallax
        vec4 diff = pos_right - pos_left;
        finalPos = pos_left + diff * 3.0;
    }
    else if (blend_mode == 4) {
        // Depth-based blend
        float depth = (pos_left.z + pos_right.z) * 0.5;
        float blend_factor = smoothstep(-1.0, 1.0, depth);
        finalPos = mix(pos_left, pos_right, blend_factor);
    }
    else if (blend_mode == 5) {
        // Barrel distortion (wide angle lens effect)
        vec4 avg = (pos_left + pos_right) * 0.5;
        vec2 ndc = avg.xy / avg.w;
        vec2 distorted = applyBarrelDistortion((ndc + 1.0) * 0.5, 0.3) * 2.0 - 1.0;
        finalPos = vec4(distorted * avg.w, avg.z, avg.w);
    }
    else if (blend_mode == 6) {
        // Pincushion distortion (telephoto lens effect)
        vec4 avg = (pos_left + pos_right) * 0.5;
        vec2 ndc = avg.xy / avg.w;
        vec2 distorted = applyBarrelDistortion((ndc + 1.0) * 0.5, -0.3) * 2.0 - 1.0;
        finalPos = vec4(distorted * avg.w, avg.z, avg.w);
    }
    else if (blend_mode == 7) {
        // Fisheye lens
        vec4 avg = (pos_left + pos_right) * 0.5;
        vec2 ndc = avg.xy / avg.w;
        vec2 distorted = applyBarrelDistortion((ndc + 1.0) * 0.5, 0.8) * 2.0 - 1.0;
        finalPos = vec4(distorted * avg.w, avg.z, avg.w);
    }

    gl_Position = finalPos;
}
"#;

const FRAGMENT_SHADER: &str = r#"
#version 330 core
in vec3 FragPos;
in vec3 Normal;
in vec4 LeftEyeClip;
in vec4 RightEyeClip;

out vec4 FragColor;

uniform vec3 viewPos;
uniform vec3 objectColor;
uniform bool anaglyph_mode; // Toggle for red/cyan 3D glasses effect

// Point lights (3 lights)
uniform vec3 lightPositions[3];
uniform vec3 lightColors[3];

vec3 calculatePointLight(vec3 lightPos, vec3 lightColor, vec3 normal, vec3 fragPos, vec3 viewDir) {
    // Ambient
    vec3 ambient = 0.1 * lightColor;

    // Diffuse
    vec3 lightDir = normalize(lightPos - fragPos);
    float diff = max(dot(normal, lightDir), 0.0);
    vec3 diffuse = diff * lightColor;

    // Specular
    vec3 reflectDir = reflect(-lightDir, normal);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), 32.0);
    vec3 specular = 0.5 * spec * lightColor;

    // Attenuation
    float distance = length(lightPos - fragPos);
    float attenuation = 1.0 / (1.0 + 0.09 * distance + 0.032 * distance * distance);

    return (ambient + diffuse + specular) * attenuation;
}

void main() {
    vec3 norm = normalize(Normal);
    vec3 viewDir = normalize(viewPos - FragPos);

    vec3 result = vec3(0.0);

    // Calculate lighting from all point lights
    for(int i = 0; i < 3; i++) {
        result += calculatePointLight(lightPositions[i], lightColors[i], norm, FragPos, viewDir);
    }

    result *= objectColor;

    // Apply anaglyph coloring if enabled (for red/cyan 3D glasses)
    if (anaglyph_mode) {
        // Calculate depth difference for chromatic effect
        float depthDiff = (LeftEyeClip.z / LeftEyeClip.w - RightEyeClip.z / RightEyeClip.w) * 5.0;

        // Red from "left eye", Cyan from "right eye"
        // This creates a red/cyan anaglyph effect
        float red = result.r * (1.0 + depthDiff * 0.5);
        vec2 cyan = result.gb * (1.0 - depthDiff * 0.5);

        FragColor = vec4(red, cyan.x, cyan.y, 1.0);
    } else {
        FragColor = vec4(result, 1.0);
    }
}
"#;

fn main() {
    let width = 1280;
    let height = 720;

    // Initialize GLFW
    let mut glfw = glfw::init_no_callbacks().unwrap();
    glfw.window_hint(glfw::WindowHint::Resizable(false));
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    let (mut window, receiver) = glfw
        .create_window(
            width,
            height,
            "Dual Camera Cube",
            glfw::WindowMode::Windowed,
        )
        .unwrap();

    window.make_current();
    window.set_key_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_mouse_button_polling(true);
    window.set_cursor_mode(glfw::CursorMode::Disabled);

    // Load OpenGL function pointers
    gl::load_with(|symbol| {
        window
            .get_proc_address(symbol)
            .map_or(ptr::null(), |f| f as *const c_void)
    });
    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

    // Compile shaders
    let vert = cgl::compile_shader(gl::VERTEX_SHADER, VERTEX_SHADER).unwrap();
    let frag = cgl::compile_shader(gl::FRAGMENT_SHADER, FRAGMENT_SHADER).unwrap();
    let program = cgl::create_vert_frag_prog(vert, frag).unwrap();
    let mvp_left_loc = cgl::get_location(program, "mvp_left").unwrap();
    let mvp_right_loc = cgl::get_location(program, "mvp_right").unwrap();
    let blend_mode_loc = cgl::get_location(program, "blend_mode").unwrap();
    let model_loc = cgl::get_location(program, "model").unwrap();
    let normal_matrix_loc = cgl::get_location(program, "normal_matrix").unwrap();
    let view_pos_loc = cgl::get_location(program, "viewPos").unwrap();
    let object_color_loc = cgl::get_location(program, "objectColor").unwrap();
    let anaglyph_mode_loc = cgl::get_location(program, "anaglyph_mode").unwrap();

    // Cube vertices (position + normal, interleaved)
    #[rustfmt::skip]
    let vertices: [f32; 24 * 6] = [
        // Front face (normal: 0, 0, 1)
        -0.5, -0.5,  0.5,  0.0, 0.0, 1.0,
         0.5, -0.5,  0.5,  0.0, 0.0, 1.0,
         0.5,  0.5,  0.5,  0.0, 0.0, 1.0,
        -0.5,  0.5,  0.5,  0.0, 0.0, 1.0,
        // Back face (normal: 0, 0, -1)
        -0.5, -0.5, -0.5,  0.0, 0.0, -1.0,
         0.5, -0.5, -0.5,  0.0, 0.0, -1.0,
         0.5,  0.5, -0.5,  0.0, 0.0, -1.0,
        -0.5,  0.5, -0.5,  0.0, 0.0, -1.0,
        // Left face (normal: -1, 0, 0)
        -0.5, -0.5, -0.5,  -1.0, 0.0, 0.0,
        -0.5, -0.5,  0.5,  -1.0, 0.0, 0.0,
        -0.5,  0.5,  0.5,  -1.0, 0.0, 0.0,
        -0.5,  0.5, -0.5,  -1.0, 0.0, 0.0,
        // Right face (normal: 1, 0, 0)
         0.5, -0.5, -0.5,  1.0, 0.0, 0.0,
         0.5, -0.5,  0.5,  1.0, 0.0, 0.0,
         0.5,  0.5,  0.5,  1.0, 0.0, 0.0,
         0.5,  0.5, -0.5,  1.0, 0.0, 0.0,
        // Top face (normal: 0, 1, 0)
        -0.5,  0.5,  0.5,  0.0, 1.0, 0.0,
         0.5,  0.5,  0.5,  0.0, 1.0, 0.0,
         0.5,  0.5, -0.5,  0.0, 1.0, 0.0,
        -0.5,  0.5, -0.5,  0.0, 1.0, 0.0,
        // Bottom face (normal: 0, -1, 0)
        -0.5, -0.5,  0.5,  0.0, -1.0, 0.0,
         0.5, -0.5,  0.5,  0.0, -1.0, 0.0,
         0.5, -0.5, -0.5,  0.0, -1.0, 0.0,
        -0.5, -0.5, -0.5,  0.0, -1.0, 0.0,
    ];

    #[rustfmt::skip]
    let indices: [u32; 36] = [
        0,  1,  2,   2,  3,  0,   // Front
        4,  6,  5,   6,  4,  7,   // Back
        8,  9,  10,  10, 11, 8,   // Left
        12, 14, 13,  14, 12, 15,  // Right
        16, 17, 18,  18, 19, 16,  // Top
        20, 22, 21,  22, 20, 23,  // Bottom
    ];
    // Create VAO and buffers
    let vao = cgl::create_vao().unwrap();
    cgl::bind_vao(vao);

    let vbo = cgl::create_buffer().unwrap();
    cgl::bind_buffer(vbo, gl::ARRAY_BUFFER);
    cgl::buffer_data(gl::ARRAY_BUFFER, &vertices, gl::STATIC_DRAW);

    let stride = 6 * std::mem::size_of::<f32>() as i32;
    unsafe {
        // Position attribute
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::EnableVertexAttribArray(0);
        // Normal attribute
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            stride,
            (3 * std::mem::size_of::<f32>()) as *const _,
        );
        gl::EnableVertexAttribArray(1);
    }

    let ebo = cgl::create_buffer().unwrap();
    cgl::bind_buffer(ebo, gl::ELEMENT_ARRAY_BUFFER);
    cgl::buffer_data(gl::ELEMENT_ARRAY_BUFFER, &indices, gl::STATIC_DRAW);

    // Setup main camera (this will be the center point between the two eyes)
    let aspect = width as f32 / height as f32;
    let mut camera =
        cgl::camera::Camera::with_perspective(aspect, 45.0_f32.to_radians(), 0.1, 100.0);
    camera.transform.position = glm::vec3(0.0, 0.0, 3.0);

    // Eye separation distance (interpupillary distance)
    let eye_separation = 0.3; // Exaggerated for visible effect (realistic would be 0.065)
    let eye_separation = 0.1; // Exaggerated for visible effect (realistic would be 0.065)

    // Focus distance (convergence point for dual cameras)
    let mut look_ahead_distance = 5.0;
    let mut auto_focus = true; // Automatically adjust focus based on nearest object

    // Scene setup - multiple cubes at different positions
    let cube_positions = vec![
        glm::vec3(0.0, 0.0, 0.0),
        glm::vec3(2.0, 0.5, -3.0),
        glm::vec3(-2.5, -0.5, -2.0),
        glm::vec3(1.5, 1.0, -1.0),
        glm::vec3(-1.0, -1.0, -5.0),
        glm::vec3(0.0, 2.0, -4.0),
        glm::vec3(3.0, 0.0, -6.0),
        glm::vec3(-3.0, 1.5, -7.0),
    ];

    let cube_colors = vec![
        glm::vec3(0.8, 0.3, 0.3), // Red
        glm::vec3(0.3, 0.8, 0.3), // Green
        glm::vec3(0.3, 0.3, 0.8), // Blue
        glm::vec3(0.8, 0.8, 0.3), // Yellow
        glm::vec3(0.8, 0.3, 0.8), // Magenta
        glm::vec3(0.3, 0.8, 0.8), // Cyan
        glm::vec3(0.9, 0.6, 0.2), // Orange
        glm::vec3(0.6, 0.4, 0.8), // Purple
    ];

    // Point light setup
    let light_positions = [
        glm::vec3(3.0, 3.0, 0.0),
        glm::vec3(-3.0, 2.0, -3.0),
        glm::vec3(0.0, -2.0, -5.0),
    ];

    let light_colors = [
        glm::vec3(1.0, 0.9, 0.8), // Warm white
        glm::vec3(0.8, 0.8, 1.0), // Cool white
        glm::vec3(1.0, 0.7, 0.5), // Orange
    ];

    // Enable depth testing
    cgl::enable(gl::DEPTH_TEST);

    // Camera control state
    let mut last_mouse_pos: Option<(f64, f64)> = None;
    let mouse_sensitivity = 0.1;
    let move_speed = 2.5;

    // Dual camera mode toggle
    let mut use_dual_cameras = true;
    let mut blend_mode = 0; // 0-7: avg, min/max, swap, exaggerate, depth, barrel, pincushion, fisheye
    let mut anaglyph_mode = false; // Red/cyan 3D glasses effect

    println!("Controls:");
    println!("  T: Toggle dual camera mode ON/OFF");
    println!(
        "  1-8: Select blend mode (1=Avg, 2=MinMax, 3=Swap, 4=Exag, 5=Depth, 6=Barrel, 7=Pincushion, 8=Fisheye)"
    );
    println!("  M: Cycle through blend modes");
    println!("  A: Toggle Anaglyph mode (red/cyan 3D glasses)");
    println!("  F: Toggle auto-focus ON/OFF");
    println!("  +/-: Manually adjust focus distance (when auto-focus is off)");
    println!("  WASD: Move, Mouse: Look, Space/Shift: Up/Down, Esc: Exit");
    println!("\nDual camera mode: ON");
    println!("Blend mode: 0 (Average)");
    println!("Anaglyph mode: OFF");
    println!("Auto-focus: ON");

    let mut last_frame_time = glfw.get_time() as f32;

    // Main loop
    while !window.should_close() {
        // Calculate delta time
        let current_time = glfw.get_time() as f32;
        let delta_time = current_time - last_frame_time;
        last_frame_time = current_time;

        // Process events
        glfw.poll_events();

        // Handle keyboard input (WASD + Space/Shift)
        let velocity = move_speed * delta_time;

        if window.get_key(Key::W) == Action::Press {
            camera.transform.move_local(&glm::vec3(0.0, 0.0, -velocity));
        }
        if window.get_key(Key::S) == Action::Press {
            camera.transform.move_local(&glm::vec3(0.0, 0.0, velocity));
        }
        if window.get_key(Key::A) == Action::Press {
            camera.transform.move_local(&glm::vec3(-velocity, 0.0, 0.0));
        }
        if window.get_key(Key::D) == Action::Press {
            camera.transform.move_local(&glm::vec3(velocity, 0.0, 0.0));
        }
        if window.get_key(Key::Space) == Action::Press {
            camera.transform.move_(&glm::vec3(0.0, velocity, 0.0));
        }
        if window.get_key(Key::LeftShift) == Action::Press {
            camera.transform.move_(&glm::vec3(0.0, -velocity, 0.0));
        }
        if window.get_key(Key::Escape) == Action::Press {
            window.set_should_close(true);
        }

        // Manual focus distance adjustment
        if !auto_focus {
            if window.get_key(Key::Equal) == Action::Press
                || window.get_key(Key::KpAdd) == Action::Press
            {
                look_ahead_distance += delta_time * 2.0;
                look_ahead_distance = look_ahead_distance.min(20.0);
            }
            if window.get_key(Key::Minus) == Action::Press
                || window.get_key(Key::KpSubtract) == Action::Press
            {
                look_ahead_distance -= delta_time * 2.0;
                look_ahead_distance = look_ahead_distance.max(0.5);
            }
        }

        for (_, e) in glfw::flush_messages(&receiver) {
            if matches!(e, WindowEvent::Key(Key::T, _, Action::Press, _)) {
                use_dual_cameras = !use_dual_cameras;
                println!(
                    "Dual camera mode: {}",
                    if use_dual_cameras { "ON" } else { "OFF" }
                );
            }
            if matches!(e, WindowEvent::Key(Key::M, _, Action::Press, _)) {
                blend_mode = (blend_mode + 1) % 8;
                let mode_name = match blend_mode {
                    0 => "Average",
                    1 => "MinMax",
                    2 => "ComponentSwap",
                    3 => "Exaggerate",
                    4 => "DepthBlend",
                    5 => "Barrel",
                    6 => "Pincushion",
                    7 => "Fisheye",
                    _ => "Unknown",
                };
                println!("Blend mode: {} ({})", blend_mode, mode_name);
            }
            if matches!(e, WindowEvent::Key(Key::F, _, Action::Press, _)) {
                auto_focus = !auto_focus;
                println!(
                    "Auto-focus: {} (focus distance: {:.2})",
                    if auto_focus { "ON" } else { "OFF" },
                    look_ahead_distance
                );
            }
            // if matches!(e, WindowEvent::Key(Key::A, _, Action::Press, _)) {
            //     anaglyph_mode = !anaglyph_mode;
            //     println!(
            //         "Anaglyph mode: {} (use red/cyan glasses)",
            //         if anaglyph_mode { "ON" } else { "OFF" }
            //     );
            // }

            // Direct blend mode selection with number keys
            let mode_key = match e {
                WindowEvent::Key(Key::Num1, _, Action::Press, _)
                | WindowEvent::Key(Key::Kp1, _, Action::Press, _) => Some(0),
                WindowEvent::Key(Key::Num2, _, Action::Press, _)
                | WindowEvent::Key(Key::Kp2, _, Action::Press, _) => Some(1),
                WindowEvent::Key(Key::Num3, _, Action::Press, _)
                | WindowEvent::Key(Key::Kp3, _, Action::Press, _) => Some(2),
                WindowEvent::Key(Key::Num4, _, Action::Press, _)
                | WindowEvent::Key(Key::Kp4, _, Action::Press, _) => Some(3),
                WindowEvent::Key(Key::Num5, _, Action::Press, _)
                | WindowEvent::Key(Key::Kp5, _, Action::Press, _) => Some(4),
                WindowEvent::Key(Key::Num6, _, Action::Press, _)
                | WindowEvent::Key(Key::Kp6, _, Action::Press, _) => Some(5),
                WindowEvent::Key(Key::Num7, _, Action::Press, _)
                | WindowEvent::Key(Key::Kp7, _, Action::Press, _) => Some(6),
                WindowEvent::Key(Key::Num8, _, Action::Press, _)
                | WindowEvent::Key(Key::Kp8, _, Action::Press, _) => Some(7),
                _ => None,
            };

            if let Some(new_mode) = mode_key {
                blend_mode = new_mode;
                let mode_name = match blend_mode {
                    0 => "Average",
                    1 => "MinMax",
                    2 => "ComponentSwap",
                    3 => "Exaggerate",
                    4 => "DepthBlend",
                    5 => "Barrel",
                    6 => "Pincushion",
                    7 => "Fisheye",
                    _ => "Unknown",
                };
                println!("Blend mode: {} ({})", blend_mode, mode_name);
            }
        }

        // Handle mouse input
        let current_mouse_pos = window.get_cursor_pos();

        if let Some(last_pos) = last_mouse_pos {
            let delta_x = (current_mouse_pos.0 - last_pos.0) as f32;
            let delta_y = (last_pos.1 - current_mouse_pos.1) as f32; // Reversed Y

            let yaw = delta_x * mouse_sensitivity;
            let pitch = delta_y * mouse_sensitivity;

            // Rotate camera: yaw around world Y, pitch around local X
            camera.transform.rotate(&glm::vec3(0.0, yaw, 0.0));
            camera.transform.rotate_local(&glm::vec3(pitch, 0.0, 0.0));
        }

        last_mouse_pos = Some(current_mouse_pos);

        // Auto-focus: find nearest cube in view direction
        if auto_focus {
            let (_, _, forward) = camera.transform.get_local_axes();
            let mut min_distance = f32::MAX;
            let mut found_target = false;

            for cube_pos in &cube_positions {
                // Vector from camera to cube
                let to_cube = cube_pos - camera.transform.position;

                // Project onto forward direction (negative because we look down -Z)
                let distance = glm::dot(&to_cube, &(-forward));

                // Only consider cubes in front of camera and find closest
                if distance > 0.5 && distance < min_distance {
                    // Also check if cube is roughly in view (within a cone)
                    let to_cube_normalized = glm::normalize(&to_cube);
                    let alignment = glm::dot(&to_cube_normalized, &(-forward));

                    // Only consider cubes within ~60 degree cone
                    if alignment > 0.5 {
                        min_distance = distance;
                        found_target = true;
                    }
                }
            }

            if found_target {
                // Smooth transition to new focus distance
                let target_distance = min_distance;
                look_ahead_distance += (target_distance - look_ahead_distance) * delta_time * 5.0;
            }
        }

        // Calculate view matrices based on mode
        let (left_view, right_view) = if use_dual_cameras {
            // Dual camera mode: calculate separate view for each eye
            // Get the camera's local axes
            let (right, _, forward) = camera.transform.get_local_axes();

            // Calculate look-at point ahead of the camera
            let look_at_point = camera.transform.position - forward * look_ahead_distance;

            // Left eye: offset to the left
            let left_eye_pos = camera.transform.position - right * (eye_separation / 2.0);

            // Right eye: offset to the right
            let right_eye_pos = camera.transform.position + right * (eye_separation / 2.0);

            // Create view matrices for both eyes looking at the same point
            let left_view = glm::look_at(&left_eye_pos, &look_at_point, &glm::Vec3::y_axis());
            let right_view = glm::look_at(&right_eye_pos, &look_at_point, &glm::Vec3::y_axis());

            (left_view, right_view)
        } else {
            // Standard single camera mode: use the same view for both
            let view = camera.transform.view();
            (view, view)
        };

        // Render
        unsafe {
            gl::ClearColor(0.1, 0.1, 0.15, 1.0);
        }
        cgl::clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

        // Set up shader and lighting uniforms (once per frame)
        cgl::use_program(program);
        unsafe {
            gl::Uniform1i(blend_mode_loc, blend_mode);
            gl::Uniform1i(anaglyph_mode_loc, if anaglyph_mode { 1 } else { 0 });
            gl::Uniform3fv(view_pos_loc, 1, camera.transform.position.as_ptr());

            // Upload light positions and colors
            for i in 0..3 {
                let pos_loc =
                    cgl::get_location(program, &format!("lightPositions[{}]", i)).unwrap();
                let col_loc = cgl::get_location(program, &format!("lightColors[{}]", i)).unwrap();
                gl::Uniform3fv(pos_loc, 1, light_positions[i].as_ptr());
                gl::Uniform3fv(col_loc, 1, light_colors[i].as_ptr());
            }
        }

        // Render all cubes
        cgl::bind_vao(vao);
        for (i, pos) in cube_positions.iter().enumerate() {
            // Model matrix for this cube
            let model = glm::translation(pos);
            let normal_matrix = glm::transpose(&glm::inverse(&model));

            // Calculate MVP matrices for both eyes
            let mvp_left = camera.projection * left_view * model;
            let mvp_right = camera.projection * right_view * model;

            unsafe {
                gl::UniformMatrix4fv(mvp_left_loc, 1, gl::FALSE, mvp_left.as_ptr());
                gl::UniformMatrix4fv(mvp_right_loc, 1, gl::FALSE, mvp_right.as_ptr());
                gl::UniformMatrix4fv(model_loc, 1, gl::FALSE, model.as_ptr());
                gl::UniformMatrix4fv(normal_matrix_loc, 1, gl::FALSE, normal_matrix.as_ptr());
                gl::Uniform3fv(object_color_loc, 1, cube_colors[i].as_ptr());
            }

            cgl::draw_elements(gl::TRIANGLES, 36, 0, gl::UNSIGNED_INT);
        }

        window.swap_buffers();
    }
}
