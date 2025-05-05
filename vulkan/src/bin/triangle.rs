use ash::vk;
use glfw::{Action, ClientApiHint, Key, WindowHint};
use std::ffi::CString;

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
    glfw.window_hint(WindowHint::ClientApi(ClientApiHint::NoApi));

    let (mut window, events) = glfw
        .create_window(600, 400, "", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");
    assert!(glfw.vulkan_supported());
    window.set_key_polling(true);

    let extensions = glfw
        .get_required_instance_extensions()
        .unwrap()
        .into_iter()
        .map(|ext| CString::new(ext).unwrap())
        .collect::<Vec<_>>();
    let extension_pointers = extensions
        .iter()
        .map(|ext| ext.as_ptr())
        .collect::<Vec<_>>();
    println!("{extensions:?}");

    let entry = unsafe { ash::Entry::load().unwrap() };

    let application_info = vk::ApplicationInfo::default()
        .application_name(c"")
        .application_version(0)
        .engine_name(c"")
        .engine_version(0)
        .api_version(vk::make_api_version(0, 1, 0, 0));

    let validation_layers = vec![c"VK_LAYER_KHRONOS_validation"];
    let validation_layers_pointers = validation_layers
        .iter()
        .map(|ext| ext.as_ptr())
        .collect::<Vec<_>>();

    if cfg!(debug_assertions) {
        let avaialble_layers = unsafe { entry.enumerate_instance_layer_properties().unwrap() };
        let debug_layers = validation_layers.iter().all(|layer| {
            avaialble_layers
                .iter()
                .any(|l| (*layer).eq(l.layer_name_as_c_str().unwrap()))
        });
        assert!(debug_layers);
    }

    let create_info = vk::InstanceCreateInfo::default()
        .application_info(&application_info)
        .enabled_extension_names(&extension_pointers)
        .enabled_layer_names(&validation_layers_pointers);

    let instance = unsafe { entry.create_instance(&create_info, None).unwrap() };
    let devices = unsafe { instance.enumerate_physical_devices().unwrap() };
    let extensions = unsafe { entry.enumerate_instance_extension_properties(None).unwrap() };
    for ext in extensions {
        println!("{ext:?}");
    }

    // window.create_window_surface(instance, allocator, surface)

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        _ => {}
    }
}
