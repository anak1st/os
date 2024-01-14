///! 本模块利用 log crate 提供了日志功能
use log::{Level, LevelFilter, Log, Metadata, Record};
use crate::syscall;

struct SimpleLogger;

impl Log for SimpleLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }
    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let color = match record.level() {
            Level::Error => 31, // Red
            Level::Warn => 93,  // BrightYellow
            Level::Info => 34,  // Blue
            Level::Debug => 32, // Green
            Level::Trace => 90, // BrightBlack
        };

        let current_time = syscall::syscall(syscall::SYSCALL_GET_TIME, [0, 0, 0]);
        let current_sec = current_time / 1000;
        let current_ms = current_time % 1000 / 10;

        println!(
            "\u{1B}[{}m[{:>2}.{:02}|{:<5}] {}\u{1B}[0m",
            color,
            current_sec,
            current_ms,
            record.level(),
            record.args(),
        );
    }
    fn flush(&self) {}
}

/// 初始化日志模块
/// `level`: 日志级别: 
/// `0` 为关闭日志，
/// `1` 为 Error，
/// `2` 为 Warn，
/// `3` 为 Info，
/// `4` 为 Debug，
/// `5` 为 Trace
pub fn init(level: i32) {
    static LOGGER: SimpleLogger = SimpleLogger;
    log::set_logger(&LOGGER).unwrap();

    // 设置日志级别
    let max_log_level = match level {
        0 => LevelFilter::Off,
        1 => LevelFilter::Error,
        2 => LevelFilter::Warn,
        3 => LevelFilter::Info,
        4 => LevelFilter::Debug,
        5 => LevelFilter::Trace,
        _ => LevelFilter::Off,
    };

    log::set_max_level(max_log_level);
}
