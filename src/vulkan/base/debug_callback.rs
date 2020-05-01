use ash::extensions::ext::DebugReport;
use ash::vk;
use slog::Logger;
use std::ffi::CStr;
use std::os::raw::{c_char, c_void};

pub struct DebugCallback {
    debug_callback: vk::DebugReportCallbackEXT,
    debug_report_loader: DebugReport,
    logger: Logger,
}

impl DebugCallback {
    pub fn new(entry: &ash::Entry, instance: &ash::Instance, logger: Logger) -> Self {
        let debug_report_loader = DebugReport::new(entry, instance);
        let debug_info = vk::DebugReportCallbackCreateInfoEXT::builder()
            .flags(Self::debug_report_flags())
            .pfn_callback(Some(vulkan_debug_callback));
        let debug_callback = unsafe {
            debug_report_loader
                .create_debug_report_callback(&debug_info, None)
                .expect("Can't create debug report callback")
        };
        Self {
            debug_callback,
            debug_report_loader,
            logger,
        }
    }

    fn debug_report_flags() -> vk::DebugReportFlagsEXT {
        vk::DebugReportFlagsEXT::ERROR
            | vk::DebugReportFlagsEXT::WARNING
            | vk::DebugReportFlagsEXT::PERFORMANCE_WARNING
            | vk::DebugReportFlagsEXT::DEBUG
            | vk::DebugReportFlagsEXT::INFORMATION
    }

    pub fn destroy(&mut self) {
        debug!(self.logger, "DebugCallback destroy() called");
        unsafe {
            self.debug_report_loader
                .destroy_debug_report_callback(self.debug_callback, None);
            debug!(self.logger, "\tvk::DebugCallbackReport destroyed");
        }
    }
}

unsafe extern "system" fn vulkan_debug_callback(
    _: vk::DebugReportFlagsEXT,
    _: vk::DebugReportObjectTypeEXT,
    _: u64,
    _: usize,
    _: i32,
    _: *const c_char,
    p_message: *const c_char,
    _: *mut c_void,
) -> u32 {
    println!("DEBUG_REPORT_CALLBACK: {:?}", CStr::from_ptr(p_message));
    vk::FALSE
}
