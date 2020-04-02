mod cpu;
#[cfg(feature = "gpu")]
pub(crate) mod gpu;

mod image_data;
pub use image_data::*;

mod image_dimensions;
pub use image_dimensions::*;

use failure::Fail;

/// Image that can be sent accross for FFI
pub struct Image {
    data: Box<dyn ImageData>,
}

impl std::ops::Deref for Image {
    type Target = dyn ImageData;

    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        self.data.as_ref()
    }
}

impl std::ops::DerefMut for Image {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        self.data.as_mut()
    }
}

/// Type of elements in an image
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageDataType {
    /// Unsigned bytes (8 bits)
    UInt8,
    /// Single-precision floating point (32 bits)
    Float32,
}

impl ImageDataType {
    pub fn byte_size(&self) -> usize {
        match self {
            Self::UInt8 => std::mem::size_of::<u8>(),
            Self::Float32 => std::mem::size_of::<f32>(),
        }
    }
}

#[derive(Debug, Fail, Clone, Eq, PartialEq)]
pub enum ImageCreationError {
    #[fail(display = "the context cannot create this type of images")]
    ContextNotSupported,
    #[fail(display = "failed to create the image: {}", 0)]
    ImageCreationFailed(String),
}

impl From<String> for ImageCreationError {
    fn from(msg: String) -> Self {
        Self::ImageCreationFailed(msg)
    }
}

impl Image {
    pub fn new_cpu(dim: ImageDim, element_type: ImageDataType) -> Self {
        Self {
            data: match element_type {
                ImageDataType::UInt8 => Box::new(cpu::UInt8ImageData::new(dim)),
                ImageDataType::Float32 => Box::new(cpu::FloatImageData::new(dim)),
            },
        }
    }

    #[cfg(feature = "gpu")]
    pub fn new_gpu_1d(
        dim: ImageDim,
        element_type: ImageDataType,
        context: &crate::context::Context,
    ) -> Result<Self, ImageCreationError> {
        context
            .gpu()
            .ok_or(ImageCreationError::ContextNotSupported)
            .and_then(|gpu_context| {
                Ok(Self {
                    data: Box::new(gpu::GpuImageData::new_1d(
                        &gpu_context.gl,
                        dim,
                        element_type,
                    )?),
                })
            })
    }

    #[cfg(feature = "gpu")]
    pub fn new_gpu_2d(
        dim: ImageDim,
        element_type: ImageDataType,
        context: &crate::context::Context,
    ) -> Result<Self, ImageCreationError> {
        context
            .gpu()
            .ok_or(ImageCreationError::ContextNotSupported)
            .and_then(|gpu_context| {
                Ok(Self {
                    data: Box::new(gpu::GpuImageData::new_2d(
                        &gpu_context.gl,
                        dim,
                        element_type,
                    )?),
                })
            })
    }

    #[cfg(feature = "gpu")]
    pub fn new_gpu_3d(
        dim: ImageDim,
        element_type: ImageDataType,
        context: &crate::context::Context,
    ) -> Result<Self, ImageCreationError> {
        context
            .gpu()
            .ok_or(ImageCreationError::ContextNotSupported)
            .and_then(|gpu_context| {
                Ok(Self {
                    data: Box::new(gpu::GpuImageData::new_3d(
                        &gpu_context.gl,
                        dim,
                        element_type,
                    )?),
                })
            })
    }
}

/// Create a new image for CPU-based computations
///
/// # Parameters
///
/// * `dim`: dimensions of the image
/// * `element_type`: type of the elements in the image
///
/// # Returns
///
/// Allocated image.
#[no_mangle]
pub extern "C" fn txkit_image_new_cpu(dim: ImageDim, element_type: ImageDataType) -> *mut Image {
    Box::into_raw(Box::new(Image::new_cpu(dim, element_type)))
}

/// Create a new 1D image for GPU-based computations
///
/// # Parameters
///
/// * `dim`: dimensions of the image
/// * `element_type`: type of the elements in the image
///
/// # Returns
///
/// Allocated image.
#[no_mangle]
pub extern "C" fn txkit_image_new_gpu_1d(
    dim: ImageDim,
    element_type: ImageDataType,
    context: &crate::context::Context,
) -> *mut Image {
    crate::api::wrap_result(
        Image::new_gpu_1d(dim, element_type, context)
            .map(Box::new)
            .map(Box::into_raw),
    )
    .unwrap_or(std::ptr::null_mut())
}

/// Create a new 2D image for GPU-based computations
///
/// # Parameters
///
/// * `dim`: dimensions of the image
/// * `element_type`: type of the elements in the image
///
/// # Returns
///
/// Allocated image.
#[no_mangle]
pub extern "C" fn txkit_image_new_gpu_2d(
    dim: ImageDim,
    element_type: ImageDataType,
    context: &crate::context::Context,
) -> *mut Image {
    crate::api::wrap_result(
        Image::new_gpu_2d(dim, element_type, context)
            .map(Box::new)
            .map(Box::into_raw),
    )
    .unwrap_or(std::ptr::null_mut())
}

