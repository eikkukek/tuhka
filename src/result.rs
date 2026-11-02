use crate::vk;

pub type VkResult<T> = Result<T, vk::Result>;

impl vk::Result {

    pub fn success(
        self,
    ) -> VkResult<()> {
        if self != vk::Result::SUCCESS {
            Err(self)
        } else {
            Ok(())
        }
    }

    pub fn success_with<T>(
        self,
        f: impl FnOnce() -> T
    ) -> VkResult<T> {
        if self != vk::Result::SUCCESS {
            Err(self)
        } else {
            Ok(f())
        }
    }

    pub fn success_with_value<T>(
        self,
        value: T,
    ) -> VkResult<T> {
        if self != vk::Result::SUCCESS {
            Err(self)
        } else {
            Ok(value)
        }
    }

    /// # Safety
    /// `value` *must* be initialized
    pub unsafe fn success_with_assume_init<T>(
        self,
        value: core::mem::MaybeUninit<T>
    ) -> VkResult<T> {
        unsafe {
            if self != vk::Result::SUCCESS {
                Err(self)
            } else {
                Ok(value.assume_init())
            }
        }
    }

    pub fn filter_success(
        self,
        success: &[vk::Result],
    ) -> VkResult<vk::Result>
    {
        if success.contains(&self) {
            Ok(self)
        } else { Err(self) }
    }

    pub fn filter_success_with<T>(
        self,
        success: &[vk::Result],
        f: impl FnOnce() -> T
    ) -> VkResult<(T, vk::Result)> {
        if success.contains(&self) {
            Ok((f(), self))
        } else { Err(self) }
    }

    pub fn filter_success_with_value<T>(
        self,
        success: &[vk::Result],
        value: T,
    ) -> VkResult<(T, vk::Result)> {
        if success.contains(&self) {
            Ok((value, self))
        } else { Err(self) }
    }

    /// # Safety
    /// `value` *must* be initialized
    pub unsafe fn filter_success_with_assume_init<T>(
        self,
        success: &[vk::Result],
        value: core::mem::MaybeUninit<T>
    ) -> VkResult<(T, vk::Result)> {
        unsafe {
            if success.contains(&self) {
                Ok((value.assume_init(), self))
            } else { Err(self) }
        }
    }
}

impl core::error::Error for vk::Result {}
