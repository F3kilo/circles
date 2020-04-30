pub mod depth_image;
pub mod framebuffers;
pub mod render_pass;
pub mod semaphores;

use super::base::VulkanBase;
use super::present::VulkanPresent;
use crate::vulkan::render::framebuffers::Framebuffers;
use crate::vulkan::render::render_pass::RenderPass;
use depth_image::DepthImage;
use semaphores::Semaphores;
use slog::Logger;

pub struct VulkanRenderer {
    framebuffers: Framebuffers,
    render_pass: RenderPass,
    semaphores: Semaphores,
    depth_image: DepthImage,
    logger: Logger,
}

impl VulkanRenderer {
    pub fn new(base: &VulkanBase, presenter: &VulkanPresent, logger: Logger) -> Self {
        let depth_image = DepthImage::new(base, presenter, logger.clone());
        let semaphores = Semaphores::new(base, logger.clone());
        let render_pass = RenderPass::new(base, presenter, logger.clone());
        let framebuffers =
            Framebuffers::new(base, presenter, &depth_image, &render_pass, logger.clone());
        Self {
            depth_image,
            logger,
            semaphores,
            render_pass,
            framebuffers,
        }
    }

    pub fn destroy(&mut self, base: &VulkanBase) {
        debug!(self.logger, "Renderer destroy() called");
        let vk_device = base.get_device().get_vk_device();
        self.framebuffers.destroy(vk_device);
        self.render_pass.destroy(vk_device);
        self.semaphores.destroy(vk_device);
        self.depth_image.destroy(vk_device);
    }
}
