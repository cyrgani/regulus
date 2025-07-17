import(type_id),

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
            error("unsupported addition"),
        )
    )
)),