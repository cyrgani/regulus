import(random),
=(x, rand()),
=(y, rand()),
=(z, rand()),
=(x_equals_y, ==(x, y)),
=(x_equals_z, ==(x, z)),
=(y_equals_z, ==(y, z)),
# this might (?) fail very occasionally and would not be a bug
assert(!(&&(&&(x_equals_y, x_equals_z), y_equals_z))),
