use ash::vk;
use glfw::{Action, ClientApiHint, Key, WindowHint};
use raw_window_handle::{HasDisplayHandle as _, HasWindowHandle as _};
use rustc_hash::FxHashSet as HashSet;
use std::ffi::CString;
use vulkan_demo::{Vulkan, util};

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
    glfw.window_hint(WindowHint::ClientApi(ClientApiHint::NoApi));

    let (mut window, events) = glfw
        .create_window(600, 400, "", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");
    assert!(glfw.vulkan_supported());
    window.set_key_polling(true);

    let entry = unsafe { ash::Entry::load().unwrap() };

    let application_info = vk::ApplicationInfo::default()
        .application_name(c"")
        .application_version(0)
        .engine_name(c"")
        .engine_version(0)
        .api_version(vk::make_api_version(0, 1, 0, 0));
    let extensions = glfw
        .get_required_instance_extensions()
        .unwrap()
        .into_iter()
        .map(|ext| CString::new(ext).unwrap())
        .collect::<Vec<_>>();
    let extension_pointers = util::to_pointers(&extensions);

    let mut create_info = vk::InstanceCreateInfo::default()
        .application_info(&application_info)
        .enabled_extension_names(&extension_pointers);

    let validation_layers = [c"VK_LAYER_KHRONOS_validation"];
    let validation_layers_pointers = util::to_pointers(&validation_layers);
    if cfg!(debug_assertions) {
        let avaialble_layers = unsafe { entry.enumerate_instance_layer_properties().unwrap() };
        let debug_layers_available = validation_layers.iter().all(|target| {
            avaialble_layers
                .iter()
                .any(|layer| (*target).eq(layer.layer_name_as_c_str().unwrap()))
        });
        assert!(debug_layers_available);
        create_info = create_info.enabled_layer_names(&validation_layers_pointers);
    }

    let instance = unsafe { entry.create_instance(&create_info, None).unwrap() };
    let surface_instance = ash::khr::surface::Instance::new(&entry, &instance);

    let vk = Vulkan {
        surface_instance,
        instance,
        entry,
    };

    let surface = unsafe {
        ash_window::create_surface(
            &vk.entry,
            &vk.instance,
            window.display_handle().unwrap().as_raw(),
            window.window_handle().unwrap().as_raw(),
            None,
        )
        .unwrap()
    };

    let device_extentions = [ash::vk::KHR_SWAPCHAIN_NAME];
    let device_extentions_pointers = util::to_pointers(&device_extentions);

    let (physical_device, family_indices) =
        vulkan_demo::find_graphics_device(&vk, surface, &device_extentions).unwrap();

    let unique_queue_families = HashSet::from_iter(
        [
            family_indices.graphics_family,
            family_indices.present_family,
        ]
        .into_iter(),
    );
    let mut queue_create_infos = vec![];
    for queue_family in unique_queue_families {
        let queue_create_info = vk::DeviceQueueCreateInfo::default()
            .queue_family_index(queue_family)
            .queue_priorities(&[1.0]);
        queue_create_infos.push(queue_create_info);
    }
    let physical_device_features = vk::PhysicalDeviceFeatures::default();

    let device_create_info = vk::DeviceCreateInfo::default()
        .queue_create_infos(&queue_create_infos)
        .enabled_features(&physical_device_features)
        .enabled_extension_names(&device_extentions_pointers);

    let logical_device = unsafe {
        vk.instance
            .create_device(physical_device, &device_create_info, None)
            .unwrap()
    };

    let surface_capabilities = unsafe {
        vk.surface_instance
            .get_physical_device_surface_capabilities(physical_device, surface)
            .unwrap()
    };
    let formats = unsafe {
        vk.surface_instance
            .get_physical_device_surface_formats(physical_device, surface)
            .unwrap()
    };
    let presentation_modes = unsafe {
        vk.surface_instance
            .get_physical_device_surface_present_modes(physical_device, surface)
            .unwrap()
    };
    println!("{surface_capabilities:?}");
    println!("{formats:?}");
    println!("{presentation_modes:?}");

    let graphics_queue =
        unsafe { logical_device.get_device_queue(family_indices.graphics_family, 0) };
    let present_queue =
        unsafe { logical_device.get_device_queue(family_indices.present_family, 0) };
    println!("{graphics_queue:?}");
    println!("{present_queue:?}");

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
