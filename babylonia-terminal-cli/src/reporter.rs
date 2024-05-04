use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use std::fmt::Write;

struct DownloadReporterPrivate {
    last_update: std::time::Instant,
    max_progress: Option<u64>,
    last_current: u64,
    message: String,
    pb: ProgressBar,
}

pub struct DownloadReporter {
    private: std::sync::Mutex<Option<DownloadReporterPrivate>>,
    relative: bool,
}

impl DownloadReporter {
    pub fn create(relative: bool) -> std::sync::Arc<Self> {
        std::sync::Arc::new(Self {
            private: std::sync::Mutex::new(None),
            relative,
        })
    }
}

impl downloader_for_babylonia_terminal::progress::Reporter for DownloadReporter {
    fn setup(&self, max_progress: Option<u64>, message: &str) {
        let pb = ProgressBar::new(max_progress.unwrap());
        pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.white}] {bytes}/{total_bytes} ({eta})")
                .unwrap()
                .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
                .progress_chars("=>-"));

        let private = DownloadReporterPrivate {
            last_update: std::time::Instant::now(),
            max_progress,
            last_current: 0,
            message: message.to_owned(),
            pb: pb,
        };

        println!("{}", private.message);
        let mut guard = self.private.lock().unwrap();
        *guard = Some(private);
    }

    fn progress(&self, current: u64) {
        if let Some(p) = self.private.lock().unwrap().as_mut() {
            if p.last_update.elapsed().as_millis() >= 1000 {
                if self.relative {
                    let new_current = current - p.last_current;
                    p.pb.set_position(new_current);
                    p.last_current = new_current;
                } else {
                    p.pb.set_position(current);
                }
                p.last_update = std::time::Instant::now();
            } else if current == p.max_progress.unwrap() {
                p.pb.set_position(current);
                p.last_update = std::time::Instant::now();
            }
        }
    }

    fn set_message(&self, message: &str) {
        println!("File state: {}", message);
    }

    fn done(&self) {
        let mut guard = self.private.lock().unwrap();
        *guard = None;
    }
}
