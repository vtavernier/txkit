#ifndef _TXKIT_H_
#define _TXKIT_H_

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include "txkit_extra.h"

/**
 * No error occurred
 */
#define TxKit_SUCCESS 0

/**
 * Type of elements in an image
 */
enum TxKit_ImageDataType
#ifdef __cplusplus
  : uint32_t
#endif // __cplusplus
 {
    /**
     * Unsigned bytes (8 bits)
     */
    TxKit_ImageDataType_UInt8,
    /**
     * Single-precision floating point (32 bits)
     */
    TxKit_ImageDataType_Float32,
};
#ifndef __cplusplus
typedef uint32_t TxKit_ImageDataType;
#endif // __cplusplus

typedef struct TxKit_Context TxKit_Context;

typedef struct TxKit_Context TxKit_Context;

/**
 * Image that can be sent accross for FFI
 */
typedef struct TxKit_Image TxKit_Image;

typedef struct TxKit_ImageIo TxKit_ImageIo;

/**
 * Wrapped read-only mapping for FFI
 */
typedef struct TxKit_MappedImageDataRead TxKit_MappedImageDataRead;

/**
 * Wrapped read-write mapping for FFI
 */
typedef struct TxKit_MappedImageDataWrite TxKit_MappedImageDataWrite;

/**
 * Wrapped method for FFI
 */
typedef struct TxKit_Method TxKit_Method;

/**
 * Wrapped registry for FFI
 */
typedef struct TxKit_Registry TxKit_Registry;

typedef struct {
    uintptr_t width;
    uintptr_t height;
    uintptr_t depth;
    uintptr_t channels;
} TxKit_ImageDimensions_usize;

typedef TxKit_ImageDimensions_usize TxKit_ImageDim;

typedef struct {
    float alpha_value;
} TxKit_DebugParams;

/**
 * A 2-dimensional vector.
 *
 * This type is marked as `#[repr(C)]`.
 */
typedef struct {
    /**
     * The x component of the vector.
     */
    float x;
    /**
     * The y component of the vector.
     */
    float y;
} TxKit_Vector2_f32;

typedef struct {
    /**
     * pseudo-random seed
     */
    uint32_t global_seed;
    /**
     * lattice scale (size in pixels)
     */
    float scale;
    /**
     * stats mode (0: normal, 1: process, 2: lookat)
     */
    int32_t stats_mode;
    /**
     * look-at parameter (if stats_mode == lookat) in [0, 1]^2
     */
    TxKit_Vector2_f32 stats_look_at;
} TxKit_GradientNoiseParams;

typedef struct {
    /**
     * pseudo-random seed
     */
    uint32_t global_seed;
    /**
     * lattice scale (size in pixels)
     */
    float scale;
    /**
     * stats mode (0: normal, 1: process, 2: lookat)
     */
    int32_t stats_mode;
    /**
     * look-at parameter (if stats_mode == lookat) in [0, 1]^2
     */
    TxKit_Vector2_f32 stats_look_at;
    /**
     * cell lookahead: number of cells to check for contributions
     */
    int32_t noise_lookahead;
    /**
     * kernel count per cell
     */
    int32_t kernel_count;
    /**
     * noise profile function
     */
    int32_t noise_profile;
    /**
     * noise weights
     */
    int32_t noise_weights;
    /**
     * point distribution
     */
    int32_t noise_point_distribution;
    /**
     * noise frequency (in oscillations / noise cell)
     */
    float noise_frequency;
    /**
     * noise angle (in radians)
     */
    float noise_angle;
    /**
     * jittering amount, 0 = no random, 1 = full subcell random
     */
    float jitter_amount;
    /**
     * max jittering subcells, 0 = no limit
     */
    int32_t jitter_max;
    /**
     * texture inputs
     */
    TxKit_ImageIo *io;
} TxKit_PhasorNoiseParams;

typedef struct {
    /**
     * pseudo-random seed
     */
    uint32_t global_seed;
    /**
     * lattice scale (size in pixels)
     */
    float scale;
    /**
     * stats mode (0: normal, 1: process, 2: lookat)
     */
    int32_t stats_mode;
    /**
     * look-at parameter (if stats_mode == lookat) in [0, 1]^2
     */
    TxKit_Vector2_f32 stats_look_at;
} TxKit_SimplexNoiseParams;

typedef struct {
    /**
     * pseudo-random seed
     */
    uint32_t global_seed;
    /**
     * lattice scale (size in pixels)
     */
    float scale;
    /**
     * stats mode (0: normal, 1: process, 2: lookat)
     */
    int32_t stats_mode;
    /**
     * look-at parameter (if stats_mode == lookat) in [0, 1]^2
     */
    TxKit_Vector2_f32 stats_look_at;
} TxKit_ValueNoiseParams;

