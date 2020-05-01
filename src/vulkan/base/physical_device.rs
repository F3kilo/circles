use super::Instance;
use ash::version::InstanceV1_0;
use ash::vk;

pub struct PhysicalDevice {
    pdevice: vk::PhysicalDevice,
    queue_family_index: u32,
    memory_properties: vk::PhysicalDeviceMemoryProperties,
}

impl PhysicalDevice {
    pub fn select(instance: &Instance) -> Self {
        let vk_instance = instance.get_vk_instance();
        let pdevices = unsafe { vk_instance.enumerate_physical_devices() }
            .expect("Can't get physical devices list");

        let (pdevice, queue_family_index) =
            Self::try_select_descrete_device(vk_instance, pdevices.iter()).unwrap_or_else(|| {
                Self::try_select_some_device(vk_instance, pdevices.iter())
                    .expect("Can't select suit physical device")
            });

        let memory_properties =
            unsafe { vk_instance.get_physical_device_memory_properties(pdevice) };

        Self {
            pdevice,
            queue_family_index,
            memory_properties,
        }
    }

    fn try_select_descrete_device<'a>(
        instance: &ash::Instance,
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
        instance: &ash::Instance,
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

    fn select_queue_family_index(
        instance: &ash::Instance,
        pdevice: vk::PhysicalDevice,
    ) -> Option<u32> {
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

    pub fn get_queue_family_index(&self) -> u32 {
        self.queue_family_index
    }

    pub fn get_vk_physical_device(&self) -> vk::PhysicalDevice {
        self.pdevice
    }

    pub fn find_memorytype_index(
        &self,
        memory_req: &vk::MemoryRequirements,
        flags: vk::MemoryPropertyFlags,
    ) -> Option<u32> {
        // Try to find an exactly matching memory flag
        let best_suitable_index =
            self.find_memorytype_index_f(memory_req, flags, |property_flags, flags| {
                property_flags == flags
            });
        if best_suitable_index.is_some() {
            return best_suitable_index;
        }
        // Otherwise find a memory flag that works
        self.find_memorytype_index_f(memory_req, flags, |property_flags, flags| {
            property_flags & flags == flags
        })
    }

    pub fn find_memorytype_index_f<
        F: Fn(vk::MemoryPropertyFlags, vk::MemoryPropertyFlags) -> bool,
    >(
        &self,
        memory_req: &vk::MemoryRequirements,
        flags: vk::MemoryPropertyFlags,
        f: F,
    ) -> Option<u32> {
        let mut memory_type_bits = memory_req.memory_type_bits;
        for (index, ref memory_type) in self.memory_properties.memory_types.iter().enumerate() {
            if memory_type_bits & 1 == 1 && f(memory_type.property_flags, flags) {
                return Some(index as u32);
            }
            memory_type_bits >>= 1;
        }
        None
    }
}
