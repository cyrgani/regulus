_(
    # NOTE: loop variable shadowing was removed!
    =(x, 0),
    =(sum, 0),
    for_in(list(2, 3, 4), x, =(sum, +(sum, x))),
    assert_eq(x, 4), # used to be 0
    assert_eq(sum, 9),
)