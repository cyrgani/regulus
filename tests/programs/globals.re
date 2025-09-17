def(f, _(
    global(G, 4),
)),

def(e, _(
    =(G, 2),
)),

__builtin_print_catch(G),
e(),
__builtin_print_catch(G),
f(),
assert_eq(G, 4),
e(),
assert_eq(G, 2),

import(globals_helper),
__builtin_print_catch(use_global()),
=(GLOBALS_HELPER, 5),
use_global(),
square_glob(),
assert_eq(GLOBALS_HELPER, 25),
