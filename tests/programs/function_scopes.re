def(foo, x, _(
    =(y, 0),
    x
)),

assert_eq(foo(2), 2),
__builtin_file_catch_assert_eq(y)