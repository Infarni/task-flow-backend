pub const NAME_MIN_LENGTH: usize = 3;
pub const NAME_MAX_LENGTH: usize = 16;
pub const NAME_PATTERN: &str = r"^[a-zA-Z0-9_]+$";

pub const EMAIL_MAX_LENGTH: usize = 320;

pub const PASSWORD_MIN_LENGTH: usize = 8;
pub const PASSWORD_MAX_LENGTH: usize = 128;

pub const AVATAR_MAX_SIZE: usize = 5 * 1024 * 1024;
pub const AVATAR_SIDE_SIZE: usize = 518;

pub const TASK_NAME_MIN_LENGTH: usize = 4;
pub const TASK_NAME_MAX_LENGTH: usize = 512;

pub const TASK_DESCRIPTION_MIN_LENGTH: usize = 4;
pub const TASK_DESCRIPTION_MAX_LENGTH: usize = 4096;

pub const TASK_COMMENT_TEXT_MIN_LENGTH: usize = 4;
pub const TASK_COMMENT_TEXT_MAX_LENGTH: usize = 4096;
