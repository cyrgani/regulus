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
    ifelse(
        &&(is_int(lhs), is_int(rhs)),
        __builtin_int_add(lhs, rhs),
        ifelse(
            &&(is_string(lhs), is_string(rhs)),
            __builtin_str_add(lhs, rhs),
            ifelse(
                &&(is_object(lhs), is_object(rhs)),
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
    ifelse(
        &&(is_int(lhs), is_int(rhs)),
        __builtin_int_sub(lhs, rhs),
        ifelse(
            &&(is_object(lhs), is_object(rhs)),
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
    ifelse(
        &&(is_int(lhs), is_int(rhs)),
        __builtin_int_mul(lhs, rhs),
        ifelse(
            &&(is_object(lhs), is_object(rhs)),
            @(lhs, *, rhs),
            __stl_arith_err("multiplication"),
        )
    )
)),

# Divides the first value through the second.
# If they are both integers, `lhs / rhs` is returned (rounded to an integer), raising an error if `rhs` is 0.
# If they are both objects, this calls the `/` method of `lhs` with `rhs` as the only argument.
# Otherwise, this raises an error.
def(/, lhs, rhs, _(
    ifelse(
        &&(is_int(lhs), is_int(rhs)),
        __builtin_int_div(lhs, rhs),
        ifelse(
            &&(is_object(lhs), is_object(rhs)),
            @(lhs, /, rhs),
            __stl_arith_err("division"),
        )
    )
)),

# Calculates the remainder when dividing the first value through the second.
# If they are both integers, `lhs % rhs` is returned, raising an error if `rhs` is 0.
# If they are both objects, this calls the `%` method of `lhs` with `rhs` as the only argument.
# Otherwise, this raises an error.
def(%, lhs, rhs, _(
    ifelse(
        &&(is_int(lhs), is_int(rhs)),
        __builtin_int_rem(lhs, rhs),
        ifelse(
            &&(is_object(lhs), is_object(rhs)),
            @(lhs, %, rhs),
            __stl_arith_err("remainder"),
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
    &&(is_object(lhs), is_object(rhs)),
    @(lhs, ==, rhs),
    __builtin_atom_eq(lhs, rhs),
)),

# Compares the two values and returns whether they are not equal.
# This is just a short form for `!(==(lhs, rhs))`.
# See the documentation of `==` for the precise behavior.
def(!=, lhs, rhs, !(==(lhs, rhs))),

# Negates the given value.
# If it is a boolean, this maps `true` to `false` and `false` to `true`.
# If it is an object, this calls the `!` method of `val` with no arguments.
# Otherwise, this raises an error.
def(!, val, ifelse(
    is_object(val),
    @(val, !),
    ifelse(val, false, true)
)),

# Returns whether `lhs` is less than `rhs`.
# This function, as well as `<=`, `>=` and ">", compare by the following rules:
#
# If `lhs` and `rhs` are both objects, this calls the `<` (or `<=`, `>=`, `>`) method of `lhs` with `rhs` as the only argument.
# If they are both integers, this compares them naturally.
# If they are both booleans, this compares them according to `false < true`.
# If they are both null, this compares them according to `null <= null`.
# Otherwise, this raises an exception.
def(<, lhs, rhs, ifelse(
    &&(is_object(lhs), is_object(rhs)),
    @(lhs, <, rhs),
    __builtin_atom_eq(__builtin_atom_cmp(lhs, rhs), 2)
)),

# Returns whether `lhs` is less than or equal to `rhs`.
# See the documentation of `<` for more details.
def(<=, lhs, rhs, ifelse(
    &&(is_object(lhs), is_object(rhs)),
    @(lhs, <=, rhs),
    _(
        =(c, __builtin_atom_cmp(lhs, rhs)),
        ||(__builtin_atom_eq(c, 0), __builtin_atom_eq(c, 2))
    )
)),

# Returns whether `lhs` is greater than or equal to `rhs`.
# See the documentation of `<` for more details.
def(>=, lhs, rhs, ifelse(
    &&(is_object(lhs), is_object(rhs)),
    @(lhs, >=, rhs),
    _(
        =(c, __builtin_atom_cmp(lhs, rhs)),
        ||(__builtin_atom_eq(c, 0), __builtin_atom_eq(c, 1))
    )
)),

# Returns whether `lhs` is greater than `rhs`.
# See the documentation of `<` for more details.
def(>, lhs, rhs, ifelse(
    &&(is_object(lhs), is_object(rhs)),
    @(lhs, >, rhs),
    __builtin_atom_eq(__builtin_atom_cmp(lhs, rhs), 1)
)),
