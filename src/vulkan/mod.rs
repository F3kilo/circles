pub mod command_buffers;
pub mod debug_callback;
pub mod device;
pub mod instance;
pub mod physical_device;
pub mod surface;
pub mod swapchain;
use ash::Entry;
use winit::window::Window;

/// Order of fields defines order of drop!
pub struct Vulkan {
    instance: instance::Instance,
    entry: Entry,
}

impl Vulkan {
    pub fn new(app_name: &str, window: &Window) -> Self {
        let entry = Entry::new().expect("Can't init vk entry!");
        let instance = instance::Instance::new(&entry, app_name, window);
        Self { entry, instance }
    }
}
