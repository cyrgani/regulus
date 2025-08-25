import(help),
=(x, help(doc)),
assert_eq(x, null),
def(f, a, _()),
assert_eq(argc(f), 1),
assert_eq(argc(_), null),
assert_eq(argc(endl), 0),
help(help),

# Great function
def(d, 0),
assert_eq(doc(d), "Great function"),

# Very great function
# with
#
# newlines and
# foobar
# and
#unaligned
#  comments.
def(d, 0),
print(doc(d)),
