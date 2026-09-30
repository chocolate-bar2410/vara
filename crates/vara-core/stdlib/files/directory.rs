use std::path::PathBuf;
use std::time::UNIX_EPOCH;

use mlua::ExternalError;

use crate::stdlib as vara_stdlib;

use vara_stdlib::chrono::datetime::DateTime as vara_datetime;
use vara_stdlib::files::file_obj::File as vara_file;

use std::rc::Rc;
use std::cell::RefCell;

#[derive(mlua::UserData)]
pub struct Directory{
    #[lua(get)]
    path: PathBuf
}

#[mlua::userdata_impl]
impl Directory{
    #[lua(skip)]
    pub fn new(path: &PathBuf) -> mlua::Result<Self>{
        let absolute_path = std::path::absolute(path)?;
        let path = absolute_path.as_path();

        if std::fs::exists(&absolute_path)?{
            let metadata = std::fs::symlink_metadata(path)?;

            if !metadata.is_dir() {
                return Err(format!("path {} does not lead to a directory",path.display()).into_lua_err());
            }
        }

        Ok(Self{
            path: absolute_path
        })
    }

    fn make(this: mlua::AnyUserData) -> mlua::Result<mlua::AnyUserData>{

        {
            let directory = this.borrow_mut::<Directory>()?;
            std::fs::create_dir(&directory.path)?;
        }

        Ok(this)
    }

    #[lua(name = "move")]
    fn move_dir(this : mlua::AnyUserData,new_path : PathBuf) -> mlua::Result<mlua::AnyUserData>{
        {
            let mut directory = this.borrow_mut::<Directory>()?;

            std::fs::rename(&directory.path, &new_path)
                .map_err(|err| format!(
                    "Could not move directory at {} to {}: {}", 
                    directory.path.display(),
                    new_path.display(),
                    err
                ).into_lua_err())?;
                
            directory.path = std::fs::canonicalize(new_path)?;
        }

        Ok(this)
    }

    fn delete(&mut self) -> mlua::Result<()>{
        std::fs::remove_dir_all(&self.path)
            .map_err(|err| format!("Could not delete directory at {}: {}", self.path.display(),err).into_lua_err())?;

        Ok(())
    }

    fn walk(&self,context : &mlua::Lua) -> mlua::Result<mlua::Function>{
        let iter = Rc::new(RefCell::new(std::fs::read_dir(&self.path)?));

        context.create_function(move |context : &mlua::Lua,()|{
            let mut borrowed = iter.borrow_mut();

            match borrowed.next(){
                Some(Ok(entry)) => {
                    let path = entry.path();
                   
                    if path.is_file() {
                        let file = vara_file::new(&path)?;
                        Ok(Some(context.create_userdata(file)?))
                    } else if path.is_dir() {
                        let directory = Directory::new(&path)?;
                        Ok(Some(context.create_userdata(directory)?))
                    } else {
                        Ok(None)
                    }
                }
                Some(Err(err)) => Err(mlua::Error::RuntimeError(err.to_string())),
                None => Ok(None),
            }
        })
    }
    
     #[lua(meta,name="__tostring")]
    fn to_string(&self) -> mlua::Result<String> {

        Ok(format!("directory {} ",self.path.display()))
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
    #[lua(getter,name="exists")]
    fn does_exist(&self) -> mlua::Result<bool> {
        Ok(self.path.exists())
    }
}