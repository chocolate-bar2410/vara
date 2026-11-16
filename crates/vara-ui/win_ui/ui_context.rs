
use mlua::{ExternalError, ExternalResult};
use windows::{
    Win32::{
        Foundation::*, Graphics::Gdi::{COLOR_WINDOW, HBRUSH}, System::LibraryLoader::GetModuleHandleW, UI::WindowsAndMessaging::*,
    }, core::*
};

const BUTTON_ID : usize = 1001;

#[derive(mlua::UserData)]
pub struct UiApplication{

}

#[mlua::userdata_impl]
impl UiApplication {
    #[lua(skip)]
    pub fn new() -> Self {
        Self {}
    }

    fn test(&self) -> mlua::Result<()> {
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

            #[allow(unused)]
            ShowWindow(hwnd, SW_SHOW);

            let mut message = MSG::default();

            while GetMessageW(&mut message,None,0,0).as_bool() {
                let _ = TranslateMessage(&message);
                DispatchMessageW(&message);
            }
        }
        
        Ok(())
    }
}

extern "system" fn window_proc(hwnd: HWND,msg: u32,wparam: WPARAM,lparam: LPARAM) -> LRESULT{
    unsafe{
        match msg {
            WM_COMMAND => {
                let control_id = wparam.0 & 0xffff;
                let notification = (wparam.0 >> 16) & 0xffff;

                if control_id == BUTTON_ID && notification == BN_CLICKED as usize {
                    println!("button clicked");
                    return LRESULT(0);
                }
            
                DefWindowProcW(hwnd, msg, wparam, lparam)
            },

            WM_DESTROY => {
                PostQuitMessage(0);
                LRESULT(0)
            },

            _ => DefWindowProcW(hwnd, msg, wparam, lparam),
        }
    }
}