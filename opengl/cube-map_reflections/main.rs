use glfw::Context as _;
use nalgebra_glm as glm;

fn main() {
    let width = 1280;
    let height = 720;

    let mut glfw = glfw::init_no_callbacks().unwrap();
    glfw.window_hint(glfw::WindowHint::Resizable(false));

    let (mut window, _) = glfw
        .create_window(width, height, "", glfw::WindowMode::Windowed)
        .unwrap();

    gl::load_with(|symbol| window.get_proc_address(symbol));
    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

    let cube_mesh = cgl::utils::load_mesh(r"cube-map_reflections\assets\cube.glb");
    let icosphere_mesh = cgl::utils::load_mesh(r"cube-map_reflections\assets\icosphere.glb");

    let reflection_shader = ReflectionShader::new();
    let background_shader =
        BackgroundShader::new(include_str!(r"..\environment_mapping\shaders\main.vert"));
    let cubemap_background_shader = BackgroundShader::new(include_str!(r"shaders\to_cubemap.vert"));
    let object_shader = ObjectShader::new();

    let aspect = width as f32 / height as f32;
    let projection = glm::perspective(aspect, 45.0_f32.to_radians(), 0.1, 1000.0);

    let images = [
        (
            image::open(r"environment_mapping\skybox\front.jpg").unwrap(),
            gl::TEXTURE_CUBE_MAP_POSITIVE_Z,
        ),
        (
            image::open(r"environment_mapping\skybox\back.jpg").unwrap(),
            gl::TEXTURE_CUBE_MAP_NEGATIVE_Z,
        ),
        (
            image::open(r"environment_mapping\skybox\left.jpg").unwrap(),
            gl::TEXTURE_CUBE_MAP_NEGATIVE_X,
        ),
        (
            image::open(r"environment_mapping\skybox\right.jpg").unwrap(),
            gl::TEXTURE_CUBE_MAP_POSITIVE_X,
        ),
        (
            image::open(r"environment_mapping\skybox\top.jpg").unwrap(),
            gl::TEXTURE_CUBE_MAP_POSITIVE_Y,
        ),
        (
            image::open(r"environment_mapping\skybox\bottom.jpg").unwrap(),
            gl::TEXTURE_CUBE_MAP_NEGATIVE_Y,
        ),
    ];

    let env_map = cgl::create_texture(gl::TEXTURE_CUBE_MAP).unwrap();
    cgl::bind_texture(env_map, gl::TEXTURE_CUBE_MAP);
    for (img, face) in images {
        cgl::texture_image2d(
            face,
            0,
            gl::RGB,
            img.width() as i32,
            img.height() as i32,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            img.as_bytes(),
        );
    }
    cgl::generate_mipmaps(gl::TEXTURE_CUBE_MAP);
    cgl::texture_parameter(
        gl::TEXTURE_CUBE_MAP,
        gl::TEXTURE_MIN_FILTER,
        gl::LINEAR_MIPMAP_LINEAR,
    );
    cgl::texture_parameter(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MIN_FILTER, gl::LINEAR);
    cgl::texture_parameter(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE);
    cgl::texture_parameter(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE);
    cgl::texture_parameter(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_R, gl::CLAMP_TO_EDGE);

    let err = unsafe { gl::GetError() };
    println!("{err}");

    let res = 1024;

    let reflection_map = cgl::create_texture(gl::TEXTURE_CUBE_MAP).unwrap();
    cgl::bind_texture(reflection_map, gl::TEXTURE_CUBE_MAP);
    cgl::texture_parameter(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MIN_FILTER, gl::LINEAR);
    cgl::texture_parameter(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE);
    cgl::texture_parameter(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE);
    cgl::texture_parameter(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_R, gl::CLAMP_TO_EDGE);

    let err = unsafe { gl::GetError() };
    println!("{err}");

    for target in gl::TEXTURE_CUBE_MAP_POSITIVE_X..=gl::TEXTURE_CUBE_MAP_NEGATIVE_Z {
        cgl::texture_storage2d(target, 1, gl::RGBA8, res, res);
    }

    let err = unsafe { gl::GetError() };
    println!("{err}");

    let renderbuffer = cgl::create_renderbuffer().unwrap();
    cgl::bind_renderbuffer(renderbuffer, gl::RENDERBUFFER);
    cgl::renderbuffer_storage(gl::RENDERBUFFER, gl::DEPTH_COMPONENT24, res, res);

    let framebuffer = cgl::create_framebuffer().unwrap();
    cgl::bind_framebuffer(framebuffer, gl::FRAMEBUFFER);
    for target in gl::TEXTURE_CUBE_MAP_POSITIVE_X..=gl::TEXTURE_CUBE_MAP_NEGATIVE_Z {
        let attachment = gl::COLOR_ATTACHMENT0 + (target - gl::TEXTURE_CUBE_MAP_POSITIVE_X);
        cgl::framebuffer_texture2d(gl::FRAMEBUFFER, attachment, target, reflection_map, 0);
    }
    cgl::framebuffer_renderbuffer(
        gl::FRAMEBUFFER,
        gl::DEPTH_ATTACHMENT,
        gl::RENDERBUFFER,
        renderbuffer,
    );

    cgl::enable(gl::TEXTURE_CUBE_MAP_SEAMLESS);
    cgl::enable(gl::DEPTH_TEST);

    let err = unsafe { gl::GetError() };
    println!("{err}");

    while !window.should_close() {
        glfw.poll_events();

        let time = glfw.get_time() as f32;
        // let view = glm::rotation(time * std::f32::consts::PI / 12.0, &glm::Vec3::x_axis());
        let view = glm::identity();
        let clip2world = glm::inverse(&(projection * view));

        let objects_transforms = transforms(time);
        let reflective_object_pos = glm::vec3(0.0, -0.5, -10.0);

        // ------------ draw into cubemap ------------ //
        cgl::viewport(0, 0, res, res);
        cgl::bind_texture(env_map, gl::TEXTURE_CUBE_MAP);
        cgl::bind_framebuffer(framebuffer, gl::FRAMEBUFFER);

        let cubemap_projection = glm::perspective(1.0, 90_f32.to_radians(), 0.1, 100.0);
        let rotations = [
            // right
            glm::rotation(90.0_f32.to_radians(), &glm::Vec3::y_axis()),
            // left
            glm::rotation(-90.0_f32.to_radians(), &glm::Vec3::y_axis()),
            // top
            glm::rotation(90.0_f32.to_radians(), &glm::Vec3::x_axis()),
            // bottom
            glm::rotation(-90.0_f32.to_radians(), &glm::Vec3::x_axis()),
            // front
            glm::rotation(0.0, &glm::Vec3::y_axis()),
            // back
            glm::rotation(180.0_f32.to_radians(), &glm::Vec3::y_axis()),
        ];

        for (target, rot) in (gl::TEXTURE_CUBE_MAP_POSITIVE_X..=gl::TEXTURE_CUBE_MAP_NEGATIVE_Z)
            .zip(rotations.iter())
        {
            let cubemap_clip2world = glm::inverse(&(cubemap_projection * rot));

            let attachment = gl::COLOR_ATTACHMENT0 + (target - gl::TEXTURE_CUBE_MAP_POSITIVE_X);
            cgl::draw_buffer(attachment);
            cgl::clear(gl::DEPTH_BUFFER_BIT);

            cgl::use_program(cubemap_background_shader.program);
            unsafe {
                gl::UniformMatrix4fv(
                    cubemap_background_shader.clip2world_location,
                    1,
                    0,
                    cubemap_clip2world.as_ptr(),
                );
            }

            cgl::draw_arrays(gl::TRIANGLES, 0, 3);
        }

        // ------------
        cgl::bind_framebuffer(0, gl::FRAMEBUFFER);
        cgl::clear(gl::DEPTH_BUFFER_BIT);
        cgl::viewport(0, 0, width as i32, height as i32);

        cgl::use_program(object_shader.program);
        let model = glm::translation(&glm::vec3(0.0, 0.0, -10.0))
            * glm::rotation(time, &glm::Vec3::x_axis());
        let mvp = projection * view * model;
        unsafe {
            gl::UniformMatrix4fv(object_shader.mvp_location, 1, 0, mvp.as_ptr());
            gl::UniformMatrix4fv(object_shader.model_location, 1, 0, model.as_ptr());
        }
        cgl::utils::draw_mesh(&cube_mesh);

        // for t in &objects_transforms[..] {
        // }

        cgl::bind_texture(reflection_map, gl::TEXTURE_CUBE_MAP);
        cgl::use_program(background_shader.program);
        unsafe {
            gl::UniformMatrix4fv(
                background_shader.clip2world_location,
                1,
                0,
                clip2world.as_ptr(),
            );
        }
        cgl::draw_arrays(gl::TRIANGLES, 0, 3);

        window.swap_buffers();

        // let rotations = [
        //     // right
        //     glm::rotation(90.0_f32.to_radians(), &glm::Vec3::y_axis()),
        //     // left
        //     glm::rotation(-90.0_f32.to_radians(), &glm::Vec3::y_axis()),
        //     // top
        //     glm::rotation(90.0_f32.to_radians(), &glm::Vec3::x_axis()),
        //     // bottom
        //     glm::rotation(-90.0_f32.to_radians(), &glm::Vec3::x_axis()),
        //     // front
        //     glm::rotation(0.0, &glm::Vec3::y_axis()),
        //     // back
        //     glm::look_at(
        //         &reflective_object_pos,
        //         &glm::vec3(0.0, 0.0, 1.0),
        //         &glm::vec3(0.0, -1.0, 0.0),
        // ),
        // glm::rotation(180.0_f32.to_radians(), &glm::Vec3::y_axis()),
        // ];
        // cgl::bind_framebuffer(framebuffer, gl::FRAMEBUFFER);
        // cgl::bind_texture(env_map, gl::TEXTURE_CUBE_MAP);
        // cgl::viewport(0, 0, res, res);
        // for (target, rot) in (gl::TEXTURE_CUBE_MAP_POSITIVE_X..=gl::TEXTURE_CUBE_MAP_NEGATIVE_Z)
        //     .zip(rotations.iter())
        // {
        //     let view_projection =
        //         cubemap_projection * rot * glm::translation(&-reflective_object_pos);
        //     let clip2world = glm::inverse(&(cubemap_projection * rot));
        //     let attachment = gl::COLOR_ATTACHMENT0 + (target - gl::TEXTURE_CUBE_MAP_POSITIVE_X);

        //     cgl::draw_buffer(attachment);
        //     cgl::clear(gl::DEPTH_BUFFER_BIT);

        // cgl::use_program(object_shader.program);

        // for t in &objects_transforms {
        //     let mvp = view_projection * t;
        //     unsafe {
        //         gl::UniformMatrix4fv(object_shader.mvp_location, 1, 0, mvp.as_ptr());
        //         gl::UniformMatrix4fv(object_shader.model_location, 1, 0, t.as_ptr());
        //     }
        //     cgl::utils::draw_mesh(&icosphere_mesh);
        // }

        //     cgl::use_program(background_shader.program);
        //     unsafe {
        //         gl::UniformMatrix4fv(
        //             background_shader.clip2world_location,
        //             1,
        //             0,
        //             clip2world.as_ptr(),
        //         );
        //     }
        //     cgl::draw_arrays(gl::TRIANGLES, 0, 3);
        // }

        // draw reflective object
        // let angle = time * std::f32::consts::PI / 15.0;
        // let angle = 0.0;
        // let model =
        //     glm::translation(&reflective_object_pos) * glm::rotation(angle, &glm::Vec3::y_axis());
        // let mvp = projection * view * model;
        // cgl::bind_texture(reflection_map, gl::TEXTURE_CUBE_MAP);
        // cgl::use_program(reflection_shader.program);
        // unsafe {
        //     gl::UniformMatrix4fv(reflection_shader.mvp_location, 1, 0, mvp.as_ptr());
        //     gl::UniformMatrix4fv(reflection_shader.model_location, 1, 0, model.as_ptr());
        //     gl::Uniform3f(reflection_shader.eye_location, 0.0, 0.0, 0.0);
        // }
        // cgl::utils::draw_mesh(&cube_mesh);

        // draw common objects

        // draw backgroung
    }
}

