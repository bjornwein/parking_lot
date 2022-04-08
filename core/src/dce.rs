/// When running under a DCE discrete simulation, thread context switches
/// are never forced except when the current thread blocks.
/// Therefore we must sometimes explicitly indicate that we expect time to be passing
/// to avoid getting stuck in a busy-loop.
pub fn pass_time() {
    #[cfg(feature = "dce")]
    std::thread::sleep(std::time::Duration::from_nanos(1));
}
