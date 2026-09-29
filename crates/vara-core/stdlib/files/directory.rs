use std::path::PathBuf;
use std::time::UNIX_EPOCH;

use mlua::ExternalError;

use crate::stdlib::chrono::datetime::DateTime as vara_datetime;

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