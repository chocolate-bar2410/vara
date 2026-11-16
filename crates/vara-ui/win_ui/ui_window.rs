use std::{cell::RefCell, rc::Rc};

use mlua::ExternalResult;
use windows::{
    Win32::{
        Foundation::*, UI::WindowsAndMessaging::*,
    }, core::*
};



#[derive(mlua::UserData)]
pub struct UiWindow{
    #[lua(skip)]
    hwnd: HWND,
    #[lua(get)]
    destroyed: bool,
    #[lua(get)]
    pub id: usize,
    #[lua(skip)]
    destroy_queue: Rc<RefCell<Vec<usize>>>
}

#[mlua::userdata_impl]
impl UiWindow {
    #[lua(skip)]
    pub fn new(destroy_queue: Rc<RefCell<Vec<usize>>>,instance: HINSTANCE,class_name: PCWSTR,title: PCWSTR) -> mlua::Result<Box<Self>> {
        unsafe{
            let mut window = Box::new(Self {
                hwnd: HWND::default(),
                destroyed: false,
                id: 0,
                destroy_queue
            });

            let hwnd = CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                class_name,
                title,
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                500,
                300,
                None,
                None,
                Some(instance),
                Some(&mut *window as *mut UiWindow as *const _)
            ).into_lua_err()?;

            window.hwnd = hwnd;

            Ok(window)
        }
    }

    #[lua(infallible)]
    pub fn close(&self){
        unsafe{
            #[allow(unused)]
            DestroyWindow(self.hwnd);
        }
    }

    #[lua(skip)]
    pub fn window_proc(&mut self,hwnd: HWND,msg: u32,wparam: WPARAM,lparam: LPARAM) -> LRESULT {
        unsafe{
            match msg  {
                WM_COMMAND => {
                    let _control_id = wparam.0 & 0xffff;
                    let _notification = (wparam.0 >> 16) & 0xffff;
                
                    DefWindowProcW(hwnd, msg, wparam, lparam)
                },

                WM_CLOSE => {
                    self.close();
                    LRESULT(0)
                }

                WM_DESTROY => {
                    if !self.destroyed {
                        self.destroyed = true;
                        self.destroy_queue.borrow_mut().push(self.id);
                    }

                    return LRESULT(0)
                }

                WM_NCDESTROY => {
                    SetWindowLongPtrW(hwnd, GWLP_USERDATA,0);
                    return DefWindowProcW(hwnd, msg, wparam, lparam);
                }

                _ => DefWindowProcW(hwnd, msg, wparam, lparam),
            }
        }
    }
}