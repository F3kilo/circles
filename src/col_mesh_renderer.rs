use crate::vulkan::surface::Surface;
use crate::vulkan::swapchain::Swapchain;
use crate::vulkan::Vulkan;
use ash::vk;
use raw_window_handle::RawWindowHandle;
use slog::Logger;
use std::rc::Rc;

pub struct ColMeshRenderer {
    surface: Surface,
    swapchain: Swapchain,
    vk: Rc<Vulkan>,
    logger: Logger,
}

impl ColMeshRenderer {
    pub fn new(
        window_handle: RawWindowHandle,
        vk: Rc<Vulkan>,
        window_size: vk::Extent2D,
        logger: Logger,
    ) -> Self {
        let surface = Surface::new(
            vk.get_entry(),
            vk.get_instance().get_vk_instance(),
            window_handle,
            logger.clone(),
        );

        let swapchain = Swapchain::new(vk.get_instance(), &surface, window_size, logger.clone());

        Self {
            vk,
            surface,
            swapchain,
            logger,
        }
    }
}

impl Drop for ColMeshRenderer {
    fn drop(&mut self) {
        debug!(self.logger, "ColMeshRenderer drop called");
        self.swapchain.destroy();
        self.surface.destroy();
    }
}
