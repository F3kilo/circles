pub mod surface;
pub mod swapchain;
use crate::vulkan::base::VulkanBase;
use raw_window_handle::RawWindowHandle;
use slog::Logger;
use surface::Surface;
use swapchain::Swapchain;

pub struct WindowData {
    pub window_handle: RawWindowHandle,
    pub width: u32,
    pub height: u32,
}

pub struct VulkanPresent {
    swapchain: Swapchain,
    surface: Surface,
    logger: Logger,
}

impl VulkanPresent {
    pub fn new(base: &VulkanBase, window_data: WindowData, logger: Logger) -> Self {
        let surface = Surface::new(base, window_data.window_handle, logger.clone());
        let swapchain = Swapchain::new(
            base,
            &surface,
            ash::vk::Extent2D {
                width: window_data.width,
                height: window_data.height,
            },
            logger.clone(),
        );
        Self {
            logger,
            surface,
            swapchain,
        }
    }

    pub fn destroy(&mut self) {
        debug!(self.logger, "VulkanPresent destroy() called.");
        self.swapchain.destroy();
        self.surface.destroy();
    }
}
