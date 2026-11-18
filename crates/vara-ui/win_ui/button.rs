use windows::{
    Win32::{
        Foundation::*, UI::WindowsAndMessaging::*,
    }, core::*
};
pub struct UiButton{
    pub hwnd: HWND,
}

impl UiButton {
    pub fn new(instance: HINSTANCE,parent_hwnd: HWND,id: usize,text: PCWSTR,position: mlua::Vector, size: mlua::Vector) -> Result<Self> {
        unsafe {
            let hwnd = CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                w!("BUTTON"),
                text,
                WS_CHILD | WS_VISIBLE | WS_TABSTOP | WINDOW_STYLE(BS_DEFPUSHBUTTON as u32),
                position.x() as i32,
                position.y() as i32,
                size.x() as i32,
                size.y() as i32,
                Some(parent_hwnd),
                Some(HMENU(id as _)),
                Some(instance),
                None
            )?;

            Ok(Self{hwnd})
        }
    }

}