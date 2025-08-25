# Prints the documentation string for a function as well its argc.
#
# Use `doc(1)` to return it instead.
def(help, f, write(
    strconcat("<function>(", string(argc(f)), "): ", endl(), endl(), doc(f), endl())
)),
