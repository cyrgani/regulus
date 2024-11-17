_(
    import("random"),
    =(x, random_u16()),
    =(y, random_u16()),
    =(z, random_u16()),
    =(x_equals_y, ==(x, y)),
    =(x_equals_z, ==(x, z)),
    =(y_equals_z, ==(y, z)),
    # TODO: this could fail very occasionally and would not be a bug
    assert(!(&&(&&(x_equals_y, x_equals_z), y_equals_z))),
)
