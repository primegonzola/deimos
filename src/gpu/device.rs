// SPDX-License-Identifier: MIT

#![allow(dead_code)]

use anyhow::{Ok, Result};
use vulkanalia::prelude::v1_0::*;
use winit::window::Window;

use crate::gpu::structs::*;
use crate::vulkan::{self};

use super::geometry;

pub struct GPUImageCopyExternalImageSource {}

pub struct OffscreenCanvas {}

pub struct GPUBindingResource {
    pub sampler: Option<GPUSampler>,
    pub texture_view: Option<GPUTextureView>,
    pub buffer_binding: Option<GPUBufferBinding>,
    pub external_texture: Option<GPUExternalTexture>,
}
pub struct GPUBindGroupDescriptor {
    /**
     * The {@link GPUBindGroupLayout} the entries of this bind group will conform to.
     */
    pub layout: GPUBindGroupLayout,
    /**
     * A list of entries describing the resources to expose to the shader for each binding
     * described by the {@link GPUBindGroupDescriptor#layout}.
     */
    pub entries: Vec<GPUBindGroupEntry>,
}
pub struct GPUBindGroupEntry {
    /**
     * A unique identifier for a resource binding within the {@link GPUBindGroup}, corresponding to a
     * {@link GPUBindGroupLayoutEntry#binding|GPUBindGroupLayoutEntry.binding} and a @binding
     * attribute in the {@link GPUShaderModule}.
     */
    pub binding: GPUIndex32,
    /**
     * The resource to bind, which may be a {@link GPUSampler}, {@link GPUTextureView},
     * {@link GPUExternalTexture}, or {@link GPUBufferBinding}.
     */
    pub resource: GPUBindingResource,
}
pub struct GPUBindGroupLayoutDescriptor {
    pub entries: Vec<GPUBindGroupLayoutEntry>,
}
pub struct GPUBindGroupLayoutEntry {
    /**
     * A unique identifier for a resource binding within the {@link GPUBindGroupLayout}, corresponding
     * to a {@link GPUBindGroupEntry#binding|GPUBindGroupEntry.binding} and a @binding
     * attribute in the {@link GPUShaderModule}.
     */
    pub binding: GPUIndex32,
    /**
     * A bitset of the members of {@link GPUShaderStage}.
     * Each set bit indicates that a {@link GPUBindGroupLayoutEntry}'s resource
     * will be accessible from the associated shader stage.
     */
    pub visibility: GPUShaderStageFlags,
    /**
     * When map/exist|provided, indicates the binding resource type for this {@link GPUBindGroupLayoutEntry}
     * is {@link GPUBufferBinding}.
     */
    pub buffer: Option<GPUBufferBindingLayout>,
    /**
     * When map/exist|provided, indicates the binding resource type for this {@link GPUBindGroupLayoutEntry}
     * is {@link GPUSampler}.
     */
    pub sampler: Option<GPUSamplerBindingLayout>,
    /**
     * When map/exist|provided, indicates the binding resource type for this {@link GPUBindGroupLayoutEntry}
     * is {@link GPUTextureView}.
     */
    pub texture: Option<GPUTextureBindingLayout>,
    /**
     * When map/exist|provided, indicates the binding resource type for this {@link GPUBindGroupLayoutEntry}
     * is {@link GPUTextureView}.
     */
    pub storage_texture: Option<GPUStorageTextureBindingLayout>,
    /**
     * When map/exist|provided, indicates the binding resource type for this {@link GPUBindGroupLayoutEntry}
     * is {@link GPUExternalTexture}.
     */
    pub external_texture: Option<GPUExternalTextureBindingLayout>,
}
pub struct GPUBlendComponent {
    /**
     * Defines the {@link GPUBlendOperation} used to calculate the values written to the target
     * attachment components.
     */
    pub operation: Option<GPUBlendOperation>,
    /**
     * Defines the {@link GPUBlendFactor} operation to be performed on values from the fragment shader.
     */
    pub src_factor: Option<GPUBlendFactor>,
    /**
     * Defines the {@link GPUBlendFactor} operation to be performed on values from the target attachment.
     */
    pub dst_factor: Option<GPUBlendFactor>,
}
pub struct GPUBlendState {
    /**
     * Defines the blending behavior of the corresponding render target for color channels.
     */
    pub color: GPUBlendComponent,
    /**
     * Defines the blending behavior of the corresponding render target for the alpha channel.
     */
    pub alpha: GPUBlendComponent,
}
pub struct GPUBufferBinding {
    /**
     * The {@link GPUBuffer} to bind.
     */
    pub buffer: GPUBuffer,
    /**
     * The offset, in bytes, from the beginning of {@link GPUBufferBinding#buffer} to the
     * beginning of the range exposed to the shader by the buffer binding.
     */
    pub offset: Option<GPUSize64>,
    /**
     * The size, in bytes, of the buffer binding.
     * If not map/exist|provided, specifies the range starting at
     * {@link GPUBufferBinding#offset} and ending at the end of {@link GPUBufferBinding#buffer}.
     */
    pub size: Option<GPUSize64>,
}
pub struct GPUBufferBindingLayout {
    /**
     * Indicates the type required for buffers bound to this bindings.
     */
    pub r#type: Option<GPUBufferBindingType>,
    /**
     * Indicates whether this binding requires a dynamic offset.
     */
    pub has_dynamic_offset: Option<bool>,
    /**
     * Indicates the minimum {@link GPUBufferBinding#size} of a buffer binding used with this bind point.
     * Bindings are always validated against this size in {@link GPUDevice#createBindGroup}.
     * If this *is not* `0`, pipeline creation additionally [$validating shader binding|validates$]
     * that this value &ge; the minimum buffer binding size of the variable.
     * If this *is* `0`, it is ignored by pipeline creation, and instead draw/dispatch commands
     * [$Validate encoder bind groups|validate$] that each binding in the {@link GPUBindGroup}
     * satisfies the minimum buffer binding size of the variable.
     * Note:
     * Similar execution-time validation is theoretically possible for other
     * binding-related fields specified for early validation, like
     * {@link GPUTextureBindingLayout#sampleType} and {@link GPUStorageTextureBindingLayout#format},
     * which currently can only be validated in pipeline creation.
     * However, such execution-time validation could be costly or unnecessarily complex, so it is
     * available only for {@link GPUBufferBindingLayout#minBindingSize} which is expected to have the
     * most ergonomic impact.
     */
    pub min_binding_size: Option<GPUSize64>,
}
pub struct GPUBufferDescriptor {
    /**
     * The size of the buffer in bytes.
     */
    pub size: GPUSize64,
    /**
     * The allowed usages for the buffer.
     */
    pub usage: GPUBufferUsageFlags,
    /**
     * If `true` creates the buffer in an already mapped state, allowing
     * {@link GPUBuffer#getMappedRange} to be called immediately. It is valid to set
     * {@link GPUBufferDescriptor#mappedAtCreation} to `true` even if {@link GPUBufferDescriptor#usage}
     * does not contain {@link GPUBufferUsage#MAP_READ} or {@link GPUBufferUsage#MAP_WRITE}. This can be
     * used to set the buffer's initial data.
     * Guarantees that even if the buffer creation eventually fails, it will still appear as if the
     * mapped range can be written/read to until it is unmapped.
     */
    pub mapped_at_creation: Option<bool>,
}
pub struct GPUCanvasConfiguration {
    /**
     * The {@link GPUDevice} that textures returned by {@link GPUCanvasContext#getCurrentTexture} will be
     * compatible with.
     */
    pub device: GPUDevice,
    /**
     * The format that textures returned by {@link GPUCanvasContext#getCurrentTexture} will have.
     * Must be one of the Supported context formats.
     */
    pub format: GPUTextureFormat,
    /**
     * The usage that textures returned by {@link GPUCanvasContext#getCurrentTexture} will have.
     * {@link GPUTextureUsage#RENDER_ATTACHMENT} is the default, but is not automatically included
     * if the usage is explicitly set. Be sure to include {@link GPUTextureUsage#RENDER_ATTACHMENT}
     * when setting a custom usage if you wish to use textures returned by
     * {@link GPUCanvasContext#getCurrentTexture} as color targets for a render pass.
     */
    pub usage: Option<GPUTextureUsageFlags>,
    /**
     * The formats that views created from textures returned by
     * {@link GPUCanvasContext#getCurrentTexture} may use.
     */
    pub view_formats: Option<Vec<GPUTextureFormat>>,
    /**
     * The color space that values written into textures returned by
     * {@link GPUCanvasContext#getCurrentTexture} should be displayed with.
     */
    pub color_space: Option<PredefinedColorSpace>,
    /**
     * Determines the effect that alpha values will have on the content of textures returned by
     * {@link GPUCanvasContext#getCurrentTexture} when read, displayed, or used as an image source.
     */
    pub alpha_mode: Option<GPUCanvasAlphaMode>,
}
pub struct GPUColor {
    /**
     * The red channel value.
     */
    pub r: f32,
    /**
     * The green channel value.
     */
    pub g: f32,
    /**
     * The blue channel value.
     */
    pub b: f32,
    /**
     * The alpha channel value.
     */
    pub a: f32,
}

impl GPUColor {
    pub fn from_vec(data: &Vec<f32>) -> GPUColor {
        GPUColor {
            r: data[0],
            g: data[1],
            b: data[2],
            a: data[3],
        }
    }

    pub fn to_vect(&self) -> Vec<f32> {
        vec![self.r, self.g, self.b, self.a]
    }
}
pub struct GPUColorTargetState {
    /**
     * The {@link GPUTextureFormat} of this color target. The pipeline will only be compatible with
     * {@link GPURenderPassEncoder}s which use a {@link GPUTextureView} of this format in the
     * corresponding color attachment.
     */
    pub format: GPUTextureFormat,
    /**
     * The blending behavior for this color target. If left undefined, disables blending for this
     * color target.
     */
    pub blend: Option<GPUBlendState>,
    /**
     * Bitmask controlling which channels are are written to when drawing to this color target.
     */
    pub write_mask: Option<GPUColorWriteFlags>,
}