typedef struct {
    /**
     * pseudo-random seed
     */
    uint32_t global_seed;
} TxKit_WhiteNoiseParams;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * Destroy a context
 *
 * # Parameters
 *
 * * `ctx`: context to destroy
 */
TXKIT_API void txkit_context_destroy(TxKit_Context *ctx);

/**
 * Create a new CPU context
 *
 * # Returns
 *
 * Pointer to the created context, or null if the creation failed.
 */
TXKIT_API TxKit_Context *txkit_context_new_cpu(void);

/**
 * Create a new GPU context
 *
 * # Returns
 *
 * Pointer to the created context, or null if the creation failed.
 */
TXKIT_API TxKit_Context *txkit_context_new_gpu(void);

/**
 * Get the description of the last error that occurred in the txkit API
 *
 * # Returns
 *
 * Null pointer if no error occurred, or error message for the last error.
 */
TXKIT_API const char *txkit_get_last_error(void);

/**
 * Destroy an image
 *
 * # Parameters
 *
 * * `image`: image to destroy
 */
TXKIT_API void txkit_image_destroy(TxKit_Image *image);

/**
 * Return the dimensions of the image
 *
 * # Parameters
 *
 * * `image`: target image
 */
TXKIT_API TxKit_ImageDim txkit_image_dim(const TxKit_Image *image);

/**
 * Download the device data of an image to the host memory
 *
 * # Parameters
 *
 * * `image`: image to sync
 */
TXKIT_API int32_t txkit_image_download(TxKit_Image *image);

/**
 * Return the element type of the image
 *
 * # Parameters
 *
 * * `image`: target image
 */
TXKIT_API TxKit_ImageDataType txkit_image_element_type(const TxKit_Image *image);

/**
 * Destroy an ImageIo object
 *
 * # Parameters
 *
 * * `io`: ImageIo object to destroy
 */
TXKIT_API void txkit_image_io_destroy(TxKit_ImageIo *io);

/**
 * Create a new ImageIo object
 */
TXKIT_API TxKit_ImageIo *txkit_image_io_new(void);

/**
 * Set an image binding on an ImageIo object
 *
 * # Parameters
 *
 * * `io`: ImageIo object to change
 * * `index`: image unit index
 * * `image`: image to bind, or NULL to clear bindings
 *
 * # Returns
 *
 * TxKit_SUCCESS on success, non-zero on error
 */
TXKIT_API
int32_t txkit_image_io_set_image_binding(TxKit_ImageIo *io,
                                         uintptr_t index,
                                         TxKit_Image *image);

/**
 * Set an texture binding on an ImageIo object
 *
 * # Parameters
 *
 * * `io`: ImageIo object to change
 * * `index`: texture unit index
 * * `image`: image to bind, or NULL to clear bindings
 *
 * # Returns
 *
 * TxKit_SUCCESS on success, non-zero on error
 */
TXKIT_API
int32_t txkit_image_io_set_texture_binding(TxKit_ImageIo *io,
                                           uintptr_t index,
                                           TxKit_Image *image);

/**
 * Map the image pixels for read access. The image must be unmapped after being used.
 *
 * # Parameters
 *
 * * `image`: image to map for read access
 */
TXKIT_API TxKit_MappedImageDataRead *txkit_image_map_read(const TxKit_Image *image);

/**
 * Get a pointer to the image pixels through the given map.
 *
 * # Parameters
 *
 * * `read_map`: map to access
 *
 * # Returns
 *
 * Pointer to the pixel data, or null if the conversion failed.
 */
TXKIT_API const float *txkit_image_map_read_data_f32(const TxKit_MappedImageDataRead *read_map);

/**
 * Get a pointer to the image pixels through the given map.
 *
 * # Parameters
 *
 * * `read_map`: map to access
 *
 * # Returns
 *
 * Pointer to the pixel data, or null if the conversion failed.
 */
TXKIT_API const uint8_t *txkit_image_map_read_data_u8(const TxKit_MappedImageDataRead *read_map);

/**
 * Map the image pixels for write access. The image must be unmapped after being used.
 *
 * # Parameters
 *
 * * `image`: image to map for write access
 */
TXKIT_API TxKit_MappedImageDataWrite *txkit_image_map_write(TxKit_Image *image);

