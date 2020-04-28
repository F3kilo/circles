use crate::vulkan::physical_device::PhysicalDevice;
use ash::version::DeviceV1_0;
use ash::vk;

pub struct DepthImage {
    image: vk::Image,
}

impl DepthImage {
    pub fn new(device: &ash::Device, pdevice: &PhysicalDevice, resolution: vk::Extent2D) -> Self {
        let depth_image = DepthImage::create_image(device, resolution);

        let memory_requirements = unsafe { device.get_image_memory_requirements(depth_image) };
        let memory_type_index = DepthImage::get_memory_type_index(pdevice, &memory_requirements);
        let allocate_info = vk::MemoryAllocateInfo::builder()
            .allocation_size(memory_requirements.size)
            .memory_type_index(memory_type_index)
            .build();
        let memory = unsafe { device.allocate_memory(&allocate_info, None) }
            .expect("Can't allocate memory for depth image");
        unsafe {
            device
                .bind_image_memory(depth_image, memory, 0)
                .expect("Can't bind depth image memory");
        };
    }

    fn get_memory_type_index(
        pdevice: &PhysicalDevice,
        depth_image_memory_req: &vk::MemoryRequirements,
    ) -> u32 {
        pdevice
            .find_memorytype_index(
                &depth_image_memory_req,
                vk::MemoryPropertyFlags::DEVICE_LOCAL,
            )
            .expect("Unable to find suitable memory index for depth image.")
    }

    fn create_image(device: &ash::Device, resolution: vk::Extent2D) -> vk::Image {
        let depth_image_create_info = vk::ImageCreateInfo::builder()
            .image_type(vk::ImageType::TYPE_2D)
            .format(vk::Format::D16_UNORM)
            .extent(vk::Extent3D {
                width: resolution.width,
                height: resolution.height,
                depth: 1,
            })
            .mip_levels(1)
            .array_layers(1)
            .samples(vk::SampleCountFlags::TYPE_1)
            .tiling(vk::ImageTiling::OPTIMAL)
            .usage(vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);

        let depth_image = unsafe { device.create_image(&depth_image_create_info, None) }.unwrap();
        depth_image
    }
}
