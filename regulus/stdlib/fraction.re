import(math),

# TODO: make the functions methods
def(numerator, frac, .(frac, numerator)),
def(denominator, frac, .(frac, denominator)),

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
),

def(frac_to_int, frac, _(
    /(
        numerator(frac),
        denominator(frac),
    )
)),
def(frac_from_int, n, Fraction(n, 1)),

def(frac_add, f1, f2, _(
    # todo: use `math.lcm` here instead once it exists to reduce the denominator size
    =(f1_old_denom, denominator(f1)),
    =(f1, frac_extend(f1, denominator(f2))),    
    =(f2, frac_extend(f2, f1_old_denom)),
    Fraction(
        +(numerator(f1), numerator(f2)),
        denominator(f1),
    )
)),
def(frac_sub, f1, f2, _(
    frac_add(f1, frac_neg(f2))
)),
def(frac_mul, f1, f2, Fraction(
    *(numerator(f1), numerator(f2)),
    *(denominator(f1), denominator(f2)),
)),
def(frac_div, f1, f2, frac_mul(f1, frac_invert(f2))),

def(frac_neg, frac, Fraction(
    -(0, numerator(frac)),
    denominator(frac),
)),

def(frac_invert, frac, Fraction(
    .(frac, denominator),
    .(frac, numerator)
)),

def(frac_extend, frac, n, Fraction(
    *(n, numerator(frac)),
    *(n, denominator(frac))
)),

def(frac_simplify, frac, _(
    error("TODO", "not yet implemented")
    # todo: requires `math.gcd` to exist
)),

def(frac_compare, f1, f2, _(
    error("TODO", "not yet implemented")
    # todo: requires `frac_simplify` to be implemented (?)
)),
