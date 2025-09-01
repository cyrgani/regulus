# Calculates the absolute value of a number.
def(abs, x, ifelse(>=(x, 0), x, -(0, x))),

# Calculates the greatest common divisor of two numbers.
# Raises an exception if one of them is zero.
def(gcd, a, b, _(
    if(||(==(a, 0), ==(b, 0)), error("DivideByZero", "cannot calculate gcd when one argument is zero")),
    =(a, abs(a)),
    =(b, abs(b)),
    ifelse(
        >(a, b),
        gcd(b, a),
        ifelse(
            ==(%(b, a), 0),
            a,
            gcd(%(b, a), a)
        )
    )
)),
