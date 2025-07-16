type(
    Foo,
    a,

    =(double, fn(self,
        Foo(*(.(self, a), 2))
    )),
),

=(b, Foo(3)),
=(d, .(b, double)),
=(c, d(b)),
assert_eq(.(c, a), 6),

=(x, Foo(5)),
=(y, @(x, double)),
assert_eq(.(y, a), 10),

assert_eq(catch(@(x, nonexistent)), "NameError: object has no method `nonexistent`"),
assert_eq(catch(@(x, double, 2)), "ArgumentError: expected `1` args, found `2` args for `<object>.double`"),
assert_eq(catch(@(5, foo)), "TypeError: 5 is not a Object!"),
assert_eq(catch(@(x, fn(self, 2))), "ArgumentError: `@` expected the name of a method as second arg"),

import(aliases),
=(y, call_method(x, double)),
assert_eq(.(y, a), 10),