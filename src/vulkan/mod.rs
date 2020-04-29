pub mod base;
pub mod present;
use base::VulkanBase;
use slog::Logger;

pub struct Vulkan {
    base: VulkanBase,
    logger: Logger,
}

impl Vulkan {
    pub fn new(app_name: &str, logger: Logger) -> Self {
        let base = VulkanBase::new(app_name, logger.clone());
        Self { base, logger }
    }
}

impl Drop for Vulkan {
    fn drop(&mut self) {
        debug!(self.logger, "Vulkan drop() called");
        self.base.destroy();
    }
}
