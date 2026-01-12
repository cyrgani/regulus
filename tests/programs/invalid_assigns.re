# TODO: with optimizations, `_(a)` is rewritten into `a` and that call becomes valid
# __builtin_no_test_if_opt,
__builtin_print_catch(=(2, 3)),
__builtin_print_catch(=(true, 3)),
__builtin_print_catch(=(_(a), 3)),
