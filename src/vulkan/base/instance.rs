use super::debug_callback::DebugCallback;
use ash::extensions::ext::DebugReport;
use ash::version::{EntryV1_0, InstanceV1_0};
use ash::vk;
use ash::Entry;
use std::ffi::{CStr, CString};

#[cfg(all(windows))]
use ash::extensions::khr::Win32Surface;
#[cfg(all(unix, not(target_os = "android"), not(target_os = "macos")))]
use ash::extensions::khr::XlibSurface;
use slog::Logger;

pub struct Instance {
    debug_callback: DebugCallback,
    instance: ash::Instance,
    logger: Logger,
}

impl Instance {
    pub fn new(entry: &ash::Entry, app_name: &str, logger: Logger) -> Self {
        let app_name = CString::new(app_name.as_bytes()).unwrap();
        let engine_name = CString::new("test").unwrap();
        let app_info = Self::app_info(&app_name, &engine_name);
        let layers = vec!["VK_LAYER_LUNARG_standard_validation\0".as_ptr() as *const i8];
        let ext_names = Self::extension_names();
        let create_info = vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_layer_names(&layers)
            .enabled_extension_names(&ext_names);

        let instance = Self::create_instance(&entry, &create_info);
        let debug_callback = DebugCallback::new(entry, &instance, logger.clone());
        Self {
            debug_callback,

            instance,
            logger,
        }
    }

    pub fn get_vk_instance(&self) -> &ash::Instance {
        &self.instance
    }

    fn app_info<'a>(app_name: &'a CStr, engine_name: &'a CStr) -> vk::ApplicationInfoBuilder<'a> {
        vk::ApplicationInfo::builder()
            .application_name(app_name)
            .application_version(0)
            .engine_name(engine_name)
            .engine_version(0)
            .api_version(vk::make_version(1, 2, 0))
    }

    #[cfg(all(unix, not(target_os = "android"), not(target_os = "macos")))]
    fn extension_names() -> Vec<*const i8> {
        vec![
            ash::extensions::khr::Surface::name().as_ptr(),
            XlibSurface::name().as_ptr(),
            DebugReport::name().as_ptr(),
        ]
    }

    #[cfg(target_os = "macos")]
    fn extension_names() -> Vec<*const i8> {
        vec![
            Surface::name().as_ptr(),
            MacOSSurface::name().as_ptr(),
            DebugReport::name().as_ptr(),
        ]
    }

    #[cfg(all(windows))]
    fn extension_names() -> Vec<*const i8> {
        vec![
            ash::extensions::khr::Surface::name().as_ptr(),
            Win32Surface::name().as_ptr(),
            DebugReport::name().as_ptr(),
        ]
    }

    fn create_instance(entry: &Entry, create_info: &vk::InstanceCreateInfo) -> ash::Instance {
        unsafe {
            entry
                .create_instance(&create_info, None)
                .expect("Instance creation error")
        }
    }

    pub fn destroy(&mut self) {
        debug!(self.logger, "Instance destroy() called");
        self.debug_callback.destroy();
        unsafe {
            self.instance.destroy_instance(None);
            debug!(self.logger, "\tvk::Instance destroyed");
        };
    }
}