pub type GPUCommandBufferDescriptor = GPUObjectDescriptorBase;
pub type GPUCommandEncoderDescriptor = GPUObjectDescriptorBase;
pub struct GPUComputePassDescriptor {
    /**
     * Defines which timestamp values will be written for this pass, and where to write them to.
     */
    pub timestamp_writes: Option<GPUComputePassTimestampWrites>,
}
pub struct GPUComputePassTimestampWrites {
    /**
     * The {@link GPUQuerySet}, of type {@link GPUQueryType#"timestamp"}, that the query results will be
     * written to.
     */
    pub query_set: GPUQuerySet,
    /**
     * If defined, indicates the query index in {@link GPURenderPassTimestampWrites#querySet} into
     * which the timestamp at the beginning of the compute pass will be written.
     */
    pub beginning_of_pass_write_index: Option<GPUSize32>,
    /**
     * If defined, indicates the query index in {@link GPURenderPassTimestampWrites#querySet} into
     * which the timestamp at the end of the compute pass will be written.
     */
    pub end_of_pass_write_index: Option<GPUSize32>,
}
pub struct GPUComputePipelineDescriptor {
    /**
     * Describes the compute shader entry point of the pipeline.
     */
    pub compute: GPUProgrammableStage,
}
pub struct GPUDepthStencilState {
    /**
     * The {@link GPUTextureViewDescriptor#format} of {@link GPURenderPassDescriptor#depthStencilAttachment}
     * this {@link GPURenderPipeline} will be compatible with.
     */
    pub format: GPUTextureFormat,
    /**
     * Indicates if this {@link GPURenderPipeline} can modify
     * {@link GPURenderPassDescriptor#depthStencilAttachment} depth values.
     */
    pub depth_write_enabled: bool,
    /**
     * The comparison operation used to test fragment depths against
     * {@link GPURenderPassDescriptor#depthStencilAttachment} depth values.
     */
    pub depth_compare: GPUCompareFunction,
    /**
     * Defines how stencil comparisons and operations are performed for front-facing primitives.
     */
    pub stencil_front: Option<GPUStencilFaceState>,
    /**
     * Defines how stencil comparisons and operations are performed for back-facing primitives.
     */
    pub stencil_back: Option<GPUStencilFaceState>,
    /**
     * Bitmask controlling which {@link GPURenderPassDescriptor#depthStencilAttachment} stencil value
     * bits are read when performing stencil comparison tests.
     */
    pub stencil_read_mask: Option<GPUStencilValue>,
    /**
     * Bitmask controlling which {@link GPURenderPassDescriptor#depthStencilAttachment} stencil value
     * bits are written to when performing stencil operations.
     */
    pub stencil_write_mask: Option<GPUStencilValue>,
    /**
     * Constant depth bias added to each fragment. See [$biased fragment depth$] for details.
     */
    pub depth_bias: Option<GPUDepthBias>,
    /**
     * Depth bias that scales with the fragment’s slope. See [$biased fragment depth$] for details.
     */
    pub depth_bias_slope_scale: Option<f32>,
    /**
     * The maximum depth bias of a fragment. See [$biased fragment depth$] for details.
     */
    pub depth_bias_clamp: Option<f32>,
}
pub struct GPUDeviceDescriptor {
    /**
     * Specifies the features that are required by the device request.
     * The request will fail if the adapter cannot provide these features.
     * Exactly the specified set of features, and no more or less, will be allowed in validation
     * of API calls on the resulting device.
     */
    pub required_features: Option<Vec<GPUFeatureName>>,
    /**
     * Specifies the limits that are required by the device request.
     * The request will fail if the adapter cannot provide these limits.
     * Each key must be the name of a member of supported limits.
     * Exactly the specified limits, and no limit/better or worse,
     * will be allowed in validation of API calls on the resulting device.
     * <!-- If we ever need limit types other than GPUSize32/GPUSize64, we can change the value
     * type to `double` or `any` in the future and write out the type conversion explicitly (by
     * reference to WebIDL spec). Or change the entire type to `any` and add back a `dictionary
     * GPULimits` and define the conversion of the whole object by reference to WebIDL. -->
     */
    // requiredLimits?: Record<
    //   string,
    //   GPUSize64
    // >;
    /**
     * The descriptor for the default {@link GPUQueue}.
     */
    pub default_queue: Option<GPUQueueDescriptor>,
}
pub struct GPUExtent3D {
    /**
     * The width of the extent.
     */
    pub width: GPUIntegerCoordinate,
    /**
     * The height of the extent.
     */
    pub height: Option<GPUIntegerCoordinate>,
    /**
     * The depth of the extent or the number of array layers it contains.
     * If used with a {@link GPUTexture} with a {@link GPUTextureDimension} of {@link GPUTextureDimension#"3d"}
     * defines the depth of the texture. If used with a {@link GPUTexture} with a {@link GPUTextureDimension}
     * of {@link GPUTextureDimension#"2d"} defines the number of array layers in the texture.
     */
    pub depth_or_array_layers: Option<GPUIntegerCoordinate>,
}
pub struct GPUExternalTextureBindingLayout {}
pub struct GPUExternalTextureDescriptor {
    // source:
    //   | HTMLVideoElement
    //   | VideoFrame;
    pub color_space: Option<PredefinedColorSpace>,
}
pub struct GPUFragmentState {
    /**
     * The {@link GPUShaderModule} containing the code that this programmable stage will execute.
     */
    pub module: GPUShaderModule,
    /**
     * The name of the function in {@link GPUProgrammableStage#module} that this stage will use to
     * perform its work.
     */
    pub entry_point: String,
    /**
     * Specifies the values of pipeline-overridable constants in the shader module
     * {@link GPUProgrammableStage#module}.
     * Each such pipeline-overridable constant is uniquely identified by a single
     * pipeline-overridable constant identifier string (representing the numeric ID of the
     * constant, if one is specified, and otherwise the constant's identifier name).
     * WGSL names (identifiers) in source maps follow the rules defined in WGSL identifier comparison.
     * The key of each key-value pair must equal the identifier string of one such constant.
     * When the pipeline is executed, that constant will have the specified value.
     * Values are specified as <dfn typedef for="">GPUPipelineConstantValue</dfn>, which is a {@link double}.
     * They are converted [$to WGSL type$] of the pipeline-overridable constant (`bool`/`i32`/`u32`/`f32`/`f16`).
     * If conversion fails, a validation error is generated.
     */
    pub constants: Option<std::collections::HashMap<String, GPUPipelineConstantValue>>,
    /**
     * A list of {@link GPUColorTargetState} defining the formats and behaviors of the color targets
     * this pipeline writes to.
     */
    pub targets: Option<Vec<GPUColorTargetState>>,
}
pub struct GPUImageCopyBuffer {
    /**
     * The offset, in bytes, from the beginning of the image data source (such as a
     * {@link GPUImageCopyBuffer#buffer|GPUImageCopyBuffer.buffer}) to the start of the image data
     * within that source.
     */
    pub offset: Option<GPUSize64>,
    /*
     * The stride, in bytes, between the beginning of each block row and the subsequent block row.
     * Required if there are multiple block rows (i.e. the copy height or depth is more than one block).
     */
    pub bytes_per_row: Option<GPUSize32>,
    /**
     * Number of block rows per single image of the texture.
     * {@link GPUImageDataLayout#rowsPerImage} &times;
     * {@link GPUImageDataLayout#bytesPerRow} is the stride, in bytes, between the beginning of each image of data and the subsequent image.
     * Required if there are multiple images (i.e. the copy depth is more than one).
     */
    pub rows_per_image: Option<GPUSize32>,
    /**
     * A buffer which either contains image data to be copied or will store the image data being
     * copied, depending on the method it is being passed to.
     */
    pub buffer: GPUBuffer,
}
pub struct GPUImageCopyExternalImage {
    /**
     * The source of the image copy. The copy source data is captured at the moment that
     * {@link GPUQueue#copyExternalImageToTexture} is issued. Source size is defined by source
     * type, given by this table:
     *
     * <table class=data>
     * <thead>
     * <tr>
     * <th>Source type
     * <th>Width
     * <th>Height
     * </thead>
     * <tbody>
     * <tr>
     * <td>{@link ImageBitmap}
     * <td>{@link ImageBitmap#width|ImageBitmap.width}
     * <td>{@link ImageBitmap#height|ImageBitmap.height}
     * <tr>
     * <td>{@link HTMLVideoElement}
     * <td>video/intrinsic width|intrinsic width of the frame
     * <td>video/intrinsic height|intrinsic height of the frame
     * <tr>
     * <td>{@link VideoFrame}
     * <td>{@link VideoFrame#codedWidth|VideoFrame.codedWidth}
     * <td>{@link VideoFrame#codedHeight|VideoFrame.codedHeight}
     * <tr>
     * <td>{@link HTMLCanvasElement}
     * <td>{@link HTMLCanvasElement#width|HTMLCanvasElement.width}
     * <td>{@link HTMLCanvasElement#height|HTMLCanvasElement.height}
     * <tr>
     * <td>{@link OffscreenCanvas}
     * <td>{@link OffscreenCanvas#width|OffscreenCanvas.width}
     * <td>{@link OffscreenCanvas#height|OffscreenCanvas.height}
     * </tbody>
     * </table>
     */
    pub source: GPUImageCopyExternalImageSource,
    /**
     * Defines the origin of the copy - the minimum (top-left) corner of the source sub-region to copy from.
     * Together with `copySize`, defines the full copy sub-region.
     */
    pub origin: Option<GPUOrigin2D>,
    /**
     * Describes whether the source image is vertically flipped, or not.
     * If this option is set to `true`, the copy is flipped vertically: the bottom row of the source
     * region is copied into the first row of the destination region, and so on.
     * The {@link GPUImageCopyExternalImage#origin} option is still relative to the top-left corner
     * of the source image, increasing downward.
     */
    pub flip_y: Option<bool>,
}
pub struct GPUImageCopyTexture {
    /**
     * Texture to copy to/from.
     */
    pub texture: GPUTexture,
    /**
     * Mip-map level of the {@link GPUImageCopyTexture#texture} to copy to/from.
     */
    pub mip_level: Option<GPUIntegerCoordinate>,
    /**
     * Defines the origin of the copy - the minimum corner of the texture sub-region to copy to/from.
     * Together with `copySize`, defines the full copy sub-region.
     */
    pub origin: Option<GPUOrigin3D>,
    /**
     * Defines which aspects of the {@link GPUImageCopyTexture#texture} to copy to/from.
     */
    pub aspect: Option<GPUTextureAspect>,
}
pub struct GPUImageCopyTextureTagged {
    /**
     * Describes the color space and encoding used to encode data into the destination texture.
     * This [[#color-space-conversions|may result]] in values outside of the range [0, 1]
     * being written to the target texture, if its format can represent them.
     * Otherwise, the results are clamped to the target texture format's range.
     * Note:
     * If {@link GPUImageCopyTextureTagged#colorSpace} matches the source image,
     * conversion may not be necessary. See [[#color-space-conversion-elision]].
     */
    pub color_space: Option<PredefinedColorSpace>,
    /**
     * Describes whether the data written into the texture should have its RGB channels
     * premultiplied by the alpha channel, or not.
     * If this option is set to `true` and the {@link GPUImageCopyExternalImage#source} is also
     * premultiplied, the source RGB values must be preserved even if they exceed their
     * corresponding alpha values.
     * Note:
     * If {@link GPUImageCopyTextureTagged#premultipliedAlpha} matches the source image,
     * conversion may not be necessary. See [[#color-space-conversion-elision]].
     */
    pub premultiplied_alpha: Option<bool>,
}
pub struct GPUImageDataLayout {
    /**
     * The offset, in bytes, from the beginning of the image data source (such as a
     * {@link GPUImageCopyBuffer#buffer|GPUImageCopyBuffer.buffer}) to the start of the image data
     * within that source.
     */
    pub offset: Option<GPUSize64>,
    /*
     * The stride, in bytes, between the beginning of each block row and the subsequent block row.
     * Required if there are multiple block rows (i.e. the copy height or depth is more than one block).
     */
    pub bytes_per_row: Option<GPUSize32>,
    /**
     * Number of block rows per single image of the texture.
     * {@link GPUImageDataLayout#rowsPerImage} &times;
     * {@link GPUImageDataLayout#bytesPerRow} is the stride, in bytes, between the beginning of each image of data and the subsequent image.
     * Required if there are multiple images (i.e. the copy depth is more than one).
     */
    pub rows_per_image: Option<GPUSize32>,
}
pub struct GPUMultisampleState {
    /**
     * Number of samples per pixel. This {@link GPURenderPipeline} will be compatible only
     * with attachment textures ({@link GPURenderPassDescriptor#colorAttachments}
     * and {@link GPURenderPassDescriptor#depthStencilAttachment})
     * with matching {@link GPUTextureDescriptor#sampleCount}s.
     */
    pub count: Option<GPUSize32>,
    /**
     * Mask determining which samples are written to.
     */
    pub mask: Option<GPUSampleMask>,
    /**
     * When `true` indicates that a fragment's alpha channel should be used to generate a sample
     * coverage mask.
     */
    pub alpha_to_coverage_enabled: Option<bool>,
}
pub struct GPUObjectDescriptorBase {}
pub struct GPUOrigin2D {
    pub x: Option<GPUIntegerCoordinate>,
    pub y: Option<GPUIntegerCoordinate>,
}
pub struct GPUOrigin3D {
    pub x: Option<GPUIntegerCoordinate>,
    pub y: Option<GPUIntegerCoordinate>,
    pub z: Option<GPUIntegerCoordinate>,
}
pub struct GPUPipelineDescriptorBase {
    // /**
    //  * The {@link GPUPipelineLayout} for this pipeline, or {@link GPUAutoLayoutMode#"auto"} to generate
    //  * the pipeline layout automatically.
    //  * Note: If {@link GPUAutoLayoutMode#"auto"} is used the pipeline cannot share {@link GPUBindGroup}s
    //  * with any other pipelines.
    //  */
    // layout:
    //   | GPUPipelineLayout
    //   | GPUAutoLayoutMode;
}
pub struct GPUPipelineErrorInit {
    pub reason: GPUPipelineErrorReason,
}
pub struct GPUPipelineLayoutDescriptor {
    /**
     * A list of {@link GPUBindGroupLayout}s the pipeline will use. Each element corresponds to a
     * @group attribute in the {@link GPUShaderModule}, with the `N`th element corresponding with
     * `@group(N)`.
     */
    pub bind_group_layouts: Vec<GPUBindGroupLayout>,
}
pub struct GPUPrimitiveState {
    /**
     * The type of primitive to be constructed from the vertex inputs.
     */
    pub topology: Option<GPUPrimitiveTopology>,
    /**
     * For pipelines with strip topologies
     * ({@link GPUPrimitiveTopology#"line-strip"} or {@link GPUPrimitiveTopology#"triangle-strip"}),
     * this determines the index buffer format and primitive restart value
     * ({@link GPUIndexFormat#"uint16"}/`0xFFFF` or {@link GPUIndexFormat#"uint32"}/`0xFFFFFFFF`).
     * It is not allowed on pipelines with non-strip topologies.
     * Note: Some implementations require knowledge of the primitive restart value to compile
     * pipeline state objects.
     * To use a strip-topology pipeline with an indexed draw call
     * ({@link GPURenderCommandsMixin#drawIndexed()} or {@link GPURenderCommandsMixin#drawIndexedIndirect}),
     * this must be set, and it must match the index buffer format used with the draw call
     * (set in {@link GPURenderCommandsMixin#setIndexBuffer}).
     * See [[#primitive-assembly]] for additional details.
     */
    pub strip_index_format: Option<GPUIndexFormat>,
    /**
     * Defines which polygons are considered front-facing.
     */
    pub front_face: Option<GPUFrontFace>,
    /**
     * Defines which polygon orientation will be culled, if any.
     */
    pub cull_mode: Option<GPUCullMode>,
    /**
     * If true, indicates that depth clipping is disabled.
     * Requires the {@link GPUFeatureName#"depth-clip-control"} feature to be enabled.
     */
    pub unclipped_depth: Option<bool>,
}
pub struct GPUProgrammableStage {
    /**
     * The {@link GPUShaderModule} containing the code that this programmable stage will execute.
     */
    pub module: GPUShaderModule,
    /**
     * The name of the function in {@link GPUProgrammableStage#module} that this stage will use to
     * perform its work.
     */
    pub entry_point: String,
    // /**
    //  * Specifies the values of pipeline-overridable constants in the shader module
    //  * {@link GPUProgrammableStage#module}.
    //  * Each such pipeline-overridable constant is uniquely identified by a single
    //  * pipeline-overridable constant identifier string (representing the numeric ID of the
    //  * constant, if one is specified, and otherwise the constant's identifier name).
    //  * WGSL names (identifiers) in source maps follow the rules defined in WGSL identifier comparison.
    //  * The key of each key-value pair must equal the identifier string of one such constant.
    //  * When the pipeline is executed, that constant will have the specified value.
    //  * Values are specified as <dfn typedef for="">GPUPipelineConstantValue</dfn>, which is a {@link double}.
    //  * They are converted [$to WGSL type$] of the pipeline-overridable constant (`bool`/`i32`/`u32`/`f32`/`f16`).
    //  * If conversion fails, a validation error is generated.
    //  */
    // constants?: Record<
    //   string,
    //   GPUPipelineConstantValue
    // >;
}
pub struct GPUQuerySetDescriptor {
    /**
     * The type of queries managed by {@link GPUQuerySet}.
     */
    pub r#type: GPUQueryType,
    /**
     * The number of queries managed by {@link GPUQuerySet}.
     */
    pub count: GPUSize32,
}

