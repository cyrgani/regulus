import(control_flow),
import(lists),
import(type_id),

def(__stl_char_to_int, c, switch(c,
    '0', 0,
    '1', 1,
    '2', 2,
    '3', 3,
    '4', 4,
    '5', 5,
    '6', 6,
    '7', 7,
    '8', 8,
    '9', 9,
    error("Value", strconcat("char is not a digit: `", string(c), "`"))
)),

def(__stl_str_to_int, s, _(
    =(number, 0),
    =(factor, 1),
    for_in(reverse(s), c, _(
        =(digit, __stl_char_to_int(c)),
        =(number, +(number, *(factor, digit))),
        =(factor, *(factor, 10)),
    )),
    number
)),

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
# It is only supported to cast ints, bools (false -> 0, true -> 1), chars and lists of chars to ints.
def(int, val, switch(type_id(val),
    INT_TY_ID, val,
    BOOL_TY_ID, ifelse(val, 1, 0),
    CHAR_TY_ID, __stl_char_to_int(val),
    ifelse(
        __builtin_is_char_list(val),
        __stl_str_to_int(val),
        error("Type", strconcat("cannot cast ", printable(val), " to int"))
    ),
)),

# Converts the given value into a string, raising an exception if it is not possible to cast.
#
# This method is fallible and is currently only able to cast ints, bools, chars, lists of chars and nulls (to "null").
# If you want to display an arbitrary atom (such as for error messages), use `printable(1)`
# instead, which is infallible.
# TODO: rethink this method, now that Atom::String is gone
def(string, val, switch(type_id(val),
    INT_TY_ID, printable(val),
    BOOL_TY_ID, ifelse(val, "true", "false"),
    NULL_TY_ID, "null",
    CHAR_TY_ID, list(val),
    ifelse(
        __builtin_is_char_list(val),
        val,
        error("Type", strconcat("cannot cast ", printable(val), " to string"))
    ),
)),