fn transforms(time: f32) -> Vec<glm::Mat4> {
    let scale = glm::scaling(&glm::Vec3::from_element(0.5));
    let num = 5;
    let mut ret = vec![];
    let center = glm::vec3(0.0, -1.0, -10.0);
    for i in 0..num {
        let angle =
            time * std::f32::consts::FRAC_PI_3 + std::f32::consts::PI * 2.0 / num as f32 * i as f32;
        let x = angle.sin() * 3.0;
        let z = angle.cos() * 3.0;

        let position = center + glm::vec3(x, 0.0, z);
        ret.push(glm::translation(&position) * scale);
    }
    ret
}

pub struct ReflectionShader {
    pub program: u32,
    pub mvp_location: i32,
    pub model_location: i32,
    pub eye_location: i32,
}

impl ReflectionShader {
    pub fn new() -> Self {
        let vertex_shader_source = include_str!(r"shaders\main.vert");
        let fragment_shader_source = include_str!(r"shaders\reflection.frag");
        let vert = cgl::compile_shader(gl::VERTEX_SHADER, vertex_shader_source).unwrap();
        let frag = cgl::compile_shader(gl::FRAGMENT_SHADER, fragment_shader_source).unwrap();
        let program = cgl::create_vert_frag_prog(vert, frag).unwrap();
        let mvp_location = cgl::get_location(program, "mvp").unwrap();
        let model_location = cgl::get_location(program, "model").unwrap();
        let eye_location = cgl::get_location(program, "eye").unwrap();

        Self {
            program,
            mvp_location,
            model_location,
            eye_location,
        }
    }
}

