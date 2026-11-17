use mlua::ExternalResult;
use windows::{
    Win32::{
        Foundation::*, UI::WindowsAndMessaging::*,
    }, core::*
};

use crate::win_ui::DestroyQueue;

pub struct UiWindowInternal{
    pub hwnd: HWND,
    pub id: usize,
    destroy_queue: DestroyQueue,
    close_queue: DestroyQueue,
    pub close_pending: bool,

    pub resized: bool,
    
    min_size_x: i32,
    min_size_y: i32,
    max_size_x: i32,
    max_size_y: i32,
}

impl UiWindowInternal {
    pub fn new(destroy_queue: DestroyQueue,close_queue: DestroyQueue,instance: HINSTANCE,class_name: PCWSTR,title: PCWSTR,min_size: mlua::Vector,max_size: mlua::Vector) -> mlua::Result<Box<Self>> {
        unsafe{
            let mut window = Box::new(Self {
                hwnd: HWND::default(),
                id: 0,
                destroy_queue,
                close_queue,
                close_pending: false,
                resized: false,
                min_size_x: min_size.x() as i32,
                min_size_y: min_size.y() as i32,
                max_size_x: max_size.x() as i32,
                max_size_y: max_size.y() as i32,
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

    pub fn window_proc(&mut self,hwnd: HWND,msg: u32,wparam: WPARAM,lparam: LPARAM) -> LRESULT {
        unsafe{
            match msg  {
                WM_COMMAND => {
                    let _control_id = wparam.0 & 0xffff;
                    let _notification = (wparam.0 >> 16) & 0xffff;
                
                    DefWindowProcW(hwnd, msg, wparam, lparam)
                },

                WM_SIZE => {
                    self.resized = true;
                    LRESULT(0)
                }

                WM_GETMINMAXINFO => {
                    if self.min_size_x == 0 || self.min_size_y == 0 || self.max_size_x == 0 || self.max_size_y == 0 {
                        return DefWindowProcW(hwnd, msg, wparam, lparam);
                    }

                    let mmi = &mut *(lparam.0 as *mut MINMAXINFO);

                    mmi.ptMinTrackSize.x = self.min_size_x;
                    mmi.ptMinTrackSize.y = self.min_size_y;
                    mmi.ptMaxTrackSize.x = self.max_size_x;
                    mmi.ptMaxTrackSize.y = self.max_size_y;

                    LRESULT(0)
                }

                WM_CLOSE => {
                    if !self.close_pending{
                        self.close_queue.borrow_mut().push(self.id);
                        self.close_pending = true
                    }
                    LRESULT(0)
                }

                WM_DESTROY => {
                    self.destroy_queue.borrow_mut().push(self.id);
                    
                    LRESULT(0)
                }

                WM_NCDESTROY => {
                    SetWindowLongPtrW(hwnd, GWLP_USERDATA,0);
                    DefWindowProcW(hwnd, msg, wparam, lparam)
                }

                _ => DefWindowProcW(hwnd, msg, wparam, lparam),
            }
        }
    }
}