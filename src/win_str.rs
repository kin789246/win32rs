use windows::core::*;

pub fn hstr_to_pcwstr(h: &HSTRING) -> PCWSTR {
    PCWSTR::from_raw(h.as_ptr())
}

pub fn str_to_pcwstr(s: &str) -> PCWSTR {
    PCWSTR::from_raw(HSTRING::from(s).as_ptr())
}

pub fn str_to_hstring(s: &str) -> HSTRING {
    HSTRING::from(s)
}