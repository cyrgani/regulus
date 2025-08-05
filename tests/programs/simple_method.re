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

__builtin_file_catch_assert_eq(@(x, nonexistent)),
__builtin_file_catch_assert_eq(@(x, double, 2)),
__builtin_file_catch_assert_eq(@(5, foo)),
__builtin_file_catch_assert_eq(@(x, fn(self, 2))),

import(aliases),
=(y, call_method(x, double)),
assert_eq(.(y, a), 10),