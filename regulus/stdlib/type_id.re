=(INT_TY_ID, 0),
=(BOOL_TY_ID, 1),
=(NULL_TY_ID, 2),
=(LIST_TY_ID, 3),
=(STRING_TY_ID, 4),
=(FUNCTION_TY_ID, 5),
=(MIN_OBJECT_TY_ID, 6),

# Returns whether the given value is an integer (according to its type id).
def(is_int, val, __builtin_atom_eq(type_id(val), INT_TY_ID)),
# Returns whether the given value is a boolean (according to its type id).
def(is_bool, val, __builtin_atom_eq(type_id(val), BOOL_TY_ID)),
# Returns whether the given value is null (according to its type id).
def(is_null, val, __builtin_atom_eq(type_id(val), NULL_TY_ID)),
# Returns whether the given value is a list (according to its type id).
def(is_list, val, __builtin_atom_eq(type_id(val), LIST_TY_ID)),
# Returns whether the given value is a string (according to its type id).
def(is_string, val, __builtin_atom_eq(type_id(val), STRING_TY_ID)),
# Returns whether the given value is a function (according to its type id).
def(is_function, val, __builtin_atom_eq(type_id(val), FUNCTION_TY_ID)),
# Returns whether the given value is an object (according to its type id).
def(is_object, val, >=(type_id(val), MIN_OBJECT_TY_ID)),
# TODO: once `>=` is in the STL, `is_object` must use the __builtin version of `>=` instead
null,
