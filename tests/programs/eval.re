assert_eq(eval("+(2,2)"), 4),
assert_eq(catch(eval("(")), "SyntaxError: missing or invalid tokens for s_step"),
=(x, "_("),
=(y, strconcat(x, "/(1, 0))")),
assert_eq(catch(eval(y)), "OverflowError: overflow occured during /!"),

eval("=(aaaaa, 2)"),
assert_eq(catch(aaaaa), "NameError: No variable named `aaaaa` found!"),