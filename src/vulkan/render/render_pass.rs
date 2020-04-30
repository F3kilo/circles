use crate::vulkan::base::VulkanBase;
use crate::vulkan::present::VulkanPresent;
use crate::vulkan::render::depth_image::DepthImage;
use ash::version::DeviceV1_0;
use ash::vk;
use slog::Logger;

pub struct RenderPass {
    render_pass: vk::RenderPass,
    logger: Logger,
}

impl RenderPass {
    pub fn new(base: &VulkanBase, presenter: &VulkanPresent, logger: Logger) -> Self {
        let color_format = presenter.get_swapchain().get_surface_format().format;
        let attachments = Self::create_attachment_descrs(color_format);

        let color_attachment_refs = [vk::AttachmentReference {
            attachment: 0,
            layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
        }];
        let depth_attachment_ref = vk::AttachmentReference {
            attachment: 1,
            layout: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
        };

        let dependencies = [vk::SubpassDependency {
            src_subpass: vk::SUBPASS_EXTERNAL,
            src_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            dst_access_mask: vk::AccessFlags::COLOR_ATTACHMENT_READ
                | vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
            dst_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            ..Default::default()
        }];

        let subpasses = [vk::SubpassDescription::builder()
            .color_attachments(&color_attachment_refs)
            .depth_stencil_attachment(&depth_attachment_ref)
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .build()];

        let renderpass_create_info = vk::RenderPassCreateInfo::builder()
            .attachments(&attachments)
            .subpasses(&subpasses)
            .dependencies(&dependencies);

        let vk_device = base.get_device().get_vk_device();

        let render_pass = unsafe { vk_device.create_render_pass(&renderpass_create_info, None) }
            .expect("Can't create render pass");

        Self { render_pass, logger }
    }

    fn create_attachment_descrs(color_format: vk::Format) -> [vk::AttachmentDescription; 2] {
        let color_attachment_descr = Self::create_color_attachment_descr(color_format);
        let depth_attachment_descr = Self::create_depth_attachment_descr();
        [color_attachment_descr, depth_attachment_descr]
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

    fn create_depth_attachment_descr() -> vk::AttachmentDescription {
        vk::AttachmentDescription {
            format: DepthImage::get_format(),
            samples: vk::SampleCountFlags::TYPE_1,
            load_op: vk::AttachmentLoadOp::CLEAR,
            final_layout: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
            ..Default::default()
        }
    }
    
    pub fn destroy(&mut self, device: &ash::Device) {
        debug!(self.logger, "Render pass destroy() called");
        unsafe { device.destroy_render_pass(self.render_pass, None); }
    }
}
