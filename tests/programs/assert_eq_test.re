assert_eq(
    catch(assert_eq(2, 4)),
    "AssertionError: Equality assertion failed! lhs: `2`, rhs: `4`!"
),
assert_eq(
    catch(assert_eq(true, 1)),
    "AssertionError: Equality assertion failed! lhs: `true`, rhs: `1`!"
),
assert_eq(
    catch(assert_eq(fn(_()), 1)),
    "AssertionError: Equality assertion failed! lhs: `<function>(0)`, rhs: `1`!"
),
type(F),
assert_eq(
    catch(assert_eq(F(), null)),
    "AssertionError: Equality assertion failed! lhs: `{}`, rhs: `null`!"
),
assert_eq(
    catch(assert_eq(_, _)),
    "AssertionError: Equality assertion failed! lhs: `<function>(_)`, rhs: `<function>(_)`!"
),
