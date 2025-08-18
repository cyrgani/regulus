# BUG: should output 2 1, outputs 2 2
def(foo, a, b, print(a, b)),

def(bar, a, b, _(
    print("a is", a, "b is", b),
    foo(b, a),
)),

bar(1, 2),

# BUG: should error instead of outputting 0
def(silly, print(x)),

def(other, _(
    =(x, 0),
    silly(),
)),

other(),
