#[test]
fn test_queue() {
    let a = Queue::new()
        .push_back(1)
        .push_back(2);
    let b = a
        .push_back(3)
        .push_back(4);
    let c = a
        .push_back(5)
        .push_back(6);
    let b1 = b
        .pop_front().unwrap().0
        .pop_front().unwrap().0;
    let c1 = c
        .pop_front().unwrap().0
        .pop_front().unwrap().0;

    assert!(Vec::from_iter(&a) == vec![ 1, 2 ]);
    assert!(Vec::from_iter(&b) == vec![ 1, 2, 3, 4 ]);
    assert!(Vec::from_iter(&c) == vec![ 1, 2, 5, 6 ]);
    assert!(Vec::from_iter(&b1) == vec![ 3, 4 ]);
    assert!(Vec::from_iter(&c1) == vec![ 5, 6 ]);
}
