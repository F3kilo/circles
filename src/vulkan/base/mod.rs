pub mod command_buffers;
pub mod debug_callback;
pub mod device;
pub mod instance;
pub mod physical_device;
use ash::Entry;
use instance::Instance;
use slog::Logger;

/// Order of fields defines order of drop!
pub struct VulkanBase {
    instance: Instance,
    entry: Entry,
    logger: Logger,
}

impl VulkanBase {
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

    pub fn destroy(&mut self) {
        debug!(self.logger, "VulkanBase destroy called");
        self.instance.destroy();
    }
}
