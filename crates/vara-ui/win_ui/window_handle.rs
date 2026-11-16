use windows::Win32::{Foundation::HWND};

use crate::win_ui::{CallbackSlot, DestroyQueue};

#[derive(mlua::UserData)]
pub struct UiWindow{
    #[lua(skip)]
    hwnd: HWND,
    #[lua(skip)]
    pub id: usize,  
    #[lua(skip)]
    close_queue: DestroyQueue,
    #[lua(name="Destroyed")]
    destroyed: bool,
    #[lua(skip)]
    on_close: CallbackSlot,
}

#[mlua::userdata_impl]
impl UiWindow {
    #[lua(skip)]
    pub fn new(hwnd: HWND,id: usize,close_queue: DestroyQueue,on_close: CallbackSlot) -> Self{
        Self { 
            hwnd, 
            id, 
            close_queue,
            destroyed: false,
            on_close
        }
    }

    #[lua(name="Close")]
    fn close(&mut self) -> mlua::Result<()>{
        if self.destroyed {
            return Ok(());
        }

        self.close_queue.borrow_mut().push(self.id);

        Ok(())
    }

    // getters
    #[lua(getter,name="OnClose")]
    fn get_on_close(&self,context: &mlua::Lua) -> mlua::Result<Option<mlua::Function>> {
        get_callback(context,&self.on_close)
    }

    // setters
    #[lua(setter,name="OnClose")]
    fn set_on_close(&mut self,context: &mlua::Lua,callback_fn: mlua::Function) -> mlua::Result<()> {
        set_callback(context,&self.on_close,callback_fn)
    }

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