type GPUQueueDescriptor = GPUObjectDescriptorBase;
type GPURenderBundleDescriptor = GPUObjectDescriptorBase;
pub struct GPURenderBundleEncoderDescriptor {
    /**
     * If `true`, indicates that the render bundle does not modify the depth component of the
     * {@link GPURenderPassDepthStencilAttachment} of any render pass the render bundle is executed
     * in.
     */
    pub depth_read_only: Option<bool>,
    /**
     * If `true`, indicates that the render bundle does not modify the stencil component of the
     * {@link GPURenderPassDepthStencilAttachment} of any render pass the render bundle is executed
     * in.
     */
    pub stencil_read_only: Option<bool>,
}
pub struct GPURenderPassColorAttachment {
    /**
     * A {@link GPUTextureView} describing the texture subresource that will be output to for this
     * color attachment.
     */
    pub view: GPUTextureView,
    /**
     * A {@link GPUTextureView} describing the texture subresource that will receive the resolved
     * output for this color attachment if {@link GPURenderPassColorAttachment#view} is
     * multisampled.
     */
    pub resolve_target: Option<GPUTextureView>,
    /**
     * Indicates the value to clear {@link GPURenderPassColorAttachment#view} to prior to executing the
     * render pass. If not map/exist|provided, defaults to `{r: 0, g: 0, b: 0, a: 0}`. Ignored
     * if {@link GPURenderPassColorAttachment#loadOp} is not {@link GPULoadOp#"clear"}.
     * The components of {@link GPURenderPassColorAttachment#clearValue} are all double values.
     * They are converted [$to a texel value of texture format$] matching the render attachment.
     * If conversion fails, a validation error is generated.
     */
    pub clear_value: Option<GPUColor>,
    /**
     * Indicates the load operation to perform on {@link GPURenderPassColorAttachment#view} prior to
     * executing the render pass.
     * Note: It is recommended to prefer clearing; see {@link GPULoadOp#"clear"} for details.
     */
    pub load_op: GPULoadOp,
    /**
     * The store operation to perform on {@link GPURenderPassColorAttachment#view}
     * after executing the render pass.
     */
    pub store_op: GPUStoreOp,
}
pub struct GPURenderPassDepthStencilAttachment {
    /**
     * A {@link GPUTextureView} describing the texture subresource that will be output to
     * and read from for this depth/stencil attachment.
     */
    pub view: GPUTextureView,
    /**
     * Indicates the value to clear {@link GPURenderPassDepthStencilAttachment#view}'s depth component
     * to prior to executing the render pass. Ignored if {@link GPURenderPassDepthStencilAttachment#depthLoadOp}
     * is not {@link GPULoadOp#"clear"}. Must be between 0.0 and 1.0, inclusive.
     * <!-- POSTV1(unrestricted-depth): unless unrestricted depth is enabled -->
     */
    pub depth_clear_value: Option<f32>,
    /**
     * Indicates the load operation to perform on {@link GPURenderPassDepthStencilAttachment#view}'s
     * depth component prior to executing the render pass.
     * Note: It is recommended to prefer clearing; see {@link GPULoadOp#"clear"} for details.
     */
    pub depth_load_op: Option<GPULoadOp>,
    /**
     * The store operation to perform on {@link GPURenderPassDepthStencilAttachment#view}'s
     * depth component after executing the render pass.
     */
    pub depth_store_op: Option<GPUStoreOp>,
    /**
     * Indicates that the depth component of {@link GPURenderPassDepthStencilAttachment#view}
     * is read only.
     */
    pub depth_read_only: Option<bool>,
    /**
     * Indicates the value to clear {@link GPURenderPassDepthStencilAttachment#view}'s stencil component
     * to prior to executing the render pass. Ignored if {@link GPURenderPassDepthStencilAttachment#stencilLoadOp}
     * is not {@link GPULoadOp#"clear"}.
     * The value will be converted to the type of the stencil aspect of `view` by taking the same
     * number of LSBs as the number of bits in the stencil aspect of one texel block of `view`.
     */
    pub stencil_clear_value: Option<GPUStencilValue>,
    /**
     * Indicates the load operation to perform on {@link GPURenderPassDepthStencilAttachment#view}'s
     * stencil component prior to executing the render pass.
     * Note: It is recommended to prefer clearing; see {@link GPULoadOp#"clear"} for details.
     */
    pub stencil_load_op: Option<GPULoadOp>,
    /**
     * The store operation to perform on {@link GPURenderPassDepthStencilAttachment#view}'s
     * stencil component after executing the render pass.
     */
    pub stencil_store_op: Option<GPUStoreOp>,
    /**
     * Indicates that the stencil component of {@link GPURenderPassDepthStencilAttachment#view}
     * is read only.
     */
    pub stencil_read_only: Option<bool>,
}
pub struct GPURenderPassDescriptor {
    /**
     * The set of {@link GPURenderPassColorAttachment} values in this sequence defines which
     * color attachments will be output to when executing this render pass.
     * Due to compatible usage list|usage compatibility, no color attachment
     * may alias another attachment or any resource used inside the render pass.
     */
    pub color_attachments: Option<Vec<GPURenderPassColorAttachment>>,
    /**
     * The {@link GPURenderPassDepthStencilAttachment} value that defines the depth/stencil
     * attachment that will be output to and tested against when executing this render pass.
     * Due to compatible usage list|usage compatibility, no writable depth/stencil attachment
     * may alias another attachment or any resource used inside the render pass.
     */
    pub depth_stencil_attachment: Option<GPURenderPassDepthStencilAttachment>,
    /**
     * The {@link GPUQuerySet} value defines where the occlusion query results will be stored for this pass.
     */
    pub occlusion_query_set: Option<GPUQuerySet>,
    /**
     * Defines which timestamp values will be written for this pass, and where to write them to.
     */
    pub timestamp_writes: Option<GPURenderPassTimestampWrites>,
    /**
     * The maximum number of draw calls that will be done in the render pass. Used by some
     * implementations to size work injected before the render pass. Keeping the default value
     * is a good default, unless it is known that more draw calls will be done.
     */
    pub max_draw_count: Option<GPUSize64>,
}
pub struct GPURenderPassLayout {
    /**
     * A list of the {@link GPUTextureFormat}s of the color attachments for this pass or bundle.
     */
    pub color_formats: Option<Vec<GPUTextureFormat>>,
    /**
     * The {@link GPUTextureFormat} of the depth/stencil attachment for this pass or bundle.
     */
    pub depth_stencil_format: Option<GPUTextureFormat>,
    /**
     * Number of samples per pixel in the attachments for this pass or bundle.
     */
    pub sample_count: Option<GPUSize32>,
}
pub struct GPURenderPassTimestampWrites {
    /**
     * The {@link GPUQuerySet}, of type {@link GPUQueryType#"timestamp"}, that the query results will be
     * written to.
     */
    pub query_set: GPUQuerySet,
    /**
     * If defined, indicates the query index in {@link GPURenderPassTimestampWrites#querySet} into
     * which the timestamp at the beginning of the render pass will be written.
     */
    pub beginning_of_pass_write_index: Option<GPUSize32>,
    /**
     * If defined, indicates the query index in {@link GPURenderPassTimestampWrites#querySet} into
     * which the timestamp at the end of the render pass will be written.
     */
    pub end_of_pass_write_index: Option<GPUSize32>,
}
pub struct GPURenderPipelineDescriptor {
    pub layout: Option<GPUPipelineLayout>,
    /**
     * Describes the vertex shader entry point of the pipeline and its input buffer layouts.
     */
    pub vertex: GPUVertexState,
    /**
     * Describes the primitive-related properties of the pipeline.
     */
    pub primitive: Option<GPUPrimitiveState>,
    /**
     * Describes the optional depth-stencil properties, including the testing, operations, and bias.
     */
    pub depth_stencil: Option<GPUDepthStencilState>,
    /**
     * Describes the multi-sampling properties of the pipeline.
     */
    pub multisample: Option<GPUMultisampleState>,
    /**
     * Describes the fragment shader entry point of the pipeline and its output colors. If
     * not map/exist|provided, the [[#no-color-output]] mode is enabled.
     */
    pub fragment: Option<GPUFragmentState>,
}
pub struct GPURequestAdapterOptions {
    /**
     * Optionally provides a hint indicating what class of adapter should be selected from
     * the system's available adapters.
     * The value of this hint may influence which adapter is chosen, but it must not
     * influence whether an adapter is returned or not.
     * Note:
     * The primary utility of this hint is to influence which GPU is used in a multi-GPU system.
     * For instance, some laptops have a low-power integrated GPU and a high-performance
     * discrete GPU. This hint may also affect the power configuration of the selected GPU to
     * match the requested power preference.
     * Note:
     * Depending on the exact hardware configuration, such as battery status and attached displays
     * or removable GPUs, the user agent may select different adapters given the same power
     * preference.
     * Typically, given the same hardware configuration and state and
     * `powerPreference`, the user agent is likely to select the same adapter.
     */
    pub power_preference: Option<GPUPowerPreference>,
    /**
     * When set to `true` indicates that only a fallback adapter may be returned. If the user
     * agent does not support a fallback adapter, will cause {@link GPU#requestAdapter} to
     * resolve to `null`.
     * Note:
     * {@link GPU#requestAdapter} may still return a fallback adapter if
     * {@link GPURequestAdapterOptions#forceFallbackAdapter} is set to `false` and either no
     * other appropriate adapter is available or the user agent chooses to return a
     * fallback adapter. Developers that wish to prevent their applications from running on
     * fallback adapters should check the {@link GPUAdapter}.{@link GPUAdapter#isFallbackAdapter}
     * attribute prior to requesting a {@link GPUDevice}.
     */
    pub force_fallback_adapter: Option<bool>,
}
pub struct GPUSamplerBindingLayout {
    /**
     * Indicates the required type of a sampler bound to this bindings.
     */
    pub r#type: Option<GPUSamplerBindingType>,
}
pub struct GPUSamplerDescriptor {
    /**
     */
    pub address_mode_u: Option<GPUAddressMode>,
    /**
     */
    pub address_mode_v: Option<GPUAddressMode>,
    /**
     * Specifies the {{GPUAddressMode|address modes}} for the texture width, height, and depth
     * coordinates, respectively.
     */
    pub address_mode_w: Option<GPUAddressMode>,
    /**
     * Specifies the sampling behavior when the sample footprint is smaller than or equal to one
     * texel.
     */
    pub mag_filter: Option<GPUFilterMode>,
    /**
     * Specifies the sampling behavior when the sample footprint is larger than one texel.
     */
    pub min_filter: Option<GPUFilterMode>,
    /**
     * Specifies behavior for sampling between mipmap levels.
     */
    pub mipmap_filter: Option<GPUMipmapFilterMode>,
    /**
     */
    pub lod_min_clamp: Option<f32>,
    /**
     * Specifies the minimum and maximum levels of detail, respectively, used internally when
     * sampling a texture.
     */
    pub lod_max_clamp: Option<f32>,
    /**
     * When provided the sampler will be a comparison sampler with the specified
     * {@link GPUCompareFunction}.
     * Note: Comparison samplers may use filtering, but the sampling results will be
     * implementation-dependent and may differ from the normal filtering rules.
     */
    pub compare: Option<GPUCompareFunction>,
    /**
     * Specifies the maximum anisotropy value clamp used by the sampler.
     * Note: Most implementations support {@link GPUSamplerDescriptor#maxAnisotropy} values in range
     * between 1 and 16, inclusive. The used value of {@link GPUSamplerDescriptor#maxAnisotropy} will
     * be clamped to the maximum value that the platform supports.
     */
    pub max_anisotropy: Option<f32>,
}
pub struct GPUShaderModuleCompilationHint {
    // /**
    //  * A {@link GPUPipelineLayout} that the {@link GPUShaderModule} may be used with in a future
    //  * {@link GPUDevice#createComputePipeline()} or {@link GPUDevice#createRenderPipeline} call.
    //  * If set to {@link GPUAutoLayoutMode#"auto"} the layout will be the [$default pipeline layout$]
    //  * for the entry point associated with this hint will be used.
    //  */
    // layout?:
    //   | GPUPipelineLayout
    //   | GPUAutoLayoutMode;
}
pub struct GPUShaderModuleDescriptor {
    /**
     * The <a href="https://gpuweb.github.io/gpuweb/wgsl/">WGSL</a> source code for the shader
     * module.
     */
    pub code: Option<String>,
    /**
     * The <a href="https://gpuweb.github.io/gpuweb/wgsl/">WGSL</a> byte code for the shader
     * module.
     */
    pub byte_code: Option<Vec<u8>>,
    // /**
    //  * If defined MAY be interpreted as a source-map-v3 format.
    //  * Source maps are optional, but serve as a standardized way to support dev-tool
    //  * integration such as source-language debugging [[SourceMap]].
    //  * WGSL names (identifiers) in source maps follow the rules defined in WGSL identifier
    //  * comparison.
    //  */
    // sourceMap: Option<object>,
    // /**
    //  * If defined maps an entry point name from the shader to a {@link GPUShaderModuleCompilationHint}.
    //  * No validation is performed with any of these {@link GPUShaderModuleCompilationHint}.
    //  * Implementations should use any information present in the {@link GPUShaderModuleCompilationHint}
    //  * to perform as much compilation as is possible within {@link GPUDevice#createShaderModule}.
    //  * Entry point names follow the rules defined in WGSL identifier comparison.
    //  * Note: Supplying information in {@link GPUShaderModuleDescriptor#hints} does not have any
    //  * observable effect, other than performance. Because a single shader module can hold
    //  * multiple entry points, and multiple pipelines can be created from a single shader
    //  * module, it can be more performant for an implementation to do as much compilation as
    //  * possible once in {@link GPUDevice#createShaderModule} rather than multiple times in
    //  * the multiple calls to {@link GPUDevice#createComputePipeline} /
    //  * {@link GPUDevice#createRenderPipeline}.
    //  */
    // hints: Option<Record<
    //   string,
    //   GPUShaderModuleCompilationHint
    // >;
}
pub struct GPUStencilFaceState {
    /**
     * The {@link GPUCompareFunction} used when testing fragments against
     * {@link GPURenderPassDescriptor#depthStencilAttachment} stencil values.
     */
    pub compare: Option<GPUCompareFunction>,
    /**
     * The {@link GPUStencilOperation} performed if the fragment stencil comparison test described by
     * {@link GPUStencilFaceState#compare} fails.
     */
    pub fail_op: Option<GPUStencilOperation>,
    /**
     * The {@link GPUStencilOperation} performed if the fragment depth comparison described by
     * {@link GPUDepthStencilState#depthCompare} fails.
     */
    pub depth_fail_op: Option<GPUStencilOperation>,
    /**
     * The {@link GPUStencilOperation} performed if the fragment stencil comparison test described by
     * {@link GPUStencilFaceState#compare} passes.
     */
    pub pass_op: Option<GPUStencilOperation>,
}
pub struct GPUStorageTextureBindingLayout {
    /**
     * The access mode for this binding, indicating readability and writability.
     * Note:
     * There is currently only one access mode, {@link GPUStorageTextureAccess#"write-only"},
     * but this will expand in the future.
     */
    pub access: Option<GPUStorageTextureAccess>,
    /**
     * The required {@link GPUTextureViewDescriptor#format} of texture views bound to this binding.
     */
    pub format: GPUTextureFormat,
    /**
     * Indicates the required {@link GPUTextureViewDescriptor#dimension} for texture views bound to
     * this binding.
     */
    pub view_dimension: Option<GPUTextureViewDimension>,
}
pub struct GPUTextureBindingLayout {
    /**
     * Indicates the type required for texture views bound to this binding.
     */
    pub sample_type: Option<GPUTextureSampleType>,
    /**
     * Indicates the required {@link GPUTextureViewDescriptor#dimension} for texture views bound to
     * this binding.
     */
    pub view_dimension: Option<GPUTextureViewDimension>,
    /**
     * Indicates whether or not texture views bound to this binding must be multisampled.
     */
    pub multisampled: Option<bool>,
}
pub struct GPUTextureDescriptor {
    /**
     * The width, height, and depth or layer count of the texture.
     */
    pub size: GPUExtent3D,
    /**
     * The number of mip levels the texture will contain.
     */
    pub mip_level_count: Option<GPUIntegerCoordinate>,
    /**
     * The sample count of the texture. A {@link GPUTextureDescriptor#sampleCount} &gt; `1` indicates
     * a multisampled texture.
     */
    pub sample_count: Option<GPUSize32>,
    /**
     * Whether the texture is one-dimensional, an array of two-dimensional layers, or three-dimensional.
     */
    pub dimension: Option<GPUTextureDimension>,
    /**
     * The format of the texture.
     */
    pub format: GPUTextureFormat,
    /**
     * The allowed usages for the texture.
     */
    pub usage: GPUTextureUsageFlags,
    /**
     * Specifies what view {@link GPUTextureViewDescriptor#format} values will be allowed when calling
     * {@link GPUTexture#createView} on this texture (in addition to the texture's actual
     * {@link GPUTextureDescriptor#format}).
     * <div class=note>
     * Note:
     * Adding a format to this list may have a significant performance impact, so it is best
     * to avoid adding formats unnecessarily.
     * The actual performance impact is highly dependent on the target system; developers must
     * test various systems to find out the impact on their particular application.
     * For example, on some systems any texture with a {@link GPUTextureDescriptor#format} or
     * {@link GPUTextureDescriptor#viewFormats} entry including
     * {@link GPUTextureFormat#"rgba8unorm-srgb"} will perform less optimally than a
     * {@link GPUTextureFormat#"rgba8unorm"} texture which does not.
     * Similar caveats exist for other formats and pairs of formats on other systems.
     * </div>
     * Formats in this list must be texture view format compatible with the texture format.
     * <div algorithm>
     * Two {@link GPUTextureFormat}s `format` and `viewFormat` are <dfn dfn for="">texture view format compatible</dfn> if:
     * - `format` equals `viewFormat`, or
     * - `format` and `viewFormat` differ only in whether they are `srgb` formats (have the `-srgb` suffix).
     * </div>
     */
    pub view_formats: Option<Vec<GPUTextureFormat>>,
}
pub struct GPUTextureViewDescriptor {
    /**
     * The format of the texture view. Must be either the {@link GPUTextureDescriptor#format} of the
     * texture or one of the {@link GPUTextureDescriptor#viewFormats} specified during its creation.
     */
    pub format: Option<GPUTextureFormat>,
    /**
     * The dimension to view the texture as.
     */
    pub dimension: Option<GPUTextureViewDimension>,
    /**
     * Which {@link GPUTextureAspect|aspect(s)} of the texture are accessible to the texture view.
     */
    pub aspect: Option<GPUTextureAspect>,
    /**
     * The first (most detailed) mipmap level accessible to the texture view.
     */
    pub base_mip_level: Option<GPUIntegerCoordinate>,
    /**
     * How many mipmap levels, starting with {@link GPUTextureViewDescriptor#baseMipLevel}, are accessible to
     * the texture view.
     */
    pub mip_level_count: Option<GPUIntegerCoordinate>,
    /**
     * The index of the first array layer accessible to the texture view.
     */
    pub base_array_layer: Option<GPUIntegerCoordinate>,
    /**
     * How many array layers, starting with {@link GPUTextureViewDescriptor#baseArrayLayer}, are accessible
     * to the texture view.
     */
    pub array_layer_count: Option<GPUIntegerCoordinate>,
}
pub struct GPUUncapturedErrorEventInit {
    pub error: GPUError,
}
pub struct GPUVertexAttribute {
    /**
     * The {@link GPUVertexFormat} of the attribute.
     */
    pub format: GPUVertexFormat,
    /**
     * The offset, in bytes, from the beginning of the element to the data for the attribute.
     */
    pub offset: GPUSize64,
    /**
     * The numeric location associated with this attribute, which will correspond with a
     * <a href="https://gpuweb.github.io/gpuweb/wgsl/#input-output-locations">"@location" attribute</a>
     * declared in the {@link GPURenderPipelineDescriptor#vertex}.{@link GPUProgrammableStage#module|module}.
     */
    pub shader_location: GPUIndex32,
}
pub struct GPUVertexBufferLayout {
    /**
     * The stride, in bytes, between elements of this array.
     */
    pub array_stride: GPUSize64,
    /**
     * Whether each element of this array represents per-vertex data or per-instance data
     */
    pub step_mode: Option<GPUVertexStepMode>,
    /**
     * An array defining the layout of the vertex attributes within each element.
     */
    pub attributes: Vec<GPUVertexAttribute>,
}
pub struct GPUVertexState {
    /**
     * The {@link GPUShaderModule} containing the code that this programmable stage will execute.
     */
    pub module: GPUShaderModule,
    /**
     * The name of the function in {@link GPUProgrammableStage#module} that this stage will use to
     * perform its work.
     */
    pub entry_point: String,
    /**
     * Specifies the values of pipeline-overridable constants in the shader module
     * {@link GPUProgrammableStage#module}.
     * Each such pipeline-overridable constant is uniquely identified by a single
     * pipeline-overridable constant identifier string (representing the numeric ID of the
     * constant, if one is specified, and otherwise the constant's identifier name).
     * WGSL names (identifiers) in source maps follow the rules defined in WGSL identifier comparison.
     * The key of each key-value pair must equal the identifier string of one such constant.
     * When the pipeline is executed, that constant will have the specified value.
     * Values are specified as <dfn typedef for="">GPUPipelineConstantValue</dfn>, which is a {@link double}.
     * They are converted [$to WGSL type$] of the pipeline-overridable constant (`bool`/`i32`/`u32`/`f32`/`f16`).
     * If conversion fails, a validation error is generated.
     */
    pub constants: Option<std::collections::HashMap<String, GPUPipelineConstantValue>>,

