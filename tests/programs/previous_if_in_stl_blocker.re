def(stl_if, cond, $body, ifelse(cond, body(), null)),

# this correctly returns 1
def(foo_with_builtin_if, _(
    =(x, 0),
    ifelse(
        true,
        =(x, 1),
        null,
    ),
    x
)),

assert_eq(foo_with_builtin_if(), 1),

# but the following function should also return 1, but used to return 0 because the `=(x, 1)` was executed in the scope of
# `stl_if` and is therefore defining a local variable only valid in `stl_if`, instead of the required local in
# `foo_with_stl_if`.
def(foo_with_stl_if, _(
    =(x, 0),
    stl_if(
        true,
        =(x, 1)
    ),
    x
)),

assert_eq(foo_with_stl_if(), 1),

if(false, error("x", "y")),
