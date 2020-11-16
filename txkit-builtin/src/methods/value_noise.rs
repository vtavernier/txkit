use txkit_core::{context::Context, Error, Result};

#[derive(Default, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ValueNoiseParams {}

#[cfg(feature = "gpu")]
pub struct ValueNoiseGpu {
    program: tinygl::wrappers::GlHandle<crate::shaders::ValueNoiseProgram>,
}

#[cfg(feature = "gpu")]
impl ValueNoiseGpu {
    pub fn new(ctx: &txkit_core::context::GpuContext) -> Result<Self> {
        let gl = ctx.gl().clone();

        Ok(Self {
            program: tinygl::wrappers::GlHandle::new(
                &gl,
                crate::shaders::ValueNoiseProgram::build(&*gl)?,
            ),
        })
    }
}

#[cfg(feature = "gpu")]
impl txkit_core::method::TextureMethod for ValueNoiseGpu {
    type Params = ValueNoiseParams;
}

#[cfg(feature = "gpu")]
impl txkit_core::method::GpuMethod for ValueNoiseGpu {
    fn compute_gpu(
        &mut self,
        ctx: &mut txkit_core::context::GpuContext,
        tgt: &mut txkit_core::image::gpu::GpuImageData,
        _params: &Self::Params,
    ) -> Result<()> {
        use tinygl::wrappers::ProgramCommonExt;
        use txkit_core::image::{ImageDataBase, ImageDimGpuExt};

        let dim = tgt.dim().into_cgmath();
        ctx.render_to_framebuffer(tgt, |gl, layer| {
            unsafe {
                self.program.use_program(gl);
            }

            // Common parameters
            self.program.set_i_resolution(gl, dim);
            self.program.set_i_layer(gl, layer);

            unsafe {
                gl.draw_arrays(tinygl::gl::TRIANGLES, 0, 3);
            }

            Ok(())
        })
    }
}

#[derive(Default)]
pub struct ValueNoise {
    #[cfg(feature = "gpu")]
    gpu: Option<ValueNoiseGpu>,
}

impl ValueNoise {
    pub fn new() -> Self {
        Self::default()
    }
}

impl txkit_core::method::Method for ValueNoise {
    fn compute(
        &mut self,
        ctx: &mut txkit_core::context::Context,
        tgt: &mut txkit_core::image::Image,
        params: Option<&dyn std::any::Any>,
    ) -> Result<()> {
        let mut default_params = None;
        let params = txkit_core::method::downcast_params(params, &mut default_params)?;

        match ctx {
            #[cfg(feature = "gpu")]
            Context::Gpu(gpu_context) => tgt
                .as_gpu_image_mut()
                .ok_or_else(|| Error::FormatNotSupported)
                .and_then(|tgt| {
                    use txkit_core::method::GpuMethod;

                    // Initialize GPU if needed
                    if let None = self.gpu {
                        self.gpu = Some(ValueNoiseGpu::new(gpu_context)?);
                    }

                    // Compute result using initialized GPU resources
                    let gpu = self.gpu.as_mut().unwrap();
                    gpu.compute_gpu(gpu_context, tgt, params)
                }),
            #[cfg(not(feature = "gpu"))]
            Context::Gpu(_) => Err(Error::ContextNotSupported),
            Context::Cpu(_) => Err(Error::ContextNotSupported),
        }
    }
}
