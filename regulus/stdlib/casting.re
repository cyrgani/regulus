import(type_id),

# Converts the given value into a boolean, raising an exception if it is not possible to cast.
#
# It is only supported to cast ints (0 -> false, all others -> 1), bools and nulls (returns false) to bools.
def(bool, val, _(
    =(id, type_id(val)),
    ifelse(
        ==(id, INT_TY_ID),
        ifelse(==(val, 0), false, true),
        ifelse(
            ==(id, BOOL_TY_ID),
            val,
            ifelse(
                ==(id, NULL_TY_ID),
                false,
                error("Type", strconcat("cannot cast ", printable(val), " to bool"))
            )
        )
    )
)),