use std::{cell::RefCell, rc::Rc};

use mlua::ExternalResult;
use windows::{
    Win32::{
        Foundation::*, UI::WindowsAndMessaging::*,
    }, core::*
};

use crate::win_ui::window_handle::UiWindow;

pub struct UiWindowInternal{
    pub hwnd: HWND,
    pub id: usize,
    destroy_queue: Rc<RefCell<Vec<usize>>>
}

impl UiWindowInternal {
    pub fn new(destroy_queue: Rc<RefCell<Vec<usize>>>,instance: HINSTANCE,class_name: PCWSTR,title: PCWSTR) -> mlua::Result<Box<Self>> {
        unsafe{
            let mut window = Box::new(Self {
                hwnd: HWND::default(),
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
                Some(&mut *window as *mut UiWindowInternal as *const _)
            ).into_lua_err()?;

            window.hwnd = hwnd;

            Ok(window)
        }
    }

    pub fn close(&self){
        unsafe{
            #[allow(unused)]
            DestroyWindow(self.hwnd);
        }
    }

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
                    self.destroy_queue.borrow_mut().push(self.id);
                    
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

    pub fn gen_handle(&self) -> UiWindow {
        UiWindow::new(self.hwnd, self.id, self.destroy_queue.clone())
    }
}