    /**
     * A list of {@link GPUVertexBufferLayout}s defining the layout of the vertex attribute data in the
     * vertex buffers used by this pipeline.
     */
    pub buffers: Option<Vec<GPUVertexBufferLayout>>,
}

pub struct GPUAdapter {
    /**
     * The set of values in `this`.{@link GPUAdapter#[[adapter]]}.{@link adapter#[[features]]}.
     */
    pub features: GPUSupportedFeatures,
    /**
     * The limits in `this`.{@link GPUAdapter#[[adapter]]}.{@link adapter#[[limits]]}.
     */
    pub limits: GPUSupportedLimits,
    /**
     * Returns the value of {@link GPUAdapter#[[adapter]]}.{@link adapter#[[fallback]]}.
     */
    pub is_fallback_adapter: bool,
    // /**
    //  * Requests a device from the adapter.
    //  * This is a one-time action: if a device is returned successfully,
    //  * the adapter becomes invalid.
    //  * @param descriptor - Description of the {@link GPUDevice} to request.
    //  */
    // requestDevice(&self,descriptor: Option<GPUDeviceDescriptor>
    // )-> Result<GPUDevice> {
    //   not_implemented!()
    // }
    // /**
    //  * Requests the {@link GPUAdapterInfo} for this {@link GPUAdapter}.
    //  * Note: Adapter info values are returned with a Promise to give user agents an
    //  * opportunity to perform potentially long-running checks when requesting unmasked values,
    //  * such as asking for user consent before returning. If no `unmaskHints` are specified,
    //  * however, no dialogs should be displayed to the user.
    //  * @param unmaskHints - A list of {@link GPUAdapterInfo} attribute names for which unmasked
    //  * 	values are desired if available.
    //  */
    // requestAdapterInfo(
    //   unmaskHints: Option<Array<string>
    // ): Promise<GPUAdapterInfo>;
}
pub struct GPUAdapterInfo {
    /**
     * The name of the vendor of the adapter, if available. Empty string otherwise.
     */
    pub vendor: String,
    /**
     * The name of the family or class of GPUs the adapter belongs to, if available. Empty
     * string otherwise.
     */
    pub architecture: String,
    /**
     * A vendor-specific identifier for the adapter, if available. Empty string otherwise.
     * Note: This is a value that represents the type of adapter. For example, it may be a
     * [PCI device ID](https://pcisig.com/). It does not uniquely identify a given piece of
     * hardware like a serial number.
     */
    pub device: String,
    /**
     * A human readable string describing the adapter as reported by the driver, if available.
     * Empty string otherwise.
     * Note: Because no formatting is applied to {@link GPUAdapterInfo#description} attempting to parse
     * this value is not recommended. Applications which change their behavior based on the
     * {@link GPUAdapterInfo}, such as applying workarounds for known driver issues, should rely on the
     * other fields when possible.
     */
    pub description: String,
}

#[derive(Clone, Copy)]
pub struct GPUBindGroup {}

impl GPUBindGroup {
    pub fn destroy(&self) {}
}

#[derive(Clone, Copy)]
pub struct GPUBindGroupLayout {}

impl GPUBindGroupLayout {
    pub fn destroy(&self) {}
}

#[derive(Clone)]
pub struct GPUBuffer {
    pub size: GPUSize64Out,
    pub usage: GPUBufferUsageFlags,
    pub map_state: GPUBufferMapState,
    pub buffer: vk::Buffer,
    pub memory: vk::DeviceMemory,
}

impl Default for GPUBuffer {
    #[inline]
    fn default() -> Self {
        Self::null()
    }
}

impl GPUBuffer {
    #[inline]
    pub fn null() -> Self {
        Self {
            usage: GPUBufferUsageFlags::empty(),
            size: 0,
            map_state: GPUBufferMapState::Mapped,
            buffer: vk::Buffer::null(),
            memory: vk::DeviceMemory::null(),
        }
    }

    pub fn write<T>(&self, device: &GPUDevice, data: &Vec<T>) -> Result<()> {
        // delegate
        device.handle.api.write_buffer::<T>(self.memory, 0, data)
    }

    // /**
    //  * Maps the given range of the {@link GPUBuffer} and resolves the returned {@link Promise} when the
    //  * {@link GPUBuffer}'s content is ready to be accessed with {@link GPUBuffer#getMappedRange}.
    //  * The resolution of the returned {@link Promise} **only** indicates that the buffer has been mapped.
    //  * It does not guarantee the completion of any other operations visible to the content timeline,
    //  * and in particular does not imply that any other {@link Promise} returned from
    //  * {@link GPUQueue#onSubmittedWorkDone()} or {@link GPUBuffer#mapAsync} on other {@link GPUBuffer}s
    //  * have resolved.
    //  * The resolution of the {@link Promise} returned from {@link GPUQueue#onSubmittedWorkDone}
    //  * **does** imply the completion of
    //  * {@link GPUBuffer#mapAsync} calls made prior to that call,
    //  * on {@link GPUBuffer}s last used exclusively on that queue.
    //  * @param mode - Whether the buffer should be mapped for reading or writing.
    //  * @param offset - Offset in bytes into the buffer to the start of the range to map.
    //  * @param size - Size in bytes of the range to map.
    //  */
    // map_async(
    //   mode: GPUMapModeFlags,
    //   offset: Option<GPUSize64,
    //   size: Option<GPUSize64
    // ): Promise<undefined>;
    /**
     * Returns an {@link ArrayBuffer} with the contents of the {@link GPUBuffer} in the given mapped range.
     * @param offset - Offset in bytes into the buffer to return buffer contents from.
     * @param size - Size in bytes of the {@link ArrayBuffer} to return.
     */
    pub fn get_mapped_range(
        &self,
        _offset: Option<GPUSize64>,
        _size: Option<GPUSize64>,
    ) -> Result<Vec<u8>> {
        unimplemented!("get_mapped_range")
    }
    /**
     * Unmaps the mapped range of the {@link GPUBuffer} and makes it's contents available for use by the
     * GPU again.
     */
    pub fn unmap(&self) -> Result<()> {
        unimplemented!("unmap")
    }
    /**
     * Destroys the {@link GPUBuffer}.
     * Note: It is valid to destroy a buffer multiple times.
     * Note: Since no further operations can be enqueued using this buffer, implementations can
     * free resource allocations, including mapped memory that was just unmapped.
     */
    pub fn destroy(&self) {}
}

