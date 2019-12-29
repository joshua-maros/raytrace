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
