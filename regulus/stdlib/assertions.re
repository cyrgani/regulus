# Evaluates the argument as a boolean and returns `null` if it is true.
# If it is false, raise an `AssertionError` exception.
def(assert, cond, ifelse(
    cond,
    null,
    error("Assertion", "Assertion failed!")
)),

# Evaluates both arguments and compares then, returning `null` if they are equal.
# If not, raise an `AssertionError` exception with a message containing both values.
def(assert_eq, lhs, rhs, ifelse(
    ==(lhs, rhs),
    null,
    error("Assertion", strconcat("Equality assertion failed! lhs: `", printable(lhs), "`, rhs: `", printable(rhs), "`!"))
)),
