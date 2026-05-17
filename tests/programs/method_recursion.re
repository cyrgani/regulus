type(I, x),

impl(I, half, self, _(
    =(x, .(self, x)),
    if(
        >(x, 0),
        _(
            print("halving", x),
            =(s2, I(/(x, 2))),
            @(s2, decr),
        )
    )
)),

impl(I, decr, self, _(
    =(x, .(self, x)),
    if(
        >(x, 0),
        _(
            print("decrementing", x),
            =(s2, I(-(x, 1))),
            @(s2, half),
        )
    )
)),

=(i, I(10)),
@(i, decr),
