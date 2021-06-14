pub mod core_components;
pub mod core_resources;
pub mod core_systems;

use gl_renderer::{
    Viewport
};

use anyhow::{
    Result,
    anyhow
};

pub fn create_gl_window() -> Result<(), anyhow::Error>{
    let sdl = sdl2::init().map_err(|err|anyhow!(err))?;
    let video_subsystem = sdl.video().map_err(|err|anyhow!(err))?;
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);
    let window = video_subsystem
        .window("Roest", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().map_err(|err|anyhow!(err))?;
    gl::load_with(
        |s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
    );

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::DEBUG_OUTPUT);

        gl::DebugMessageCallback(Some(gl::error_callback), std::ptr::null())
    }

    let mut viewport = Viewport::for_window(900, 700);
    viewport.set_used();


    Ok(())
}