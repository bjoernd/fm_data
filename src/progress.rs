use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

/// Events that can be emitted during operations to report progress
#[derive(Debug, Clone)]
pub enum ProgressEvent {
    /// Operation started with a total number of steps
    Started { total_steps: u64, description: String },
    /// A step was completed with an optional message
    StepCompleted { step: u64, message: String },
    /// Operation finished with a summary message
    Finished { summary: String },
    /// Update message without changing progress
    MessageUpdate { message: String },
    /// Increment progress by a delta
    Increment { delta: u64 },
}

/// Trait for handling progress events
pub trait ProgressEventHandler: Send + Sync {
    fn handle_event(&self, event: ProgressEvent);
}

/// Publisher that can emit progress events to multiple handlers
pub struct ProgressPublisher {
    handlers: Vec<Box<dyn ProgressEventHandler>>,
}

impl ProgressPublisher {
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }

    pub fn add_handler(&mut self, handler: Box<dyn ProgressEventHandler>) {
        self.handlers.push(handler);
    }

    pub fn emit(&self, event: ProgressEvent) {
        for handler in &self.handlers {
            handler.handle_event(event.clone());
        }
    }

    pub fn start(&self, total_steps: u64, description: impl Into<String>) {
        self.emit(ProgressEvent::Started {
            total_steps,
            description: description.into(),
        });
    }

    pub fn step(&self, step: u64, message: impl Into<String>) {
        self.emit(ProgressEvent::StepCompleted {
            step,
            message: message.into(),
        });
    }

    pub fn message(&self, message: impl Into<String>) {
        self.emit(ProgressEvent::MessageUpdate {
            message: message.into(),
        });
    }

    pub fn increment(&self, delta: u64) {
        self.emit(ProgressEvent::Increment { delta });
    }

    pub fn finish(&self, summary: impl Into<String>) {
        self.emit(ProgressEvent::Finished {
            summary: summary.into(),
        });
    }
}

impl Default for ProgressPublisher {
    fn default() -> Self {
        Self::new()
    }
}

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

impl ProgressEventHandler for ProgressTracker {
    fn handle_event(&self, event: ProgressEvent) {
        match event {
            ProgressEvent::Started { total_steps, description } => {
                self.bar.set_length(total_steps);
                self.bar.set_position(0);
                self.bar.set_message(description);
            }
            ProgressEvent::StepCompleted { step, message } => {
                self.bar.set_position(step);
                self.bar.set_message(message);
            }
            ProgressEvent::Finished { summary } => {
                self.bar.finish_with_message(summary);
            }
            ProgressEvent::MessageUpdate { message } => {
                self.bar.set_message(message);
            }
            ProgressEvent::Increment { delta } => {
                self.bar.inc(delta);
            }
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

pub struct NoOpEventHandler;

impl ProgressEventHandler for NoOpEventHandler {
    fn handle_event(&self, _event: ProgressEvent) {
        // Do nothing
    }
}

pub fn create_progress_reporter(enabled: bool, verbose: bool) -> Box<dyn ProgressReporter> {
    if enabled && !verbose {
        Box::new(ProgressTracker::new(100, true))
    } else {
        Box::new(NoOpProgressReporter)
    }
}

/// Creates a progress publisher with appropriate handlers based on configuration
pub fn create_progress_publisher(enabled: bool, verbose: bool) -> ProgressPublisher {
    let mut publisher = ProgressPublisher::new();
    
    if enabled && !verbose {
        let handler = Box::new(ProgressTracker::new(100, true));
        publisher.add_handler(handler);
    } else {
        let handler = Box::new(NoOpEventHandler);
        publisher.add_handler(handler);
    }
    
    publisher
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

    #[test]
    fn test_progress_event_creation() {
        let start_event = ProgressEvent::Started {
            total_steps: 10,
            description: "Test operation".to_string(),
        };
        
        let step_event = ProgressEvent::StepCompleted {
            step: 5,
            message: "Half done".to_string(),
        };
        
        let finish_event = ProgressEvent::Finished {
            summary: "Complete".to_string(),
        };

        // Events should be cloneable
        let _cloned_start = start_event.clone();
        let _cloned_step = step_event.clone();
        let _cloned_finish = finish_event.clone();
    }

    #[test]
    fn test_progress_publisher_creation() {
        let publisher = ProgressPublisher::new();
        assert!(publisher.handlers.is_empty());
    }

    #[test]
    fn test_progress_publisher_add_handler() {
        let mut publisher = ProgressPublisher::new();
        let handler = Box::new(NoOpEventHandler);
        publisher.add_handler(handler);
        assert_eq!(publisher.handlers.len(), 1);
    }

    #[test]
    fn test_progress_publisher_emit() {
        let publisher = ProgressPublisher::new();
        // Should not panic even with no handlers
        publisher.emit(ProgressEvent::MessageUpdate {
            message: "Test".to_string(),
        });
    }

    #[test]
    fn test_progress_publisher_convenience_methods() {
        let publisher = ProgressPublisher::new();
        
        // Test convenience methods don't panic
        publisher.start(10, "Test operation");
        publisher.step(5, "Half done");
        publisher.message("Update");
        publisher.increment(1);
        publisher.finish("Complete");
    }

    #[test]
    fn test_create_progress_publisher_enabled() {
        let publisher = create_progress_publisher(true, false);
        assert_eq!(publisher.handlers.len(), 1);
    }

    #[test]
    fn test_create_progress_publisher_disabled() {
        let publisher = create_progress_publisher(false, false);
        assert_eq!(publisher.handlers.len(), 1);
    }

    #[test]
    fn test_create_progress_publisher_verbose() {
        let publisher = create_progress_publisher(true, true);
        assert_eq!(publisher.handlers.len(), 1);
    }

    #[test]
    fn test_no_op_event_handler() {
        let handler = NoOpEventHandler;
        // Should not panic
        handler.handle_event(ProgressEvent::Started {
            total_steps: 10,
            description: "Test".to_string(),
        });
    }
}
