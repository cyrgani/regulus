def(f, _(
    global(G),
)),

def(e, _(
    =(G, 2),
)),

assert_eq(catch(G), "NameError: No variable named `G` found!"),
e(),
assert_eq(catch(G), "NameError: No variable named `G` found!"),
f(),
assert_eq(catch(G), "NameError: No variable named `G` found!"),
e(),
assert_eq(G, 2),

import(globals_helper),
assert_eq(catch(use_global()), "NameError: No variable named `GLOBALS_HELPER` found!"),
=(GLOBALS_HELPER, 5),
use_global(),
square_glob(),
assert_eq(GLOBALS_HELPER, 25),
