use log::warn;
use solana_debug_locks::stacktrace_util::{backtrack_frame, BacktrackError, Frame, Stracktrace};

pub fn print_stacktrace() {
    let frames: Result<Stracktrace, BacktrackError> = backtrack_frame(
        |symbol_name| symbol_name.starts_with("rust_debugging_locks::") || symbol_name.starts_with("solana_geyser_plugin_scaffold::debug_util::"));
    debug_frames(&frames.unwrap().frames);
}

fn debug_frames(frames: &Vec<Frame>) {
    for frame in frames {
        warn!("\t> {}:{}:{}", frame.filename, frame.method, frame.line_no);
    }
}

