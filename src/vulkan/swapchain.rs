use super::physical_device::PhysicalDevice;
use super::surface::Surface;
use crate::vulkan::instance::Instance;
use ash::version::DeviceV1_0;
use ash::vk;
use slog::Logger;

pub struct Swapchain {
    swapchain: vk::SwapchainKHR,
    swapchain_loader: ash::extensions::khr::Swapchain,
    images: Vec<vk::Image>,
    image_views: Vec<vk::ImageView>,
    logger: Logger,
}

impl Swapchain {
    pub fn new(
        instance: &Instance,
        surface: &Surface,
        window_size: vk::Extent2D,
        logger: Logger,
    ) -> Self {
        let pdevice = instance.get_physical_device();
        let device = instance.get_device();
        let surface_info = surface.get_surface_info(pdevice);
        let format = Self::select_format(&surface_info.formats);
        let image_count = Self::select_image_count(&surface_info.capabilities);
        let resolution = Self::select_resolution(&surface_info.capabilities, window_size);
        let pre_transform = Self::select_pre_transform(&surface_info.capabilities);
        let present_mode = Self::select_present_mode(&surface_info.present_modes);

        let vk_device = device.get_vk_device();
        let swapchain_loader =
            ash::extensions::khr::Swapchain::new(instance.get_vk_instance(), vk_device);
        let swapchain_create_info = vk::SwapchainCreateInfoKHR::builder()
            .surface(*surface.get_vk_surface())
            .min_image_count(image_count)
            .image_color_space(format.color_space)
            .image_format(format.format)
            .image_extent(resolution)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
            .pre_transform(pre_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(present_mode)
            .clipped(true)
            .image_array_layers(1);
        let swapchain = unsafe { swapchain_loader.create_swapchain(&swapchain_create_info, None) }
            .expect("Can't create swapchain");

        let images = unsafe { swapchain_loader.get_swapchain_images(swapchain) }
            .expect("Can't get swapchain images");
        let image_views = Self::create_image_views(vk_device, &images, format.format);

        Self {
            swapchain_loader,
            swapchain,
            images,
            image_views,
            logger,
        }
    }

    fn create_image_views(
        device: &ash::Device,
        images: &[vk::Image],
        format: vk::Format,
    ) -> Vec<vk::ImageView> {
        images
            .iter()
            .map(|&image| {
                let create_view_info = vk::ImageViewCreateInfo::builder()
                    .view_type(vk::ImageViewType::TYPE_2D)
                    .format(format)
                    .components(vk::ComponentMapping {
                        r: vk::ComponentSwizzle::R,
                        g: vk::ComponentSwizzle::G,
                        b: vk::ComponentSwizzle::B,
                        a: vk::ComponentSwizzle::A,
                    })
                    .subresource_range(vk::ImageSubresourceRange {
                        aspect_mask: vk::ImageAspectFlags::COLOR,
                        base_mip_level: 0,
                        level_count: 1,
                        base_array_layer: 0,
                        layer_count: 1,
                    })
                    .image(image);
                unsafe { device.create_image_view(&create_view_info, None) }.unwrap()
            })
            .collect()
    }

    pub fn select_format(formats: &[vk::SurfaceFormatKHR]) -> vk::SurfaceFormatKHR {
        formats
            .iter()
            .map(|sfmt| match sfmt.format {
                vk::Format::UNDEFINED => vk::SurfaceFormatKHR {
                    format: vk::Format::B8G8R8_UNORM,
                    color_space: sfmt.color_space,
                },
                _ => *sfmt,
            })
            .next()
            .expect("Unable to find suitable surface format.")
    }

    pub fn select_image_count(capabilities: &vk::SurfaceCapabilitiesKHR) -> u32 {
        let mut desired_image_count = capabilities.min_image_count + 1;
        if capabilities.max_image_count > 0 && desired_image_count > capabilities.max_image_count {
            desired_image_count = capabilities.max_image_count;
        }
        desired_image_count
    }

    pub fn select_resolution(
        capabilities: &vk::SurfaceCapabilitiesKHR,
        window_size: vk::Extent2D,
    ) -> vk::Extent2D {
        match capabilities.current_extent.width {
            std::u32::MAX => vk::Extent2D {
                width: window_size.width,
                height: window_size.height,
            },
            _ => capabilities.current_extent,
        }
    }

    pub fn select_pre_transform(
        capabilities: &vk::SurfaceCapabilitiesKHR,
    ) -> vk::SurfaceTransformFlagsKHR {
        if capabilities
            .supported_transforms
            .contains(vk::SurfaceTransformFlagsKHR::IDENTITY)
        {
            vk::SurfaceTransformFlagsKHR::IDENTITY
        } else {
            capabilities.current_transform
        }
    }

    pub fn select_present_mode(present_modes: &[vk::PresentModeKHR]) -> vk::PresentModeKHR {
        present_modes
            .iter()
            .cloned()
            .find(|&mode| mode == vk::PresentModeKHR::MAILBOX)
            .unwrap_or(vk::PresentModeKHR::FIFO)
    }

    pub fn destroy(&mut self) {
        debug!(self.logger, "Swapchain destroy() called");
        unsafe {
            self.swapchain_loader
                .destroy_swapchain(self.swapchain, None);
        }
    }
}
