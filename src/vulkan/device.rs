use super::command_buffers::CommandBuffers;
use super::physical_device::PhysicalDevice;
use ash::version::{DeviceV1_0, InstanceV1_0};
use ash::{vk, Instance};
use slog::Logger;

pub struct Device {
    device: ash::Device,
    queue: vk::Queue,
    logger: Logger,
}

impl Device {
    pub fn new(instance: &Instance, pdevice: &PhysicalDevice, logger: Logger) -> Self {
        let ext_names = [ash::extensions::khr::Swapchain::name().as_ptr()];
        let priorities = [1f32];
        let queue_info = [vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(pdevice.queue_family_index())
            .queue_priorities(&priorities)
            .build()];

        let create_info = vk::DeviceCreateInfo::builder()
            .queue_create_infos(&queue_info)
            .enabled_extension_names(&ext_names);

        let device =
            unsafe { instance.create_device(pdevice.vk_physical_device(), &create_info, None) }
                .expect("Can't create device");
        let queue = unsafe { device.get_device_queue(pdevice.queue_family_index(), 0) };

        Self {
            device,
            queue,
            logger,
        }
    }

    pub fn get_vk_device(&self) -> &ash::Device {
        &self.device
    }

    pub fn destroy(&mut self) {
        debug!(self.logger, "Device destroy() called");
        unsafe {
            self.device.destroy_device(None);
        }
    }
}
