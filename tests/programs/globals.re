def(f, _(
    global(G),
)),

def(e, _(
    =(G, 2),
)),

__builtin_file_catch_assert_eq(G),
e(),
__builtin_file_catch_assert_eq(G),
f(),
__builtin_file_catch_assert_eq(G),
e(),
assert_eq(G, 2),

import(globals_helper),
__builtin_file_catch_assert_eq(use_global()),
=(GLOBALS_HELPER, 5),
use_global(),
square_glob(),
assert_eq(GLOBALS_HELPER, 25),
