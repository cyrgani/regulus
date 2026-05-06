_(
    def(f, x, +(x, 1)),

    =(g, f),
    assert_eq(g(2), 3),
    assert_ne(g, f),
)