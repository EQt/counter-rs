use counter::Counter;

#[derive(Hash, PartialEq, Eq)]
struct NoDefault {
    i: isize,
}

#[test]
fn use_new() {
    let counter: Counter<NoDefault> = Counter::new();
    assert_eq!(counter.len(), 0);
}

#[test]
fn use_default() {
    let counter: Counter<NoDefault> = Counter::default();
    assert_eq!(counter.len(), 0);
}
