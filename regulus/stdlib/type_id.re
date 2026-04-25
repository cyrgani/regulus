=(INT_TY_ID, 0),
=(BOOL_TY_ID, 1),
=(CHAR_TY_ID, 2),
=(NULL_TY_ID, 3),
=(LIST_TY_ID, 4),
=(FUNCTION_TY_ID, 5),
=(MIN_OBJECT_TY_ID, 6),

# Returns whether the given value is an integer (according to its type id).
def(is_int, val, __builtin_atom_eq(type_id(val), INT_TY_ID)),
# Returns whether the given value is a boolean (according to its type id).
def(is_bool, val, __builtin_atom_eq(type_id(val), BOOL_TY_ID)),
# Returns whether the given value is a character (according to its type id).
def(is_char, val, __builtin_atom_eq(type_id(val), CHAR_TY_ID)),
# Returns whether the given value is null (according to its type id).
def(is_null, val, __builtin_atom_eq(type_id(val), NULL_TY_ID)),
# Returns whether the given value is a list (according to its type id).
def(is_list, val, __builtin_atom_eq(type_id(val), LIST_TY_ID)),
# Returns whether the given value is a function (according to its type id).
def(is_function, val, __builtin_atom_eq(type_id(val), FUNCTION_TY_ID)),
# Returns whether the given value is an object (according to its type id).
def(is_object, val, _(
    =(c, __builtin_atom_cmp(type_id(val), MIN_OBJECT_TY_ID)),
    ||(__builtin_atom_eq(c, 0), __builtin_atom_eq(c, 1))
)),
