use crate::vulkan::base::VulkanBase;
use crate::vulkan::present::VulkanPresent;
use crate::vulkan::render::depth_image::DepthImage;
use slog::Logger;

pub mod depth_image;

pub struct VulkanRenderer {
    depth_image: DepthImage,
    logger: Logger,
}

impl VulkanRenderer {
    pub fn new(base: &VulkanBase, presenter: &VulkanPresent, logger: Logger) -> Self {
        let depth_image = DepthImage::new(base, presenter, logger.clone());

        Self {
            depth_image,
            logger,
        }
    }

    pub fn destroy(&mut self, base: &VulkanBase) {
        debug!(self.logger, "Renderer destroy() called");
        self.depth_image.destroy(base.get_device().get_vk_device());
    }
}
