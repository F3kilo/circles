use super::surface::Surface;
use ash::version::InstanceV1_0;
use ash::{vk, Instance};

pub struct PhysicalDevice {
    pdevice: vk::PhysicalDevice,
    queue_famaly_index: u32,
}

impl PhysicalDevice {
    pub fn select(instance: &Instance, surface: &Surface) -> Self {
        let pdevices = unsafe { instance.enumerate_physical_devices() }
            .expect("Can't get physical devices list");
        let (pdevice, queue_famaly_index) = pdevices
            .iter()
            .map(|pdevice| {
                unsafe { instance.get_physical_device_queue_family_properties(*pdevice) }
                    .iter()
                    .enumerate()
                    .filter_map(|(index, ref info)| {
                        let supports_graphic_and_surface =
                            info.queue_flags.contains(vk::QueueFlags::GRAPHICS)
                                && surface.is_device_supported(*pdevice, index as u32);
                        if supports_graphic_and_surface {
                            Some((*pdevice, index as u32))
                        } else {
                            None
                        }
                    })
                    .next()
            })
            .filter_map(|v| v)
            .next()
            .expect("Couldn't find suitable device.");
        Self {
            pdevice,
            queue_famaly_index,
        }
    }

    pub fn queue_family_index(&self) -> u32 {
        self.queue_famaly_index
    }

    pub fn vk_physical_device(&self) -> vk::PhysicalDevice {
        self.pdevice
    }
}
