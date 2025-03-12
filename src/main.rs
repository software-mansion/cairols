use std::process::ExitCode;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() -> ExitCode {
    #[cfg(feature = "dhat-heap")]
    unsafe {
        use cairo_language_server::PROFILER;
        PROFILER = Some(dhat::Profiler::new_heap());
    };

    cairo_language_server::start()
}
