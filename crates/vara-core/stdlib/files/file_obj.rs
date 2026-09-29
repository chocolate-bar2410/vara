use std::path::PathBuf;
use std::io::Write;
use std::io::BufWriter;
use std::io::Read;
use std::time::UNIX_EPOCH;

use mlua::ExternalError;

use crate::stdlib::chrono::datetime::DateTime as vara_datetime;

#[derive(mlua::UserData)]
pub struct File{
    #[lua(get)]
    path: PathBuf,
    #[lua(get)]
    size: u64,
}

#[mlua::userdata_impl]
impl File {
    #[lua(skip)]
    pub fn new(pathbuffer: &PathBuf) -> mlua::Result<Self> {
        
        let absolute_path = std::path::absolute(pathbuffer)?;

        let mut size = 0;
        let path = absolute_path.as_path();

        if std::fs::exists(&absolute_path)?{
            let metadata = std::fs::symlink_metadata(path)?;

            if !metadata.is_file() {
                return Err(format!("path {} does not lead to a file",path.display()).into_lua_err());
            }
            
            size = metadata.len();
        }

        Ok(Self {
            path: absolute_path.to_path_buf(),
            size : size,
        })
    }

    fn read(&self,context: &mlua::Lua) -> mlua::Result<mlua::Buffer>{
        let mut file = std::fs::File::open(&self.path)
            .map_err(|err| format!("Could not read file at {}: {}", self.path.display(),err).into_lua_err())?;
        let mut contents: Vec<u8> = Vec::new();

        file.read_to_end(&mut contents)?;

        context.create_buffer(contents)
    }

    fn write(this : mlua::AnyUserData,contents : mlua::Buffer) -> mlua::Result<mlua::AnyUserData>{
        {
            let mut file_obj = this.borrow_mut::<File>()?;

            let bytes = contents.to_vec();
            let file = std::fs::File::create(&file_obj.path)?;
            let mut writer = BufWriter::new(file);

            writer.write_all(&bytes)?;

            let metadata = std::fs::symlink_metadata(&file_obj.path)?;
            file_obj.size = metadata.len();
        }

        Ok(this)
    }

    fn append(this : mlua::AnyUserData,contents : mlua::Buffer) -> mlua::Result<mlua::AnyUserData>{
        {
            let mut file_obj = this.borrow_mut::<File>()?;

            let bytes = contents.to_vec();
            let file = std::fs::File::options()
                .append(true)
                .create(true)
                .open(&file_obj.path)
                .map_err(|err| format!("Could not append file at {}: {}", file_obj.path.display(),err).into_lua_err())?;

            let mut file_writer = BufWriter::new(file);
            file_writer.write_all(&bytes)?;

            let metadata = std::fs::symlink_metadata(&file_obj.path)?;
            file_obj.size = metadata.len();
        }

        Ok(this)
    }

    fn delete(&mut self) -> mlua::Result<()>{
        std::fs::remove_file(&self.path)
            .map_err(|err| format!("Could not delete file at {}: {}", self.path.display(),err).into_lua_err())?;
        self.size = 0;

        Ok(())
    }

    #[lua(name = "move")]
    fn move_file(this : mlua::AnyUserData,new_path : PathBuf) -> mlua::Result<mlua::AnyUserData>{
        {
            let mut file_obj = this.borrow_mut::<File>()?;

            std::fs::rename(&file_obj.path, &new_path)
                .map_err(|err| format!(
                    "Could not move file at {} to {}: {}", 
                    file_obj.path.display(),
                    new_path.display(),
                    err
                ).into_lua_err())?;
                
            file_obj.path = std::fs::canonicalize(new_path)?;
        }

        Ok(this)
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

    #[lua(getter,name="created")]
    fn get_creation_date(&self) -> mlua::Result<vara_datetime> {
        if !std::fs::exists(&self.path)?{
            return Ok(vara_datetime::new(0,0));
        }

        let metadata = std::fs::symlink_metadata(&self.path)?;

        #[cfg(target_family = "windows")]
        let timeoffset = vara_util::win_compat::get_timezone_offset();

        #[cfg(target_family = "unix")]
        let timeoffset = 0;

        let created = vara_datetime::new(metadata.created()?
            .duration_since(UNIX_EPOCH)
            .map_err(|err| err.into_lua_err())?
            .as_millis() as i64, timeoffset);
    
        Ok(created)
    }

    #[lua(getter,name="accessed")]
    fn get_accessed_date(&self) -> mlua::Result<vara_datetime> {
        if !std::fs::exists(&self.path)?{
            return Ok(vara_datetime::new(0,0));
        }

        let metadata = std::fs::symlink_metadata(&self.path)?;

        #[cfg(target_family = "windows")]
        let timeoffset = vara_util::win_compat::get_timezone_offset();

        #[cfg(target_family = "unix")]
        let timeoffset = 0;

        let created = vara_datetime::new(metadata.accessed()?
            .duration_since(UNIX_EPOCH)
            .map_err(|err| err.into_lua_err())?
            .as_millis() as i64, timeoffset);
    
        Ok(created)
    }
    #[lua(getter,name="modified")]
    fn get_modified_date(&self) -> mlua::Result<vara_datetime> {
        if !std::fs::exists(&self.path)?{
            return Ok(vara_datetime::new(0,0));
        }

        let metadata = std::fs::symlink_metadata(&self.path)?;

        #[cfg(target_family = "windows")]
        let timeoffset = vara_util::win_compat::get_timezone_offset();

        #[cfg(target_family = "unix")]
        let timeoffset = 0;

        let created = vara_datetime::new(metadata.modified()?
            .duration_since(UNIX_EPOCH)
            .map_err(|err| err.into_lua_err())?
            .as_millis() as i64, timeoffset);
    
        Ok(created)
    }
}