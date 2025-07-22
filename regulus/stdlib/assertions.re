def(assert_eq, lhs, rhs, ifelse(
    ==(lhs, rhs),
    null,
    error(strconcat("Equality assertion failed! lhs: `", printable(lhs), "`, rhs: `", printable(rhs), "`!"))
)),
def(assert, cond, ifelse(
    cond,
    null,
    error("Assertion failed!")
)),