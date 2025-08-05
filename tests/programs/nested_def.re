def(outer, x, _(
    def(inner, print("inner!")),
    def(inner2, print(x)),
    print("outer!"),
    inner2
)),

outer(2),
__builtin_file_catch_assert_eq(inner()),
=(inner2, outer(2)),
__builtin_file_catch_assert_eq(inner2()),
