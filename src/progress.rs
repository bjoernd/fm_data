use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

pub trait ProgressCallback: Send + Sync {
    fn update(&self, current: u64, total: u64, message: &str);
    fn finish(&self, message: &str);
    fn set_message(&self, message: &str);
    fn inc(&self, delta: u64);
}

pub struct ProgressTracker {
    bar: ProgressBar,
}

impl ProgressTracker {
    pub fn new(total: u64, show_progress: bool) -> Self {
        let bar = if show_progress {
            let pb = ProgressBar::new(total);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
                    .unwrap()
                    .progress_chars("#>-")
            );
            pb.enable_steady_tick(Duration::from_millis(100));
            pb
        } else {
            ProgressBar::hidden()
        };

        Self { bar }
    }

    pub fn new_spinner(show_progress: bool, message: &str) -> Self {
        let bar = if show_progress {
            let pb = ProgressBar::new_spinner();
            pb.set_style(
                ProgressStyle::default_spinner()
                    .template("{spinner:.green} {elapsed_precise} {msg}")
                    .unwrap(),
            );
            pb.set_message(message.to_string());
            pb.enable_steady_tick(Duration::from_millis(100));
            pb
        } else {
            ProgressBar::hidden()
        };

        Self { bar }
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
        tracker.update(50, 100, "Test message");
        tracker.finish("Done");
        // Should work without showing anything
    }

    #[test]
    fn test_spinner_creation() {
        let spinner = ProgressTracker::new_spinner(true, "Loading...");
        spinner.set_message("Updated message");
        spinner.finish("Complete");
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
}
