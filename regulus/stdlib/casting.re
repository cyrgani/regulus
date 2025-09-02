import(control_flow),
import(type_id),

# Converts the given value into a boolean, raising an exception if it is not possible to cast.
#
# It is only supported to cast ints (0 -> false, all others -> 1), bools and nulls (returns false) to bools.
def(bool, val, switch(type_id(val),
    INT_TY_ID, ifelse(==(val, 0), false, true),
    BOOL_TY_ID, val,
    NULL_TY_ID, false,
    error("Type", strconcat("cannot cast ", printable(val), " to bool"))
)),

# Converts the given value into an integer, raising an exception if it is not possible to cast.
#
# It is only supported to cast ints, bools (false -> 0, true -> 1) and strings to ints.
def(int, val, switch(type_id(val),
    INT_TY_ID, val,
    BOOL_TY_ID, ifelse(val, 1, 0),
    STRING_TY_ID, __builtin_str_to_int(val),
    error("Type", strconcat("cannot cast ", printable(val), " to int"))
)),

# Converts the given value into a string, raising an exception if it is not possible to cast.
#
# This method is fallible and is currently only able to cast ints, bools, strings and nulls (to "null").
# If you want to display an arbitrary atom (such as for error messages), use `printable(1)`
# instead, which is infallible.
def(string, val, switch(type_id(val),
    INT_TY_ID, printable(val),
    BOOL_TY_ID, ifelse(val, "true", "false"),
    STRING_TY_ID, val,
    NULL_TY_ID, "null",
    error("Type", strconcat("cannot cast ", printable(val), " to string"))
)),
