global(GLOBALS_HELPER),

def(use_global, assert_eq(GLOBALS_HELPER, 5)),

def(square_glob, =(GLOBALS_HELPER, *(GLOBALS_HELPER, GLOBALS_HELPER))),