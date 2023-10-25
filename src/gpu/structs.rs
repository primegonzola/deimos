// SPDX-License-Identifier: MIT

#![allow(dead_code)]

use vulkanalia::prelude::v1_0::*;

pub type BufferSource = [u8];
pub type SharedArrayBuffer = [u8];
pub type GPUBufferDynamicOffset = u64;
pub type GPUColorWriteFlags = u32;
pub type GPUDepthBias = f32;
pub type GPUFlagsConstant = u32;
pub type GPUIndex32 = u32;
pub type GPUIntegerCoordinate = u32;
pub type GPUIntegerCoordinateOut = i32;
pub type GPUMapModeFlags = u32;

pub type GPUPipelineConstantValue = f32;
pub type GPUSampleMask = u32;
pub type GPUSignedOffset32 = i32;
pub type GPUSize32 = u32;
pub type GPUSize32Out = u32;
pub type GPUSize64 = u64;
pub type GPUSize64Out = u64;
pub type GPUStencilValue = u32;

pub enum PredefinedColorSpace {
    Srgb,
    DisplayP3,
}
pub enum GPUAddressMode {
    ClampToEdge,
    Repeat,
    MirrorRepeat,
}
pub enum GPUAutoLayoutMode {
    Auto,
}
pub enum GPUBlendFactor {
    Zero,
    One,
    Src,
    OneMinusSrc,
    SrcAlpha,
    OneMinusSrcAlpha,
    Dst,
    OneMinusDst,
    DstAlpha,
    OneMinusDstAlpha,
    SrcAlphaSaturated,
    Constant,
    OneMinusConstant,
}
pub enum GPUBlendOperation {
    Add,
    Subtract,
    ReverseSubtract,
    Min,
    Max,
}
pub enum GPUBufferBindingType {
    Uniform,
    Storage,
    ReadOnlyStorage,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum GPUBufferMapState {
    Mapped,
    Unmapped,
    Pending,
}
pub enum GPUCanvasAlphaMode {
    Premultiplied,
    Opaque,
}
pub enum GPUCompareFunction {
    Never,
    Less,
    Equal,
    LessEqual,
    Greater,
    NotEqual,
    GreaterEqual,
    Always,
}
pub enum GPUCompilationMessageType {
    Error,
    Warning,
    Info,
}
pub enum GPUCullMode {
    None,
    Front,
    Back,
}
pub enum GPUDeviceLostReason {
    Destroyed,
    Unknown,
}
pub enum GPUErrorFilter {
    OutOfMemory,
    Validation,
    Internal,
}
pub enum GPUFeatureName {
    DepthClipControl,
    Depth32floatStencil8,
    TextureCompressionBc,
    TextureCompressionEtc2,
    TextureCompressionAstc,
    TimestampQuery,
    IndirectFirstInstance,
    ShaderF116,
    Rg11b10ufloatRenderable,
    Bgra8unormStorage,
    Float32Filterable,
}
pub enum GPUFilterMode {
    Nearest,
    Linear,
}
pub enum GPUFrontFace {
    CCW,
    CW,
}
pub enum GPUIndexFormat {
    Uint16,
    Uint32,
}
pub enum GPULoadOp {
    Load,
    Clear,
}
pub enum GPUMipmapFilterMode {
    Nearest,
    Linear,
}
pub enum GPUPipelineErrorReason {
    Validation,
    Internal,
}
pub enum GPUPowerPreference {
    LowPower,
    HighPerformance,
}
pub enum GPUPrimitiveTopology {
    PointList,
    LineList,
    LineStrip,
    TriangleList,
    TriangleStrip,
}
pub enum GPUQueryType {
    Occlusion,
    Timestamp,
}
pub enum GPUSamplerBindingType {
    Filtering,
    NonFiltering,
    Comparison,
}
pub enum GPUStencilOperation {
    Keep,
    Zero,
    Replace,
    Invert,
    IncrementClamp,
    DecrementClamp,
    IncrementWrap,
    DecrementWrap,
}
pub enum GPUStorageTextureAccess {
    WriteOnly,
}
pub enum GPUStoreOp {
    Store,
    Discard,
}
pub enum GPUTextureAspect {
    All,
    StencilOnly,
    DepthOnly,
}
pub enum GPUTextureDimension {
    OneD,
    TwoD,
    ThreeD,
}
pub enum GPUTextureSampleType {
    Float,
    UnfilterableFloat,
    Depth,
    Sint,
    Uint,
}
pub enum GPUTextureViewDimension {
    OneD,
    TwoD,
    TwoDArray,
    Cube,
    CubeArray,
    ThreeD,
}
pub enum GPUVertexFormat {
    Uint8x2,
    Uint8x4,
    Sint8x2,
    Sint8x4,
    Unorm8x2,
    Unorm8x4,
    Snorm8x2,
    Snorm8x4,
    Uint16x2,
    Uint16x4,
    Sint16x2,
    Sint16x4,
    Unorm16x2,
    Unorm16x4,
    Snorm16x2,
    Snorm16x4,
    Float16x2,
    Float16x4,
    Float32,
    Float32x2,
    Float32x3,
    Float32x4,
    Uint32,
    Uint32x2,
    Uint32x3,
    Uint32x4,
    Sint32,
    Sint32x2,
    Sint32x3,
    Sint32x4,
}
pub enum GPUVertexStepMode {
    Vertex,
    Instance,
}

pub enum GPUTextureFormat {
    None,
    R8Unorm,
    R8Snorm,
    R8Uint,
    R8Sint,
    R16Uint,
    R16Sint,
    R16Float,
    Rg8Unorm,
    Rg8Snorm,
    Rg8Uint,
    Rg8Sint,
    R32Uint,
    R32Sint,
    R32Float,
    Rg16Uint,
    Rg16Sint,
    Rg16Float,
    Rgba8Unorm,
    Rgba8UnormSrgb,
    Rgba8Snorm,
    Rgba8Uint,
    Rgba8Sint,
    Bgra8Unorm,
    Bgra8UnormSrgb,
    Rgb9e5Ufloat,
    Rgb10a2Unorm,
    Rg11b10Ufloat,
    Rg32uint,
    Rg32Sint,
    Rg32Float,
    Rgba16Uint,
    Rgba16Sint,
    Rgba16Float,
    Rgba32Uint,
    Rgba32Sint,
    Rgba32Float,
    Stencil8,
    Depth16Unorm,
    Depth24Plus,
    Depth24PlusStencil8,
    Depth32Float,
    Depth32FloatStencil8,
    Bc1RgbaUnorm,
    Bc1RgbaUnormSrgb,
    Bc2RgbaUnorm,
    Bc2RgbaUnormSrgb,
    Bc3RgbaUnorm,
    Bc3RgbaUnormSrgb,
    Bc4RUnorm,
    Bc4RSnorm,
    Bc5RgUnorm,
    Bc5RgSnorm,
    Bc6hRgbUfloat,
    Bc6hRgbFloat,
    Bc7RgbaUnorm,
    Bc7RgbaUnormSrgb,
    Etc2Rgb8Unorm,
    Etc2Rgb8UnormSrgb,
    Etc2Rgb8a1Unorm,
    Etc2Rgb8a1UnormSrgb,
    Etc2Rgba8Unorm,
    Etc2Rgba8UnormSrgb,
    EacR11Unorm,
    EacR11Snorm,
    EacRg11Unorm,
    EacRg11Snorm,
    Astc4x4Unorm,
    Astc4x4UnormSrgb,
    Astc5x4Unorm,
    Astc5x4UnormSrgb,
    Astc5x5Unorm,
    Astc5x5UnormSrgb,
    Astc6x5Unorm,
    Astc6x5UnormSrgb,
    Astc6x6Unorm,
    Astc6x6UnormSrgb,
    Astc8x5Unorm,
    Astc8x5UnormSrgb,
    Astc8x6Unorm,
    Astc8x6UnormSrgb,
    Astc8x8Unorm,
    Astc8x8UnormSrgb,
    Astc10x5Unorm,
    Astc10x5UnormSrgb,
    Astc10x6Unorm,
    Astc10x6UnormSrgb,
    Astc10x8Unorm,
    Astc10x8UnormSrgb,
    Astc10x10Unorm,
    Astc10x10UnormSrgb,
    Astc12x10Unorm,
    Astc12x10UnormSrgb,
    Astc12x12Unorm,
    Astc12x12UnormSrgb,
}

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Default)]
    pub struct GPUShaderStageFlags : u32 {
        const VERTEX = 1 << 0;
        const FRAGMENT = 1 << 1;
        const COMPUTE = 1 << 2;
    }
}

