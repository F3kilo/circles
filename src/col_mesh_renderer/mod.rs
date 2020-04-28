mod depth_image;

use crate::vulkan::command_buffers::CommandBuffers;
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
    command_buffers: CommandBuffers,
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
        let instance = vk.get_instance();
        let surface = Surface::new(
            vk.get_entry(),
            instance.get_vk_instance(),
            window_handle,
            logger.clone(),
        );

        let swapchain = Swapchain::new(instance, &surface, window_size, logger.clone());

        let vk_device = instance.get_device().get_vk_device();
        let queue_family_index = instance.get_physical_device().queue_family_index();
        let command_buffers = CommandBuffers::new(vk_device, queue_family_index, logger.clone());
        Self {
            vk,
            surface,
            swapchain,
            command_buffers,
            logger,
        }
    }
}

impl Drop for ColMeshRenderer {
    fn drop(&mut self) {
        debug!(self.logger, "ColMeshRenderer drop called");
        self.swapchain.destroy();
        self.surface.destroy();
        let vk_device = self.vk.get_instance().get_device().get_vk_device();
        self.command_buffers.destroy(vk_device);
    }
}
