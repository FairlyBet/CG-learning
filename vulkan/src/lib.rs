pub mod util;

pub struct Vulkan {
    pub surface_instance: ash::khr::surface::Instance,
    pub instance: ash::Instance,
    pub entry: ash::Entry,
}

pub struct QueueFamilyIndices {
    pub graphics_family: u32,
    pub present_family: u32,
}

pub fn find_graphics_device(
    vk: &Vulkan,
    surface: ash::vk::SurfaceKHR,
    required_device_extentions: &[ &std::ffi::CStr ]
) -> Option<(ash::vk::PhysicalDevice, QueueFamilyIndices)> {
    let devices = unsafe { vk.instance.enumerate_physical_devices().unwrap() };

    for device in devices {
        let mut graphics_family = None;
        let mut present_family = None;
        let families = unsafe {
            vk.instance
                .get_physical_device_queue_family_properties(device)
        };
        for (i, family) in families.iter().enumerate() {
            let graphics_support = family.queue_flags.contains(ash::vk::QueueFlags::GRAPHICS);
            let surface_support = unsafe {
                vk.surface_instance
                    .get_physical_device_surface_support(device, i as u32, surface)
                    .unwrap_or_default()
            };
            if graphics_support && graphics_family.is_none() {
                graphics_family = Some(i as u32);
            }
            if surface_support && present_family.is_none() {
                present_family = Some(i as u32);
            }
        }

        let properties = unsafe { vk.instance.get_physical_device_properties(device) };
        let is_discrete = properties.device_type == ash::vk::PhysicalDeviceType::DISCRETE_GPU;

        let extensions = unsafe {
            vk.instance
                .enumerate_device_extension_properties(device)
                .unwrap()
        };
        let supports_extensions = required_device_extentions.iter().all(|target| {
            extensions
                .iter()
                .any(|ext| ext.extension_name_as_c_str().unwrap().eq(target))
        });

        if graphics_family.is_some()
            && present_family.is_some()
            && is_discrete
            && supports_extensions
        {
            let indices = QueueFamilyIndices {
                graphics_family: graphics_family.unwrap(),
                present_family: present_family.unwrap(),
            };
            return Some((device, indices));
        }
    }
    None
}