#[derive(Clone, Copy)]
pub struct GPUCommandBuffer {
    command_buffer: vk::CommandBuffer,
}

impl GPUCommandBuffer {
    pub fn destroy(&self) {}
}

pub struct GPUCommandEncoder {
    device: vulkan::VulkanDevice,
    data: std::rc::Rc<std::cell::RefCell<GPUDeviceData>>,
}

impl GPUCommandEncoder {
    /**
     * Begins encoding a render pass described by `descriptor`.
     * @param descriptor - Description of the {@link GPURenderPassEncoder} to create.
     */
    pub fn begin_render_pass(
        &self,
        _descriptor: GPURenderPassDescriptor,
    ) -> Result<GPURenderPassEncoder> {
        println!("-------------------------------------------------------------------");
        println!(
            "GPUDevice::begin_render_pass \t\t\tbuffer: 0x{:x}",
            self.data.borrow().context.command_buffers[self.device.swap_index].as_raw()
        );
        self.device.begin_render_pass(
            self.data.borrow().context.render_pass,
            self.data.borrow().context.framebuffers[self.device.swap_index],
            self.device.data.swapchain.extent,
            self.data.borrow().context.command_buffers[self.device.swap_index],
            Some([0.0, 0.0, 0.0, 1.0]),
            Some(1.0),
        )?;
        Ok(GPURenderPassEncoder {
            data: self.data.clone(),
            device: self.device.clone(),
        })
    }
    /**
     * Begins encoding a compute pass described by `descriptor`.
     * 	descriptor:
     */
    pub fn begin_compute_pass(
        &self,
        _descriptor: Option<GPUComputePassDescriptor>,
    ) -> Result<GPUComputePassEncoder> {
        unimplemented!("begin_compute_pass")
    }
    /**
     * Encode a command into the {@link GPUCommandEncoder} that copies data from a sub-region of a
     * {@link GPUBuffer} to a sub-region of another {@link GPUBuffer}.
     * @param source - The {@link GPUBuffer} to copy from.
     * @param sourceOffset - Offset in bytes into `source` to begin copying from.
     * @param destination - The {@link GPUBuffer} to copy to.
     * @param destinationOffset - Offset in bytes into `destination` to place the copied data.
     * @param size - Bytes to copy.
     */
    pub fn copy_buffer_to_buffer(
        &self,
        _source: GPUBuffer,
        _source_offset: GPUSize64,
        _destination: GPUBuffer,
        _destination_offset: GPUSize64,
        _size: GPUSize64,
    ) -> Result<()> {
        unimplemented!("copy_buffer_to_buffer")
    }
    /**
     * Encode a command into the {@link GPUCommandEncoder} that copies data from a sub-region of a
     * {@link GPUBuffer} to a sub-region of one or multiple continuous texture subresources.
     * @param source - Combined with `copySize`, defines the region of the source buffer.
     * @param destination - Combined with `copySize`, defines the region of the destination texture subresource.
     * 	`copySize`:
     */
    pub fn copy_buffer_to_texture(
        &self,
        _source: GPUImageCopyBuffer,
        _destination: GPUImageCopyTexture,
        _copy_size: GPUExtent3D,
    ) -> Result<()> {
        unimplemented!("copy_buffer_to_texture")
    }
    /**
     * Encode a command into the {@link GPUCommandEncoder} that copies data from a sub-region of one or
     * multiple continuous texture subresources to a sub-region of a {@link GPUBuffer}.
     * @param source - Combined with `copySize`, defines the region of the source texture subresources.
     * @param destination - Combined with `copySize`, defines the region of the destination buffer.
     * 	`copySize`:
     */
    pub fn copy_texture_to_buffer(
        &self,
        _source: GPUImageCopyTexture,
        _destination: GPUImageCopyBuffer,
        _copy_size: GPUExtent3D,
    ) -> Result<()> {
        unimplemented!("copy_texture_to_buffer")
    }
    /**
     * Encode a command into the {@link GPUCommandEncoder} that copies data from a sub-region of one
     * or multiple contiguous texture subresources to another sub-region of one or
     * multiple continuous texture subresources.
     * @param source - Combined with `copySize`, defines the region of the source texture subresources.
     * @param destination - Combined with `copySize`, defines the region of the destination texture subresources.
     * 	`copySize`:
     */
    pub fn copy_texture_to_texture(
        &self,
        _source: GPUImageCopyTexture,
        _destination: GPUImageCopyTexture,
        _copy_size: GPUExtent3D,
    ) -> Result<()> {
        unimplemented!("copy_texture_to_texture")
    }
    /**
     * Encode a command into the {@link GPUCommandEncoder} that fills a sub-region of a
     * {@link GPUBuffer} with zeros.
     * @param buffer - The {@link GPUBuffer} to clear.
     * @param offset - Offset in bytes into `buffer` where the sub-region to clear begins.
     * @param size - Size in bytes of the sub-region to clear. Defaults to the size of the buffer minus `offset`.
     */
    pub fn clear_buffer(
        &self,
        _buffer: GPUBuffer,
        _offset: Option<GPUSize64>,
        _size: Option<GPUSize64>,
    ) -> Result<()> {
        unimplemented!("clear_buffer")
    }
    /**
     * Writes a timestamp value into a querySet when all previous commands have completed executing.
     * Note: Timestamp query values are written in nanoseconds, but how the value is determined is
     * implementation-defined and may not increase monotonically. See [[#timestamp]] for details.
     * @param querySet - The query set that will store the timestamp values.
     * @param queryIndex - The index of the query in the query set.
     */
    pub fn write_timestamp(&self, _query_set: GPUQuerySet, _query_index: GPUSize32) -> Result<()> {
        unimplemented!("write_timestamp")
    }
    /**
     * Resolves query results from a {@link GPUQuerySet} out into a range of a {@link GPUBuffer}.
     * 	querySet:
     * 	firstQuery:
     * 	queryCount:
     * 	destination:
     * 	destinationOffset:
     */
    pub fn resolve_query_set(
        &self,
        _query_set: GPUQuerySet,
        _first_query: GPUSize32,
        _query_count: GPUSize32,
        _destination: GPUBuffer,
        _destination_offset: GPUSize64,
    ) -> Result<()> {
        unimplemented!("resolve_query_set")
    }
    /**
     * Completes recording of the commands sequence and returns a corresponding {@link GPUCommandBuffer}.
     * 	descriptor:
     */
    pub fn finish(
        &self,
        _descriptor: Option<GPUCommandBufferDescriptor>,
    ) -> Result<GPUCommandBuffer> {
        self.device
            .api
            .end_command_buffer(self.data.borrow().context.command_buffers[self.device.swap_index])?;
        // done
        Ok(GPUCommandBuffer {
            command_buffer: self.data.borrow().context.command_buffers[self.device.swap_index],
        })
    }
}

impl GPUCommandEncoder {
    pub fn destroy(&self) {}
}
pub struct GPUCompilationInfo {
    messages: Vec<GPUCompilationMessage>,
}
pub struct GPUCompilationMessage {
    /**
     * The human-readable, localizable text for this compilation message.
     * Note: The {@link GPUCompilationMessage#message} should follow the best practices for language
     * and direction information. This includes making use of any future standards which may
     * emerge regarding the reporting of string language and direction metadata.
     * <p class="note editorial">Editorial:
     * At the time of this writing, no language/direction recommendation is available that provides
     * compatibility and consistency with legacy APIs, but when there is, adopt it formally.
     */
    pub message: String,
    /**
     * The severity level of the message.
     * If the {@link GPUCompilationMessage#type} is {@link GPUCompilationMessageType#"error"}, it
     * corresponds to a shader-creation error.
     */
    pub r#type: GPUCompilationMessageType,
    /**
     * The line number in the shader {@link GPUShaderModuleDescriptor#code} the
     * {@link GPUCompilationMessage#message} corresponds to. Value is one-based, such that a lineNum of
     * `1` indicates the first line of the shader {@link GPUShaderModuleDescriptor#code}. Lines are
     * delimited by line breaks.
     * If the {@link GPUCompilationMessage#message} corresponds to a substring this points to
     * the line on which the substring begins. Must be `0` if the {@link GPUCompilationMessage#message}
     * does not correspond to any specific point in the shader {@link GPUShaderModuleDescriptor#code}.
     */
    pub line_num: u32,
    /**
     * The offset, in UTF-16 code units, from the beginning of line {@link GPUCompilationMessage#lineNum}
     * of the shader {@link GPUShaderModuleDescriptor#code} to the point or beginning of the substring
     * that the {@link GPUCompilationMessage#message} corresponds to. Value is one-based, such that a
     * {@link GPUCompilationMessage#linePos} of `1` indicates the first code unit of the line.
     * If {@link GPUCompilationMessage#message} corresponds to a substring this points to the
     * first UTF-16 code unit of the substring. Must be `0` if the {@link GPUCompilationMessage#message}
     * does not correspond to any specific point in the shader {@link GPUShaderModuleDescriptor#code}.
     */
    pub line_pos: u32,
    /**
     * The offset from the beginning of the shader {@link GPUShaderModuleDescriptor#code} in UTF-16
     * code units to the point or beginning of the substring that {@link GPUCompilationMessage#message}
     * corresponds to. Must reference the same position as {@link GPUCompilationMessage#lineNum} and
     * {@link GPUCompilationMessage#linePos}. Must be `0` if the {@link GPUCompilationMessage#message}
     * does not correspond to any specific point in the shader {@link GPUShaderModuleDescriptor#code}.
     */
    pub offset: u32,
    /**
     * The number of UTF-16 code units in the substring that {@link GPUCompilationMessage#message}
     * corresponds to. If the message does not correspond with a substring then
     * {@link GPUCompilationMessage#length} must be 0.
     */
    pub length: u32,
}
pub struct GPUComputePassEncoder {
    // /**
    //  * Sets the current {@link GPUComputePipeline}.
    //  * @param pipeline - The compute pipeline to use for subsequent dispatch commands.
    //  */
    // setPipeline(
    //   pipeline: GPUComputePipeline
    // ): undefined;
    // /**
    //  * Dispatch work to be performed with the current {@link GPUComputePipeline}.
    //  * See [[#computing-operations]] for the detailed specification.
    //  * @param workgroupCountX - X dimension of the grid of workgroups to dispatch.
    //  * @param workgroupCountY - Y dimension of the grid of workgroups to dispatch.
    //  * @param workgroupCountZ - Z dimension of the grid of workgroups to dispatch.
    //  */
    // dispatchWorkgroups(
    //   workgroupCountX: GPUSize32,
    //   workgroupCountY: Option<GPUSize32,
    //   workgroupCountZ: Option<GPUSize32
    // ): undefined;
    // /**
    //  * Dispatch work to be performed with the current {@link GPUComputePipeline} using parameters read
    //  * from a {@link GPUBuffer}.
    //  * See [[#computing-operations]] for the detailed specification.
    //  * packed block of **three 32-bit unsigned integer values (12 bytes total)**,
    //  * given in the same order as the arguments for {@link GPUComputePassEncoder#dispatchWorkgroups}.
    //  * For example:
    //  * @param indirectBuffer - Buffer containing the indirect dispatch parameters.
    //  * @param indirectOffset - Offset in bytes into `indirectBuffer` where the dispatch data begins.
    //  */
    // dispatchWorkgroupsIndirect(
    //   indirectBuffer: GPUBuffer,
    //   indirectOffset: GPUSize64
    // ): undefined;
    // /**
    //  * Completes recording of the compute pass commands sequence.
    //  */
    // end(): undefined;
}
pub struct GPUComputePipeline {}

#[derive(Clone, Debug)]
pub struct GPUDeviceContext {
    render_pass: vk::RenderPass,
    descriptor_set_layout: vk::DescriptorSetLayout,
    pipeline_layout: vk::PipelineLayout,
    pipeline: vk::Pipeline,
    uniform_buffers: Vec<(vk::Buffer, vk::DeviceMemory)>,
    framebuffers: Vec<vk::Framebuffer>,
    command_pool: vk::CommandPool,
    descriptor_pool: vk::DescriptorPool,
    descriptor_sets: Vec<vk::DescriptorSet>,
    command_buffers: Vec<vk::CommandBuffer>,
    command_buffer_queues: Vec<Vec<vk::CommandBuffer>>,
}

impl GPUDeviceContext {
    #[inline]
    fn null() -> Self {
        Self {
            render_pass: vk::RenderPass::null(),
            descriptor_set_layout: vk::DescriptorSetLayout::null(),
            pipeline_layout: vk::PipelineLayout::null(),
            pipeline: vk::Pipeline::null(),
            uniform_buffers: vec![],
            framebuffers: vec![],
            command_pool: vk::CommandPool::null(),
            descriptor_pool: vk::DescriptorPool::null(),
            descriptor_sets: vec![],
            command_buffers: vec![],
            command_buffer_queues: vec![],
        }
    }

