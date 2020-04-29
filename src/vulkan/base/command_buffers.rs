use super::Device;
use ash::version::DeviceV1_0;
use ash::vk;
use slog::Logger;

pub struct CommandBuffers {
    pool: vk::CommandPool,
    render: vk::CommandBuffer,
    present: vk::CommandBuffer,
    logger: Logger,
}

impl CommandBuffers {
    pub fn new(device: &Device, queue_family_index: u32, logger: Logger) -> Self {
        let create_info = vk::CommandPoolCreateInfo::builder()
            .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
            .queue_family_index(queue_family_index);
        let vk_device = device.get_vk_device();
        let pool = unsafe {
            vk_device
                .create_command_pool(&create_info, None)
                .expect("Can't create command pool")
        };

        let buffers_alloc_info = vk::CommandBufferAllocateInfo::builder()
            .command_buffer_count(2)
            .command_pool(pool);

        let buffers = unsafe { vk_device.allocate_command_buffers(&buffers_alloc_info) }
            .expect("Can't allocate command buffers");
        let render = buffers[0];
        let present = buffers[1];

        Self {
            pool,
            render,
            present,
            logger,
        }
    }

    pub fn destroy(&mut self, device: &Device) {
        debug!(self.logger, "Command buffers destroy() called");
        let vk_device = device.get_vk_device();
        unsafe {
            vk_device.free_command_buffers(self.pool, &[self.render, self.present]);
            vk_device.destroy_command_pool(self.pool, None);
        }
    }
}
