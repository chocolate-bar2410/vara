use std::{cell::RefCell, rc::Rc};

use windows::Win32::{Foundation::HWND};

use crate::win_ui::{CallbackSlot, DestroyQueue, event_manager::{EventManager, EventType}};

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
    event_manager: Rc<RefCell<EventManager>>,
}

#[mlua::userdata_impl]
impl UiWindow {
    #[lua(skip)]
    pub fn new(hwnd: HWND,id: usize,close_queue: DestroyQueue,event_manager: Rc<RefCell<EventManager>>) -> Self{
        Self { 
            hwnd, 
            id, 
            close_queue,
            destroyed: false,
            event_manager
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
        self.event_manager.borrow().get(context,EventType::CLOSE,self.id)
    }

    #[lua(getter,name="OnResize")]
    fn get_on_resize(&self,context: &mlua::Lua) -> mlua::Result<Option<mlua::Function>> {
        self.event_manager.borrow().get(context,EventType::RESIZE,self.id)
    }

    // setters
    #[lua(setter,name="OnClose")]
    fn set_on_close(&mut self,context: &mlua::Lua,callback_fn: mlua::Function) -> mlua::Result<()> {
        self.event_manager.borrow_mut().set(context,EventType::CLOSE,self.id,callback_fn)
    }

    #[lua(setter,name="OnResize")]
    fn set_on_resize(&mut self,context: &mlua::Lua,callback_fn: mlua::Function) -> mlua::Result<()> {
        self.event_manager.borrow_mut().set(context,EventType::RESIZE,self.id,callback_fn)
    }

}