//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_GOOGLE_display_timing.html>

use crate::prelude::*;
use crate::vk;
use alloc::vec::Vec;
use core::mem;
pub use vk::google::display_timing::NAME;

#[derive(Clone)]
pub struct Device {
    handle: vk::Device,
    fp: vk::google::display_timing::DeviceFn,
}

impl Device {
    pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
        let handle = device.handle();
        let fp = vk::google::display_timing::DeviceFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPastPresentationTimingGOOGLE.html>
    #[inline]
    pub unsafe fn get_past_presentation_timing(
        &self,
        swapchain: vk::SwapchainKHR,
    ) -> VkResult<Vec<vk::PastPresentationTimingGOOGLE>> {
        read_into_uninitialized_vector(|count, data| {
            (self.fp.get_past_presentation_timing_google)(self.handle, swapchain, count, data)
        })
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetRefreshCycleDurationGOOGLE.html>
    #[inline]
    pub unsafe fn get_refresh_cycle_duration(
        &self,
        swapchain: vk::SwapchainKHR,
    ) -> VkResult<vk::RefreshCycleDurationGOOGLE> {
        let mut properties = mem::MaybeUninit::uninit();
        (self.fp.get_refresh_cycle_duration_google)(self.handle, swapchain, properties.as_mut_ptr())
            .assume_init_on_success(properties)
    }

    #[inline]
    pub fn fp(&self) -> &vk::google::display_timing::DeviceFn {
        &self.fp
    }

    #[inline]
    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
