//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_video_encode_queue.html>

use crate::prelude::VkResult;
use crate::{vk, RawPtr};
use core::mem;
use std::ptr;

impl crate::khr::video_queue::Instance {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceVideoCapabilitiesKHR.html>
    #[inline]
    pub unsafe fn get_physical_device_video_capabilities(
        &self,
        physical_device: vk::PhysicalDevice,
        video_profile: &vk::VideoProfileInfoKHR<'_>,
        capabilities: &mut vk::VideoCapabilitiesKHR<'_>
    ) -> VkResult<()> {
        (self.fp.get_physical_device_video_capabilities_khr)(
            physical_device,
            video_profile,
            capabilities,
        ).result()
    }

    // Retrieve the number of elements to pass to [`get_physical_device_video_format_properties`][Self::get_physical_device_video_format_properties]
    #[inline]
    pub unsafe fn get_physical_device_video_format_properties_len(
        &self,
        physical_device: vk::PhysicalDevice,
        video_format_info: &vk::PhysicalDeviceVideoFormatInfoKHR<'_>,
    ) -> usize {
        let mut memory_requirements_count = mem::MaybeUninit::uninit();
        let _ = (self.fp.get_physical_device_video_format_properties_khr)(
            physical_device,
            video_format_info,
            memory_requirements_count.as_mut_ptr(),
            ptr::null_mut(),
        );
        memory_requirements_count.assume_init() as _
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceVideoFormatPropertiesKHR.html>
    ///
    /// Call [`get_physical_device_video_format_properties_len()`][Self::get_physical_device_video_format_properties_len()] to query the number of elements to pass to `out`.
    /// Be sure to [`Default::default()`]-initialize these elements and optionally set their `p_next` pointer.
    #[inline]
    pub unsafe fn get_physical_device_video_format_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        video_format_info: &vk::PhysicalDeviceVideoFormatInfoKHR<'_>,
        out: &mut [vk::VideoFormatPropertiesKHR<'_>],
    ) -> VkResult<()> {
        let mut count = out.len() as u32;
        (self.fp.get_physical_device_video_format_properties_khr)(
            physical_device,
            video_format_info,
            &mut count,
            out.as_mut_ptr(),
        ).result()
    }
}

impl crate::khr::video_queue::Device {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateVideoSessionKHR.html>
    #[inline]
    pub unsafe fn create_video_session(
        &self,
        create_info: &vk::VideoSessionCreateInfoKHR<'_>,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<vk::VideoSessionKHR> {
        let mut video_session = mem::MaybeUninit::uninit();
        (self.fp.create_video_session_khr)(
            self.handle,
            create_info,
            allocation_callbacks.as_raw_ptr(),
            video_session.as_mut_ptr(),
        )
        .assume_init_on_success(video_session)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyVideoSessionKHR.html>
    #[inline]
    pub unsafe fn destroy_video_session(
        &self,
        video_session: vk::VideoSessionKHR,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) {
        (self.fp.destroy_video_session_khr)(
            self.handle,
            video_session,
            allocation_callbacks.as_raw_ptr(),
        )
    }

    // Retrieve the number of elements to pass to [`get_video_session_memory_requirements`][Self::get_video_session_memory_requirements]
    #[inline]
    pub unsafe fn get_video_session_memory_requirements_len(
        &self,
        video_session: vk::VideoSessionKHR,
    ) -> usize {
        let mut memory_requirements_count = mem::MaybeUninit::uninit();
        let _ = (self.fp.get_video_session_memory_requirements_khr)(
            self.handle,
            video_session,
            memory_requirements_count.as_mut_ptr(),
            ptr::null_mut(),
        );
        memory_requirements_count.assume_init() as _
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetVideoSessionMemoryRequirementsKHR.html>
    ///
    /// Call [`get_video_session_memory_requirements_len()`][Self::get_video_session_memory_requirements_len()] to query the number of elements to pass to `out`.
    /// Be sure to [`Default::default()`]-initialize these elements and optionally set their `p_next` pointer.
    #[inline]
    pub unsafe fn get_video_session_memory_requirements(
        &self,
        video_session: vk::VideoSessionKHR,
        out: &mut [vk::VideoSessionMemoryRequirementsKHR<'_>],
    ) -> VkResult<()> {
        let mut count = out.len() as u32;
        (self.fp.get_video_session_memory_requirements_khr)(
            self.handle,
            video_session,
            &mut count,
            out.as_mut_ptr(),
        ).result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkBindVideoSessionMemoryKHR.html>
    #[inline]
    pub unsafe fn bind_video_session_memory(
        &self,
        video_session: vk::VideoSessionKHR,
        bind_session_memory_infos: &[vk::BindVideoSessionMemoryInfoKHR<'_>],
    ) -> VkResult<()> {
        (self.fp.bind_video_session_memory_khr)(
            self.handle,
            video_session,
            bind_session_memory_infos.len() as _,
            bind_session_memory_infos.as_ptr(),
        )
        .result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateVideoSessionParametersKHR.html>
    #[inline]
    pub unsafe fn create_video_session_parameters(
        &self,
        create_info: &vk::VideoSessionParametersCreateInfoKHR<'_>,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<vk::VideoSessionParametersKHR> {
        let mut parameters = mem::MaybeUninit::uninit();
        (self.fp.create_video_session_parameters_khr)(
            self.handle,
            create_info,
            allocation_callbacks.as_raw_ptr(),
            parameters.as_mut_ptr(),
        )
        .assume_init_on_success(parameters)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkUpdateVideoSessionParametersKHR.html>
    #[inline]
    pub unsafe fn update_video_session_parameters(
        &self,
        video_session_parameters: vk::VideoSessionParametersKHR,
        update_info: &vk::VideoSessionParametersUpdateInfoKHR<'_>,
    ) -> VkResult<()> {
        (self.fp.update_video_session_parameters_khr)(
            self.handle,
            video_session_parameters,
            update_info,
        )
        .result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyVideoSessionParametersKHR.html>
    #[inline]
    pub unsafe fn destroy_video_session_parameters(
        &self,
        video_session_parameters: vk::VideoSessionParametersKHR,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) {
        (self.fp.destroy_video_session_parameters_khr)(
            self.handle,
            video_session_parameters,
            allocation_callbacks.as_raw_ptr(),
        )
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBeginVideoCodingKHR.html>
    #[inline]
    pub unsafe fn cmd_begin_video_coding(
        &self,
        command_buffer: vk::CommandBuffer,
        begin_info: &vk::VideoBeginCodingInfoKHR<'_>,
    ) {
        (self.fp.cmd_begin_video_coding_khr)(command_buffer, begin_info)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdEndVideoCodingKHR.html>
    #[inline]
    pub unsafe fn cmd_end_video_coding(
        &self,
        command_buffer: vk::CommandBuffer,
        end_coding_info: &vk::VideoEndCodingInfoKHR<'_>,
    ) {
        (self.fp.cmd_end_video_coding_khr)(command_buffer, end_coding_info)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdControlVideoCodingKHR.html>
    #[inline]
    pub unsafe fn cmd_control_video_coding(
        &self,
        command_buffer: vk::CommandBuffer,
        coding_control_info: &vk::VideoCodingControlInfoKHR<'_>,
    ) {
        (self.fp.cmd_control_video_coding_khr)(command_buffer, coding_control_info)
    }
}
