use windows::Win32::{Foundation::HWND, UI::WindowsAndMessaging::DestroyWindow};

use crate::win_ui::DestroyQueue;

#[derive(mlua::UserData)]
pub struct UiWindow{
    #[lua(skip)]
    hwnd: HWND,
    #[lua(skip)]
    pub id: usize,  
    #[lua(skip)]
    destroy_queue: DestroyQueue,
    #[lua(name="Destroyed")]
    destroyed: bool,
}

#[mlua::userdata_impl]
impl UiWindow {
    #[lua(skip)]
    pub fn new(hwnd: HWND,id: usize,destroy_queue: DestroyQueue) -> Self{
        Self { 
            hwnd, 
            id, 
            destroy_queue,
            destroyed: false,
        }
    }

    #[lua(infallible)]
    fn close(&mut self){
        if self.destroyed {
            return;
        }

        unsafe {
            #[allow(unused)]
            DestroyWindow(self.hwnd);
        }

        self.destroy_queue.borrow_mut().push(self.id);
    }
}