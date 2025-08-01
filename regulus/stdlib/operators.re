import(type_id),

def(__stl_arith_err, op, error(
    "Arithmetic",
    append("Unsupported ", op),
)),

def(+, lhs, rhs, _(
    =(lid, type_id(lhs)),
    =(rid, type_id(rhs)),
    =(ids, list(lid, rid)),
    ifelse(
        ==(ids, list(INT_TY_ID, INT_TY_ID)),
        __builtin_int_add(lhs, rhs),
        ifelse(
            &&(>=(lid, MIN_OBJECT_TY_ID), >=(rid, MIN_OBJECT_TY_ID)),
            @(lhs, +, rhs),
            __stl_arith_err("addition"),
        )
    )
)),

def(-, lhs, rhs, _(
    =(lid, type_id(lhs)),
    =(rid, type_id(rhs)),
    =(ids, list(lid, rid)),
    ifelse(
        ==(ids, list(INT_TY_ID, INT_TY_ID)),
        __builtin_int_sub(lhs, rhs),
        ifelse(
            &&(>=(lid, MIN_OBJECT_TY_ID), >=(rid, MIN_OBJECT_TY_ID)),
            @(lhs, -, rhs),
            __stl_arith_err("subtraction"),
        )
    )
)),

def(*, lhs, rhs, _(
    =(lid, type_id(lhs)),
    =(rid, type_id(rhs)),
    =(ids, list(lid, rid)),
    ifelse(
        ==(ids, list(INT_TY_ID, INT_TY_ID)),
        __builtin_int_mul(lhs, rhs),
        ifelse(
            &&(>=(lid, MIN_OBJECT_TY_ID), >=(rid, MIN_OBJECT_TY_ID)),
            @(lhs, *, rhs),
            __stl_arith_err("multiplication"),
        )
    )
)),

def(/, lhs, rhs, _(
    =(lid, type_id(lhs)),
    =(rid, type_id(rhs)),
    =(ids, list(lid, rid)),
    ifelse(
        ==(ids, list(INT_TY_ID, INT_TY_ID)),
        __builtin_int_div(lhs, rhs),
        ifelse(
            &&(>=(lid, MIN_OBJECT_TY_ID), >=(rid, MIN_OBJECT_TY_ID)),
            @(lhs, /, rhs),
            __stl_arith_err("division"),
        )
    )
)),
