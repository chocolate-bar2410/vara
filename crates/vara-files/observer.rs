use std::path::PathBuf;

#[derive(mlua::UserData)]
pub struct FsObserver{
    #[lua(name="Path")]
    file_path: PathBuf,
    #[lua(name="Recursive")]
    recursive: bool,
    #[lua(skip)]
    on_created: Option<mlua::RegistryKey>,
    #[lua(skip)]
    on_modified: Option<mlua::RegistryKey>,
}

#[mlua::userdata_impl]
impl FsObserver {
    #[lua(skip)]
    pub fn new(file_path: PathBuf,recursive: bool) -> Self {
        Self {
            file_path,
            recursive,
            on_created: None,
            on_modified: None,
        }
    }

    // getters

    #[lua(getter,name="OnCreated")]
    fn get_on_created(&self,context: &mlua::Lua) -> mlua::Result<Option<mlua::Function>> {
        let Some(on_created) = &self.on_created else {
            return Ok(None);
        };

        context.registry_value(&on_created)
    }

    #[lua(getter,name="OnModified")]
    fn get_on_modified(&self,context: &mlua::Lua) -> mlua::Result<Option<mlua::Function>> {
        let Some(on_modified) = &self.on_modified else {
            return Ok(None);
        };

        context.registry_value(&on_modified)
    }

    // setters

    #[lua(setter,name="OnCreated")]
    fn set_on_created(&mut self,context: &mlua::Lua,callback_fn: mlua::Function) -> mlua::Result<()> {

        if let Some(mut on_created) = self.on_created.as_mut() {
            context.replace_registry_value(&mut on_created, callback_fn)
        } else {
            self.on_created = Some(context.create_registry_value(callback_fn)?);

            return Ok(());
        }
    }

    #[lua(setter,name="OnModified")]
    fn set_on_modified(&mut self,context: &mlua::Lua,callback_fn: mlua::Function) -> mlua::Result<()> {

        if let Some(mut on_modified) = self.on_modified.as_mut() {
            context.replace_registry_value(&mut on_modified, callback_fn)
        } else {
            self.on_modified = Some(context.create_registry_value(callback_fn)?);

            return Ok(());
        }
    }


    #[lua(meta,name="__tostring")]
    pub fn to_string(&self) -> mlua::Result<String>{
        Ok(format!("fs observer {}",self.file_path.display()))
    }

}
