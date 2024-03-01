use cached::proc_macro::cached;

#[cached]
pub fn choose(n: usize, k: usize) -> usize {
    if k == 0 {
        return 1;
    }
    if k > n {
        return 0;
    }
    match choose(n - 1, k - 1).checked_add(choose(n - 1, k)) {
        Some(x) => x,
        None => panic!("Overflow"),
    }
}

#[cached]
fn log_fact(n: usize) -> f64 {
    if n < 2 {
        return 0.0;
    }
    (2..=n).map(|i| (i as f64).ln()).sum()
}

pub fn log_choose(n: usize, k: usize) -> f64 {
    log_fact(n) - log_fact(k) - log_fact(n - k)
}
