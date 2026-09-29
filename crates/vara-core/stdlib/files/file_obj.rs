use std::path::PathBuf;
use std::io::Write;
use std::io::BufWriter;
use std::io::Read;

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
                return Err(format!("path {} does not lead to a file",path.display()).into_lua_err());
            }

            size = metadata.len();
        }

        Ok(Self {
            path: pathbuffer.to_path_buf(),
            size : size
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
                
            file_obj.path = new_path;
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

}