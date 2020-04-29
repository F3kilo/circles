pub mod base;
pub mod present;
pub mod render;
use crate::vulkan::present::WindowData;
use base::VulkanBase;
use present::VulkanPresent;
use render::VulkanRenderer;
use slog::Logger;

pub struct Vulkan {
    base: VulkanBase,
    present: VulkanPresent,
    render: VulkanRenderer,
    logger: Logger,
}

impl Vulkan {
    pub fn new(app_name: &str, window_data: WindowData, logger: Logger) -> Self {
        let base = VulkanBase::new(app_name, logger.clone());
        let present = VulkanPresent::new(&base, window_data, logger.clone());
        let render = VulkanRenderer::new(&base, &present, logger.clone());
        Self {
            base,
            present,
            render,
            logger,
        }
    }
}

impl Drop for Vulkan {
    fn drop(&mut self) {
        debug!(self.logger, "Vulkan drop() called");
        self.render.destroy(&self.base);
        self.present.destroy(&self.base);
        self.base.destroy();
    }
}
