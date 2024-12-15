use cgl::{camera::Camera, transform::Transform, utils};
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

    let cube_mesh = utils::load_mesh("assets/cube.glb");
    let sphere_mesh = utils::load_mesh("assets/sphere.glb");

    let shadow_res = 1024;
    let shadow_map = cgl::create_texture(gl::TEXTURE_2D).unwrap();
    cgl::bind_texture(shadow_map, gl::TEXTURE_2D);
    cgl::texture_parameter(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR);
    // cgl::texture_storage2d(gl::TEXTURE_2D, 1, gl::DEPTH_COMPONENT, shadow_res, shadow_res);
    unsafe {
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::DEPTH_COMPONENT as i32,
            shadow_res,
            shadow_res,
            0,
            gl::DEPTH_COMPONENT,
            gl::FLOAT,
            std::ptr::null(),
        );
    }

    let framebuffer = cgl::create_framebuffer().unwrap();
    cgl::bind_framebuffer(framebuffer, gl::FRAMEBUFFER);
    cgl::framebuffer_texture2d(
        gl::FRAMEBUFFER,
        gl::DEPTH_ATTACHMENT,
        gl::TEXTURE_2D,
        shadow_map,
        0,
    );
    cgl::draw_buffer(gl::NONE);
    cgl::read_buffer(gl::NONE);

    let vert = include_str!("../shaders/position_only.vert");
    let frag = include_str!("../shaders/shadowmap.frag");
    let vert = cgl::compile_shader(gl::VERTEX_SHADER, vert).unwrap();
    let frag = cgl::compile_shader(gl::FRAGMENT_SHADER, frag).unwrap();
    let shadowmap_program = cgl::create_vert_frag_prog(vert, frag).unwrap();
    let shadowmap_mvp = cgl::get_location(shadowmap_program, "mvp").unwrap();

    let vert = include_str!("../shaders/screen_rasterize.vert");
    let frag = include_str!("../shaders/screen_texture.frag");
    let vert = cgl::compile_shader(gl::VERTEX_SHADER, vert).unwrap();
    let frag = cgl::compile_shader(gl::FRAGMENT_SHADER, frag).unwrap();
    let screen_program = cgl::create_vert_frag_prog(vert, frag).unwrap();

    let objects = [
        (
            Transform {
                position: glm::vec3(4.0, 4.0, -10.0),
                ..Default::default()
            },
            &sphere_mesh,
        ),
        (
            Transform {
                position: glm::vec3(0.0, 0.0, 0.0),
                ..Default::default()
            },
            &cube_mesh,
        ),
        (
            Transform {
                position: glm::vec3(-4.0, -2.0, -7.0),
                ..Default::default()
            },
            &sphere_mesh,
        ),
        (
            Transform {
                position: glm::vec3(-3.0, 5.0, -11.0),
                ..Default::default()
            },
            &cube_mesh,
        ),
    ];

    unsafe {
        gl::DepthFunc(gl::LESS);
    }
    let light = Camera::with_perspective(1.0, 90.0_f32.to_radians(), 0.1, 100.0);
    // light.transform.position.z -= 7.0;

    while !window.should_close() {
        glfw.poll_events();
        let _time = glfw.get_time() as f32;

        cgl::enable(gl::DEPTH_TEST);
        cgl::viewport(0, 0, shadow_res, shadow_res);
        cgl::bind_framebuffer(framebuffer, gl::FRAMEBUFFER);
        cgl::clear(gl::DEPTH_BUFFER_BIT);
        cgl::use_program(shadowmap_program);

        for object in &objects {
            let mvp = light.view_projection() * object.0.model();
            unsafe {
                gl::UniformMatrix4fv(shadowmap_mvp, 1, 0, mvp.as_ptr());
            }
            for mesh in object.1 {
                cgl::bind_vao(mesh.0);
                cgl::draw_elements(gl::TRIANGLES, mesh.1, 0, gl::UNSIGNED_INT);
            }
        }

        cgl::disable(gl::DEPTH_TEST);
        cgl::viewport(0, 0, width as i32, height as i32);
        cgl::bind_framebuffer(0, gl::FRAMEBUFFER);
        cgl::bind_texture(shadow_map, gl::TEXTURE_2D);
        cgl::use_program(screen_program);
        cgl::draw_arrays(gl::TRIANGLES, 0, 3);

        window.swap_buffers();
    }
}
