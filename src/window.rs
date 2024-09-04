use windows::core::*;
use windows::Win32::{
    Foundation::*,
    UI::WindowsAndMessaging::*,
    System::LibraryLoader::*,
    Graphics::Gdi::*,
};
use std::mem::zeroed;
use crate::*;

#[derive(Default)]
pub struct StateData {
    text: HSTRING,
}

pub(crate) struct HSTRData {
    pub(crate) open: HSTRING,
    pub(crate) save: HSTRING,
    pub(crate) exit: HSTRING,
    pub(crate) file: HSTRING,
    pub(crate) btn_txt: HSTRING,
    pub(crate) clicked: HSTRING,
}

impl HSTRData {
    pub(crate) fn new() -> Self {
        Self {
            open: HSTRING::from("開啟"),
            save: HSTRING::from("儲存"),
            exit: HSTRING::from("離開"),
            file: HSTRING::from("檔案"),
            btn_txt: HSTRING::from("好"),
            clicked: HSTRING::from("你按了!")
        }
    }
}

pub struct Wndrs {
    handle: HWND,
    title: HSTRING, 
    state: StateData,
    local: HSTRData,
}

impl Wndrs {
    const ID_BTN_LOAD: usize = 1;
    const ID_MENU_OPEN: usize = 2;
    const ID_MENU_SAVE: usize = 3;
    const ID_MENU_EXIT: usize = 4;

    pub fn new(t: &str) -> Result<Self> {
        Ok(
            Wndrs { 
                handle: Default::default(),
                title: HSTRING::from(t),
                state: StateData { text: HSTRING::from("測試RUST中文介面") },
                local: HSTRData::new(),
            }
        )
    }

    extern "system" fn wndproc(
        window: HWND,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        unsafe {
            if message == WM_NCCREATE {
                let cs = lparam.0 as *const CREATESTRUCTA;
                let this = (*cs).lpCreateParams as *mut Self;
                (*this).handle = window;

                SetWindowLongPtrW(window, GWLP_USERDATA, this as _);
            } else {
                let this = GetWindowLongPtrW(window, GWLP_USERDATA) as *mut Self;

                if !this.is_null() {
                    return (*this).message_handler(message, wparam, lparam);
                }
            }

            DefWindowProcW(window, message, wparam, lparam)
        }
    }

    fn message_handler(
        &mut self, message: u32, wparam: WPARAM, lparam: LPARAM
    ) -> LRESULT {
        unsafe {
            match message {
                WM_DESTROY => {
                    PostQuitMessage(0);
                    LRESULT(0)
                },
                WM_PAINT => {
                    let mut ps: PAINTSTRUCT = zeroed();
                    let hdc = BeginPaint(self.handle, &mut ps);
                    let mut rect: RECT = zeroed();
                    GetClientRect(self.handle, &mut rect).unwrap();
                    FillRect(hdc, &rect, GetSysColorBrush(COLOR_WINDOW));
                    DrawTextW(
                        hdc,
                        self.state.text.as_wide().to_vec().as_mut(),
                        &mut rect,
                        DT_CENTER | DT_VCENTER | DT_SINGLELINE | DT_WORD_ELLIPSIS,
                    );
                    EndPaint(self.handle, &ps).unwrap();
                    LRESULT(0)
                },
                WM_COMMAND => {
                    match wparam.0 as usize {
                        Self::ID_MENU_OPEN => {
                            popup::pop_info(&self.local.open);
                        },
                        Self::ID_MENU_SAVE => {
                            popup::pop_info(&self.local.save);
                        },
                        Self::ID_BTN_LOAD => {
                            popup::pop_info(&self.local.clicked);
                        },
                        Self::ID_MENU_EXIT => {
                            PostQuitMessage(0);
                        }
                        _ => (),
                    }
                    LRESULT(0)
                },
                _ => DefWindowProcW(self.handle, message, wparam, lparam),
            }
        }
    }

    pub fn create_window(&mut self) -> Result<()> {
        unsafe {
            let instance = GetModuleHandleW(None)?;

            let window_class = w!("window");
            let wc = WNDCLASSW {
                hCursor: LoadCursorW(None, IDC_ARROW)?,
                hInstance: instance.into(),
                lpszClassName: window_class,
                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(Self::wndproc),
                ..Default::default()
            };

            let atom = RegisterClassW(&wc);
            debug_assert!(atom != 0);

            let handle = CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                window_class,
                PCWSTR::from_raw(self.title.as_ptr()),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                None,
                None,
                instance,
                Some(self as *mut _ as _),
            )?;
            
            self.creat_button(
                handle, 
                instance, 
                win_str::hstr_to_pcwstr(&self.local.btn_txt), 
                10, 
                10, 
                80, 
                80,
                HMENU(Self::ID_BTN_LOAD as _)
            )?;
            
            // Attach the menu to the window
            let hmenu = self.create_menu()?;
            SetMenu(handle, hmenu)?;

            debug_assert!(!handle.is_invalid());
            debug_assert!(handle == self.handle);
            let mut message = MSG::default();

            while GetMessageW(&mut message, None, 0, 0).into() {
                // translates keystrokes (key down, key up) into characters
                let _ = TranslateMessage(&message);
                DispatchMessageW(&message);
            }
            Ok(())
        }
    }

    fn create_menu(&self) -> Result<HMENU> {
        unsafe {
            // Create the main menu
            let hmenu = CreateMenu()?;

            // Create a submenu for "File"
            let hmenu_file = CreatePopupMenu()?;
            AppendMenuW(
                hmenu_file, 
                MF_STRING, 
                Self::ID_MENU_OPEN, 
                win_str::hstr_to_pcwstr(&self.local.open)
            )?;
            AppendMenuW(
                hmenu_file, 
                MF_STRING, 
                Self::ID_MENU_SAVE, 
                win_str::hstr_to_pcwstr(&self.local.save)
            )?;
            AppendMenuW(hmenu_file, MF_SEPARATOR, 0, win_str::str_to_pcwstr(""))?;
            AppendMenuW(
                hmenu_file, 
                MF_STRING, 
                Self::ID_MENU_EXIT, 
                win_str::hstr_to_pcwstr(&self.local.exit)
            )?;

            // Attach the submenu to the main menu
            AppendMenuW(hmenu, 
                MF_POPUP, 
                hmenu_file.0 as usize, 
                win_str::hstr_to_pcwstr(&self.local.file)
            )?;

            // Return the menu handle
            Ok(hmenu)
        }
    }

    fn creat_button(
        &self,
        handle: HWND, 
        instance: HMODULE,
        btn_txt: PCWSTR,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        btn_id: HMENU,
    ) -> Result<()> {
        unsafe {
            // create a button
            CreateWindowExW( 
                WINDOW_EX_STYLE::default(),
                w!("BUTTON"),  // Predefined class; Unicode assumed 
                btn_txt,      // Button text 
                WINDOW_STYLE(WS_TABSTOP.0 | WS_VISIBLE.0 | WS_CHILD.0 | BS_DEFPUSHBUTTON as u32),  // Styles 
                x,         // x position 
                y,         // y position 
                width,        // Button width
                height,        // Button height
                handle,       // Parent window
                btn_id, // BUTTON_ID as menu.
                instance, 
                None // Pointer not needed.
            )?;      
            Ok(())
        }
    }
}