bitflags::bitflags! {
  #[repr(transparent)]
  #[derive(Default)]
  pub struct GPUBufferUsageFlags: u32{
      const MAP_READ = 1 << 0;
      const MAP_WRITE= 1 << 1;
      const COPY_SRC= 1 << 2;
      const COPY_DST= 1 << 3;
      const INDEX= 1 << 4;
      const VERTEX= 1 << 5;
      const UNIFORM= 1 << 6;
      const STORAGE= 1 << 7;
      const INDIRECT= 1 << 8;
      const QUERY_RESOLVE= 1 << 9;
  }
}

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Default)]
    pub struct GPUSampleCountFlags: u32 {
        const SC_1 = 1 << 0;
        const SC_2 = 1 << 1;
        const SC_4 = 1 << 2;
        const SC_8 = 1 << 3;
        const SC_16 = 1 << 4;
        const SC_32 = 1 << 5;
        const SC_64 = 1 << 6;
    }
}

bitflags::bitflags! {
  #[repr(transparent)]
  #[derive(Default)]
  pub struct GPUTextureUsageFlags : u32{
      const COPY_SRC = 1 << 0;
      const COPY_DST = 1 << 1;
      const TEXTURE_BINDING = 1 << 1;
      const STORAGE_BINDING = 1 << 1;
      const RENDER_ATTACHMENT =1 << 1;
  }
}

