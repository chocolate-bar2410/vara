use std::{
    path::PathBuf,
    rc::Rc,
    cell::RefCell
};

use mlua::ExternalResult;
use mlua_luau_scheduler::LuaSpawnExt;

use notify::{
    EventKind,
    RecursiveMode,
    Watcher,
    recommended_watcher,
};

type CallbackSlot = Rc<RefCell<Option<mlua::RegistryKey>>>;

#[derive(mlua::UserData)]
pub struct FsObserver{
    #[lua(name="Path")]
    file_path: PathBuf,
    #[lua(name="Recursive")]
    recursive: bool,

    #[lua(skip)]
    on_created: CallbackSlot,
    #[lua(skip)]
    on_modified: CallbackSlot,
    #[lua(skip)]
    watcher: Option<notify::RecommendedWatcher>,
    #[lua(skip)]
    cancel: async_channel::Sender<()>,
    
}

fn get_callback(context: &mlua::Lua,slot: &CallbackSlot) -> mlua::Result<Option<mlua::Function>>{
    let slot = slot.borrow();

    match slot.as_ref() {
        Some(key) => context.registry_value(key).map(Some),
        None => Ok(None)
    }
}

fn set_callback(context: &mlua::Lua,slot: &CallbackSlot,callback_fn: mlua::Function) -> mlua::Result<()>{
    let mut slot = slot.borrow_mut();

    if let Some(key) = slot.as_mut(){
        context.replace_registry_value(key, callback_fn)
    } else {
        *slot = Some(context.create_registry_value(callback_fn)?);
        Ok(())
    }
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
            on_created: Rc::new(RefCell::new(None)),
            on_modified: Rc::new(RefCell::new(None)),
            watcher: Some(watcher),
            cancel: cancel_tx
        };

        let created_slot = observer.on_created.clone();
        let modified_slot = observer.on_modified.clone();

        let vm = context.clone();

        context.spawn_local(async move {
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

                let slot = match event.kind{
                    EventKind::Create(_) => &created_slot,
                    EventKind::Modify(_) => &modified_slot,
                    _ => continue,
                };

                match get_callback(&vm, slot) {
                    Ok(Some(callback)) => {

                        if let Err(err) = callback.call::<()>(event.paths) {
                            eprintln!("FsObserver error: {err}"); 
                        }
                    }

                    Ok(None) => {},
                    Err(err) => {
                        eprintln!("FsObserver error: {err}"); 
                        continue;
                    }
                };
    
            }

        });

        Ok(observer)
    }

    #[lua(name="Disconnect")]
    fn disconnect(&mut self) -> mlua::Result<()>{

        #[allow(unused)]
        self.cancel.try_send(());
        self.watcher.take();
        Ok(())
    }

    // getters    

    #[lua(getter,name="OnCreated")]
    fn get_on_created(&self,context: &mlua::Lua) -> mlua::Result<Option<mlua::Function>> {
        get_callback(context,&self.on_created)
    }

    #[lua(getter,name="OnModified")]
    fn get_on_modified(&self,context: &mlua::Lua) -> mlua::Result<Option<mlua::Function>> {
        get_callback(context,&self.on_modified)
    }

    // setters

    #[lua(setter,name="OnCreated")]
    fn set_on_created(&mut self,context: &mlua::Lua,callback_fn: mlua::Function) -> mlua::Result<()> {
        set_callback(context,&self.on_created,callback_fn)
    }

    #[lua(setter,name="OnModified")]
    fn set_on_modified(&mut self,context: &mlua::Lua,callback_fn: mlua::Function) -> mlua::Result<()> {
        set_callback(context,&self.on_modified,callback_fn)
    }


    #[lua(meta,name="__tostring")]
    pub fn to_string(&self) -> mlua::Result<String>{
        Ok(format!("fs observer {}",self.file_path.display()))
    }

}
