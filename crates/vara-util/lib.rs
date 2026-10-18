pub mod path;
pub mod table_builder;

#[cfg(windows)]
pub mod win_compat;

#[cfg(unix)]
pub mod unix_compat;

pub mod constants {
    pub const FILE_CHUNK_PREFIX: char = '@';
    pub const FILE_NAME_INIT: &str = "init";
    pub const FILE_NAME_CONFIG: &str = ".luaurc";
    pub const FILE_EXTENSIONS: [&str; 1] = ["luau"];
}
