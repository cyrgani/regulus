def(f, x, try_except(
    print(int(x)),
    print("error"),
)),

# should print to stdout
f(2),
f("a"),

# should not print to stdout
f(/(0, 0)),