# Prints the documentation string for a function as well its argc.
#
# Use `doc(1)` to return it instead.
__builtin_doc_def("Prints the documentation string for a function as well its argc.

Use `doc(1)` to return it instead.", help, f, write(
    strconcat("<function>(", string(argc(f)), "): ", endl(), endl(), doc(f), endl())
)),
