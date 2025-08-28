# inspired by documentation in `lists.re`.
# foo bar baz
# note: `null` is put here to avoid merging the above comment with the doc comment below.
null,

# Applies the second argument function to each element of the first argument list and returns
# the updated list.
def(map, 0),
assert_eq(doc(map), "Applies the second argument function to each element of the first argument list and returns
the updated list."),

# Should not be part of the docs but is at the moment

# Surely a part
def(f, 0),

assert_eq(doc(f), "Should not be part of the docs but is at the moment
Surely a part"),
