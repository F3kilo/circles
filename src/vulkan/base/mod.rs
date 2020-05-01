pub mod command_buffers;
pub mod debug_callback;
pub mod device;
pub mod instance;
pub mod physical_device;
use ash::Entry;
use command_buffers::CommandBuffers;
use device::Device;
use instance::Instance;
use physical_device::PhysicalDevice;
use slog::Logger;

pub struct VulkanBase {
    command_buffers: CommandBuffers,
    device: Device,
    pdevice: PhysicalDevice,
    instance: Instance,
    entry: Entry,
    logger: Logger,
}

impl VulkanBase {
    pub fn new(app_name: &str, logger: Logger) -> Self {
        let entry = Entry::new().expect("Can't create vulkan entry!");
        let instance = instance::Instance::new(&entry, app_name, logger.clone());
        let pdevice = PhysicalDevice::select(&instance);
        let device = Device::new(&instance, &pdevice, logger.clone());
        let command_buffers =
            CommandBuffers::new(&device, pdevice.get_queue_family_index(), logger.clone());
        Self {
            command_buffers,
            device,
            pdevice,
            instance,
            entry,
            logger,
        }
    }

    pub fn get_command_buffers(&self) -> &CommandBuffers {
        &self.command_buffers
    }

    pub fn get_device(&self) -> &Device {
        &self.device
    }

    pub fn get_physical_device(&self) -> &PhysicalDevice {
        &self.pdevice
    }

    pub fn get_instance(&self) -> &Instance {
        &self.instance
    }

    pub fn get_entry(&self) -> &Entry {
        &self.entry
    }

    pub fn destroy(&mut self) {
        debug!(self.logger, "VulkanBase destroy called");
        self.command_buffers.destroy(&self.device);
        self.device.destroy();
        self.instance.destroy();
    }
}
