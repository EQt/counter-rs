use counter::Counter;

#[derive(Debug, Hash, PartialEq, Eq)]
struct Inty {
    i: usize,
}

impl Inty {
    pub fn new(i: usize) -> Inty {
        Inty { i: i }
    }
}

#[test]
fn advanced_usage() {
    // <https://en.wikipedia.org/wiki/867-5309/Jenny>
    let intys = vec![
        Inty::new(8),
        Inty::new(0),
        Inty::new(0),
        Inty::new(8),
        Inty::new(6),
        Inty::new(7),
        Inty::new(5),
        Inty::new(3),
        Inty::new(0),
        Inty::new(9),
    ];

    let inty_counts = intys.iter().collect::<Counter<_>>();
    println!("{:?}", inty_counts);
    // {Inty { i: 8 }: 2, Inty { i: 0 }: 3, Inty { i: 9 }: 1, Inty { i: 3 }: 1,
    //  Inty { i: 7 }: 1, Inty { i: 6 }: 1, Inty { i: 5 }: 1}
    assert!(inty_counts.get(&Inty { i: 8 }) == Some(&2));
    assert!(inty_counts.get(&Inty { i: 0 }) == Some(&3));
    assert!(inty_counts.get(&Inty { i: 6 }) == Some(&1));
}
