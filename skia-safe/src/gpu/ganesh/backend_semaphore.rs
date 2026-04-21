use std::fmt;

use skia_bindings::{self as sb, GrBackendSemaphore};

use crate::gpu;
use crate::prelude::*;

/// Wrapper type for passing into and receiving data from Ganesh about a
/// backend semaphore object.
pub type BackendSemaphore = Handle<GrBackendSemaphore>;
unsafe_send_sync!(BackendSemaphore);

impl NativeDrop for GrBackendSemaphore {
    fn drop(&mut self) {
        unsafe { sb::C_GrBackendSemaphore_destruct(self) }
    }
}

impl NativeClone for GrBackendSemaphore {
    fn clone(&self) -> Self {
        construct(|semaphore| unsafe { sb::C_GrBackendSemaphore_CopyConstruct(semaphore, self) })
    }
}

impl fmt::Debug for BackendSemaphore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("BackendSemaphore");
        d.field("backend", &self.backend())
            .field("is_initialized", &self.is_initialized());
        #[cfg(feature = "vulkan")]
        d.field("vk_semaphore", &self.vk_semaphore());
        d.finish()
    }
}

impl Default for BackendSemaphore {
    fn default() -> Self {
        Self::new()
    }
}

impl BackendSemaphore {
    pub fn new() -> Self {
        Self::construct(|semaphore| unsafe { sb::C_GrBackendSemaphore_Construct(semaphore) })
    }

    pub fn backend(&self) -> sb::GrBackendApi {
        unsafe { sb::C_GrBackendSemaphore_backend(self.native()) }
    }

    pub fn is_initialized(&self) -> bool {
        unsafe { sb::C_GrBackendSemaphore_isInitialized(self.native()) }
    }

    #[cfg(feature = "vulkan")]
    pub fn new_vulkan(semaphore: gpu::vk::Semaphore) -> Self {
        Self::construct(|backend_semaphore| unsafe {
            sb::C_GrBackendSemaphore_ConstructVk(backend_semaphore, semaphore)
        })
    }

    #[cfg(feature = "vulkan")]
    pub fn vk_semaphore(&self) -> Option<gpu::vk::Semaphore> {
        (self.backend() == sb::GrBackendApi::Vulkan && self.is_initialized())
            .then(|| unsafe { sb::C_GrBackendSemaphores_GetVkSemaphore(self.native()) })
    }
}