pub enum GPUColorWrite {
    RED,
    GREEN,
    BLUE,
    ALPHA,
    ALL,
}

pub enum GPUMapMode {
    READ,
    WRITE,
}

pub struct VulkanConverter {}

impl VulkanConverter {
    pub fn convert_vk_texture_format(format: vk::Format) -> GPUTextureFormat {
        match format {
            vk::Format::UNDEFINED => GPUTextureFormat::None,
            vk::Format::R8_UNORM => GPUTextureFormat::R8Unorm,
            vk::Format::R8_SNORM => GPUTextureFormat::R8Snorm,
            vk::Format::R8_UINT => GPUTextureFormat::R8Uint,
            vk::Format::R8_SINT => GPUTextureFormat::R8Sint,
            vk::Format::R16_UINT => GPUTextureFormat::R16Uint,
            vk::Format::R16_SINT => GPUTextureFormat::R16Sint,
            vk::Format::R16_SFLOAT => GPUTextureFormat::R16Float,
            vk::Format::R8G8_UNORM => GPUTextureFormat::Rg8Unorm,
            vk::Format::R8G8_SNORM => GPUTextureFormat::Rg8Snorm,
            vk::Format::R8G8_UINT => GPUTextureFormat::Rg8Uint,
            vk::Format::R8G8_SINT => GPUTextureFormat::Rg8Sint,
            vk::Format::R32_UINT => GPUTextureFormat::R32Uint,
            vk::Format::R32_SINT => GPUTextureFormat::R32Sint,
            vk::Format::R32_SFLOAT => GPUTextureFormat::R32Float,
            vk::Format::R16G16_UINT => GPUTextureFormat::Rg16Uint,
            vk::Format::R16G16_SINT => GPUTextureFormat::Rg16Sint,
            vk::Format::R16G16_SFLOAT => GPUTextureFormat::Rg16Float,
            vk::Format::R8G8B8A8_UNORM => GPUTextureFormat::Rgba8Unorm,
            vk::Format::R8G8B8A8_SRGB => GPUTextureFormat::Rgba8UnormSrgb,
            vk::Format::R8G8B8A8_SNORM => GPUTextureFormat::Rgba8Snorm,
            vk::Format::R8G8B8A8_UINT => GPUTextureFormat::Rgba8Uint,
            vk::Format::R8G8B8A8_SINT => GPUTextureFormat::Rgba8Sint,
            vk::Format::B8G8R8A8_UNORM => GPUTextureFormat::Bgra8Unorm,
            vk::Format::B8G8R8A8_SRGB => GPUTextureFormat::Bgra8UnormSrgb,
            vk::Format::E5B9G9R9_UFLOAT_PACK32 => GPUTextureFormat::Rgb9e5Ufloat,
            vk::Format::A2R10G10B10_UNORM_PACK32 => GPUTextureFormat::Rgb10a2Unorm,
            vk::Format::B10G11R11_UFLOAT_PACK32 => GPUTextureFormat::Rg11b10Ufloat,
            vk::Format::R32G32_UINT => GPUTextureFormat::Rg32uint,
            vk::Format::R32G32_SINT => GPUTextureFormat::Rg32Sint,
            vk::Format::R32G32_SFLOAT => GPUTextureFormat::Rg32Float,
            vk::Format::R16G16B16A16_UINT => GPUTextureFormat::Rgba16Uint,
            vk::Format::R16G16B16A16_SINT => GPUTextureFormat::Rgba16Sint,
            vk::Format::R16G16B16A16_SFLOAT => GPUTextureFormat::Rgba16Float,
            vk::Format::R32G32B32A32_UINT => GPUTextureFormat::Rgba32Uint,
            vk::Format::R32G32B32A32_SINT => GPUTextureFormat::Rgba32Sint,
            vk::Format::R32G32B32A32_SFLOAT => GPUTextureFormat::Rgba32Float,
            vk::Format::S8_UINT => GPUTextureFormat::Stencil8,
            vk::Format::D16_UNORM => GPUTextureFormat::Depth16Unorm,
            vk::Format::X8_D24_UNORM_PACK32 => GPUTextureFormat::Depth24Plus,
            vk::Format::D24_UNORM_S8_UINT => GPUTextureFormat::Depth24PlusStencil8,
            vk::Format::D32_SFLOAT => GPUTextureFormat::Depth32Float,
            vk::Format::D32_SFLOAT_S8_UINT => GPUTextureFormat::Depth32FloatStencil8,
            vk::Format::BC1_RGBA_UNORM_BLOCK => GPUTextureFormat::Bc1RgbaUnorm,
            vk::Format::BC1_RGBA_SRGB_BLOCK => GPUTextureFormat::Bc1RgbaUnormSrgb,
            vk::Format::BC2_UNORM_BLOCK => GPUTextureFormat::Bc2RgbaUnorm,
            vk::Format::BC2_SRGB_BLOCK => GPUTextureFormat::Bc2RgbaUnormSrgb,
            vk::Format::BC3_UNORM_BLOCK => GPUTextureFormat::Bc3RgbaUnorm,
            vk::Format::BC3_SRGB_BLOCK => GPUTextureFormat::Bc3RgbaUnormSrgb,
            vk::Format::BC4_UNORM_BLOCK => GPUTextureFormat::Bc4RUnorm,
            vk::Format::BC4_SNORM_BLOCK => GPUTextureFormat::Bc4RSnorm,
            vk::Format::BC5_UNORM_BLOCK => GPUTextureFormat::Bc5RgUnorm,
            vk::Format::BC5_SNORM_BLOCK => GPUTextureFormat::Bc5RgSnorm,
            vk::Format::BC6H_UFLOAT_BLOCK => GPUTextureFormat::Bc6hRgbUfloat,
            vk::Format::BC6H_SFLOAT_BLOCK => GPUTextureFormat::Bc6hRgbFloat,
            vk::Format::BC7_UNORM_BLOCK => GPUTextureFormat::Bc7RgbaUnorm,
            vk::Format::BC7_SRGB_BLOCK => GPUTextureFormat::Bc7RgbaUnormSrgb,
            vk::Format::ETC2_R8G8B8_UNORM_BLOCK => GPUTextureFormat::Etc2Rgb8Unorm,
            vk::Format::ETC2_R8G8B8_SRGB_BLOCK => GPUTextureFormat::Etc2Rgb8UnormSrgb,
            vk::Format::ETC2_R8G8B8A1_UNORM_BLOCK => GPUTextureFormat::Etc2Rgb8a1Unorm,
            vk::Format::ETC2_R8G8B8A1_SRGB_BLOCK => GPUTextureFormat::Etc2Rgb8a1UnormSrgb,
            vk::Format::ETC2_R8G8B8A8_UNORM_BLOCK => GPUTextureFormat::Etc2Rgba8Unorm,
            vk::Format::ETC2_R8G8B8A8_SRGB_BLOCK => GPUTextureFormat::Etc2Rgba8UnormSrgb,
            vk::Format::EAC_R11_UNORM_BLOCK => GPUTextureFormat::EacR11Unorm,
            vk::Format::EAC_R11_SNORM_BLOCK => GPUTextureFormat::EacR11Snorm,
            vk::Format::EAC_R11G11_UNORM_BLOCK => GPUTextureFormat::EacRg11Unorm,
            vk::Format::EAC_R11G11_SNORM_BLOCK => GPUTextureFormat::EacRg11Snorm,
            vk::Format::ASTC_4X4_UNORM_BLOCK => GPUTextureFormat::Astc4x4Unorm,
            vk::Format::ASTC_4X4_SRGB_BLOCK => GPUTextureFormat::Astc4x4UnormSrgb,
            vk::Format::ASTC_5X4_UNORM_BLOCK => GPUTextureFormat::Astc5x4Unorm,
            vk::Format::ASTC_5X4_SRGB_BLOCK => GPUTextureFormat::Astc5x4UnormSrgb,
            vk::Format::ASTC_5X5_UNORM_BLOCK => GPUTextureFormat::Astc5x5Unorm,
            vk::Format::ASTC_5X5_SRGB_BLOCK => GPUTextureFormat::Astc5x5UnormSrgb,
            vk::Format::ASTC_6X5_UNORM_BLOCK => GPUTextureFormat::Astc6x5Unorm,
            vk::Format::ASTC_6X5_SRGB_BLOCK => GPUTextureFormat::Astc6x5UnormSrgb,
            vk::Format::ASTC_6X6_UNORM_BLOCK => GPUTextureFormat::Astc6x6Unorm,
            vk::Format::ASTC_6X6_SRGB_BLOCK => GPUTextureFormat::Astc6x6UnormSrgb,
            vk::Format::ASTC_8X5_UNORM_BLOCK => GPUTextureFormat::Astc8x5Unorm,
            vk::Format::ASTC_8X5_SRGB_BLOCK => GPUTextureFormat::Astc8x5UnormSrgb,
            vk::Format::ASTC_8X6_UNORM_BLOCK => GPUTextureFormat::Astc8x6Unorm,
            vk::Format::ASTC_8X6_SRGB_BLOCK => GPUTextureFormat::Astc8x6UnormSrgb,
            vk::Format::ASTC_8X8_UNORM_BLOCK => GPUTextureFormat::Astc8x8Unorm,
            vk::Format::ASTC_8X8_SRGB_BLOCK => GPUTextureFormat::Astc8x8UnormSrgb,
            vk::Format::ASTC_10X5_UNORM_BLOCK => GPUTextureFormat::Astc10x5Unorm,
            vk::Format::ASTC_10X5_SRGB_BLOCK => GPUTextureFormat::Astc10x5UnormSrgb,
            vk::Format::ASTC_10X6_UNORM_BLOCK => GPUTextureFormat::Astc10x6Unorm,
            vk::Format::ASTC_10X6_SRGB_BLOCK => GPUTextureFormat::Astc10x6UnormSrgb,
            vk::Format::ASTC_10X8_UNORM_BLOCK => GPUTextureFormat::Astc10x8Unorm,
            vk::Format::ASTC_10X8_SRGB_BLOCK => GPUTextureFormat::Astc10x8UnormSrgb,
            vk::Format::ASTC_10X10_UNORM_BLOCK => GPUTextureFormat::Astc10x10Unorm,
            vk::Format::ASTC_10X10_SRGB_BLOCK => GPUTextureFormat::Astc10x10UnormSrgb,
            vk::Format::ASTC_12X10_UNORM_BLOCK => GPUTextureFormat::Astc12x10Unorm,
            vk::Format::ASTC_12X10_SRGB_BLOCK => GPUTextureFormat::Astc12x10UnormSrgb,
            vk::Format::ASTC_12X12_UNORM_BLOCK => GPUTextureFormat::Astc12x12Unorm,
            vk::Format::ASTC_12X12_SRGB_BLOCK => GPUTextureFormat::Astc12x12UnormSrgb,
            _ => panic!("Unsupported texture format: {:?}", format),
        }
    }

