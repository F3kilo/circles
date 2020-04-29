pub mod depth_image;
pub mod semaphores;
pub mod render_pass;

use super::base::VulkanBase;
use super::present::VulkanPresent;
use depth_image::DepthImage;
use semaphores::Semaphores;
use slog::Logger;

pub struct VulkanRenderer {
    depth_image: DepthImage,
    semaphores: Semaphores,
    logger: Logger,
}

impl VulkanRenderer {
    pub fn new(base: &VulkanBase, presenter: &VulkanPresent, logger: Logger) -> Self {
        let depth_image = DepthImage::new(base, presenter, logger.clone());
        let semaphores = Semaphores::new(base, logger.clone());
        Self {
            depth_image,
            logger,
            semaphores,
        }
    }

    pub fn destroy(&mut self, base: &VulkanBase) {
        debug!(self.logger, "Renderer destroy() called");
        let vk_device = base.get_device().get_vk_device();
        self.semaphores.destroy(vk_device);
        self.depth_image.destroy(vk_device);
    }
}
