use crate::vulkan::base::VulkanBase;
use ash::version::DeviceV1_0;
use ash::vk;
use slog::Logger;

pub struct Semaphores {
    rendered: vk::Semaphore,
    presented: vk::Semaphore,
    logger: Logger,
}

impl Semaphores {
    pub fn new(base: &VulkanBase, logger: Logger) -> Self {
        let vk_device = base.get_device().get_vk_device();
        let create_info = vk::SemaphoreCreateInfo::default();
        let rendered = unsafe { vk_device.create_semaphore(&create_info, None) }
            .expect("Can't create render finished semaphore.");
        let presented = unsafe { vk_device.create_semaphore(&create_info, None) }
            .expect("Can't create render finished semaphore.");

        Self {
            rendered,
            presented,
            logger,
        }
    }

    pub fn get_rendered(&self) -> vk::Semaphore {
        self.rendered
    }

    pub fn get_presented(&self) -> vk::Semaphore {
        self.presented
    }

    pub fn destroy(&mut self, vk_device: &ash::Device) {
        unsafe {
            debug!(self.logger, "Semaphores destroy() called.");
            vk_device.destroy_semaphore(self.rendered, None);
            vk_device.destroy_semaphore(self.presented, None);
        }
    }
}
