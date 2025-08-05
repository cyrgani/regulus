def(foo, x, _(
    =(y, 0),
    x
)),

assert_eq(foo(2), 2),
__builtin_print_catch(y)