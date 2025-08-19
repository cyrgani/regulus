# old bug: used to output 2 2, correct is 2 1
def(foo, a, b, print(a, b)),

def(bar, a, b, _(
    print("a is", a, "b is", b),
    foo(b, a),
)),

bar(1, 2),
