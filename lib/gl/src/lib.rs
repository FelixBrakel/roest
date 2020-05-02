mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use bindings::*;
use crate::bindings::types::GLsizei;
use std::ffi::{CString, CStr};

pub type ffi_error_callback = extern "system" fn(source: types::GLenum,
                                                    ty: types::GLenum,
                                                    id: types::GLuint,
                                                    severity: types::GLenum,
                                                    length: types::GLsizei,
                                                    message: *const types::GLchar,
                                                    user_param: *mut std::ffi::c_void);

pub extern "system" fn error_callback(
    source: types::GLenum,
    ty: types::GLenum,
    id: types::GLuint,
    severity: types::GLenum,
    length: types::GLsizei,
    message: *const types::GLchar,
    user_param: *mut std::ffi::c_void)
{
    let error_str = unsafe {
        CStr::from_ptr(message)
    };

    println!("Debug message ({}): {}", id, error_str.to_str().unwrap());

    print!("Source: ");
    match source {
        DEBUG_SOURCE_API => println!("API"),
        DEBUG_SOURCE_WINDOW_SYSTEM => println!("Window System"),
        DEBUG_SOURCE_SHADER_COMPILER => println!("Shader Compiler"),
        DEBUG_SOURCE_THIRD_PARTY => println!("Third Party"),
        DEBUG_SOURCE_APPLICATION => println!("Application"),
        DEBUG_SOURCE_OTHER => println!("Other"),
        _ => println!("Unknown")
    }

    print!("Type: ");
    match ty {
        DEBUG_TYPE_ERROR => println!(" Error"),
        DEBUG_TYPE_DEPRECATED_BEHAVIOR => println!("Deprecated Behaviour"),
        DEBUG_TYPE_UNDEFINED_BEHAVIOR => println!("Undefined Behavviour"),
        DEBUG_TYPE_PORTABILITY => println!("Portability"),
        DEBUG_TYPE_PERFORMANCE => println!("Performance"),
        DEBUG_TYPE_MARKER => println!("Marker"),
        DEBUG_TYPE_PUSH_GROUP => println!("Push Group"),
        DEBUG_TYPE_POP_GROUP => println!("Pop Group"),
        DEBUG_TYPE_OTHER => println!("Other"),
        _ => println!("Unknown")
    }

    print!("Severity: ");
    match severity {
        DEBUG_SEVERITY_HIGH => println!("High"),
        DEBUG_SEVERITY_MEDIUM => println!("Medium"),
        DEBUG_SEVERITY_LOW => println!("Low"),
        DEBUG_SEVERITY_NOTIFICATION => println!("Notification"),
        _ => println!("Unknown")
    }

    println!();
}
// pub use bindings::Gl as InnerGl;
// use std::rc::Rc;
// use std::ops::Deref;

// #[derive(Clone)]
// pub type Gl = Rc<bindings::Gl>;

// impl Gl {
//     fn load_with<F>(loadfn: F) -> Gl {
//         Rc::new(bindings::Gl::load_with(loadfn))
//     }
// }

// #[derive(Clone)]
// pub struct Gl {
//     inner: Rc<bindings::Gl>
// }

// impl Gl {
//     pub fn load_with<F>(loadfn: F) -> Gl
//         where F: FnMut(&'static str) -> *const types::GLvoid
//     {
//         Gl { inner: Rc::new(bindings::Gl::load_with(loadfn)) }
//     }
// }

// impl Deref for Gl {
//     type Target = bindings::Gl;
//
//     fn deref(&self) -> &bindings::Gl {
//         &self.inner
//     }
// }
