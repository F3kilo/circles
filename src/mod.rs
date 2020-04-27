mod vulkan;
use vulkan::Vulkan;
use winit::window::Window;

pub struct ColorMeshRenderer {
    vk: Vulkan,
}

impl ColorMeshRenderer {
    pub fn new(app_name: &str, window: &Window) -> Self {
        Self {
            vk: Vulkan::new(app_name, window),
        }
    }
}
