use super::VulkanBase;
use ash::version::{EntryV1_2, InstanceV1_2};
use ash::vk;
use raw_window_handle::RawWindowHandle;

// Libs for windows
#[cfg(all(windows))]
use ash::extensions::khr::Win32Surface;
#[cfg(all(windows))]
use std::os::raw::c_void;
#[cfg(all(windows))]
use winapi;

// Libs for linux
#[cfg(all(unix, not(target_os = "android"), not(target_os = "macos")))]
use ash::extensions::khr::XlibSurface;
use slog::Logger;
#[cfg(all(unix, not(target_os = "android"), not(target_os = "macos")))]
use winit::platform::unix::WindowExtUnix;

pub struct Surface {
    surface_loader: ash::extensions::khr::Surface,
    surface: vk::SurfaceKHR,
    logger: Logger,
}

impl Surface {
    pub fn new(base: &VulkanBase, window_handle: RawWindowHandle, logger: Logger) -> Self {
        let vk_entry = base.get_entry();
        let vk_instance = base.get_instance().get_vk_instance();
        let surface = unsafe {
            create_surface(vk_entry, vk_instance, window_handle).expect("Can't create surface")
        };
        let surface_loader = ash::extensions::khr::Surface::new(vk_entry, vk_instance);
        Self {
            surface_loader,
            surface,
            logger,
        }
    }

    pub fn is_device_supported(&self, pdevice: vk::PhysicalDevice, queue_index: u32) -> bool {
        unsafe {
            self.surface_loader.get_physical_device_surface_support(
                pdevice,
                queue_index,
                self.surface,
            )
        }
        .expect("Can't check surface support for physical device")
    }

    pub fn get_surface_info(&self, pdevice: vk::PhysicalDevice) -> SurfaceInfo {
        let formats = unsafe {
            self.surface_loader
                .get_physical_device_surface_formats(pdevice, self.surface)
        }
        .expect("");
        let capabilities = unsafe {
            self.surface_loader
                .get_physical_device_surface_capabilities(pdevice, self.surface)
        }
        .expect("");
        let present_modes = unsafe {
            self.surface_loader
                .get_physical_device_surface_present_modes(pdevice, self.surface)
        }
        .expect("");
        SurfaceInfo {
            formats,
            capabilities,
            present_modes,
        }
    }

    pub fn get_vk_surface(&self) -> &vk::SurfaceKHR {
        &self.surface
    }

    pub fn destroy(&mut self) {
        debug!(self.logger, "Surface destroy() called");
        unsafe {
            self.surface_loader.destroy_surface(self.surface, None);
        }
        debug!(self.logger, "Surface destroyed");
    }
}

pub struct SurfaceInfo {
    pub formats: Vec<vk::SurfaceFormatKHR>,
    pub capabilities: vk::SurfaceCapabilitiesKHR,
    pub present_modes: Vec<vk::PresentModeKHR>,
}

#[cfg(all(unix, not(target_os = "android"), not(target_os = "macos")))]
unsafe fn create_surface<E: EntryV1_2, I: InstanceV1_2>(
    entry: &E,
    instance: &I,
    window: &winit::window::Window,
) -> Result<vk::SurfaceKHR, vk::Result> {
    if let RawWindowHandle::Xlib(_) = window.raw_window_handle() {
        let xlib_dislplay = window.xlib_display().expect("Can't get xlib display");
        let xlib_window = window.xlib_window().expect("Can't get xlib window");
        let xlib_create_info = vk::XlibSurfaceCreateInfoKHR::builder()
            .window(xlib_window)
            .dpy(xlib_dislplay as *mut vk::Display);

        let xlib_surface_loader = XlibSurface::new(entry, instance);
        return xlib_surface_loader.create_xlib_surface(&xlib_create_info, None);
    }

    panic!("No xlib handle on linux");
}

#[cfg(target_os = "macos")]
unsafe fn create_surface<E: EntryV1_0, I: InstanceV1_0>(
    entry: &E,
    instance: &I,
    window: &winit::Window,
) -> Result<vk::SurfaceKHR, vk::Result> {
    use std::ptr;
    use winit::os::macos::WindowExt;

    let wnd: cocoa_id = mem::transmute(window.get_nswindow());

    let layer = CoreAnimationLayer::new();

    layer.set_edge_antialiasing_mask(0);
    layer.set_presents_with_transaction(false);
    layer.remove_all_animations();

    let view = wnd.contentView();

    layer.set_contents_scale(view.backingScaleFactor());
    view.setLayer(mem::transmute(layer.as_ref()));
    view.setWantsLayer(YES);

    let create_info = vk::MacOSSurfaceCreateInfoMVK {
        s_type: vk::StructureType::MACOS_SURFACE_CREATE_INFO_M,
        p_next: ptr::null(),
        flags: Default::default(),
        p_view: window.get_nsview() as *const c_void,
    };

    let macos_surface_loader = MacOSSurface::new(entry, instance);
    macos_surface_loader.create_mac_os_surface_mvk(&create_info, None)
}

#[cfg(target_os = "windows")]
unsafe fn create_surface<E: EntryV1_2, I: InstanceV1_2>(
    entry: &E,
    instance: &I,
    raw_window_handle: RawWindowHandle,
) -> Result<vk::SurfaceKHR, vk::Result> {
    use std::ptr;
    use winapi::um::libloaderapi::GetModuleHandleW;

    if let RawWindowHandle::Windows(h) = raw_window_handle {
        let hwnd = h.hwnd as winapi::shared::windef::HWND;
        let hinstance = GetModuleHandleW(ptr::null()) as *const c_void;
        let win32_create_info = vk::Win32SurfaceCreateInfoKHR {
            s_type: vk::StructureType::WIN32_SURFACE_CREATE_INFO_KHR,
            p_next: ptr::null(),
            flags: Default::default(),
            hinstance,
            hwnd: hwnd as *const c_void,
        };
        let win32_surface_loader = Win32Surface::new(entry, instance);
        return win32_surface_loader.create_win32_surface(&win32_create_info, None);
    }

    panic!("No window handle on windows");
}
