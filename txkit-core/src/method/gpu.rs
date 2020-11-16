//! GPU Procedural texturing method types

use crate::context::GpuContext;
use crate::image::gpu::GpuImageData;
use crate::Result;

use super::TextureMethod;

/// Represents a GPU procedural texturing method
pub trait GpuMethod: TextureMethod {
    /// Compute one frame of this method
    ///
    /// # Parameters
    ///
    /// * `ctx`: GPU context to perform computations in
    /// * `tgt`: frame to fill with computation results
    /// * `params`: parameters of the frame to compute
    fn compute_gpu(
        &mut self,
        ctx: &mut GpuContext,
        tgt: &mut GpuImageData,
        params: &Self::Params,
    ) -> Result<()>;
}
