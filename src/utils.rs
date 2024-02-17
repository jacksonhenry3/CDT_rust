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
        None => usize::MAX,
    }
}
