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
            Self::try_select_descrete_device(instance, pdevices.iter())
        {
            return Self {
                pdevice,
                queue_family_index,
            };
        }

        if let Some((pdevice, queue_family_index)) =
            Self::try_select_some_device(instance, pdevices.iter())
        {
            return Self {
                pdevice,
                queue_family_index,
            };
        }

        panic!("Can't select suit physical device");
    }

    fn try_select_descrete_device<'a>(
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

                let mb_queue_family_index = Self::select_queue_family_index(instance, *pdevice);
                if let Some(queue_family_index) = mb_queue_family_index {
                    return Some((*pdevice, queue_family_index));
                }
                None
            })
            .next()
    }

    fn try_select_some_device<'a>(
        instance: &Instance,
        pdevices: impl Iterator<Item = &'a vk::PhysicalDevice>,
    ) -> Option<(vk::PhysicalDevice, u32)> {
        pdevices
            .filter_map(|pdevice| {
                let mb_queue_family_index = Self::select_queue_family_index(instance, *pdevice);
                if let Some(queue_family_index) = mb_queue_family_index {
                    return Some((*pdevice, queue_family_index));
                }
                None
            })
            .next()
    }

    fn select_queue_family_index(instance: &Instance, pdevice: vk::PhysicalDevice) -> Option<u32> {
        unsafe { instance.get_physical_device_queue_family_properties(pdevice) }
            .iter()
            .enumerate()
            .filter_map(|(index, ref info)| {
                let supports_graphic = info.queue_flags.contains(vk::QueueFlags::GRAPHICS);
                if supports_graphic {
                    Some(index as u32)
                } else {
                    None
                }
            })
            .next()
    }

    pub fn queue_family_index(&self) -> u32 {
        self.queue_family_index
    }

    pub fn get_vk_physical_device(&self) -> vk::PhysicalDevice {
        self.pdevice
    }
}
