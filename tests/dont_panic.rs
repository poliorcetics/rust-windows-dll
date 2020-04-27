use windows_dll::{dll, Error};

use winapi::shared::{
    ntdef::VOID,
    minwindef::BOOL,
    windef::HWND,
    ntdef::PVOID,
    basetsd::SIZE_T,
};


#[test]
fn link_ordinal() {
    #[dll("uxtheme.dll")]
    extern "system" {
        #[link_ordinal = 137]
        fn flush_menu_themes() -> VOID;
    }
}


#[test]
fn link_ordinal_with_arguments() {
    #[dll("uxtheme.dll")]
    extern "system" {
        #[link_ordinal = 133]
        fn allow_dark_mode_for_window(hwnd: HWND, allow: BOOL) -> BOOL;
    }
}

#[test]
fn link_name() {
    #[dll("user32.dll")]
    extern "system" {
        #[link_name = "SetWindowCompositionAttribute"]
        fn set_window_composition_attribute(h_wnd: HWND, data: *mut WINDOWCOMPOSITIONATTRIBDATA) -> BOOL;
    }
}

#[allow(non_snake_case)]
type WINDOWCOMPOSITIONATTRIB = u32;
const WCA_USEDARKMODECOLORS: WINDOWCOMPOSITIONATTRIB = 26;

#[allow(non_snake_case)]
#[repr(C)]
pub struct WINDOWCOMPOSITIONATTRIBDATA {
    Attrib: WINDOWCOMPOSITIONATTRIB,
    pvData: PVOID,
    cbData: SIZE_T,
}

#[test]
fn guess_name() {
    #[dll("user32.dll")]
    extern "system" {
        #[allow(non_snake_case)]
        fn SetWindowCompositionAttribute(h_wnd: HWND, data: *mut WINDOWCOMPOSITIONATTRIBDATA) -> BOOL;
    }
}

#[test]
fn return_result() {
    #[dll("user32.dll")]
    extern "system" {
        #[allow(non_snake_case)]
        #[fallible]
        fn SetWindowCompositionAttribute(h_wnd: HWND, data: *mut WINDOWCOMPOSITIONATTRIBDATA) -> BOOL;
    }
}

#[test]
fn function_exists() {
    #[dll("user32.dll")]
    extern "system" {
        #[allow(non_snake_case)]
        fn SetWindowCompositionAttribute(h_wnd: HWND, data: *mut WINDOWCOMPOSITIONATTRIBDATA) -> BOOL;
    }

    dbg!(SetWindowCompositionAttribute::exists());
}


#[test]
fn function_exists_module() {
    mod user32 {
        use super::*;
        #[dll("user32.dll")]
        extern "system" {
            #[allow(non_snake_case)]
            pub fn SetWindowCompositionAttribute(h_wnd: HWND, data: *mut WINDOWCOMPOSITIONATTRIBDATA) -> BOOL;
        }
    }
    use user32::SetWindowCompositionAttribute;

    dbg!(SetWindowCompositionAttribute::exists());
}


#[test]
fn error_is_1_byte() {
    #[dll("user32.dll")]
    extern "system" {
        #[allow(non_snake_case)]
        #[fallible]
        fn SetWindowCompositionAttribute(h_wnd: HWND, data: *mut WINDOWCOMPOSITIONATTRIBDATA) -> BOOL;
    }

    assert_eq!(core::mem::size_of::<Error<SetWindowCompositionAttribute>>(), 1);
}
