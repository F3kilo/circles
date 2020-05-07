pub mod depth_image;
pub mod framebuffers;
pub mod geometry_buffers;
pub mod pipeline;
pub mod render_pass;
pub mod semaphores;
pub mod vertex;

use super::base::VulkanBase;
use super::present::VulkanPresent;
use crate::vulkan::render::framebuffers::Framebuffers;
use crate::vulkan::render::geometry_buffers::GeometryBuffers;
use crate::vulkan::render::pipeline::Pipeline;
use crate::vulkan::render::render_pass::RenderPass;
use ash::version::DeviceV1_0;
use ash::vk;
use depth_image::DepthImage;
use semaphores::Semaphores;
use slog::Logger;

pub struct VulkanRenderer {
    pipeline: Pipeline,
    geometry_buffers: GeometryBuffers,
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
        let mut geometry_buffers = GeometryBuffers::new(base, logger.clone());
        geometry_buffers.write_triangle(base.get_device().get_vk_device());
        let pipeline = Pipeline::new(base, presenter, logger.clone(), &render_pass);
        Self {
            pipeline,
            geometry_buffers,
            depth_image,
            logger,
            semaphores,
            render_pass,
            framebuffers,
        }
    }

    fn record_render_command_buffer(&self, base: &VulkanBase, present: &VulkanPresent) -> u32 {
        let vk_device = base.get_device().get_vk_device();
        let command_buffer = base.get_command_buffers().get_render();

        let clear_values = [
            vk::ClearValue {
                color: vk::ClearColorValue {
                    float32: [0.0, 0.0, 0.0, 0.0],
                },
            },
            vk::ClearValue {
                depth_stencil: vk::ClearDepthStencilValue {
                    depth: 1.0,
                    stencil: 0,
                },
            },
        ];

        let present_index = present
            .get_swapchain()
            .acquire_next_image(self.semaphores.get_presented());

        let render_pass_begin_info = vk::RenderPassBeginInfo::builder()
            .render_pass(self.render_pass.get_vk_render_pass())
            .framebuffer(self.framebuffers.get_framebuffer(present_index as usize))
            .render_area(vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: present.get_swapchain().get_resolution(),
            })
            .clear_values(&clear_values);

        let begin_info = vk::CommandBufferBeginInfo::builder()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);

        unsafe {
            vk_device
                .begin_command_buffer(command_buffer, &begin_info)
                .expect("Can't begin render command buffer");

            vk_device.cmd_begin_render_pass(
                command_buffer,
                &render_pass_begin_info,
                vk::SubpassContents::INLINE,
            );
            vk_device.cmd_bind_pipeline(
                command_buffer,
                vk::PipelineBindPoint::GRAPHICS,
                self.pipeline.get_vk_pipeline(),
            );

            let surface_resolution = present.get_swapchain().get_resolution();
            let viewports = Pipeline::get_viewports(surface_resolution);
            let scissors = Pipeline::get_scissors(surface_resolution);

            vk_device.cmd_set_viewport(command_buffer, 0, &viewports);
            vk_device.cmd_set_scissor(command_buffer, 0, &scissors);
            vk_device.cmd_bind_vertex_buffers(
                command_buffer,
                0,
                &[self.geometry_buffers.get_vertex_buffer()],
                &[0],
            );
            vk_device.cmd_bind_index_buffer(
                command_buffer,
                self.geometry_buffers.get_index_buffer(),
                0,
                vk::IndexType::UINT32,
            );

            vk_device.cmd_draw_indexed(command_buffer, 3, 1, 0, 0, 1);
            vk_device.cmd_end_render_pass(command_buffer);

            vk_device
                .end_command_buffer(command_buffer)
                .expect("Can't end render command buffer");
        }

        present_index
    }

    pub fn render(&self, base: &VulkanBase, present: &VulkanPresent) {
        let present_index = self.record_render_command_buffer(base, present);

        let vk_device = base.get_device().get_vk_device();
        let command_buffer = base.get_command_buffers().get_render();

        let command_buffers = vec![command_buffer];

        let wait_semaphores = [self.semaphores.get_presented()];
        let signal_semaphores = [self.semaphores.get_rendered()];
        let submit_info = vk::SubmitInfo::builder()
            .wait_semaphores(&wait_semaphores)
            .wait_dst_stage_mask(&[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT])
            .command_buffers(&command_buffers)
            .signal_semaphores(&signal_semaphores);

        let queue = base.get_device().get_vk_queue();

        unsafe { vk_device.queue_submit(queue, &[submit_info.build()], vk::Fence::null()) }
            .expect("queue submit failed.");

        let wait_semaphors = [self.semaphores.get_rendered()];
        let swapchain = present.get_swapchain();
        let swapchains = [swapchain.get_vk_swapchain()];
        let image_indices = [present_index];
        let present_info = vk::PresentInfoKHR::builder()
            .wait_semaphores(&wait_semaphors) // &base.rendering_complete_semaphore)
            .swapchains(&swapchains)
            .image_indices(&image_indices);

        swapchain.present(&present_info, queue);
        unsafe { vk_device.queue_wait_idle(queue) }.expect("Can't wait queue idle after render.");
    }

    pub fn destroy(&mut self, base: &VulkanBase) {
        debug!(self.logger, "Renderer destroy() called");
        let vk_device = base.get_device().get_vk_device();
        self.pipeline.destroy(vk_device);
        self.geometry_buffers.destroy(vk_device);
        self.framebuffers.destroy(vk_device);
        self.render_pass.destroy(vk_device);
        self.semaphores.destroy(vk_device);
        self.depth_image.destroy(vk_device);
    }
}
