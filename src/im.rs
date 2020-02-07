#![allow(non_snake_case)]

use core_foundation::base::kCFAllocatorDefault;
use core_foundation::string::{kCFStringEncodingUTF8, CFStringCreateWithCString};
use core_foundation::{
    base::{OSStatus, TCFType},
    string::{CFString, CFStringRef},
};
use std::ffi::CString;

#[repr(C)]
pub struct TISInputSource {
    _private: [u8; 0],
}

#[link(name = "Carbon", kind = "framework")]
extern "C" {
    static kTISPropertyInputSourceID: CFStringRef;

    fn TISCopyCurrentKeyboardInputSource() -> *mut TISInputSource;
    fn TISCopyInputSourceForLanguage(language: CFStringRef) -> *mut TISInputSource;
    fn TISSelectInputSource(input_source_ref: *mut TISInputSource) -> OSStatus;
    fn TISGetInputSourceProperty(
        input_source_ref: *mut TISInputSource,
        key: CFStringRef,
    ) -> CFStringRef;
}

pub fn get_input_source() -> String {
    unsafe {
        let input_source = TISCopyCurrentKeyboardInputSource();
        let input_source_id = TISGetInputSourceProperty(input_source, kTISPropertyInputSourceID);
        CFString::wrap_under_get_rule(input_source_id).to_string()
    }
}

pub fn set_input_source(language: &str) -> () {
    unsafe {
        let language = CString::new(language).unwrap();
        let language = CFStringCreateWithCString(
            kCFAllocatorDefault,
            language.as_ptr(),
            kCFStringEncodingUTF8,
        );
        let input_source_ref = TISCopyInputSourceForLanguage(language);
        TISSelectInputSource(input_source_ref);
    }
}