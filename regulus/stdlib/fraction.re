import(math),

def(numerator, frac, .(frac, numerator)),
def(denominator, frac, .(frac, denominator)),

def(new_fraction, num, denom, _(
    if(==(num, 0), error("DivideByZero", "cannot construct fraction with denominator 0")),
    @(Fraction(num, denom), simplify)
)),

def(fraction_from_int, n, Fraction(n, 1)),

type(
    Fraction,
    numerator,
    denominator,
),

default_value(Fraction, simplify, fn(self, ifelse(
    ==(numerator(self), 0),
    Fraction(0, 1),
    _(
        =(num, numerator(self)),
        =(den, denominator(self)),
        =(g, gcd(num, den)),
        if(<(den, 0), =(g, *(-1, g))),
        Fraction(/(num, g), /(den, g))
    )
))),

default_value(Fraction, ==, fn(f1, f2, _(
    =(f1, @(f1, simplify)),
    =(f2, @(f2, simplify)),
    &&(
        ==(numerator(f1), numerator(f2)),
        ==(denominator(f1), denominator(f2)),
    )
))),

default_value(Fraction, neg, fn(self, new_fraction(
    -(0, numerator(self)),
    denominator(self),
))),

default_value(Fraction, reciprocal, fn(self, new_fraction(denominator(self), numerator(self)))),

default_value(Fraction, extend, fn(self, n, ifelse(
    ==(n, 0),
    Fraction(0, 1),
    Fraction(
        *(n, numerator(self)),
        *(n, denominator(self))
    )
))),

default_value(Fraction, +, fn(f1, f2, _(
    =(f1_old_denom, denominator(f1)),
    =(f1, @(f1, extend, denominator(f2))),
    =(f2, @(f2, extend, f1_old_denom)),
    new_fraction(
        +(numerator(f1), numerator(f2)),
        denominator(f1),
    ),
))),

default_value(Fraction, -, fn(f1, f2, +(f1, @(f2, neg)))),

default_value(Fraction, *, fn(f1, f2, new_fraction(
    *(numerator(f1), numerator(f2)),
    *(denominator(f1), denominator(f2)),
))),

default_value(Fraction, /, fn(f1, f2, *(f1, @(f2, reciprocal)))),

default_value(Fraction, to_int, fn(self, /(numerator(self), denominator(self)))),
