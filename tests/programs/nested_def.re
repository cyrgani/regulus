def(outer, x, _(
    def(inner, print("inner!")),
    def(inner2, print(x)),
    print("outer!"),
    inner2
)),

outer(2),
assert_eq(catch(inner()), "NameError: No function `inner` found!"),
=(inner2, outer(2)),
assert_eq(catch(inner2()), "NameError: No variable named `x` found!")
