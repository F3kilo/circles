use crate::vulkan::base::VulkanBase;
use crate::vulkan::present::VulkanPresent;
use ash::vk;

pub struct RenderPass {
    render_pass: vk::RenderPass,
}

impl RenderPass {
    pub fn new(base: &VulkanBase, presenter: &VulkanPresent) -> Self {
        let color_attachment_descr = Self::create_color_attachment_descr(
            presenter.get_swapchain().get_surface_format().format,
        );
    }

    fn create_color_attachment_descr(format: vk::Format) -> vk::AttachmentDescription {
        vk::AttachmentDescription {
            format,
            samples: vk::SampleCountFlags::TYPE_1,
            load_op: vk::AttachmentLoadOp::CLEAR,
            store_op: vk::AttachmentStoreOp::STORE,
            final_layout: vk::ImageLayout::PRESENT_SRC_KHR,
            ..Default::default()
        }
    }
}
