use std::ptr;

use super::BackendSemaphore;
use crate::gpu;
use crate::gpu::GpuStatsFlags;
use crate::prelude::*;
use skia_bindings as sb;

pub use skia_bindings::GrBackendApi as BackendApi;
variant_name!(BackendAPI::OpenGL);

#[deprecated(since = "0.80.0", note = "use BackendApi")]
pub use BackendApi as BackendAPI;

pub const METAL_BACKEND: BackendApi = BackendApi::Metal;
pub const VULKAN_BACKEND: BackendApi = BackendApi::Vulkan;
pub const MOCK_BACKEND: BackendApi = BackendApi::Mock;

pub use gpu::Renderable;

pub use gpu::Protected;

pub use skia_bindings::GrSurfaceOrigin as SurfaceOrigin;
variant_name!(SurfaceOrigin::BottomLeft);

// Note: BackendState is in gl/types.rs/

#[derive(Debug)]
pub struct FlushInfo {
    native: sb::GrFlushInfo,
    signal_semaphores: Vec<BackendSemaphore>,
}

impl Default for FlushInfo {
    fn default() -> Self {
        let mut flush_info = Self {
            native: sb::GrFlushInfo {
                fNumSemaphores: 0,
                fGpuStatsFlags: GpuStatsFlags::NONE.bits(),
                fSignalSemaphores: ptr::null_mut(),
                fFinishedProc: None,
                fFinishedWithStatsProc: None,
                fFinishedContext: ptr::null_mut(),
                fSubmittedProc: None,
                fSubmittedContext: ptr::null_mut(),
            },
            signal_semaphores: Vec::new(),
        };
        flush_info.sync_signal_semaphores();
        flush_info
    }
}

impl FlushInfo {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn gpu_stats_flags(&self) -> GpuStatsFlags {
        GpuStatsFlags::from_bits_truncate(self.native.fGpuStatsFlags)
    }

    pub fn set_gpu_stats_flags(&mut self, flags: GpuStatsFlags) -> &mut Self {
        self.native.fGpuStatsFlags = flags.bits();
        self
    }

    pub fn signal_semaphores(&self) -> &[BackendSemaphore] {
        &self.signal_semaphores
    }

    pub fn set_signal_semaphores(
        &mut self,
        semaphores: impl IntoIterator<Item = BackendSemaphore>,
    ) -> &mut Self {
        self.signal_semaphores = semaphores.into_iter().collect();
        self.sync_signal_semaphores();
        self
    }

    pub fn push_signal_semaphore(&mut self, semaphore: BackendSemaphore) -> &mut Self {
        self.signal_semaphores.push(semaphore);
        self.sync_signal_semaphores();
        self
    }

    pub fn clear_signal_semaphores(&mut self) -> &mut Self {
        self.signal_semaphores.clear();
        self.sync_signal_semaphores();
        self
    }

    fn sync_signal_semaphores(&mut self) {
        self.native.fNumSemaphores = self.signal_semaphores.len();
        self.native.fSignalSemaphores = if self.signal_semaphores.is_empty() {
            ptr::null_mut()
        } else {
            self.signal_semaphores.native_mut().as_mut_ptr()
        };
    }
}

impl NativeAccess for FlushInfo {
    type Native = sb::GrFlushInfo;

    fn native(&self) -> &Self::Native {
        &self.native
    }

    fn native_mut(&mut self) -> &mut Self::Native {
        self.sync_signal_semaphores();
        &mut self.native
    }
}

pub use sb::GrSemaphoresSubmitted as SemaphoresSubmitted;
variant_name!(SemaphoresSubmitted::Yes);

pub use sb::GrPurgeResourceOptions as PurgeResourceOptions;
variant_name!(PurgeResourceOptions::AllResources);

pub use sb::GrSyncCpu as SyncCpu;
variant_name!(SyncCpu::Yes);

pub use sb::GrMarkFrameBoundary as MarkFrameBoundary;
variant_name!(MarkFrameBoundary::Yes);

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SubmitInfo {
    pub sync: SyncCpu,
    pub mark_boundary: MarkFrameBoundary,
    pub frame_id: u64,
}
native_transmutable!(sb::GrSubmitInfo, SubmitInfo);

impl Default for SubmitInfo {
    fn default() -> Self {
        Self {
            sync: SyncCpu::No,
            mark_boundary: MarkFrameBoundary::No,
            frame_id: 0,
        }
    }
}

impl From<SyncCpu> for SubmitInfo {
    fn from(sync: SyncCpu) -> Self {
        Self {
            sync,
            ..Self::default()
        }
    }
}

impl From<Option<SyncCpu>> for SubmitInfo {
    fn from(sync_cpu: Option<SyncCpu>) -> Self {
        match sync_cpu {
            Some(sync_cpu) => sync_cpu.into(),
            None => Self::default(),
        }
    }
}
