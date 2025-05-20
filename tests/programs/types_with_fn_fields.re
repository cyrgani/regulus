type(
    Foo,
    f
),

def(printer, print("printing!")),

=(a, Foo(printer)),
=(p, .(a, f)),
p(),
