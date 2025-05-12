_(
    def(hello, print("hello")),
    hello(),
    def(one, _(1)),
    assert_eq(one(), 1),

    def(lit, 1),
    assert_eq(lit(), 1),
    assert(!(==(lit, 1))),
)