use std::path::PathBuf;

use mlua::ExternalResult;
use mlua_luau_scheduler::LuaSpawnExt;
use notify::{
    Watcher,
    RecursiveMode,
    recommended_watcher,
};

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
    #[lua(skip)]
    watcher: notify::RecommendedWatcher,
    #[lua(skip)]
    cancel: async_channel::Sender<()>,
    
}

#[mlua::userdata_impl]
impl FsObserver {
    #[lua(skip)]
    pub fn new(context: &mlua::Lua,file_path: PathBuf,recursive: bool) -> mlua::Result<Self> {

        let (tx,rx) = async_channel::unbounded();
        let (cancel_tx,cancel_rx) = async_channel::bounded::<()>(1);

        let recursive_mode = if recursive {RecursiveMode::Recursive} else {RecursiveMode::NonRecursive};

        let mut watcher = recommended_watcher(move |result| {
            let _ = tx.send_blocking(result);
        }).into_lua_err()?;

        watcher.watch(&file_path, recursive_mode).into_lua_err()?;

        let observer = Self {
            file_path,
            recursive,
            on_created: None,
            on_modified: None,
            watcher,
            cancel: cancel_tx
        };

        context.spawn(async move {
            loop {
                let result = futures_lite::future::race(
                    async {rx.recv().await.ok()},
                    async {
                        #[allow(unused)]
                        cancel_rx.recv().await; 
                        None
                    },
                ).await;


                let Some(result) = result else {
                    break;
                };

                let event = match result {
                    Err(err) => {
                        eprintln!("FsObserver error: {err}"); 
                        continue;
                    },
                    Ok(event) => event,
                };

                println!("{:?}",event);
                
            }
        }).detach();

        Ok(observer)
    }

    #[lua(name="Disconnect")]
    fn disconnect(&self) -> mlua::Result<()>{

        #[allow(unused)]
        self.cancel.try_send(());
        Ok(())
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
