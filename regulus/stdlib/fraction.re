import(math),

# TODO: make the functions methods
def(numerator, frac, .(frac, numerator)),
def(denominator, frac, .(frac, denominator)),

def(new_fraction, num, denom, _(
    if(==(num, 0), error("DivideByZero", "cannot construct fraction with denominator 0")),
    @(Fraction(num, denom), simplify)
)),

type(
    Fraction,
    numerator,
    denominator,

    =(simplify, fn(self, ifelse(
        ==(numerator, 0),
        Fraction(0, 1),
        _(
            =(num, numerator(self)),
            =(den, denominator(self)),
            =(g, gcd(num, den)),
            if(<(den, 0), =(g, *(-1, g))),
            Fraction(/(num, g), /(den, g))
        )
    ))),

    =(==, fn(f1, f2, _(
        =(f1, @(f1, simplify)),
        =(f2, @(f2, simplify)),
        &&(
            ==(numerator(f1), numerator(f2)),
            ==(denominator(f1), denominator(f2)),
        )
    ))),

    =(neg, fn(self, new_fraction(
        -(0, numerator(self)),
        denominator(self),
    ))),

    =(reciprocal, fn(self, new_fraction(denominator(self), numerator(self)))),

    =(extend, fn(self, n, ifelse(
        ==(n, 0),
        Fraction(0, 1),
        Fraction(
            *(n, numerator(self)),
            *(n, denominator(self))
        )
    ))),

    =(+, fn(f1, f2, _(
        =(f1_old_denom, denominator(f1)),
        =(f1, @(f1, extend, denominator(f2))),
        =(f2, @(f2, extend, f1_old_denom)),
        new_fraction(
            +(numerator(f1), numerator(f2)),
            denominator(f1),
        ),
    ))),

    =(-, fn(f1, f2, +(f1, @(f2, neg)))),

    =(*, fn(f1, f2, new_fraction(
        *(numerator(f1), numerator(f2)),
        *(denominator(f1), denominator(f2)),
    ))),

    =(/, fn(f1, f2, *(f1, @(f2, reciprocal)))),
),

def(frac_to_int, frac, _(
    /(
        numerator(frac),
        denominator(frac),
    )
)),
def(frac_from_int, n, Fraction(n, 1)),