/// Create a new 3D image for GPU-based computations
///
/// # Parameters
///
/// * `dim`: dimensions of the image
/// * `element_type`: type of the elements in the image
///
/// # Returns
///
/// Allocated image.
#[no_mangle]
pub extern "C" fn txkit_image_new_gpu_3d(
    dim: ImageDim,
    element_type: ImageDataType,
    context: &crate::context::Context,
) -> *mut Image {
    crate::api::wrap_result(
        Image::new_gpu_3d(dim, element_type, context)
            .map(Box::new)
            .map(Box::into_raw),
    )
    .unwrap_or(std::ptr::null_mut())
}

/// Destroy an image
///
/// # Parameters
///
/// * `image`: image to destroy
#[no_mangle]
pub unsafe extern "C" fn txkit_image_destroy(image: *mut Image) {
    std::mem::drop(Box::from_raw(image))
}

/// Return the element type of the image
///
/// # Parameters
///
/// * `image`: target image
#[no_mangle]
pub extern "C" fn txkit_image_element_type(image: &Image) -> ImageDataType {
    image.element_type()
}

/// Return the dimensions of the image
///
/// # Parameters
///
/// * `image`: target image
#[no_mangle]
pub extern "C" fn txkit_image_dim(image: &Image) -> ImageDim {
    image.dim()
}

pub struct MappedImageDataReadBox {
    ptr: Box<dyn MappedImageData>,
}

/// Map the image pixels for read access. The image must be unmapped after being used.
///
/// # Parameters
///
/// * `image`: image to map for read access
#[no_mangle]
pub extern "C" fn txkit_image_map_read(image: &'static Image) -> *mut MappedImageDataReadBox {
    crate::api::wrap_result(
        image
            .data()
            .map(|bx| Box::into_raw(Box::new(MappedImageDataReadBox { ptr: bx }))),
    )
    .unwrap_or(std::ptr::null_mut())
}

/// Get a pointer to the image pixels through the given map.
///
/// # Parameters
///
/// * `read_map`: map to access
///
/// # Returns
///
/// Pointer to the pixel data, or null if the conversion failed.
#[no_mangle]
pub extern "C" fn txkit_image_map_read_data_u8(read_map: &MappedImageDataReadBox) -> *const u8 {
    read_map
        .ptr
        .as_u8_nd_array()
        .map(|ptr| ptr.as_ptr())
        .unwrap_or(std::ptr::null())
}

/// Get a pointer to the image pixels through the given map.
///
/// # Parameters
///
/// * `read_map`: map to access
///
/// # Returns
///
/// Pointer to the pixel data, or null if the conversion failed.
#[no_mangle]
pub extern "C" fn txkit_image_map_read_data_f32(read_map: &MappedImageDataReadBox) -> *const f32 {
    read_map
        .ptr
        .as_f32_nd_array()
        .map(|ptr| ptr.as_ptr())
        .unwrap_or(std::ptr::null())
}

/// Unmap a mapped image.
///
/// # Parameters
///
/// * `read_map`: mapped image object
#[no_mangle]
pub unsafe extern "C" fn txkit_image_unmap_read(read_map: *mut MappedImageDataReadBox) {
    std::mem::drop(Box::from_raw(read_map))
}

pub struct MappedImageDataWriteBox {
    ptr: Box<dyn MappedImageDataMut>,
}

/// Map the image pixels for write access. The image must be unmapped after being used.
///
/// # Parameters
///
/// * `image`: image to map for write access
#[no_mangle]
pub extern "C" fn txkit_image_map_write(image: &'static mut Image) -> *mut MappedImageDataWriteBox {
    crate::api::wrap_result(
        image
            .data_mut()
            .map(|bx| Box::into_raw(Box::new(MappedImageDataWriteBox { ptr: bx }))),
    )
    .unwrap_or(std::ptr::null_mut())
}

/// Get a pointer to the image pixels through the given map.
///
/// # Parameters
///
/// * `write_map`: map to access
///
/// # Returns
///
/// Pointer to the pixel data, or null if the conversion failed.
#[no_mangle]
pub extern "C" fn txkit_image_map_write_data_u8(
    write_map: &mut MappedImageDataWriteBox,
) -> *mut u8 {
    write_map
        .ptr
        .as_u8_nd_array_mut()
        .map(|mut ptr| ptr.as_mut_ptr())
        .unwrap_or(std::ptr::null_mut())
}

/// Get a pointer to the image pixels through the given map.
///
/// # Parameters
///
/// * `write_map`: map to access
///
/// # Returns
///
/// Pointer to the pixel data, or null if the conversion failed.
#[no_mangle]
pub extern "C" fn txkit_image_map_write_data_f32(
    write_map: &mut MappedImageDataWriteBox,
) -> *mut f32 {
    write_map
        .ptr
        .as_f32_nd_array_mut()
        .map(|mut ptr| ptr.as_mut_ptr())
        .unwrap_or(std::ptr::null_mut())
}

/// Unmap a mapped image.
///
/// # Parameters
///
/// * `write_map`: mapped image object
#[no_mangle]
pub unsafe extern "C" fn txkit_image_unmap_write(write_map: *mut MappedImageDataWriteBox) {
    std::mem::drop(Box::from_raw(write_map))
}
