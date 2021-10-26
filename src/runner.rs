use std::time::{SystemTime, UNIX_EPOCH};

pub type StepOutput<T> = std::io::Result<T>;

#[derive(Debug)]
pub struct StepInfo {
    name: String,
    start: Option<u64>,
    stop: Option<u64>,
    success: Option<bool>,
}

impl StepInfo {
    fn start(name: &str) -> StepInfo {
        StepInfo {
            name: name.to_owned(),
            start: Some(get_micro()),
            stop: None,
            success: None,
        }
    }

    fn done(&mut self, success: bool) {
        self.stop = Some(get_micro());
        self.success = Some(success)
    }
}

pub struct Runner {
    steps: Vec<StepInfo>,
}

impl Runner {
    pub fn execute<T>(
        &mut self,
        name: &str,
        func: impl FnOnce() -> StepOutput<T>,
    ) -> StepOutput<T> {
        let mut info = StepInfo::start(name);
        let res = func();
        info.done(res.is_ok());
        self.steps.push(info);
        res
    }

    pub fn report(&self) {
        println!("{:<8} {:<16} {:<8} Name", "#", "Duration, uS", "Status");
        self.steps.iter().enumerate().for_each(|x| {
            let (num, rec) = x;
            if let (Some(from), Some(to)) = (rec.start, rec.stop) {
                println!(
                    "{:<8} {:<16} {:<8} {}",
                    num,
                    to - from,
                    rec.success.unwrap_or(false),
                    rec.name
                );
            }
        });
    }

    pub fn new() -> Runner {
        Runner { steps: Vec::new() }
    }
}

fn get_micro() -> u64 {
    let start = SystemTime::now();
    start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_micros() as u64
}