    #[inline]
    fn is_null(self) -> bool {
        self.render_pass.is_null()
            && self.descriptor_set_layout.is_null()
            && self.pipeline_layout.is_null()
            && self.pipeline.is_null()
            && self.uniform_buffers.is_empty()
            && self.framebuffers.is_empty()
            && self.command_pool.is_null()
            && self.descriptor_pool.is_null()
            && self.descriptor_sets.is_empty()
            && self.command_buffers.is_empty()
            && self.command_buffer_queues.is_empty()
    }

    pub fn push_command_buffers(&mut self, index: usize, command_buffers: &Vec<vk::CommandBuffer>) {
        self.command_buffer_queues[index].extend(command_buffers.clone());
    }

    pub fn recreate(&mut self, device: &vulkan::VulkanDevice) -> Result<()> {
        // create uniform buffers
        self.uniform_buffers = device
            .api
            .create_uniform_buffers::<vulkan::CameraUniform>(device.data.swapchain.views.len())?;

        // create render pass
        self.render_pass = device
            .api
            .create_render_pass(device.data.swapchain.format, device.data.samples)?;

        // create descriptor set layout
        self.descriptor_set_layout = device.api.create_descriptor_set_layout()?;

        // create pipeline layout
        self.pipeline_layout = device
            .api
            .create_pipeline_layout(self.descriptor_set_layout)?;

        // create pipeline
        self.pipeline = device.api.create_pipeline(
            self.render_pass,
            self.pipeline_layout,
            device.data.swapchain.extent,
            device.data.samples,
            geometry::Mesh::binding_description(),
            geometry::Mesh::attribute_descriptions().to_vec(),
        )?;

        // create framebuffers
        self.framebuffers = device.api.create_framebuffers(
            self.render_pass,
            device.data.swapchain.extent,
            device.data.blit_albedo_view.handle,
            device.data.blit_depth_view.handle,
            &device
                .data
                .swapchain
                .views
                .iter()
                .map(|v| v.handle)
                .collect(),
        )?;

        // create descriptor pool
        self.descriptor_pool = device.api.create_descriptor_pool(100)?;

        // create descriptor sets
        self.descriptor_sets = device.api.create_descriptor_sets(
            self.descriptor_pool,
            self.descriptor_set_layout,
            device.data.swapchain.views.len(),
        )?;

        // update descriptor sets
        device.api.update_descriptor_sets::<vulkan::CameraUniform>(
            &self.descriptor_sets,
            &self.uniform_buffers,
            device.data.back_albedo_view.handle,
            device.data.back_albedo_sampler,
            device.data.swapchain.views.len(),
        )?;

        // create the command pool
        self.command_pool = device.api.create_command_pool(device.data.surface)?;

        // create command buffers
        self.command_buffers = device
            .api
            .create_command_buffers(self.command_pool, device.data.swapchain.views.len())?;

        // create command buffer queues
        self.command_buffer_queues = vec![vec![]; device.data.swapchain.views.len()];

        // done
        Ok(())
    }

    pub fn reset(&mut self, device: &vulkan::VulkanDevice) -> Result<()> {
        // destroy and recreate
        self.destroy(device);
        self.recreate(device)
    }

    pub fn destroy(&mut self, device: &vulkan::VulkanDevice) {
        unsafe {
            // wait until idle
            device.wait_until_idle();

            // clear command buffers (check if need for destroying or not)
            self.command_buffer_queues.clear();

            // check if any command buffers
            if self.command_buffers.len() > 0 {
                // destroy command buffers
                device
                    .api
                    .logical_device
                    .free_command_buffers(self.command_pool, &self.command_buffers);

                // clear
                self.command_buffers.clear();
            }

            // check if command pool
            if !self.command_pool.is_null() {
                // destroy command pool
                device
                    .api
                    .logical_device
                    .destroy_command_pool(self.command_pool, None);

                // clear
                self.command_pool = vk::CommandPool::null();
            }

            // check if descriptor sets
            if self.descriptor_sets.len() > 0 {
                // destroy descriptor sets
                device
                    .api
                    .logical_device
                    .free_descriptor_sets(self.descriptor_pool, &self.descriptor_sets)
                    .unwrap();

                // clear
                self.descriptor_sets.clear();
            }

            // check if descriptor pool
            if !self.descriptor_pool.is_null() {
                // destroy descriptor pool
                device
                    .api
                    .logical_device
                    .destroy_descriptor_pool(self.descriptor_pool, None);

                // clear
                self.descriptor_pool = vk::DescriptorPool::null();
            }

            // check if framebuffers
            if self.framebuffers.len() > 0 {
                // destroy framebuffers
                for framebuffer in self.framebuffers.iter() {
                    device
                        .api
                        .logical_device
                        .destroy_framebuffer(*framebuffer, None);
                }
                // clear
                self.framebuffers.clear();
            }

            // check if pipeline
            if !self.pipeline.is_null() {
                // destroy pipeline
                device
                    .api
                    .logical_device
                    .destroy_pipeline(self.pipeline, None);

                // clear
                self.pipeline = vk::Pipeline::null();
            }

            // check if pipeline layout
            if !self.pipeline_layout.is_null() {
                // destroy pipeline layout
                device
                    .api
                    .logical_device
                    .destroy_pipeline_layout(self.pipeline_layout, None);

                // clear
                self.pipeline_layout = vk::PipelineLayout::null();
            }

            // check if descriptor set layout
            if !self.descriptor_set_layout.is_null() {
                // destroy descriptor set layout
                device
                    .api
                    .logical_device
                    .destroy_descriptor_set_layout(self.descriptor_set_layout, None);

                // clear
                self.descriptor_set_layout = vk::DescriptorSetLayout::null();
            }

            // check if render pass
            if !self.render_pass.is_null() {
                // destroy render pass
                device
                    .api
                    .logical_device
                    .destroy_render_pass(self.render_pass, None);

                // clear
                self.render_pass = vk::RenderPass::null();
            }

            // check if uniform buffers
            if self.framebuffers.len() > 0 {
                // destroy uniform buffers
                for uniform_buffer in self.uniform_buffers.iter() {
                    device
                        .api
                        .logical_device
                        .destroy_buffer(uniform_buffer.0, None);
                    device
                        .api
                        .logical_device
                        .free_memory(uniform_buffer.1, None);
                }

                // clear
                self.uniform_buffers.clear();
            }
        }
    }
}

impl Default for GPUDeviceContext {
    #[inline]
    fn default() -> Self {
        Self::null()
    }
}

#[derive(Clone)]
pub struct GPUDevice {
    data: std::rc::Rc<std::cell::RefCell<GPUDeviceData>>,

    /**
     * Native device backing this device
     */
    pub handle: vulkan::VulkanDevice,

    /**
     * A set containing the {@link GPUFeatureName} values of the features
     * supported by the device (i.e. the ones with which it was created).
     */
    pub features: GPUSupportedFeatures,
    /**
     * Exposes the limits supported by the device
     * (which are exactly the ones with which it was created).
     */
    pub limits: GPUSupportedLimits,
    // /**
    //  * The primary {@link GPUQueue} for this device.
    //  */
    // pub queue: Option<GPUQueue>,
}

impl GPUDevice {
    pub fn is_valid(&self) -> Result<bool> {
        self.handle.is_valid()
    }

    pub fn create(window: &Window) -> Result<Self> {
        // create device
        Ok(GPUDevice {
            handle: vulkan::VulkanDevice::create(window)?,
            features: GPUSupportedFeatures::default(),
            limits: GPUSupportedLimits::default(),
            data: std::rc::Rc::new(std::cell::RefCell::new(GPUDeviceData::default())),
        })
    }

    pub fn begin_frame(&mut self, window: &Window) -> Result<()> {
        println!("-------------------------------------------------------------------");
        println!("GPUDevice::begin_frame");

        self.handle.begin_frame(window)
    }

    pub fn end_frame(&mut self, window: &Window) -> Result<()> {
        // print len
        if self.data.borrow().context.command_buffer_queues[self.handle.swap_index].len() > 0 {
            println!(
                "GPUDevice::end_frame \t\t\t\tbuffer: 0x{:x}",
                self.data.borrow().context.command_buffer_queues[self.handle.swap_index][0]
                    .as_raw()
            );
        }

        // submit command buffers
        self.handle.submit_command_buffers(
            &self.data.borrow().context.command_buffer_queues[self.handle.swap_index],
        )?;

        // end the actual frame
        self.handle.end_frame(window)?;

        // clear buffers
        self.data.borrow_mut().context.command_buffer_queues[self.handle.swap_index].clear();

        println!("GPUDevice::end_frame");
        println!("-------------------------------------------------------------------");
        Ok(())
    }

    pub fn reset(&mut self, window: &Window) -> Result<()> {
        // reset device
        self.handle.reset(window)?;
        // reset context
        self.data.borrow_mut().context.reset(&self.handle)?;
        // done
        Ok(())
    }

    pub fn load_texture(&self, _path: &str) -> Result<GPUTexture> {
        Ok(GPUTexture {
            width: 0,
            height: 0,
            depth_or_array_layers: 0,
            mip_level_count: 0,
            sample_count: 0,
            dimension: GPUTextureDimension::TwoD,
            format: GPUTextureFormat::Rgba8Unorm,
            usage: GPUTextureUsageFlags::RENDER_ATTACHMENT,
        })
    }

    /**
     * Destroys the device, preventing further operations on it.
     * Outstanding asynchronous operations will fail.
     * Note: It is valid to destroy a device multiple times.
     */
    pub fn destroy(&mut self) {
        // destroy context
        self.data.borrow_mut().context.destroy(&self.handle);
        // delegate
        self.handle.destroy()
    }

    pub fn resized(&mut self, value: bool) -> Result<()> {
        // mark device as resized
        self.handle.resized = value;

        // all fine
        Ok(())
    }

    pub fn get_current_extent(&self) -> Result<GPUExtent3D> {
        Ok(GPUExtent3D {
            width: self.handle.get_current_extent().width,
            height: Some(self.handle.get_current_extent().height),
            depth_or_array_layers: Some(1),
        })
    }

    pub fn get_presentation_format(&self) -> Result<GPUTextureFormat> {
        Ok(VulkanConverter::convert_vk_texture_format(
            self.handle.get_current_format(),
        ))
    }

    pub fn get_back_albedo_view(&self) -> Result<GPUTextureView> {
        Ok(GPUTextureView {})
    }
    pub fn get_back_depth_view(&self) -> Result<GPUTextureView> {
        Ok(GPUTextureView {})
    }

    pub fn create_typed_buffer<T>(
        &self,
        usage: GPUBufferUsageFlags,
        data: &Vec<T>,
    ) -> Result<GPUBuffer> {
        match usage {
            GPUBufferUsageFlags::INDEX => {
                // create buffer
                let buffer = self.handle.api.create_staged_buffer::<T>(
                    self.handle.data.graphics_queue,
                    self.handle.data.command_pool,
                    data,
                    vk::BufferUsageFlags::INDEX_BUFFER,
                )?;
                // done
                Ok(GPUBuffer {
                    usage,
                    size: data.len() as u64 * std::mem::size_of::<T>() as u64,
                    map_state: GPUBufferMapState::Unmapped,
                    buffer: buffer.0,
                    memory: buffer.1,
                })
            }
            GPUBufferUsageFlags::VERTEX => {
                // create buffer
                let buffer = self.handle.api.create_staged_buffer::<T>(
                    self.handle.data.graphics_queue,
                    self.handle.data.command_pool,
                    data,
                    vk::BufferUsageFlags::VERTEX_BUFFER,
                )?;
                // done
                Ok(GPUBuffer {
                    usage,
                    size: data.len() as u64 * std::mem::size_of::<T>() as u64,
                    map_state: GPUBufferMapState::Unmapped,
                    buffer: buffer.0,
                    memory: buffer.1,
                })
            }
            GPUBufferUsageFlags::UNIFORM => {
                // create buffer
                let buffer = self.handle.api.create_buffer::<T>(
                    data.len(),
                    vk::BufferUsageFlags::INDEX_BUFFER,
                    vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE,
                )?;
                // done
                Ok(GPUBuffer {
                    usage,
                    size: data.len() as u64 * std::mem::size_of::<T>() as u64,
                    map_state: GPUBufferMapState::Unmapped,
                    buffer: buffer.0,
                    memory: buffer.1,
                })
            }
            _ => unimplemented!("create_typed_buffer"),
        }
    }

