/// The Size of a chunk X
pub const CHUNK_X_SIZE: usize = 16;
/// The Size of a chunk Y
pub const CHUNK_Y_SIZE: usize = 384;
/// The Size of a chunk Z
pub const CHUNK_Z_SIZE: usize = 16;

/// Section Size X
pub const SECTION_X_SIZE: usize = 16;
/// Section Size Y
pub const SECTION_Y_SIZE: usize = 16;
/// Section Size Z
pub const SECTION_Z_SIZE: usize = 16;

/// Y Size
pub const Y_SIZE: usize = 384;

/// Section Max Size
pub const SECTION_SIZE: usize = SECTION_Y_SIZE * SECTION_X_SIZE * SECTION_Z_SIZE;
pub const BITS_PER_BLOCK: usize = 5;

pub const MIN_Y_SECTION: i8 = -4;
pub const MAX_Y_SECTION: i8 = 19;

pub const DATA_VERSION: i32 = 3120;
