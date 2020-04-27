use super::command_buffers::CommandBuffers;
use super::physical_device::PhysicalDevice;
use super::swapchain::Swapchain;
use crate::vulkan::surface::Surface;
use ash::version::{DeviceV1_0, InstanceV1_0};
use ash::{vk, Instance};

pub struct Device {
    device: ash::Device,
    queue: vk::Queue,
    command_buffers: CommandBuffers,
    swapchain: Swapchain,
}

impl Device {
    pub fn new(
        instance: &Instance,
        pdevice: &PhysicalDevice,
        surface: &Surface,
        window: &winit::window::Window,
    ) -> Self {
        let ext_names = [ash::extensions::khr::Swapchain::name().as_ptr()];
        let priorities = [1f32];
        let queue_info = [vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(pdevice.queue_family_index())
            .queue_priorities(&priorities)
            .build()];

        let create_info = vk::DeviceCreateInfo::builder()
            .queue_create_infos(&queue_info)
            .enabled_extension_names(&ext_names);

        let device =
            unsafe { instance.create_device(pdevice.vk_physical_device(), &create_info, None) }
                .expect("Can't create device");
        let queue = unsafe { device.get_device_queue(pdevice.queue_family_index(), 0) };

        let window_extent = vk::Extent2D {
            width: window.inner_size().width,
            height: window.inner_size().height,
        };
        let swapchain = Swapchain::new(instance, pdevice, &device, surface, window_extent);

        let command_buffers = CommandBuffers::new(&device, pdevice.queue_family_index());
        Self {
            device,
            queue,
            command_buffers,
            swapchain,
        }
    }

    pub fn get_vk_device(&self) -> &ash::Device {
        &self.device
    }
}
