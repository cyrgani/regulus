type(I, x,
    =(half, fn(self, _(
        =(x, .(self, x)),
        if(
            >(x, 0),
            _(
                print("halving", x),
                =(s2, I(/(x, 2))),
                @(s2, decr),
            )
        )
    ))),
    =(decr, fn(self, _(
        =(x, .(self, x)),
        if(
            >(x, 0),
            _(
                print("decrementing", x),
                =(s2, I(-(x, 1))),
                @(s2, half),
            )
        )
    )))
),

=(i, I(10)),
@(i, decr),
