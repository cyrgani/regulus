def(outer, x, _(
    def(inner, print("inner!")),
    def(inner2, print(x)),
    print("outer!"),
    inner2
)),

outer(2),
__builtin_print_catch(inner()),
=(inner2, outer(2)),
__builtin_print_catch(inner2()),
