use ash::version::DeviceV1_0;
use ash::vk;

pub struct CommandBuffers {
    pool: vk::CommandPool,
    render: vk::CommandBuffer,
    present: vk::CommandBuffer,
}

impl CommandBuffers {
    pub fn new(device: &ash::Device, queue_family_index: u32) -> Self {
        let create_info = vk::CommandPoolCreateInfo::builder()
            .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
            .queue_family_index(queue_family_index);
        let pool = unsafe {
            device
                .create_command_pool(&create_info, None)
                .expect("Can't create command pool")
        };

        let buffers_alloc_info = vk::CommandBufferAllocateInfo::builder()
            .command_buffer_count(2)
            .command_pool(pool);

        let buffers = unsafe { device.allocate_command_buffers(&buffers_alloc_info) }
            .expect("Can't allocate command buffers");
        let render = buffers[0];
        let present = buffers[1];

        Self {
            pool,
            render,
            present,
        }
    }
}