    pub fn convert_gpu_texture_format(format: GPUTextureFormat) -> vk::Format {
        match format {
            GPUTextureFormat::None => vk::Format::UNDEFINED,
            GPUTextureFormat::R8Unorm => vk::Format::R8_UNORM,
            GPUTextureFormat::R8Snorm => vk::Format::R8_SNORM,
            GPUTextureFormat::R8Uint => vk::Format::R8_UINT,
            GPUTextureFormat::R8Sint => vk::Format::R8_SINT,
            GPUTextureFormat::R16Uint => vk::Format::R16_UINT,
            GPUTextureFormat::R16Sint => vk::Format::R16_SINT,
            GPUTextureFormat::R16Float => vk::Format::R16_SFLOAT,
            GPUTextureFormat::Rg8Unorm => vk::Format::R8G8_UNORM,
            GPUTextureFormat::Rg8Snorm => vk::Format::R8G8_SNORM,
            GPUTextureFormat::Rg8Uint => vk::Format::R8G8_UINT,
            GPUTextureFormat::Rg8Sint => vk::Format::R8G8_SINT,
            GPUTextureFormat::R32Uint => vk::Format::R32_UINT,
            GPUTextureFormat::R32Sint => vk::Format::R32_SINT,
            GPUTextureFormat::R32Float => vk::Format::R32_SFLOAT,
            GPUTextureFormat::Rg16Uint => vk::Format::R16G16_UINT,
            GPUTextureFormat::Rg16Sint => vk::Format::R16G16_SINT,
            GPUTextureFormat::Rg16Float => vk::Format::R16G16_SFLOAT,
            GPUTextureFormat::Rgba8Unorm => vk::Format::R8G8B8A8_UNORM,
            GPUTextureFormat::Rgba8UnormSrgb => vk::Format::R8G8B8A8_SRGB,
            GPUTextureFormat::Rgba8Snorm => vk::Format::R8G8B8A8_SNORM,
            GPUTextureFormat::Rgba8Uint => vk::Format::R8G8B8A8_UINT,
            GPUTextureFormat::Rgba8Sint => vk::Format::R8G8B8A8_SINT,
            GPUTextureFormat::Bgra8Unorm => vk::Format::B8G8R8A8_UNORM,
            GPUTextureFormat::Bgra8UnormSrgb => vk::Format::B8G8R8A8_SRGB,
            GPUTextureFormat::Rgb9e5Ufloat => vk::Format::E5B9G9R9_UFLOAT_PACK32,
            GPUTextureFormat::Rgb10a2Unorm => vk::Format::A2R10G10B10_UNORM_PACK32,
            GPUTextureFormat::Rg11b10Ufloat => vk::Format::B10G11R11_UFLOAT_PACK32,
            GPUTextureFormat::Rg32uint => vk::Format::R32G32_UINT,
            GPUTextureFormat::Rg32Sint => vk::Format::R32G32_SINT,
            GPUTextureFormat::Rg32Float => vk::Format::R32G32_SFLOAT,
            GPUTextureFormat::Rgba16Uint => vk::Format::R16G16B16A16_UINT,
            GPUTextureFormat::Rgba16Sint => vk::Format::R16G16B16A16_SINT,
            GPUTextureFormat::Rgba16Float => vk::Format::R16G16B16A16_SFLOAT,
            GPUTextureFormat::Rgba32Uint => vk::Format::R32G32B32A32_UINT,
            GPUTextureFormat::Rgba32Sint => vk::Format::R32G32B32A32_SINT,
            GPUTextureFormat::Rgba32Float => vk::Format::R32G32B32A32_SFLOAT,
            GPUTextureFormat::Stencil8 => vk::Format::S8_UINT,
            GPUTextureFormat::Depth16Unorm => vk::Format::D16_UNORM,
            GPUTextureFormat::Depth24Plus => vk::Format::X8_D24_UNORM_PACK32,
            GPUTextureFormat::Depth24PlusStencil8 => vk::Format::D24_UNORM_S8_UINT,
            GPUTextureFormat::Depth32Float => vk::Format::D32_SFLOAT,
            GPUTextureFormat::Depth32FloatStencil8 => vk::Format::D32_SFLOAT_S8_UINT,
            GPUTextureFormat::Bc1RgbaUnorm => vk::Format::BC1_RGBA_UNORM_BLOCK,
            GPUTextureFormat::Bc1RgbaUnormSrgb => vk::Format::BC1_RGBA_SRGB_BLOCK,
            GPUTextureFormat::Bc2RgbaUnorm => vk::Format::BC2_UNORM_BLOCK,
            GPUTextureFormat::Bc2RgbaUnormSrgb => vk::Format::BC2_SRGB_BLOCK,
            GPUTextureFormat::Bc3RgbaUnorm => vk::Format::BC3_UNORM_BLOCK,
            GPUTextureFormat::Bc3RgbaUnormSrgb => vk::Format::BC3_SRGB_BLOCK,
            GPUTextureFormat::Bc4RUnorm => vk::Format::BC4_UNORM_BLOCK,
            GPUTextureFormat::Bc4RSnorm => vk::Format::BC4_SNORM_BLOCK,
            GPUTextureFormat::Bc5RgUnorm => vk::Format::BC5_UNORM_BLOCK,
            GPUTextureFormat::Bc5RgSnorm => vk::Format::BC5_SNORM_BLOCK,
            GPUTextureFormat::Bc6hRgbUfloat => vk::Format::BC6H_UFLOAT_BLOCK,
            GPUTextureFormat::Bc6hRgbFloat => vk::Format::BC6H_SFLOAT_BLOCK,
            GPUTextureFormat::Bc7RgbaUnorm => vk::Format::BC7_UNORM_BLOCK,
            GPUTextureFormat::Bc7RgbaUnormSrgb => vk::Format::BC7_SRGB_BLOCK,
            GPUTextureFormat::Etc2Rgb8Unorm => vk::Format::ETC2_R8G8B8_UNORM_BLOCK,
            GPUTextureFormat::Etc2Rgb8UnormSrgb => vk::Format::ETC2_R8G8B8_SRGB_BLOCK,
            GPUTextureFormat::Etc2Rgb8a1Unorm => vk::Format::ETC2_R8G8B8A1_UNORM_BLOCK,
            GPUTextureFormat::Etc2Rgb8a1UnormSrgb => vk::Format::ETC2_R8G8B8A1_SRGB_BLOCK,
            GPUTextureFormat::Etc2Rgba8Unorm => vk::Format::ETC2_R8G8B8A8_UNORM_BLOCK,
            GPUTextureFormat::Etc2Rgba8UnormSrgb => vk::Format::ETC2_R8G8B8A8_SRGB_BLOCK,
            GPUTextureFormat::EacR11Unorm => vk::Format::EAC_R11_UNORM_BLOCK,
            GPUTextureFormat::EacR11Snorm => vk::Format::EAC_R11_SNORM_BLOCK,
            GPUTextureFormat::EacRg11Unorm => vk::Format::EAC_R11G11_UNORM_BLOCK,
            GPUTextureFormat::EacRg11Snorm => vk::Format::EAC_R11G11_SNORM_BLOCK,
            GPUTextureFormat::Astc4x4Unorm => vk::Format::ASTC_4X4_UNORM_BLOCK,
            GPUTextureFormat::Astc4x4UnormSrgb => vk::Format::ASTC_4X4_SRGB_BLOCK,
            GPUTextureFormat::Astc5x4Unorm => vk::Format::ASTC_5X4_UNORM_BLOCK,
            GPUTextureFormat::Astc5x4UnormSrgb => vk::Format::ASTC_5X4_SRGB_BLOCK,
            GPUTextureFormat::Astc5x5Unorm => vk::Format::ASTC_5X5_UNORM_BLOCK,
            GPUTextureFormat::Astc5x5UnormSrgb => vk::Format::ASTC_5X5_SRGB_BLOCK,
            GPUTextureFormat::Astc6x5Unorm => vk::Format::ASTC_6X5_UNORM_BLOCK,
            GPUTextureFormat::Astc6x5UnormSrgb => vk::Format::ASTC_6X5_SRGB_BLOCK,
            GPUTextureFormat::Astc6x6Unorm => vk::Format::ASTC_6X6_UNORM_BLOCK,
            GPUTextureFormat::Astc6x6UnormSrgb => vk::Format::ASTC_6X6_SRGB_BLOCK,
            GPUTextureFormat::Astc8x5Unorm => vk::Format::ASTC_8X5_UNORM_BLOCK,
            GPUTextureFormat::Astc8x5UnormSrgb => vk::Format::ASTC_8X5_SRGB_BLOCK,
            GPUTextureFormat::Astc8x6Unorm => vk::Format::ASTC_8X6_UNORM_BLOCK,
            GPUTextureFormat::Astc8x6UnormSrgb => vk::Format::ASTC_8X6_SRGB_BLOCK,
            GPUTextureFormat::Astc8x8Unorm => vk::Format::ASTC_8X8_UNORM_BLOCK,
            GPUTextureFormat::Astc8x8UnormSrgb => vk::Format::ASTC_8X8_SRGB_BLOCK,
            GPUTextureFormat::Astc10x5Unorm => vk::Format::ASTC_10X5_UNORM_BLOCK,
            GPUTextureFormat::Astc10x5UnormSrgb => vk::Format::ASTC_10X5_SRGB_BLOCK,
            GPUTextureFormat::Astc10x6Unorm => vk::Format::ASTC_10X6_UNORM_BLOCK,
            GPUTextureFormat::Astc10x6UnormSrgb => vk::Format::ASTC_10X6_SRGB_BLOCK,
            GPUTextureFormat::Astc10x8Unorm => vk::Format::ASTC_10X8_UNORM_BLOCK,
            GPUTextureFormat::Astc10x8UnormSrgb => vk::Format::ASTC_10X8_SRGB_BLOCK,
            GPUTextureFormat::Astc10x10Unorm => vk::Format::ASTC_10X10_UNORM_BLOCK,
            GPUTextureFormat::Astc10x10UnormSrgb => vk::Format::ASTC_10X10_SRGB_BLOCK,
            GPUTextureFormat::Astc12x10Unorm => vk::Format::ASTC_12X10_UNORM_BLOCK,
            GPUTextureFormat::Astc12x10UnormSrgb => vk::Format::ASTC_12X10_SRGB_BLOCK,
            GPUTextureFormat::Astc12x12Unorm => vk::Format::ASTC_12X12_UNORM_BLOCK,
            GPUTextureFormat::Astc12x12UnormSrgb => vk::Format::ASTC_12X12_SRGB_BLOCK,
        }
    }

