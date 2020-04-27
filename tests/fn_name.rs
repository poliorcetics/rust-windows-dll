use windows_dll::dll;

use winapi::shared::{
    minwindef::BOOL,
    windef::HWND,
    ntdef::PVOID,
    basetsd::SIZE_T,
};

#[test]
fn fn_name() {
    #[dll("user32.dll")]
    extern "system" {
        #[allow(non_snake_case)]
        fn SetWindowCompositionAttribute(h_wnd: HWND, data: *mut WINDOWCOMPOSITIONATTRIBDATA) -> BOOL;
    }
}


#[allow(non_snake_case)]
type WINDOWCOMPOSITIONATTRIB = u32;

#[allow(non_snake_case)]
#[repr(C)]
pub struct WINDOWCOMPOSITIONATTRIBDATA {
    Attrib: WINDOWCOMPOSITIONATTRIB,
    pvData: PVOID,
    cbData: SIZE_T,
}
