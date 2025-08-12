use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

pub trait ProgressReporter: Send + Sync {
    fn set_message(&self, message: &str);
    fn set_progress(&self, current: u64, total: u64);
    fn finish(&self, message: &str);
    fn inc(&self, delta: u64);
    fn is_enabled(&self) -> bool;
    fn update(&self, current: u64, total: u64, message: &str);
}

// Keep the old trait for backward compatibility
pub trait ProgressCallback: Send + Sync {
    fn update(&self, current: u64, total: u64, message: &str);
    fn finish(&self, message: &str);
    fn set_message(&self, message: &str);
    fn inc(&self, delta: u64);
}

pub struct ProgressTracker {
    bar: ProgressBar,
    enabled: bool,
}

impl ProgressTracker {
    pub fn new(total: u64, show_progress: bool) -> Self {
        let bar = if show_progress {
            let pb = ProgressBar::new(total);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
                    .unwrap_or_else(|_| ProgressStyle::default_bar())
                    .progress_chars("#>-")
            );
            pb.enable_steady_tick(Duration::from_millis(100));
            pb
        } else {
            ProgressBar::hidden()
        };

        Self {
            bar,
            enabled: show_progress,
        }
    }

    pub fn new_spinner(show_progress: bool, message: &str) -> Self {
        let bar = if show_progress {
            let pb = ProgressBar::new_spinner();
            pb.set_style(
                ProgressStyle::default_spinner()
                    .template("{spinner:.green} {elapsed_precise} {msg}")
                    .unwrap_or_else(|_| ProgressStyle::default_spinner()),
            );
            pb.set_message(message.to_string());
            pb.enable_steady_tick(Duration::from_millis(100));
            pb
        } else {
            ProgressBar::hidden()
        };

        Self {
            bar,
            enabled: show_progress,
        }
    }
}

impl ProgressReporter for ProgressTracker {
    fn set_message(&self, message: &str) {
        self.bar.set_message(message.to_string());
    }

    fn set_progress(&self, current: u64, total: u64) {
        self.bar.set_length(total);
        self.bar.set_position(current);
    }

    fn finish(&self, message: &str) {
        self.bar.finish_with_message(message.to_string());
    }

    fn inc(&self, delta: u64) {
        self.bar.inc(delta);
    }

    fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn update(&self, current: u64, total: u64, message: &str) {
        self.bar.set_length(total);
        self.bar.set_position(current);
        self.bar.set_message(message.to_string());
    }
}

impl ProgressCallback for ProgressTracker {
    fn update(&self, current: u64, total: u64, message: &str) {
        self.bar.set_length(total);
        self.bar.set_position(current);
        self.bar.set_message(message.to_string());
    }

    fn finish(&self, message: &str) {
        self.bar.finish_with_message(message.to_string());
    }

    fn set_message(&self, message: &str) {
        self.bar.set_message(message.to_string());
    }

    fn inc(&self, delta: u64) {
        self.bar.inc(delta);
    }
}

// No-op implementation for when progress is disabled
pub struct NoOpProgress;

impl ProgressCallback for NoOpProgress {
    fn update(&self, _current: u64, _total: u64, _message: &str) {}
    fn finish(&self, _message: &str) {}
    fn set_message(&self, _message: &str) {}
    fn inc(&self, _delta: u64) {}
}

pub struct NoOpProgressReporter;

impl ProgressReporter for NoOpProgressReporter {
    fn set_message(&self, _message: &str) {}
    fn set_progress(&self, _current: u64, _total: u64) {}
    fn finish(&self, _message: &str) {}
    fn inc(&self, _delta: u64) {}
    fn is_enabled(&self) -> bool {
        false
    }
    fn update(&self, _current: u64, _total: u64, _message: &str) {}
}

pub fn create_progress_reporter(enabled: bool, verbose: bool) -> Box<dyn ProgressReporter> {
    if enabled && !verbose {
        Box::new(ProgressTracker::new(100, true))
    } else {
        Box::new(NoOpProgressReporter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_tracker_creation() {
        let tracker = ProgressTracker::new(100, true);
        // Should not panic during creation
        drop(tracker);
    }

    #[test]
    fn test_progress_tracker_hidden() {
        let tracker = ProgressTracker::new(100, false);
        ProgressCallback::update(&tracker, 50, 100, "Test message");
        ProgressCallback::finish(&tracker, "Done");
        // Should work without showing anything
    }

    #[test]
    fn test_spinner_creation() {
        let spinner = ProgressTracker::new_spinner(true, "Loading...");
        ProgressCallback::set_message(&spinner, "Updated message");
        ProgressCallback::finish(&spinner, "Complete");
    }

    #[test]
    fn test_no_op_progress() {
        let no_op = NoOpProgress;
        no_op.update(50, 100, "Test");
        no_op.finish("Done");
        no_op.set_message("Message");
        no_op.inc(10);
        // Should do nothing
    }

    #[test]
    fn test_no_op_progress_reporter() {
        let no_op = NoOpProgressReporter;
        no_op.set_message("Test");
        no_op.set_progress(50, 100);
        no_op.finish("Done");
        no_op.inc(10);
        no_op.update(75, 100, "Update");
        assert!(!no_op.is_enabled());
        // Should do nothing
    }

    #[test]
    fn test_create_progress_reporter_enabled() {
        let reporter = create_progress_reporter(true, false);
        assert!(reporter.is_enabled());
    }

    #[test]
    fn test_create_progress_reporter_disabled() {
        let reporter = create_progress_reporter(false, false);
        assert!(!reporter.is_enabled());
    }

    #[test]
    fn test_create_progress_reporter_verbose() {
        let reporter = create_progress_reporter(true, true);
        assert!(!reporter.is_enabled());
    }
}