    pub fn convert_vk_sample_count(flags: vk::SampleCountFlags) -> GPUSampleCountFlags {
        match flags {
            vk::SampleCountFlags::_1 => GPUSampleCountFlags::SC_1,
            vk::SampleCountFlags::_2 => GPUSampleCountFlags::SC_2,
            vk::SampleCountFlags::_4 => GPUSampleCountFlags::SC_4,
            vk::SampleCountFlags::_8 => GPUSampleCountFlags::SC_8,
            vk::SampleCountFlags::_16 => GPUSampleCountFlags::SC_16,
            vk::SampleCountFlags::_32 => GPUSampleCountFlags::SC_32,
            vk::SampleCountFlags::_64 => GPUSampleCountFlags::SC_64,
            _ => panic!("Unsupported sample count flags: {:?}", flags),
        }
    }

    pub fn convert_gpu_sample_count(flags: GPUSampleCountFlags) -> vk::SampleCountFlags {
        match flags {
            GPUSampleCountFlags::SC_1 => vk::SampleCountFlags::_1,
            GPUSampleCountFlags::SC_2 => vk::SampleCountFlags::_2,
            GPUSampleCountFlags::SC_4 => vk::SampleCountFlags::_4,
            GPUSampleCountFlags::SC_8 => vk::SampleCountFlags::_8,
            GPUSampleCountFlags::SC_16 => vk::SampleCountFlags::_16,
            GPUSampleCountFlags::SC_32 => vk::SampleCountFlags::_32,
            GPUSampleCountFlags::SC_64 => vk::SampleCountFlags::_64,
            _ => panic!("Unsupported sample count flags: {:?}", flags),
        }
    }