    /**
     * Creates a {@link GPUBuffer}.
     * @param descriptor - Description of the {@link GPUBuffer} to create.
     */
    pub fn create_buffer(&self, _descriptor: GPUBufferDescriptor) -> Result<GPUBuffer> {
        unimplemented!("create_buffer")
    }
    /**
     * Creates a {@link GPUTexture}.
     * @param descriptor - Description of the {@link GPUTexture} to create.
     */
    pub fn create_texture(&self, _descriptor: GPUTextureDescriptor) -> Result<GPUTexture> {
        unimplemented!("create_texture")
    }
    /**
     * Creates a {@link GPUSampler}.
     * @param descriptor - Description of the {@link GPUSampler} to create.
     */
    pub fn create_sampler(&self, _descriptor: Option<GPUSamplerDescriptor>) -> Result<GPUSampler> {
        Ok(GPUSampler {})
    }
    /**
     * Creates a {@link GPUExternalTexture} wrapping the provided image source.
     * @param descriptor - Provides the external image source object (and any creation options).
     */
    pub fn import_external_texture(
        &self,
        _descriptor: GPUExternalTextureDescriptor,
    ) -> Result<GPUExternalTexture> {
        unimplemented!("import_external_texture")
    }
    /**
     * Creates a {@link GPUBindGroupLayout}.
     * @param descriptor - Description of the {@link GPUBindGroupLayout} to create.
     */
    pub fn create_bind_group_layout(
        &self,
        _descriptor: GPUBindGroupLayoutDescriptor,
    ) -> Result<GPUBindGroupLayout> {
        Ok(GPUBindGroupLayout {})
    }
    /**
     * Creates a {@link GPUPipelineLayout}.
     * @param descriptor - Description of the {@link GPUPipelineLayout} to create.
     */
    pub fn create_pipeline_layout(
        &self,
        _descriptor: GPUPipelineLayoutDescriptor,
    ) -> Result<GPUPipelineLayout> {
        Ok(GPUPipelineLayout {})
    }
    /**
     * Creates a {@link GPUBindGroup}.
     * @param descriptor - Description of the {@link GPUBindGroup} to create.
     */
    pub fn create_bind_group(&self, _descriptor: GPUBindGroupDescriptor) -> Result<GPUBindGroup> {
        Ok(GPUBindGroup {})
    }
    /**
     * Creates a {@link GPUShaderModule}.
     * @param descriptor - Description of the {@link GPUShaderModule} to create.
     */
    pub fn create_shader_module(
        &self,
        _descriptor: GPUShaderModuleDescriptor,
    ) -> Result<GPUShaderModule> {
        Ok(GPUShaderModule {})
    }
    /**
     * Creates a {@link GPUComputePipeline} using immediate pipeline creation.
     * @param descriptor - Description of the {@link GPUComputePipeline} to create.
     */
    pub fn create_compute_pipeline(
        &self,
        _descriptor: GPUComputePipelineDescriptor,
    ) -> Result<GPUComputePipeline> {
        unimplemented!("create_compute_pipeline")
    }
    /**
     * Creates a {@link GPURenderPipeline} using immediate pipeline creation.
     * @param descriptor - Description of the {@link GPURenderPipeline} to create.
     */
    pub fn create_render_pipeline(
        &self,
        _descriptor: GPURenderPipelineDescriptor,
    ) -> Result<GPURenderPipeline> {
        Ok(GPURenderPipeline {})
    }
    /**
     * Creates a {@link GPUCommandEncoder}.
     * @param descriptor - Description of the {@link GPUCommandEncoder} to create.
     */
    pub fn create_queue(&self) -> Result<GPUQueue> {
        Ok(GPUQueue {
            data: self.data.clone(),
            device: self.handle.clone(),
        })
    }
    /**
     * Creates a {@link GPUCommandEncoder}.
     * @param descriptor - Description of the {@link GPUCommandEncoder} to create.
     */
    pub fn create_command_encoder(
        &self,
        _descriptor: Option<GPUCommandEncoderDescriptor>,
    ) -> Result<GPUCommandEncoder> {
        Ok(GPUCommandEncoder {
            data: self.data.clone(),
            device: self.handle.clone(),
        })
    }
    /**
     * Creates a {@link GPURenderBundleEncoder}.
     * @param descriptor - Description of the {@link GPURenderBundleEncoder} to create.
     */
    pub fn create_render_bundle_encoder(
        &self,
        _descriptor: GPURenderBundleEncoderDescriptor,
    ) -> Result<GPURenderBundleEncoder> {
        unimplemented!("create_render_bundle_encoder")
    }
    /**
     * Creates a {@link GPUQuerySet}.
     * @param descriptor - Description of the {@link GPUQuerySet} to create.
     */
    pub fn create_query_set(&self, _descriptor: GPUQuerySetDescriptor) -> Result<GPUQuerySet> {
        unimplemented!("create_query_set")
    }
    // /**
    //  * A slot-backed attribute holding a promise which is created with the device, remains
    //  * pending for the lifetime of the device, then resolves when the device is lost.
    //  * Upon initialization, it is set to a new promise.
    //  */
    pub fn is_lost(&self) -> Result<bool> {
        unimplemented!("is_lost")
    }
    /**
     * Pushes a new GPU error scope onto the {@link GPUDevice#[[errorScopeStack]]} for `this`.
     * @param filter - Which class of errors this error scope observes.
     */
    pub fn push_error_scope(&self, _filter: GPUErrorFilter) -> Result<()> {
        unimplemented!("push_error_scope")
    }
    /**
     * Pops a GPU error scope off the {@link GPUDevice#[[errorScopeStack]]} for `this`
     * and resolves to **any** {@link GPUError} observed by the error scope, or `null` if none.
     * There is no guarantee of the ordering of promise resolution.
     */
    pub fn pop_error_scope(&self) -> Result<Option<GPUError>> {
        unimplemented!("pop_error_scope")
    }
}

pub struct GPUDeviceLostInfo {
    /**
     * Nominal type branding.
     * https://github.com/microsoft/TypeScript/pull/33038
     * @internal
     */
    // readonly __brand: "GPUDeviceLostInfo";
    reason: GPUDeviceLostReason,
    message: String,
}
pub struct GPUError {
    /**
     * A human-readable, localizable text message providing information about the error that
     * occurred.
     * Note: This message is generally intended for application developers to debug their
     * applications and capture information for debug reports, not to be surfaced to end-users.
     * Note: User agents should not include potentially machine-parsable details in this message,
     * such as free system memory on {@link GPUErrorFilter#"out-of-memory"} or other details about the
     * conditions under which memory was exhausted.
     * Note: The {@link GPUError#message} should follow the best practices for language and
     * direction information. This includes making use of any future standards which may emerge
     * regarding the reporting of string language and direction metadata.
     * <p class="note editorial">Editorial:
     * At the time of this writing, no language/direction recommendation is available that provides
     * compatibility and consistency with legacy APIs, but when there is, adopt it formally.
     */
    message: String,
}

pub struct GPUExternalTexture {}

pub struct GPUInternalError {}

pub struct GPUOutOfMemoryError {}

#[derive(Clone, Copy)]
pub struct GPUPipelineLayout {}

impl GPUPipelineLayout {
    pub fn destroy(&self) {}
}

pub struct GPUQuerySet {
    /**
     * Destroys the {@link GPUQuerySet}.
     */
    // destroy(): undefined;
    /**
     * The type of the queries managed by this {@link GPUQuerySet}.
     */
    pub r#type: GPUQueryType,
    /**
     * The number of queries managed by this {@link GPUQuerySet}.
     */
    pub count: GPUSize32Out,
}

pub struct GPUQueue {
    device: vulkan::VulkanDevice,
    data: std::rc::Rc<std::cell::RefCell<GPUDeviceData>>,
}

impl GPUQueue {
    /**
     * Schedules the execution of the command buffers by the GPU on this queue.
     * Submitted command buffers cannot be used again.
     * 	`commandBuffers`:
     */
    pub fn submit(&self, command_buffers: &[GPUCommandBuffer]) -> Result<()> {
        if self.device.swap_index < self.data.borrow_mut().context.command_buffer_queues.len() {
            // print
            if command_buffers.len() > 0 {
                println!(
                    "GPUDevice::submit \t\t\t\tbuffer: 0x{:x}",
                    command_buffers[0].command_buffer.as_raw()
                );
            }
            // submit to local command buffer queue
            self.data.borrow_mut().context.push_command_buffers(
                self.device.swap_index,
                &command_buffers
                    .iter()
                    .map(|cb| cb.command_buffer)
                    .collect::<Vec<vk::CommandBuffer>>(),
            );
        }
        Ok(())
    }
    // /**
    //  * Returns a {@link Promise} that resolves once this queue finishes processing all the work submitted
    //  * up to this moment.
    //  * Resolution of this {@link Promise} implies the completion of
    //  * {@link GPUBuffer#mapAsync} calls made prior to that call,
    //  * on {@link GPUBuffer}s last used exclusively on that queue.
    //  */
    // onSubmittedWorkDone(): Promise<undefined>;
    /**
     * Issues a write operation of the provided data into a {@link GPUBuffer}.
     * @param buffer - The buffer to write to.
     * @param bufferOffset - Offset in bytes into `buffer` to begin writing at.
     * @param data - Data to write into `buffer`.
     * @param dataOffset - Offset in into `data` to begin writing from. Given in elements if
     * 	`data` is a `TypedArray` and bytes otherwise.
     * @param size - Size of content to write from `data` to `buffer`. Given in elements if
     * 	`data` is a `TypedArray` and bytes otherwise.
     */
    pub fn write_buffer(
        &self,
        _buffer: &GPUBuffer,
        _buffer_offset: GPUSize64,
        _data: &BufferSource,
        _data_offset: Option<GPUSize64>,
        _size: Option<GPUSize64>,
    ) -> Result<()> {
        Ok(())
    }
    /**
     * Issues a write operation of the provided data into a {@link GPUTexture}.
     * @param destination - The texture subresource and origin to write to.
     * @param data - Data to write into `destination`.
     * @param dataLayout - Layout of the content in `data`.
     * @param size - Extents of the content to write from `data` to `destination`.
     */
    pub fn write_texture(
        &self,
        _destination: GPUImageCopyTexture,
        _data: &BufferSource,
        _data_layout: GPUImageDataLayout,
        _size: GPUExtent3D,
    ) -> Result<()> {
        unimplemented!("write_texture")
    }
    /**
     * Issues a copy operation of the contents of a platform image/canvas
     * into the destination texture.
     * This operation performs [[#color-space-conversions|color encoding]] into the destination
     * encoding according to the parameters of {@link GPUImageCopyTextureTagged}.
     * Copying into a `-srgb` texture results in the same texture bytes, not the same decoded
     * values, as copying into the corresponding non-`-srgb` format.
     * Thus, after a copy operation, sampling the destination texture has
     * different results depending on whether its format is `-srgb`, all else unchanged.
     * <!-- POSTV1(srgb-linear): If added, explain here how it interacts. -->
     * @param source - source image and origin to copy to `destination`.
     * @param destination - The texture subresource and origin to write to, and its encoding metadata.
     * @param copySize - Extents of the content to write from `source` to `destination`.
     */
    pub fn copy_external_image_to_texture(
        &self,
        _source: GPUImageCopyExternalImage,
        _destination: GPUImageCopyTextureTagged,
        _copy_size: GPUExtent3D,
    ) -> Result<()> {
        unimplemented!("copy_external_image_to_texture")
    }

    pub fn destroy(&self) {
        //self.device.handle.api.logical_device.q
    }
}

pub struct GPURenderBundle {}

pub struct GPURenderBundleEncoder {}
pub struct GPURenderPassEncoder {
    data: std::rc::Rc<std::cell::RefCell<GPUDeviceData>>,
    device: vulkan::VulkanDevice,
}

impl GPURenderPassEncoder {
    /**
     * Sets the viewport used during the rasterization stage to linearly map from
     * NDC|normalized device coordinates to viewport coordinates.
     * @param x - Minimum X value of the viewport in pixels.
     * @param y - Minimum Y value of the viewport in pixels.
     * @param width - Width of the viewport in pixels.
     * @param height - Height of the viewport in pixels.
     * @param minDepth - Minimum depth value of the viewport.
     * @param maxDepth - Maximum depth value of the viewport.
     */
    pub fn set_viewport(
        &self,
        _x: f32,
        _y: f32,
        _width: f32,
        _height: f32,
        _min_depth: f32,
        _max_depth: f32,
    ) -> Result<()> {
        unimplemented!("set_viewport")
    }
    /**
     * Sets the scissor rectangle used during the rasterization stage.
     * After transformation into viewport coordinates any fragments which fall outside the scissor
     * rectangle will be discarded.
     * @param x - Minimum X value of the scissor rectangle in pixels.
     * @param y - Minimum Y value of the scissor rectangle in pixels.
     * @param width - Width of the scissor rectangle in pixels.
     * @param height - Height of the scissor rectangle in pixels.
     */
    pub fn set_scissor_rect(
        &self,
        _x: GPUIntegerCoordinate,
        _y: GPUIntegerCoordinate,
        _width: GPUIntegerCoordinate,
        _height: GPUIntegerCoordinate,
    ) -> Result<()> {
        Ok(())
    }
    /**
     * Sets the constant blend color and alpha values used with {@link GPUBlendFactor#"constant"}
     * and {@link GPUBlendFactor#"one-minus-constant"} {@link GPUBlendFactor}s.
     * @param color - The color to use when blending.
     */
    pub fn set_blend_constant(&self, _color: GPUColor) -> Result<()> {
        unimplemented!("set_blend_constant")
    }
    /**
     * Sets the {@link RenderState#[[stencilReference]]} value used during stencil tests with
     * the {@link GPUStencilOperation#"replace"} {@link GPUStencilOperation}.
     * @param reference - The new stencil reference value.
     */
    pub fn set_stencil_reference(&self, _reference: GPUStencilValue) -> Result<()> {
        unimplemented!("set_stencil_reference")
    }
    /**
     * @param queryIndex - The index of the query in the query set.
     */
    pub fn begin_occlusion_query(&self, _query_index: GPUSize32) -> Result<()> {
        unimplemented!("begin_occlusion_query")
    }
    /**
     */
    pub fn end_occlusion_query(&self) -> Result<()> {
        unimplemented!("end_occlusion_query")
    }
    /**
     * Executes the commands previously recorded into the given {@link GPURenderBundle}s as part of
     * this render pass.
     * When a {@link GPURenderBundle} is executed, it does not inherit the render pass's pipeline, bind
     * groups, or vertex and index buffers. After a {@link GPURenderBundle} has executed, the render
     * pass's pipeline, bind group, and vertex/index buffer state is cleared
     * (to the initial, empty values).
     * Note: The state is cleared, not restored to the previous state.
     * This occurs even if zero {@link GPURenderBundle|GPURenderBundles} are executed.
     * @param bundles - List of render bundles to execute.
     */
    pub fn execute_bundles(&self, _bundles: Vec<GPURenderBundle>) -> Result<()> {
        unimplemented!("execute_bundles")
    }
    /**
     * Completes recording of the render pass commands sequence.
     */
    pub fn end(&self) -> Result<()> {
        self.device
            .api
            .end_render_pass(self.data.borrow().context.command_buffers[self.device.swap_index])?;
        // all
        println!(
            "GPUDevice::end \t\t\t\t\tbuffer: 0x{:x}",
            self.data.borrow().context.command_buffers[self.device.swap_index].as_raw()
        );
        println!("-------------------------------------------------------------------");
        Ok(())
    }

