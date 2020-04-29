pub mod base;
pub mod present;
use crate::vulkan::present::WindowData;
use base::VulkanBase;
use present::VulkanPresent;
use slog::Logger;

pub struct Vulkan {
    base: VulkanBase,
    present: VulkanPresent,
    logger: Logger,
}

impl Vulkan {
    pub fn new(app_name: &str, window_data: WindowData, logger: Logger) -> Self {
        let base = VulkanBase::new(app_name, logger.clone());
        let present = VulkanPresent::new(&base, window_data, logger.clone());
        Self {
            base,
            present,
            logger,
        }
    }
}

impl Drop for Vulkan {
    fn drop(&mut self) {
        debug!(self.logger, "Vulkan drop() called");
        self.present.destroy();
        self.base.destroy();
    }
}
