type(
    Ident2,
    field1,
    field2,
    field3,
),

=(Ident, Ident2),

=(x, Ident(2, 5, "adsa")),
# read x.field1, alias getattr
assert_eq(.(x, field1), 2),
# set x.field1 to 3, alias setattr
=(x, ->(x, field1, 3)),

assert_eq(.(x, field1), 3),


import(aliases),

=(x, Ident(2, 5, "adsa")),
# read x.field1, alias .
assert_eq(getattr(x, field1), 2),
# set x.field1 to "strings", alias ->
=(x, setattr(x, field1, "strings")),

assert_eq(.(x, field1), "strings"),
