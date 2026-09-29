use std::{ path::PathBuf};

use mlua::ExternalError;


#[derive(mlua::UserData)]
pub struct File{
    path: PathBuf,
    size: u64,
}

#[mlua::userdata_impl]
impl File {
    #[lua(skip)]
    pub fn new(pathbuffer: &PathBuf) -> mlua::Result<Self> {
        
        let mut size = 0;
        let path = pathbuffer.as_path();

        if std::fs::exists(pathbuffer)?{
            let metadata = std::fs::symlink_metadata(path)?;

            if !metadata.is_file() {
                return Err(format!("path @ {} does not lead to a file",path.display()).into_lua_err());
            }

            size = metadata.len();
        }

        Ok(Self {
            path: pathbuffer.to_path_buf(),
            size : size
        })
    }

    #[lua(meta,name="__tostring")]
    fn to_string(&self) -> mlua::Result<String> {

        const SIZE_SUFFIXES : [&str;5] = ["BYTES","KB","MB","GB","TB"];

        let mut size_string = "0 BYTES".to_string();

        if self.size > 0 {
            let index = ((self.size as f64).log(1024.0).floor() as usize).min(SIZE_SUFFIXES.len() - 1);
            let suffix = SIZE_SUFFIXES[index];

            let truncated = (self.size as f64 / 1024.0f64).powi(index as i32);
            size_string = format!("{:.2} {}",truncated,suffix);
        }


        Ok(format!("file {} {}",size_string,self.path.display()))
    }

}