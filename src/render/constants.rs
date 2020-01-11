use ash::vk_make_version;

// Core constants.
pub const APPLICATION_VERSION: u32 = vk_make_version!(1, 0, 0);
pub const ENGINE_VERSION: u32 = vk_make_version!(1, 0, 0);
pub const API_VERSION: u32 = vk_make_version!(1, 0, 92);

pub const WINDOW_TITLE: &str = "Hello world";
pub const WINDOW_WIDTH: u32 = 512;
pub const WINDOW_HEIGHT: u32 = 512;
pub const ENABLE_DEBUG: bool = cfg!(debug_assertions);
pub const VALIDATION_LAYERS: &[&str] = &["VK_LAYER_KHRONOS_validation"];
pub const DEVICE_EXTENSIONS: &[&str] = &["VK_KHR_swapchain"];

// Pipeline constants.
pub const BLUE_NOISE_WIDTH: u32 = 512;
pub const BLUE_NOISE_HEIGHT: u32 = 512;
pub const BLUE_NOISE_CHANNELS: u32 = 4;
pub const BLUE_NOISE_SIZE: u32 = BLUE_NOISE_WIDTH * BLUE_NOISE_HEIGHT * BLUE_NOISE_CHANNELS;

pub const CHUNK_BLOCK_WIDTH: u32 = 8;
pub const CHUNK_BLOCK_VOLUME: u32 = CHUNK_BLOCK_WIDTH * CHUNK_BLOCK_WIDTH * CHUNK_BLOCK_WIDTH;

pub const REGION_CHUNK_WIDTH: u32 = 8;
pub const REGION_CHUNK_VOLUME: u32 = REGION_CHUNK_WIDTH * REGION_CHUNK_WIDTH * REGION_CHUNK_WIDTH;
pub const REGION_BLOCK_WIDTH: u32 = REGION_CHUNK_WIDTH * CHUNK_BLOCK_WIDTH;

pub const ROOT_REGION_WIDTH: u32 = 8;
pub const ROOT_REGION_VOLUME: u32 = ROOT_REGION_WIDTH * ROOT_REGION_WIDTH * ROOT_REGION_WIDTH;
pub const ROOT_CHUNK_WIDTH: u32 = ROOT_REGION_WIDTH * REGION_CHUNK_WIDTH;
pub const ROOT_CHUNK_VOLUME: u32 = ROOT_CHUNK_WIDTH * ROOT_CHUNK_WIDTH * ROOT_CHUNK_WIDTH;
pub const ROOT_BLOCK_WIDTH: u32 = ROOT_CHUNK_WIDTH * CHUNK_BLOCK_WIDTH;

pub const ATLAS_CHUNK_WIDTH: u32 = 64;
pub const ATLAS_BLOCK_WIDTH: u32 = ATLAS_CHUNK_WIDTH * CHUNK_BLOCK_WIDTH;
// pub const ATLAS_CHUNK_VOLUME: u32 = ATLAS_CHUNK_WIDTH * ATLAS_CHUNK_WIDTH * ATLAS_CHUNK_WIDTH;

pub const EMPTY_CHUNK_INDEX: u16 = 0xFFFF;
pub const UNLOADED_CHUNK_INDEX: u16 = 0xFFFE;
pub const REQUEST_LOAD_CHUNK_INDEX: u16 = 0xFFFD;

pub const NUM_UPLOAD_BUFFERS: usize = 32;
pub const SHADER_GROUP_SIZE: u32 = 8; // Each compute shader works on 8x8 groups.
