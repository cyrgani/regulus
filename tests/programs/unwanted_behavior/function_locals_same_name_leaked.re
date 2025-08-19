# BUG(?): should error instead of outputting 0
def(silly, print(x)),

def(other, _(
    =(x, 0),
    silly(),
)),

other(),
