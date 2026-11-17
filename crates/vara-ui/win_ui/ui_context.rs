
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use mlua::{ExternalError, ExternalResult};
use smol::Timer;
use windows::{
    Win32::{
        Foundation::*, Graphics::Gdi::{COLOR_WINDOW, HBRUSH}, System::LibraryLoader::GetModuleHandleW, UI::WindowsAndMessaging::*,
    }, core::*
};

use crate::win_ui::{CallbackSlot, DestroyQueue, ui_window::UiWindowInternal, window_handle::UiWindow};

#[derive(mlua::UserData)]
pub struct UiApplication{
    #[lua(skip)]
    class_name: PCWSTR,
    #[lua(skip)]
    instance: HINSTANCE,
    #[lua(skip)]
    windows: HashMap<usize,Box<UiWindowInternal>>,
    #[lua(skip)]
    destroy_queue: DestroyQueue,
    #[lua(skip)]
    close_queue: DestroyQueue
}

#[mlua::userdata_impl]
impl UiApplication {
    #[lua(skip)]
    pub fn new() -> mlua::Result<Self> {
        unsafe {
            let instance = GetModuleHandleW(None).into_lua_err()?.into();

            let class_name = w!("MyRustWindow");

            let wc = WNDCLASSW {
                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(window_proc),
                hInstance: instance,
                lpszClassName: class_name,
                hCursor: LoadCursorW(None,IDC_ARROW).into_lua_err()?,
                hbrBackground: HBRUSH((COLOR_WINDOW.0 + 1) as _),
                ..Default::default()
            };

            if RegisterClassW(&wc) == 0 {
                let err = GetLastError();

                if err != ERROR_CLASS_ALREADY_EXISTS {
                    return Err(format!("failed to register application: {:?}",err).into_lua_err());
                }
            }

            Ok(Self {
                instance,
                class_name,
                windows: HashMap::new(),
                destroy_queue: Rc::new(RefCell::new(Vec::new())),
                close_queue: Rc::new(RefCell::new(Vec::new()))
            })
        }
    }

    #[lua(name="CreateWindow")]
    fn create_window(&mut self,title: String) -> mlua::Result<UiWindow> {

        let title = HSTRING::from(title);

        let on_close = Rc::new(RefCell::new(None));

        let mut window = UiWindowInternal::new(self.destroy_queue.clone(),self.close_queue.clone(),self.instance, self.class_name,PCWSTR(title.as_ptr()),on_close)?;

        let id = self.windows.len() + 1;
        window.id = id;

        let handle = window.gen_handle();

        self.windows.insert(id, window);

        Ok(handle)
    }

    #[lua(name="Run")]
    async fn run(&mut self,context: &mlua::Lua) -> mlua::Result<()>{
        let mut message = MSG::default();

        while !self.windows.is_empty() {

            loop {
                let has_message = unsafe{PeekMessageW(&mut message,None,0,0,PM_REMOVE).as_bool()};

                if !has_message {break;}

                unsafe{
                    #[allow(unused)]
                    TranslateMessage(&message);
                    DispatchMessageW(&message);
                }
            }

            self.process_closed(context).await?;
            self.process_destroyed();

            Timer::after(std::time::Duration::from_millis(10)).await;
        }

        Ok(())
    }

    #[lua(skip)]
    pub fn process_destroyed(&mut self){
        let destroyed = {
            let mut queue = self.destroy_queue.borrow_mut();
            std::mem::take(&mut *queue)
        };

        for id in destroyed {
            self.windows.remove(&id);
        }
    }

    #[lua(skip)]
    pub async fn process_closed(&mut self,context: &mlua::Lua) -> mlua::Result<()>{
        let closed = {
            let mut queue = self.close_queue.borrow_mut();
            std::mem::take(&mut *queue)
        };

        for id in closed {
            let Some(window) = self.windows.get_mut(&id) else {
                unreachable!()
            };

            if let Some(close_fn) = get_callback(context, &window.on_close)? {
                let should_close = close_fn.call_async::<Option<bool>>(()).await?;
                
                match should_close {
                    Some(should_close) => if should_close{
                        unsafe {
                            DestroyWindow(window.hwnd).into_lua_err()?;
                        }
                    } else {
                        window.close_pending = false;
                    },
                    None => unsafe {
                        DestroyWindow(window.hwnd).into_lua_err()?;
                    }
                }
            }
        }

        Ok(())
    }

    /*
    fn test(&self) -> mlua::Result<()> {
        unsafe {
            let hwnd = CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                class_name,
                w!("VaraWindow"),
                WS_OVERLAPPEDWINDOW,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                500,
                300,
                None,
                None,
                Some(instance),
                None
            ).into_lua_err()?;
            
            CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                w!("STATIC"),
                w!("hello world"),
                WS_CHILD | WS_VISIBLE,
                20,
                20,
                300,
                30,
                Some(hwnd),
                None,
                Some(instance),
                None
            ).into_lua_err()?;

            CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                w!("BUTTON"),
                w!("tesing button 123"),
                WS_CHILD | WS_VISIBLE | WINDOW_STYLE(BS_PUSHBUTTON as u32),
                20,
                70,
                120,
                30,
                Some(hwnd),
                Some(HMENU(BUTTON_ID as _)),
                Some(instance),
                None
            ).into_lua_err()?;
        }
        
        Ok(())
    }
     */
}

extern "system" fn window_proc(hwnd: HWND,msg: u32,wparam: WPARAM,lparam: LPARAM) -> LRESULT{
    unsafe{
        if msg == WM_NCCREATE{
            let create_struct = &*(lparam.0 as *const CREATESTRUCTW);

            SetWindowLongPtrW(hwnd, GWLP_USERDATA,create_struct.lpCreateParams as isize);
        }

        let ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA);

        if ptr == 0 {
            return DefWindowProcW(hwnd, msg, wparam, lparam);
        }

        let window = &mut *(ptr as *mut UiWindowInternal);

        window.window_proc(hwnd,msg,wparam,lparam)
    }
}

fn get_callback(context: &mlua::Lua,slot: &CallbackSlot) -> mlua::Result<Option<mlua::Function>>{
    let slot = slot.borrow();

    match slot.as_ref() {
        Some(key) => context.registry_value(key).map(Some),
        None => Ok(None)
    }
}