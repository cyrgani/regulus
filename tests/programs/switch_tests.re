# should print 0
=(x, 0),
switch(
    x,
    # replace the value of `x` with 1
    _(=(x, 1), 1), print(1),
    0, print(0)
),

switch(2,
    0, print("hidden"),
    2, print("visible"),
    2, print("hidden again"),
    print("also hidden")
),

def(name_number, x, switch(x,
    0, "zero",
    1, "one",
    -1, "minus one",
    42, "the answer",
    "some other strange number"
)),

assert_eq(name_number(0), "zero"),
assert_eq(name_number(-1), "minus one"),
assert_eq(name_number(8), "some other strange number"),
assert_eq(name_number(1), "one"),
assert_eq(name_number(2), "some other strange number"),
assert_eq(name_number(42), "the answer"),

def(without_fallback, x, switch(x,
    1, 2,
)),

assert_eq(without_fallback(1), 2),
__builtin_print_catch(without_fallback(2)),
