import(random),
import(lists),
import(range),

=(X, "X"),
=(O, "O"),
=(E, " "),

def(index_or_num, board, idx, _(
    =(val, index(board, idx)),
    ifelse(
        ==(val, E),
        string(+(idx, 1)),
        val
    )
))

def(print_board, board, _(
    print("+-------+-------+-------+
|       |       |       |
|  ", index_or_num(board, 0), "  |  ", index_or_num(board, 1), "  |  ", index_or_num(board, 2), "  |
|       |       |       |
+-------+-------+-------+
|       |       |       |
|  ", index_or_num(board, 3), "  |  ", index_or_num(board, 4), "  |  ", index_or_num(board, 5), "  |
|       |       |       |
+-------+-------+-------+
|       |       |       |
|  ", index_or_num(board, 6), "  |  ", index_or_num(board, 7), "  |  ", index_or_num(board, 8), "  |
|       |       |       |
+-------+-------+-------+")
)),

def(request_number, board, _(
    =(idx, -1),
    while(==(idx, -1), _(
        print("Please enter the number of the field you choose (1-9): "),
        try_except(
            _(
                =(num, int(input())),
                if(
                    ||(<(num, 1), >(num, 9)),
                    error(""),
                ),
                =(val, index(board, -(num, 1))),
                if(
                    !=(val, E),
                    error("")
                ),
                =(idx, num),
            ),
            print("Invalid digit, please try again!")
        )
    )),
    idx
)),

def(empty_board, list(E, E, E, E, E, E, E, E, E)),

def(get_triple_winner, board, i1, i2, i3, _(
    =(a1, index(board, i1)),
    =(a2, index(board, i2)),
    =(a3, index(board, i3)),
    ifelse(
        &&(==(a1, a2), ==(a2, a3)),
        a1,
        E,
    )
)),

def(get_winner, board, _(
    =(wins, filter(list(
        get_triple_winner(board, 0, 1, 2),
        get_triple_winner(board, 3, 4, 5),
        get_triple_winner(board, 6, 7, 8),
        get_triple_winner(board, 0, 3, 6),
        get_triple_winner(board, 1, 4, 7),
        get_triple_winner(board, 2, 5, 8),
        get_triple_winner(board, 0, 4, 8),
        get_triple_winner(board, 2, 4, 6),
    ), fn(el, !=(el, E)))),
    ifelse(
        ==(len(wins), 0),
        null,
        first(wins),
    )
)),

def(set_random, board, player, _(
    =(random_index, choose(filter(
        range(0, 9), 
        fn(idx, ==(index(board, idx), E))
    ))),
    overwrite_at_index(board, random_index, player)
)),

def(play_game, _(
    # TODO: player should be able to choose "X" or "O"
    =(current_player, "X"),
    =(board, empty_board()),
    while(==(get_winner(board), null), ifelse(
        ==(current_player, X),
        _(
            print_board(board),
            =(player_num, request_number(board)),
            =(board, overwrite_at_index(board, -(player_num, 1), X)),
            =(current_player, O),
        ),
        _(
            print_board(board),
            =(board, set_random(board, O)),
            =(current_player, X),
        )
    ))
    print("Game over, winner:", get_winner(board)),
    print_board(board)
)),
