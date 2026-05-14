def(f, n, ifelse(
    ==(n, 0),
    0,
    +(n, f(-(n, 1))),
)),

assert_eq(f(100), 5050),
