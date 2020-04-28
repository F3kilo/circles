pub mod command_buffers;
pub mod debug_callback;
pub mod device;
pub mod instance;
pub mod physical_device;
pub mod surface;
pub mod swapchain;
use crate::vulkan::instance::Instance;
use ash::Entry;
use slog::Logger;

/// Order of fields defines order of drop!
pub struct Vulkan {
    instance: Instance,
    entry: Entry,
    logger: Logger,
}

impl Vulkan {
    pub fn new(app_name: &str, logger: Logger) -> Self {
        let entry = Entry::new().expect("Can't init vk entry!");
        let instance = instance::Instance::new(&entry, app_name, logger.clone());
        Self {
            entry,
            instance,
            logger,
        }
    }

    pub fn get_instance(&self) -> &Instance {
        &self.instance
    }

    pub fn get_entry(&self) -> &Entry {
        &self.entry
    }
}

impl Drop for Vulkan {
    fn drop(&mut self) {
        debug!(self.logger, "Vulkan drop() called");
        self.instance.destroy();
    }
}
