type(
    A,
    =(foo, fn(self, $x, 0)),
    =(blubb, fn(self, $x, x())),
    =(bar, fn(self, [x], assert_eq(len(x), 2))),
),

default_value(A, foo2, fn(self, $x, 0)),
default_value(A, blubb2, fn(self, $x, x())),
default_value(A, bar2, fn(self, [x], assert_eq(len(x), 2))),

@(A(), foo, /(0, 0)),
@(A(), blubb, write("hello")),
@(A(), bar, 3, 5),

@(A(), foo2, /(0, 0)),
@(A(), blubb2, write("hello")),
@(A(), bar2, 3, 5),

