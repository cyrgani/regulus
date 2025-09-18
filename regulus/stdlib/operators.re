import(type_id),

def(__stl_arith_err, op, error(
    "Arithmetic",
    append("Unsupported ", op),
)),

# Adds the two values together.
# If they are both integers, `lhs + rhs` is returned.
# If they are both strings, their concatenation is returned.
# If they are both objects, this calls the `+` method of `lhs` with `rhs` as the only argument.
# Otherwise, this raises an error.
def(+, lhs, rhs, _(
    =(lid, type_id(lhs)),
    =(rid, type_id(rhs)),
    ifelse(
        &&(==(lid, INT_TY_ID), ==(rid, INT_TY_ID)),
        __builtin_int_add(lhs, rhs),
        ifelse(
            &&(==(lid, STRING_TY_ID), ==(rid, STRING_TY_ID)),
            __builtin_str_add(lhs, rhs),
            ifelse(
                &&(>=(lid, MIN_OBJECT_TY_ID), >=(rid, MIN_OBJECT_TY_ID)),
                @(lhs, +, rhs),
                __stl_arith_err("addition"),
            )
        )
    )
)),

# Subtracts the second value from the first.
# If they are both integers, `lhs - rhs` is returned.
# If they are both objects, this calls the `-` method of `lhs` with `rhs` as the only argument.
# Otherwise, this raises an error.
def(-, lhs, rhs, _(
    =(lid, type_id(lhs)),
    =(rid, type_id(rhs)),
    ifelse(
        &&(==(lid, INT_TY_ID), ==(rid, INT_TY_ID)),
        __builtin_int_sub(lhs, rhs),
        ifelse(
            &&(>=(lid, MIN_OBJECT_TY_ID), >=(rid, MIN_OBJECT_TY_ID)),
            @(lhs, -, rhs),
            __stl_arith_err("subtraction"),
        )
    )
)),

# Multiplies the two values together.
# If they are both integers, `lhs * rhs` is returned.
# If they are both objects, this calls the `*` method of `lhs` with `rhs` as the only argument.
# Otherwise, this raises an error.
def(*, lhs, rhs, _(
    =(lid, type_id(lhs)),
    =(rid, type_id(rhs)),
    ifelse(
        &&(==(lid, INT_TY_ID), ==(rid, INT_TY_ID)),
        __builtin_int_mul(lhs, rhs),
        ifelse(
            &&(>=(lid, MIN_OBJECT_TY_ID), >=(rid, MIN_OBJECT_TY_ID)),
            @(lhs, *, rhs),
            __stl_arith_err("multiplication"),
        )
    )
)),

# Divides the first value through the second.
# If they are both integers, `lhs / rhs` is returned (rounded to an integer).
# If they are both objects, this calls the `/` method of `lhs` with `rhs` as the only argument.
# Otherwise, this raises an error.
def(/, lhs, rhs, _(
    =(lid, type_id(lhs)),
    =(rid, type_id(rhs)),
    ifelse(
        &&(==(lid, INT_TY_ID), ==(rid, INT_TY_ID)),
        __builtin_int_div(lhs, rhs),
        ifelse(
            &&(>=(lid, MIN_OBJECT_TY_ID), >=(rid, MIN_OBJECT_TY_ID)),
            @(lhs, /, rhs),
            __stl_arith_err("division"),
        )
    )
)),

# Compares the two values and returns whether they are equal.
# If they are both objects, this calls the `==` method of `lhs` with `rhs` as the only argument.
# If they are both functions, this always returns `false`.
# If they are something else, but both are of the same type, this compares them naturally.
# (Lists are compared element-wise).
# If they are of different types, this always returns `false`.
def(==, lhs, rhs, ifelse(
    &&(>=(type_id(lhs), MIN_OBJECT_TY_ID), >=(type_id(rhs), MIN_OBJECT_TY_ID)),
    @(lhs, ==, rhs),
    __builtin_atom_eq(lhs, rhs),
)),

# Compares the two values and returns whether they are not equal.
# This is just a short form for `!(==(lhs, rhs))`.
# See the documentation of `==` for the precise behavior.
def(!=, lhs, rhs, !(==(lhs, rhs))),
