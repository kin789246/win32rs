pub mod window;
pub mod popup;
pub mod win_str;

#[cfg(test)]
mod tests {
    use windows::{
        Win32::UI::HiDpi::*,
        core::*,
    };
    use crate::{popup, window::Wndrs};

    #[test]
    fn create_wnd() -> Result<()> {
        unsafe {
            SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2)
                .unwrap();
            // SetProcessDpiAwareness(PROCESS_PER_MONITOR_DPI_AWARE)?;
        }
            if let Ok(mut wnd) = Wndrs::new("中文測試") {
                wnd.create_window().unwrap();
            }
        Ok(())
    }

    #[test]
    fn file_open() {
        unsafe {
            SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2)
                .unwrap();
        }
        popup::file_open().unwrap();
    }
}
