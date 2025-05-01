use regulus::Runner;

fn main() {
    dbg!(Runner::new().code(r#"import(lists),
=(data, list(2, 4, 5)),
assert_eq(len(data), 3),
assert_eq(data, list(2, 4, 5)),
assert(!(==(data, list(4, 2, 5)))),
assert_eq(index(data, 1), 4),
for_in(data, x, print(x)),
def(halve, el, /(el, 2)),
assert_eq(list(1, 5, 7), map(list(2, 10, 14), halve)),

assert_eq(len("abc"), len(list(1, 2, 3))),
"#).run().0);
}