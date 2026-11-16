
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use mlua::{ExternalError, ExternalResult};
use windows::{
    Win32::{
        Foundation::*, Graphics::Gdi::{COLOR_WINDOW, HBRUSH}, System::LibraryLoader::GetModuleHandleW, UI::WindowsAndMessaging::*,
    }, core::*
};

use crate::win_ui::{ui_window::UiWindowInternal, window_handle::UiWindow};

#[derive(mlua::UserData)]
pub struct UiApplication{
    #[lua(skip)]
    class_name: PCWSTR,
    #[lua(skip)]
    instance: HINSTANCE,
    #[lua(skip)]
    windows: HashMap<usize,Box<UiWindowInternal>>,
    #[lua(skip)]
    destroy_queue: Rc<RefCell<Vec<usize>>>
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
                destroy_queue: Rc::new(RefCell::new(Vec::new()))
            })
        }
    }

    #[lua(name="CreateWindow")]
    fn create_window(&mut self,title: String) -> mlua::Result<UiWindow> {

        let title = HSTRING::from(title);

        let mut window = UiWindowInternal::new(self.destroy_queue.clone(),self.instance, self.class_name,PCWSTR(title.as_ptr()))?;

        let id = self.windows.len() + 1;
        window.id = id;

        let handle = window.gen_handle();

        self.windows.insert(id, window);

        Ok(handle)
    }

    #[lua(name="Run")]
    fn run(&mut self) -> mlua::Result<()>{
        unsafe{
            let mut message = MSG::default();

            loop {
                let result = GetMessageW(&mut message,None,0,0);

                if result.0 <= 0 {break;}

                #[allow(unused)]
                TranslateMessage(&message);
                DispatchMessageW(&message);

                self.process_destroyed();

                if self.windows.is_empty() {
                    break;
                }
            }

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