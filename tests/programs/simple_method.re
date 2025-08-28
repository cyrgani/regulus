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

__builtin_print_catch(@(x, nonexistent)),
__builtin_print_catch(@(x, double, 2)),
__builtin_print_catch(@(5, foo)),
__builtin_print_catch(@(x, fn(self, 2))),

import(aliases),
=(y, call_method(x, double)),
assert_eq(.(y, a), 10),

# TODO: error on "static" methods (methods that take 0 parameters), since they are not callable with @
# make sure that errors in methods cause no panics
type(E, =(f, fn(self, error("Foo", "Bar"))), =(g, fn(self, UNDEFINED))),

=(e, E()),
__builtin_print_catch(@(e, f)),
__builtin_print_catch(@(e, g)),

