#![allow(non_snake_case)]

extern crate core_foundation_sys;
extern crate cocoa;
extern crate clap;

use std::ffi::CStr;
use core_foundation_sys::string::CFStringRef;
use cocoa::foundation::NSString;

trait ToStr {
    fn to_str(&self) -> &str;
}

impl ToStr for CFStringRef {
    fn to_str(&self) -> &str {
        use cocoa::base::id;

        unsafe {
            let ptr = (*self as id).UTF8String();
            CStr::from_ptr(ptr).to_str().expect("Failed to convert to str")
        }
    }
}

trait TOCFStringRef {
    fn to_CFStringRef(&self) -> CFStringRef;
}

impl TOCFStringRef for str {
    fn to_CFStringRef(&self) -> CFStringRef {
        use cocoa::base::nil;

        unsafe {
            NSString::alloc(nil).init_str(self) as CFStringRef
        }
    }
}

// Opaque C struct.
enum TISInputSourceRef {}

#[link(name = "Carbon", kind = "framework")]
extern {
    fn TISCopyCurrentKeyboardInputSource() -> *mut TISInputSourceRef;
    fn TISGetInputSourceProperty(inputSource: *mut TISInputSourceRef, key: CFStringRef) -> CFStringRef;
    fn TISCopyInputSourceForLanguage(CFStringRef: CFStringRef) -> *mut TISInputSourceRef;
    static kTISPropertyInputSourceID: CFStringRef;
}


fn main() {
    let args = clap::App::new("im-switch")
        .version("0.1")
        .author("Junfeng Li <autozimu@gmail.com>")
        .about("switch input source for macOS")
        .arg(clap::Arg::with_name("source")
            .short("s")
            .long("source")
            .value_name("SOURCE")
            .help("target input source name")
            .takes_value(true))
        .get_matches();


    if args.is_present("source") {
        // Set IM.
        let name = args.value_of("source").unwrap().to_CFStringRef();

        let input_source = unsafe { TISCopyInputSourceForLanguage(name) };
        let input_source_id = unsafe { TISGetInputSourceProperty(input_source, kTISPropertyInputSourceID) };
        println!("New input source: {}", input_source_id.to_str());
    } else {
        // Get IM.
        let input_source = unsafe { TISCopyCurrentKeyboardInputSource() };
        let input_source_id = unsafe { TISGetInputSourceProperty(input_source, kTISPropertyInputSourceID) };
        println!("Current input source: {}", input_source_id.to_str());

    }
}
