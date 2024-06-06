pub(crate) fn check_gl_error(stmt: &str, fname: &str, line: u32) {
    let err = unsafe { gl::GetError() };
    if err != gl::NO_ERROR {
        println!("OpenGL error {:08x}, at {}:{} - for {}", err, fname, line, stmt);
        std::process::abort();
    }
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! gl_check {
    ($stmt:expr) => {{
        $stmt;
        crate::opengl::macros::check_gl_error(stringify!($stmt), file!(), line!());
    }};
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! gl_check {
    ($stmt:expr) => {{
        $stmt;
    }};
}