    /**
     * Sets the current {@link GPUBindGroup} for the given index.
     * @param index - The index to set the bind group at.
     * @param bindGroup - Bind group to use for subsequent render or compute commands.
     * 	<!--The overload appears to be confusing bikeshed, and it ends up expecting this to
     * 	define the arguments for the 5-arg variant of the method, despite the "for"
     * 	explicitly pointing at the 3-arg variant. See
     * @param https - //github.com/plinss/widlparser/issues/56 and
     * @param https - //github.com/tabatkins/bikeshed/issues/1740 -->
     * @param dynamicOffsets - Array containing buffer offsets in bytes for each entry in
     * 	`bindGroup` marked as {@link GPUBindGroupLayoutEntry#buffer}.{@link GPUBufferBindingLayout#hasDynamicOffset}.-->
     */
    pub fn set_bind_group(
        &self,
        _index: GPUIndex32,
        _bind_group: &GPUBindGroup,
        _dynamic_offsets: Option<Vec<GPUBufferDynamicOffset>>,
    ) -> Result<()> {
        println!(
            "GPUDevice::set_bind_group \t\t\tbuffer: 0x{:x}",
            self.data.borrow().context.command_buffers[self.device.swap_index].as_raw()
        );
        self.device.api.bind_descriptor_sets(
            self.data.borrow().context.command_buffers[self.device.swap_index],
            vk::PipelineBindPoint::GRAPHICS,
            self.data.borrow().context.pipeline_layout,
            0,
            &[self.device.data.descriptor_sets[self.device.swap_index]],
            &[],
        )
    }
    /**
     * Sets the current {@link GPUBindGroup} for the given index, specifying dynamic offsets as a subset
     * of a {@link Uint32Array}.
     * @param index - The index to set the bind group at.
     * @param bindGroup - Bind group to use for subsequent render or compute commands.
     * @param dynamicOffsetsData - Array containing buffer offsets in bytes for each entry in
     * 	`bindGroup` marked as {@link GPUBindGroupLayoutEntry#buffer}.{@link GPUBufferBindingLayout#hasDynamicOffset}.
     * @param dynamicOffsetsDataStart - Offset in elements into `dynamicOffsetsData` where the
     * 	buffer offset data begins.
     * @param dynamicOffsetsDataLength - Number of buffer offsets to read from `dynamicOffsetsData`.
     */
    pub fn set_bind_group_with_details(
        &self,
        _index: GPUIndex32,
        _bind_group: Option<GPUBindGroup>,
        _dynamic_offsets_data: Vec<u32>,
        _dynamic_offsets_data_start: GPUSize64,
        _dynamic_offsets_data_length: GPUSize32,
    ) -> Result<()> {
        unimplemented!("set_bind_group_with_details")
    }
    /**
     * Sets the current {@link GPURenderPipeline}.
     * @param pipeline - The render pipeline to use for subsequent drawing commands.
     */
    pub fn set_pipeline(&self, _pipeline: &GPURenderPipeline) -> Result<()> {
        println!(
            "GPUDevice::set_pipeline \t\t\tbuffer: 0x{:x}",
            self.data.borrow().context.command_buffers[self.device.swap_index].as_raw()
        );
        self.device.api.bind_pipeline(
            self.data.borrow().context.command_buffers[self.device.swap_index],
            self.data.borrow().context.pipeline,
        )
    }
    /**
     * Sets the current index buffer.
     * @param buffer - Buffer containing index data to use for subsequent drawing commands.
     * @param indexFormat - Format of the index data contained in `buffer`.
     * @param offset - Offset in bytes into `buffer` where the index data begins. Defaults to `0`.
     * @param size - Size in bytes of the index data in `buffer`.
     * 	Defaults to the size of the buffer minus the offset.
     */
    pub fn set_index_buffer(
        &self,
        buffer: &GPUBuffer,
        format: GPUIndexFormat,
        offset: Option<GPUSize64>,
        _size: Option<GPUSize64>,
    ) -> Result<()> {
        println!(
            "GPUDevice::set_index_buffer \t\t\tbuffer: 0x{:x}",
            self.data.borrow().context.command_buffers[self.device.swap_index].as_raw()
        );
        self.device.api.bind_index_buffer(
            self.data.borrow().context.command_buffers[self.device.swap_index],
            buffer.buffer,
            offset.unwrap_or(0),
            VulkanConverter::convert_gpu_index_format(format),
        )
    }
    /**
     * Sets the current vertex buffer for the given slot.
     * @param slot - The vertex buffer slot to set the vertex buffer for.
     * @param buffer - Buffer containing vertex data to use for subsequent drawing commands.
     * @param offset - Offset in bytes into `buffer` where the vertex data begins. Defaults to `0`.
     * @param size - Size in bytes of the vertex data in `buffer`.
     * 	Defaults to the size of the buffer minus the offset.
     */
    pub fn set_vertex_buffer(
        &self,
        slot: GPUIndex32,
        buffer: &GPUBuffer,
        offset: Option<GPUSize64>,
        _size: Option<GPUSize64>,
    ) -> Result<()> {
        println!(
            "GPUDevice::bind_vertex_buffer \t\t\tbuffer: 0x{:x}",
            self.data.borrow().context.command_buffers[self.device.swap_index].as_raw()
        );
        self.device.api.bind_vertex_buffers(
            self.data.borrow().context.command_buffers[self.device.swap_index],
            slot,
            &[buffer.buffer],
            &[offset.unwrap_or(0)],
        )
    }
    /**
     * Draws primitives.
     * See [[#rendering-operations]] for the detailed specification.
     * @param vertexCount - The number of vertices to draw.
     * @param instanceCount - The number of instances to draw.
     * @param firstVertex - Offset into the vertex buffers, in vertices, to begin drawing from.
     * @param firstInstance - First instance to draw.
     */
    pub fn draw(
        &self,
        vertex_count: GPUSize32,
        instance_count: Option<GPUSize32>,
        first_vertex: Option<GPUSize32>,
        first_instance: Option<GPUSize32>,
    ) -> Result<()> {
        self.device.api.draw(
            self.data.borrow().context.command_buffers[self.device.swap_index],
            vertex_count,
            instance_count.unwrap_or(1),
            first_vertex.unwrap_or(0),
            first_instance.unwrap_or(0),
        )
    }
    /**
     * Draws indexed primitives.
     * See [[#rendering-operations]] for the detailed specification.
     * @param indexCount - The number of indices to draw.
     * @param instanceCount - The number of instances to draw.
     * @param firstIndex - Offset into the index buffer, in indices, begin drawing from.
     * @param baseVertex - Added to each index value before indexing into the vertex buffers.
     * @param firstInstance - First instance to draw.
     */
    pub fn draw_indexed(
        &self,
        index_count: GPUSize32,
        instance_count: Option<GPUSize32>,
        first_index: Option<GPUSize32>,
        base_vertex: Option<GPUSignedOffset32>,
        first_instance: Option<GPUSize32>,
    ) -> Result<()> {
        println!(
            "GPUDevice::draw_indexed \t\t\tbuffer: 0x{:x}",
            self.data.borrow().context.command_buffers[self.device.swap_index].as_raw()
        );
        self.device.api.draw_indexed(
            self.data.borrow().context.command_buffers[self.device.swap_index],
            index_count,
            instance_count.unwrap_or(1),
            first_index.unwrap_or(0),
            base_vertex.unwrap_or(0),
            first_instance.unwrap_or(0),
        )
    }
    /**
     * Draws primitives using parameters read from a {@link GPUBuffer}.
     * See [[#rendering-operations]] for the detailed specification.
     * packed block of **four 32-bit unsigned integer values (16 bytes total)**, given in the same
     * order as the arguments for {@link GPURenderEncoderBase#draw}. For example:
     * @param indirectBuffer - Buffer containing the indirect draw parameters.
     * @param indirectOffset - Offset in bytes into `indirectBuffer` where the drawing data begins.
     */
    pub fn draw_indirect(
        &self,
        indirect_buffer: GPUBuffer,
        indirect_offset: GPUSize64,
    ) -> Result<()> {
        self.device.api.draw_indirect(
            self.data.borrow().context.command_buffers[self.device.swap_index],
            indirect_buffer.buffer,
            indirect_offset,
            1,
            0,
        )
    }
    /**
     * Draws indexed primitives using parameters read from a {@link GPUBuffer}.
     * See [[#rendering-operations]] for the detailed specification.
     * tightly packed block of **five 32-bit unsigned integer values (20 bytes total)**, given in
     * the same order as the arguments for {@link GPURenderEncoderBase#drawIndexed}. For example:
     * @param indirectBuffer - Buffer containing the indirect drawIndexed parameters.
     * @param indirectOffset - Offset in bytes into `indirectBuffer` where the drawing data begins.
     */
    pub fn draw_indexed_indirect(
        &self,
        indirect_buffer: GPUBuffer,
        indirect_offset: GPUSize64,
    ) -> Result<()> {
        self.device.api.draw_indexed_indirect(
            self.data.borrow().context.command_buffers[self.device.swap_index],
            indirect_buffer.buffer,
            indirect_offset,
            1,
            0,
        )
    }

    pub fn destroy(&self) {}
}

pub struct GPURenderPipeline {}

impl GPURenderPipeline {
    pub fn destroy(&self) {}
}

pub struct GPUSampler {}

impl GPUSampler {
    pub fn destroy(&self) {}
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct GPUShaderModule {
    // /**
    //  * Returns any messages generated during the {@link GPUShaderModule}'s compilation.
    //  * The locations, order, and contents of messages are implementation-defined.
    //  * In particular, messages may not be ordered by {@link GPUCompilationMessage#lineNum}.
    //  */
    // getCompilationInfo(): Promise<GPUCompilationInfo>;
}

impl GPUShaderModule {
    pub fn destroy(&self) {}
}

type GPUSupportedFeatures = std::collections::HashSet<String>;

#[derive(Clone, Debug, Default)]
pub struct GPUSupportedLimits {
    pub max_texture_dimension_1d: u32,
    pub max_texture_dimension_2d: u32,
    pub max_texture_dimension_3d: u32,
    pub max_texture_array_layers: u32,
    pub max_bind_groups: u32,
    pub max_bind_groups_plus_vertex_buffers: u32,
    pub max_bindings_per_bind_group: u32,
    pub max_dynamic_uniform_buffers_per_pipeline_layout: u32,
    pub max_dynamic_storage_buffers_per_pipeline_layout: u32,
    pub max_sampled_textures_per_shader_stage: u32,
    pub max_samplers_per_shader_stage: u32,
    pub max_storage_buffers_per_shader_stage: u32,
    pub max_storage_textures_per_shader_stage: u32,
    pub max_uniform_buffers_per_shader_stage: u32,
    pub max_uniform_buffer_binding_size: u32,
    pub max_storage_buffer_binding_size: u32,
    pub min_uniform_buffer_offset_alignment: u32,
    pub min_storage_buffer_offset_alignment: u32,
    pub max_vertex_buffers: u32,
    pub max_buffer_size: u32,
    pub max_vertex_attributes: u32,
    pub max_vertex_buffer_array_stride: u32,
    pub max_inter_stage_shader_components: u32,
    pub max_inter_stage_shader_variables: u32,
    pub max_color_attachments: u32,
    pub max_color_attachment_bytes_per_sample: u32,
    pub max_compute_workgroup_storage_size: u32,
    pub max_compute_invocations_per_workgroup: u32,
    pub max_compute_workgroup_size_x: u32,
    pub max_compute_workgroup_size_y: u32,
    pub max_compute_workgroup_size_z: u32,
    pub max_compute_workgroups_per_dimension: u32,
}

pub struct GPUTexture {
    /**
     * The width of this {@link GPUTexture}.
     */
    pub width: GPUIntegerCoordinateOut,
    /**
     * The height of this {@link GPUTexture}.
     */
    pub height: GPUIntegerCoordinateOut,
    /**
     * The depth or layer count of this {@link GPUTexture}.
     */
    pub depth_or_array_layers: GPUIntegerCoordinateOut,
    /**
     * The number of mip levels of this {@link GPUTexture}.
     */
    pub mip_level_count: GPUIntegerCoordinateOut,
    /**
     * The number of sample count of this {@link GPUTexture}.
     */
    pub sample_count: GPUSize32Out,
    /**
     * The dimension of the set of texel for each of this {@link GPUTexture}'s subresources.
     */
    pub dimension: GPUTextureDimension,
    /**
     * The format of this {@link GPUTexture}.
     */
    pub format: GPUTextureFormat,
    /**
     * The allowed usages for this {@link GPUTexture}.
     */
    pub usage: GPUTextureUsageFlags,
}

impl GPUTexture {
    pub fn create_view(
        &self,
        _descriptor: Option<GPUTextureViewDescriptor>,
    ) -> Result<GPUTextureView> {
        Ok(GPUTextureView {})
    }

    pub fn destroy(&self) {}
}

pub struct GPUTextureView {}

impl GPUTextureView {
    pub fn destroy(&self) {}
}

pub struct GPUUncapturedErrorEvent {
    /**
     * A slot-backed attribute holding an object representing the error that was uncaptured.
     * This has the same type as errors returned by {@link GPUDevice#popErrorScope}.
     */
    error: GPUError,
}

pub struct GPUValidationError {}

#[derive(Clone, Default)]
struct GPUDeviceData {
    /**
     * A set containing the {@link GPUFeatureName} values of the features
     * supported by the device (i.e. the ones with which it was created).
     */
    context: GPUDeviceContext,
}

impl GPUDeviceData {
    fn destroy(&mut self) {}
}
