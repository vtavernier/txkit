use std::any::Any;

use crate::context::Context;
use crate::error::*;
use crate::image::Image;

mod registry;
pub use registry::*;

#[cfg(feature = "cpu")]
mod cpu;
#[cfg(feature = "cpu")]
pub use self::cpu::*;

#[cfg(feature = "gpu-core")]
mod gpu;
#[cfg(feature = "gpu-core")]
pub use self::gpu::*;

/// Try to downcast a generic params struct into the target params type
pub fn downcast_params<'u, U: Default + 'static>(
    params: Option<&'u dyn std::any::Any>,
    default_params: &'u mut Option<U>,
) -> Result<&'u U> {
    Ok(match params {
        Some(params) => {
            if let Some(p) = params.downcast_ref() {
                p
            } else if let Some(buf) = params.downcast_ref::<&[u8]>() {
                if buf.len() != std::mem::size_of::<U>() {
                    return Err(Error::InvalidParameters);
                }

                unsafe { &*(buf.as_ptr() as *const U) }
            } else {
                return Err(Error::InvalidParameters);
            }
        }
        None => {
            *default_params = Some(U::default());
            default_params.as_ref().unwrap()
        }
    })
}

/// Base trait for all methods
pub trait TextureMethod {
    type Params;
}

/// Generic interface to a procedural texturing method
pub trait Method {
    fn compute(
        &mut self,
        ctx: &mut Context,
        tgt: &mut Image,
        params: Option<&dyn Any>,
    ) -> Result<()>;
}

/// Wrapped method for FFI
pub struct MethodBox {
    method: Box<dyn Method>,
}

/// Compute an image using the given method
///
/// # Parameters
///
/// * `ctx`: context to use for computing the image
/// * `method`: texturing method
/// * `tgt`: target image to be computed
/// * `params`: pointer to the parameter structure for this method
/// * `params_size`: size of the parameter structure
///
/// # Returns
///
/// TxKit_SUCCESS if no error occurred, else a non-zero code.
#[no_mangle]
pub unsafe extern "C" fn txkit_method_compute(
    ctx: &mut Context,
    method: &mut MethodBox,
    tgt: &mut Image,
    params: *const std::ffi::c_void,
    params_size: usize,
) -> i32 {
    let params_slice;
    let params: Option<&dyn Any> = if params == std::ptr::null() {
        None
    } else {
        params_slice = std::slice::from_raw_parts(params as *const u8, params_size);
        Some(&params_slice)
    };

    crate::api::wrap_result_code(|| method.method.compute(ctx, tgt, params))
}

/// Destroy a method
#[no_mangle]
pub unsafe extern "C" fn txkit_method_destroy(method: *mut MethodBox) {
    std::mem::drop(Box::from_raw(method))
}
