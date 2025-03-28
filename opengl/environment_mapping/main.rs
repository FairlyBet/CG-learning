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

    let vertex_shader_source = include_str!("../shaders/cubemap_background.vert");
    let fragment_shader_source = include_str!("../shaders/cubemap_background.frag");
    let vert = cgl::compile_shader(gl::VERTEX_SHADER, vertex_shader_source).unwrap();
    let frag = cgl::compile_shader(gl::FRAGMENT_SHADER, fragment_shader_source).unwrap();
    let program = cgl::create_vert_frag_prog(vert, frag).unwrap();
    let clip2world_loc = cgl::get_location(program, "clip2world").unwrap();
    cgl::use_program(program);

    let aspect = width as f32 / height as f32;
    let projection = glm::perspective(aspect, 45.0_f32.to_radians(), 0.1, 1000.0);

    let images = [
        (
            image::open("assets/skybox/front.jpg").unwrap(),
            gl::TEXTURE_CUBE_MAP_POSITIVE_Z,
        ),
        (
            image::open("assets/skybox/back.jpg").unwrap(),
            gl::TEXTURE_CUBE_MAP_NEGATIVE_Z,
        ),
        (
            image::open("assets/skybox/left.jpg").unwrap(),
            gl::TEXTURE_CUBE_MAP_NEGATIVE_X,
        ),
        (
            image::open("assets/skybox/right.jpg").unwrap(),
            gl::TEXTURE_CUBE_MAP_POSITIVE_X,
        ),
        (
            image::open("assets/skybox/top.jpg").unwrap(),
            gl::TEXTURE_CUBE_MAP_POSITIVE_Y,
        ),
        (
            image::open("assets/skybox/bottom.jpg").unwrap(),
            gl::TEXTURE_CUBE_MAP_NEGATIVE_Y,
        ),
    ];

    let cube_map = cgl::create_texture(gl::TEXTURE_CUBE_MAP).unwrap();
    cgl::bind_texture(cube_map, gl::TEXTURE_CUBE_MAP);

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

    cgl::enable(gl::TEXTURE_CUBE_MAP_SEAMLESS);
    cgl::enable(gl::DEPTH_TEST);

    while !window.should_close() {
        glfw.poll_events();

        let angle = glfw.get_time() as f32 * std::f32::consts::PI / 12.0;
        let rotation = glm::rotation(angle, &glm::Vec3::y_axis());
        // let rotation = glm::rotation(30.0_f32.to_radians(), &glm::Vec3::y_axis());
        let mvp = projection * rotation;
        let clip2world = mvp.try_inverse().unwrap();

        unsafe {
            gl::UniformMatrix4fv(clip2world_loc, 1, gl::FALSE, clip2world.as_ptr());
        }

        cgl::clear(gl::DEPTH_BUFFER_BIT);
        cgl::draw_arrays(gl::TRIANGLES, 0, 3);
        window.swap_buffers();
    }
}