pub struct BackgroundShader {
    pub program: u32,
    pub clip2world_location: i32,
}

impl BackgroundShader {
    pub fn new(vert: &str) -> Self {
        // let vert = include_str!(r"..\environment_mapping\shaders\main.vert");
        let fragment_shader_source = include_str!(r"..\environment_mapping\shaders\main.frag");
        let vert = cgl::compile_shader(gl::VERTEX_SHADER, vert).unwrap();
        let frag = cgl::compile_shader(gl::FRAGMENT_SHADER, fragment_shader_source).unwrap();
        let program = cgl::create_vert_frag_prog(vert, frag).unwrap();
        let clip2world_location = cgl::get_location(program, "clip2world").unwrap();

        Self {
            program,
            clip2world_location,
        }
    }
}

pub struct ObjectShader {
    pub program: u32,
    pub mvp_location: i32,
    pub model_location: i32,
}

impl ObjectShader {
    pub fn new() -> Self {
        let vertex_shader_source = include_str!(r"shaders\main.vert");
        let fragment_shader_source = include_str!(r"shaders\main.frag");
        let vert = cgl::compile_shader(gl::VERTEX_SHADER, vertex_shader_source).unwrap();
        let frag = cgl::compile_shader(gl::FRAGMENT_SHADER, fragment_shader_source).unwrap();
        let program = cgl::create_vert_frag_prog(vert, frag).unwrap();
        let mvp_location = cgl::get_location(program, "mvp").unwrap();
        let model_location = cgl::get_location(program, "model").unwrap();

        Self {
            program,
            mvp_location,
            model_location,
        }
    }
}
