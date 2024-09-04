use windows::core::*;
use windows::Win32::{
    UI::{
        WindowsAndMessaging::*,
        Shell::*,
        Shell::Common::*,
    },
    System::Com::*,
};

use crate::*;

pub fn pop_yesno(msg: &HSTRING) -> MESSAGEBOX_RESULT {
    unsafe {
        MessageBoxW(
            None,
            win_str::hstr_to_pcwstr(msg),
            w!("Question"),
            MB_YESNO | MB_ICONQUESTION,
        )
    }
}

pub fn pop_info(msg: &HSTRING) -> MESSAGEBOX_RESULT {
    unsafe {
        MessageBoxW(
            None,
            win_str::hstr_to_pcwstr(msg),
            w!("Information"),
            MB_OK | MB_ICONINFORMATION,
        )
    }
}

pub fn pop_error(msg: &HSTRING) -> MESSAGEBOX_RESULT {
    unsafe {
        MessageBoxW(
            None,
            win_str::hstr_to_pcwstr(msg),
            w!("Information"),
            MB_OK | MB_ICONERROR,
        )
    }
}

pub fn file_open() -> Result<()> {
    unsafe {
        CoIncrementMTAUsage()?;
        let dialog: IFileOpenDialog = CoCreateInstance(&FileOpenDialog, None, CLSCTX_ALL)?;

        dialog.SetFileTypes(&[
            COMDLG_FILTERSPEC {
                pszName: w!("Text files"),
                pszSpec: w!("*.txt"),
            },
            COMDLG_FILTERSPEC {
                pszName: w!("All files"),
                pszSpec: w!("*.*"),
            },
        ])?;

        if dialog.Show(None).is_ok() {
            let result = dialog.GetResult()?;
            let path = result.GetDisplayName(SIGDN_FILESYSPATH)?;
            let msg = format!("user picked: {}", path.display());
            popup::pop_info(&HSTRING::from(msg));
            CoTaskMemFree(Some(path.0 as _));
        } else {
            popup::pop_info(&HSTRING::from("user canceled"));
        }

        Ok(())
    }
}