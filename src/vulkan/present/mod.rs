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
        debug!(logger, "Creating surface.");
        let surface = Surface::new(base, window_data.window_handle, logger.clone());
        debug!(logger, "Creating swapchain.");
        let swapchain = Swapchain::new(
            base,
            &surface,
            ash::vk::Extent2D {
                width: window_data.width,
                height: window_data.height,
            },
            logger.clone(),
        );

        debug!(logger, "Presenter initialized.");

        Self {
            logger,
            surface,
            swapchain,
        }
    }

    pub fn get_surface(&self) -> &Surface {
        &self.surface
    }

    pub fn get_swapchain(&self) -> &Swapchain {
        &self.swapchain
    }

    pub fn destroy(&mut self, base: &VulkanBase) {
        debug!(self.logger, "VulkanPresent destroy() called.");
        let vk_device = base.get_device().get_vk_device();
        self.swapchain.destroy(vk_device);
        self.surface.destroy();
    }
}