    pub fn buffer_gpu_buffer_usage(usage: GPUBufferUsageFlags) -> vk::BufferUsageFlags {
        // start empty
        let mut flags = vk::BufferUsageFlags::empty();
        // convert
        if (usage & GPUBufferUsageFlags::COPY_SRC) != GPUBufferUsageFlags::empty() {
            flags |= vk::BufferUsageFlags::TRANSFER_SRC;
        }
        if (usage & GPUBufferUsageFlags::COPY_DST) != GPUBufferUsageFlags::empty() {
            flags |= vk::BufferUsageFlags::TRANSFER_DST;
        }
        if (usage & GPUBufferUsageFlags::INDEX) != GPUBufferUsageFlags::empty() {
            flags |= vk::BufferUsageFlags::INDEX_BUFFER;
        }
        if (usage & GPUBufferUsageFlags::VERTEX) != GPUBufferUsageFlags::empty() {
            flags |= vk::BufferUsageFlags::VERTEX_BUFFER;
        }
        if (usage & GPUBufferUsageFlags::UNIFORM) != GPUBufferUsageFlags::empty() {
            flags |= vk::BufferUsageFlags::UNIFORM_BUFFER;
        }
        if (usage & GPUBufferUsageFlags::STORAGE) != GPUBufferUsageFlags::empty() {
            flags |= vk::BufferUsageFlags::STORAGE_BUFFER;
        }
        if (usage & GPUBufferUsageFlags::INDIRECT) != GPUBufferUsageFlags::empty() {
            flags |= vk::BufferUsageFlags::INDIRECT_BUFFER;
        }
        // done
        return flags;
    }

    pub fn convert_vk_index_type(flags: vk::IndexType) -> GPUIndexFormat {
        match flags {
            vk::IndexType::UINT16 => GPUIndexFormat::Uint16,
            vk::IndexType::UINT32 => GPUIndexFormat::Uint32,
            _ => panic!("Unsupported index type: {:?}", flags),
        }
    }

    pub fn convert_gpu_index_format(flags: GPUIndexFormat) -> vk::IndexType {
        match flags {
            GPUIndexFormat::Uint16 => vk::IndexType::UINT16,
            GPUIndexFormat::Uint32 => vk::IndexType::UINT32,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct GPUCameraUniform {
    pub model: cgmath::Matrix4<f32>,
    pub view: cgmath::Matrix4<f32>,
    pub projection: cgmath::Matrix4<f32>,
}