/**
 * Get a pointer to the image pixels through the given map.
 *
 * # Parameters
 *
 * * `write_map`: map to access
 *
 * # Returns
 *
 * Pointer to the pixel data, or null if the conversion failed.
 */
TXKIT_API float *txkit_image_map_write_data_f32(TxKit_MappedImageDataWrite *write_map);

/**
 * Get a pointer to the image pixels through the given map.
 *
 * # Parameters
 *
 * * `write_map`: map to access
 *
 * # Returns
 *
 * Pointer to the pixel data, or null if the conversion failed.
 */
TXKIT_API uint8_t *txkit_image_map_write_data_u8(TxKit_MappedImageDataWrite *write_map);

/**
 * Create a new image for CPU-based computations
 *
 * # Parameters
 *
 * * `dim`: dimensions of the image
 * * `element_type`: type of the elements in the image
 *
 * # Returns
 *
 * Allocated image.
 */
TXKIT_API TxKit_Image *txkit_image_new_cpu(TxKit_ImageDim dim, TxKit_ImageDataType element_type);

/**
 * Create a new 1D image for GPU-based computations
 *
 * # Parameters
 *
 * * `dim`: dimensions of the image
 * * `element_type`: type of the elements in the image
 *
 * # Returns
 *
 * Allocated image.
 */
TXKIT_API
TxKit_Image *txkit_image_new_gpu_1d(TxKit_ImageDim dim,
                                    TxKit_ImageDataType element_type,
                                    const TxKit_Context *context);

/**
 * Create a new 2D image for GPU-based computations
 *
 * # Parameters
 *
 * * `dim`: dimensions of the image
 * * `element_type`: type of the elements in the image
 *
 * # Returns
 *
 * Allocated image.
 */
TXKIT_API
TxKit_Image *txkit_image_new_gpu_2d(TxKit_ImageDim dim,
                                    TxKit_ImageDataType element_type,
                                    const TxKit_Context *context);

/**
 * Create a new 3D image for GPU-based computations
 *
 * # Parameters
 *
 * * `dim`: dimensions of the image
 * * `element_type`: type of the elements in the image
 *
 * # Returns
 *
 * Allocated image.
 */
TXKIT_API
TxKit_Image *txkit_image_new_gpu_3d(TxKit_ImageDim dim,
                                    TxKit_ImageDataType element_type,
                                    const TxKit_Context *context);

/**
 * Unmap a mapped image.
 *
 * # Parameters
 *
 * * `read_map`: mapped image object
 */
TXKIT_API void txkit_image_unmap_read(TxKit_MappedImageDataRead *read_map);

/**
 * Unmap a mapped image.
 *
 * # Parameters
 *
 * * `write_map`: mapped image object
 */
TXKIT_API void txkit_image_unmap_write(TxKit_MappedImageDataWrite *write_map);

/**
 * Upload the host data of an image to the device memory
 *
 * # Parameters
 *
 * * `image`: image to sync
 */
TXKIT_API int32_t txkit_image_upload(TxKit_Image *image);

/**
 * Compute an image using the given method
 *
 * # Parameters
 *
 * * `ctx`: context to use for computing the image
 * * `method`: texturing method
 * * `tgt`: target image to be computed
 * * `params`: pointer to the parameter structure for this method
 * * `params_size`: size of the parameter structure
 *
 * # Returns
 *
 * TxKit_SUCCESS if no error occurred, else a non-zero code.
 */
TXKIT_API
int32_t txkit_method_compute(TxKit_Context *ctx,
                             TxKit_Method *method,
                             TxKit_Image *tgt,
                             const void *params,
                             uintptr_t params_size);

/**
 * Destroy a method
 *
 * # Parameters
 *
 * * `method`: method to destroy
 */
TXKIT_API void txkit_method_destroy(TxKit_Method *method);

/**
 * Create a new method by name
 *
 * # Parameters
 *
 * * `registry`: registry of methods to build from
 * * `method_name`: name of the method to create
 *
 * # Returns
 *
 * Null pointer if an error occurred creating the method, otherwise pointer to the allocated
 * method.
 */
TXKIT_API TxKit_Method *txkit_method_new(const TxKit_Registry *registry, const char *method_name);

/**
 * Destroy a registry
 *
 * # Parameters
 *
 * * `registry`: registry to destroy
 */
TXKIT_API void txkit_registry_destroy(TxKit_Registry *registry);

/**
 * Create a new registry with txkit built-in methods registered
 *
 * # Returns
 *
 * Pointer to the allocated registry.
 */
TXKIT_API TxKit_Registry *txkit_registry_new_builtin(void);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* _TXKIT_H_ */
