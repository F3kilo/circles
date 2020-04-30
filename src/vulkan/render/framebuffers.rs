use crate::vulkan::base::VulkanBase;
use crate::vulkan::present::VulkanPresent;
use crate::vulkan::render::depth_image::DepthImage;
use crate::vulkan::render::render_pass::RenderPass;
use ash::version::DeviceV1_0;
use ash::vk;
use slog::Logger;

pub struct Framebuffers {
    framebuffers: Vec<vk::Framebuffer>,
    logger: Logger,
}

impl Framebuffers {
    pub fn new(
        base: &VulkanBase,
        present: &VulkanPresent,
        depth_image: &DepthImage,
        render_pass: &RenderPass,
        logger: Logger,
    ) -> Self {
        let framebuffers = present
            .get_swapchain()
            .get_image_views()
            .iter()
            .map(|view| {
                let attachments = [*view, depth_image.get_view()];
                let create_info = vk::FramebufferCreateInfo::builder()
                    .render_pass(render_pass.get_vk_render_pass())
                    .attachments(&attachments)
                    .width(present.get_swapchain().get_resolution().width)
                    .height(present.get_swapchain().get_resolution().height)
                    .layers(1);
                unsafe {
                    base.get_device()
                        .get_vk_device()
                        .create_framebuffer(&create_info, None)
                }
                .expect("Can't create framebuffer.")
            })
            .collect();

        Self {
            framebuffers,
            logger,
        }
    }

    pub fn get_framebuffer(&self, index: usize) -> vk::Framebuffer {
        self.framebuffers[index]
    }

    pub fn destroy(&mut self, device: &ash::Device) {
        debug!(self.logger, "Framebuffers destroy() called");
        for framebuffer in &self.framebuffers {
            unsafe {
                device.destroy_framebuffer(*framebuffer, None);
            }
        }
    }
}
