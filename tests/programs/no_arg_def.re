_(
    def(hello, print("hello")),
    hello(),
    def(one, _(1)),
    assert_eq(one(), 1),

    print(catch(def(lit, 1))),
)