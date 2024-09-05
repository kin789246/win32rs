pub mod window;
pub mod dialog;
pub mod win_str;
pub mod control;

#[cfg(test)]
mod tests {
    use windows::{
        Win32::UI::HiDpi::*,
        core::*,
    };
    use crate::{dialog, window::Wndrs};

    #[test]
    fn create_wnd() -> Result<()> {
        unsafe {
            SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2)
                .unwrap();
            // SetProcessDpiAwareness(PROCESS_PER_MONITOR_DPI_AWARE)?;
        }
            if let Ok(mut wnd) = Wndrs::new("中文測試") {
                wnd.build().unwrap();
            }
        Ok(())
    }

    #[test]
    fn file_open() {
        unsafe {
            SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2)
                .unwrap();
        }
        dialog::file_open().unwrap();
    }
}
