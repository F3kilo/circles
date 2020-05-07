use crate::vulkan::base::physical_device::PhysicalDevice;
use crate::vulkan::base::VulkanBase;
use crate::vulkan::present::VulkanPresent;
use ash::version::DeviceV1_0;
use ash::vk;
use slog::Logger;

pub struct DepthImage {
    image: vk::Image,
    view: vk::ImageView,
    memory: vk::DeviceMemory,
    logger: Logger,
}

impl DepthImage {
    pub fn new(base: &VulkanBase, present: &VulkanPresent, logger: Logger) -> Self {
        let pdevice = base.get_physical_device();
        let vk_device = base.get_device().get_vk_device();
        let resolution = present.get_swapchain().get_resolution();
        let image = DepthImage::create_image(vk_device, resolution);

        let memory_requirements = unsafe { vk_device.get_image_memory_requirements(image) };
        let memory_type_index = DepthImage::get_memory_type_index(pdevice, &memory_requirements);
        let allocate_info = vk::MemoryAllocateInfo::builder()
            .allocation_size(memory_requirements.size)
            .memory_type_index(memory_type_index)
            .build();
        let memory = unsafe { vk_device.allocate_memory(&allocate_info, None) }
            .expect("Can't allocate memory for depth image");
        unsafe {
            vk_device
                .bind_image_memory(image, memory, 0)
                .expect("Can't bind depth image memory");
        };

        DepthImage::transit_image_layout(base, image);

        let depth_image_view_info = vk::ImageViewCreateInfo::builder()
            .subresource_range(
                vk::ImageSubresourceRange::builder()
                    .aspect_mask(vk::ImageAspectFlags::DEPTH)
                    .level_count(1)
                    .layer_count(1)
                    .build(),
            )
            .image(image)
            .format(Self::get_format())
            .view_type(vk::ImageViewType::TYPE_2D);

        let view = unsafe { vk_device.create_image_view(&depth_image_view_info, None) }
            .expect("Can't create depth image view.");

        Self {
            image,
            view,
            memory,
            logger: logger.clone(),
        }
    }

    pub fn get_view(&self) -> vk::ImageView {
        self.view
    }

    pub fn get_format() -> vk::Format {
        vk::Format::D16_UNORM
    }

    fn transit_image_layout(base: &VulkanBase, depth_image: vk::Image) {
        let vk_device = base.get_device().get_vk_device();

        let service_command_buffer = base.get_command_buffers().get_service();

        let layout_transition_barriers = vk::ImageMemoryBarrier::builder()
            .image(depth_image)
            .dst_access_mask(
                vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_READ
                    | vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE,
            )
            .new_layout(vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL)
            .old_layout(vk::ImageLayout::UNDEFINED)
            .subresource_range(
                vk::ImageSubresourceRange::builder()
                    .aspect_mask(vk::ImageAspectFlags::DEPTH)
                    .layer_count(1)
                    .level_count(1)
                    .build(),
            );

        let begin_info = vk::CommandBufferBeginInfo::builder()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);

        unsafe { vk_device.begin_command_buffer(*service_command_buffer, &begin_info) }
            .expect("Can't begin service command buffer to change depth image layout.");

        unsafe {
            vk_device.cmd_pipeline_barrier(
                *service_command_buffer,
                vk::PipelineStageFlags::BOTTOM_OF_PIPE,
                vk::PipelineStageFlags::LATE_FRAGMENT_TESTS,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                &[layout_transition_barriers.build()],
            );
        };

        unsafe { vk_device.end_command_buffer(*service_command_buffer) }
            .expect("end_command_buffer() failed for depth image layout transition.");

        let submit_info = vk::SubmitInfo::builder()
            .command_buffers(&[*service_command_buffer])
            .build();

        let submit_fence = unsafe { vk_device.create_fence(&vk::FenceCreateInfo::default(), None) }
            .expect("Create fence failed.");

        let queue = base.get_device().get_vk_queue();
        unsafe { vk_device.queue_submit(queue, &[submit_info], submit_fence) }
            .expect("Can't submit depth imagege layout change command buffer.");

        unsafe { vk_device.wait_for_fences(&[submit_fence], true, std::u64::MAX) }
            .expect("Wait for fence failed.");

        unsafe {
            vk_device.destroy_fence(submit_fence, None);
        }
    }

    fn get_memory_type_index(
        pdevice: &PhysicalDevice,
        depth_image_memory_req: &vk::MemoryRequirements,
    ) -> u32 {
        pdevice
            .find_memorytype_index(
                &depth_image_memory_req,
                vk::MemoryPropertyFlags::DEVICE_LOCAL,
            )
            .expect("Unable to find suitable memory index for depth image.")
    }

    fn create_image(device: &ash::Device, resolution: vk::Extent2D) -> vk::Image {
        let depth_image_create_info = vk::ImageCreateInfo::builder()
            .image_type(vk::ImageType::TYPE_2D)
            .format(Self::get_format())
            .extent(vk::Extent3D {
                width: resolution.width,
                height: resolution.height,
                depth: 1,
            })
            .mip_levels(1)
            .array_layers(1)
            .samples(vk::SampleCountFlags::TYPE_1)
            .tiling(vk::ImageTiling::OPTIMAL)
            .usage(vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);

        unsafe { device.create_image(&depth_image_create_info, None) }.unwrap()
    }

    pub fn destroy(&mut self, device: &ash::Device) {
        debug!(self.logger, "Depth image destroy() called");
        unsafe {
            device.destroy_image_view(self.view, None);
            debug!(self.logger, "\tDepth vk::ImageView destroyed.");
            device.free_memory(self.memory, None);
            debug!(self.logger, "\tDepth vk::DeviceMemory freed.");
            device.destroy_image(self.image, None);
            debug!(self.logger, "\tDepth vk::Image destroyed.");
        }
    }
}
