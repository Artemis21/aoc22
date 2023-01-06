use std::time::{Duration, Instant};

struct Config {
    max_repeatable: Duration,
    max_iter_count: u32,
    max_per_repeat: Duration,
    num_repeats: usize,
}

const CONFIG: Config = Config {
    max_repeatable: Duration::from_secs(20),
    max_iter_count: 1_000_000,
    max_per_repeat: Duration::from_millis(200),
    num_repeats: 10,
};

pub fn time<T, E>(code: impl Fn() -> Result<T, E>) -> Result<(Duration, T), E> {
    let (iter_count, result) = match find_iter_count(&code)? {
        (IterCountResult::IterCount(count), result) => (count, result),
        (IterCountResult::LongTime(time), result) => return Ok((time, result)),
    };
    let time = (0..CONFIG.num_repeats)
        .map(|_| one_repeat(&code, iter_count))
        .min()
        .unwrap();
    Ok((time / iter_count, result))
}

enum IterCountResult {
    IterCount(u32),
    LongTime(Duration),
}

#[allow(clippy::maybe_infinite_iter)]
fn find_iter_count<T, E>(code: &impl Fn() -> Result<T, E>) -> Result<(IterCountResult, T), E> {
    let start = Instant::now();
    let result = code()?;
    let time = start.elapsed();
    if time > CONFIG.max_repeatable {
        return Ok((IterCountResult::LongTime(time), result));
    }
    Ok((0..)
        .map(|n| 2u32.pow(n))
        .map(|iters| (iters, one_repeat(code, iters)))
        .find(|(iters, time)| *time >= CONFIG.max_per_repeat || *iters >= CONFIG.max_iter_count)
        .map(|(iters, _)| (IterCountResult::IterCount(iters), result))
        .expect("unbounded iterator to find an execution time above our limit"))
}

fn one_repeat<T>(code: impl Fn() -> T, iter_count: u32) -> Duration {
    let start = Instant::now();
    #[allow(unused_must_use)]
    for _ in 0..iter_count {
        code();
    }
    start.elapsed()
}
