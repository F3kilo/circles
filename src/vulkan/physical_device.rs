use super::surface::Surface;
use ash::version::InstanceV1_0;
use ash::{vk, Instance};

pub struct PhysicalDevice {
    pdevice: vk::PhysicalDevice,
    queue_family_index: u32,
}

impl PhysicalDevice {
    pub fn select(instance: &Instance) -> Self {
        let pdevices = unsafe { instance.enumerate_physical_devices() }
            .expect("Can't get physical devices list");

        if let Some((pdevice, queue_family_index)) =
            Self::try_get_descrete_device(instance, pdevices.iter())
        {
            return Self {
                pdevice,
                queue_family_index,
            };
        }

        if let Some((pdevice, queue_family_index)) =
            Self::try_get_some_device(instance, pdevices.iter())
        {
            return Self {
                pdevice,
                queue_family_index,
            };
        }

        panic!("Can't select suit physical device");
    }

    fn try_get_descrete_device<'a>(
        instance: &Instance,
        pdevices: impl Iterator<Item = &'a vk::PhysicalDevice>,
    ) -> Option<(vk::PhysicalDevice, u32)> {
        pdevices
            .filter_map(|pdevice| {
                let device_properties =
                    unsafe { instance.get_physical_device_properties(*pdevice) };
                let device_type = device_properties.device_type;
                if device_type != vk::PhysicalDeviceType::DISCRETE_GPU {
                    return None;
                }

                unsafe { instance.get_physical_device_queue_family_properties(*pdevice) }
                    .iter()
                    .enumerate()
                    .filter_map(|(index, ref info)| {
                        let supports_graphic_and_surface =
                            info.queue_flags.contains(vk::QueueFlags::GRAPHICS);
                        if supports_graphic_and_surface {
                            Some((*pdevice, index as u32))
                        } else {
                            None
                        }
                    })
                    .next()
            })
            .next()
    }

    fn try_get_some_device<'a>(
        instance: &Instance,
        pdevices: impl Iterator<Item = &'a vk::PhysicalDevice>,
    ) -> Option<(vk::PhysicalDevice, u32)> {
        pdevices
            .filter_map(|pdevice| {
                unsafe { instance.get_physical_device_queue_family_properties(*pdevice) }
                    .iter()
                    .enumerate()
                    .filter_map(|(index, ref info)| {
                        let supports_graphic_and_surface =
                            info.queue_flags.contains(vk::QueueFlags::GRAPHICS);
                        if supports_graphic_and_surface {
                            Some((*pdevice, index as u32))
                        } else {
                            None
                        }
                    })
                    .next()
            })
            .next()
    }

    pub fn queue_family_index(&self) -> u32 {
        self.queue_family_index
    }

    pub fn vk_physical_device(&self) -> vk::PhysicalDevice {
        self.pdevice
    }
}
