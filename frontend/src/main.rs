/*use std::alloc::{Layout, realloc};


fn split_first<T>(mut v: Vec<T>) -> Option<(T, Vec<T>)> {
    if v.is_empty() {
        None
    } else {
        let len = v.len() - 1;
        let cap = v.capacity() - 1;
        let mut ptr = v.as_mut_ptr();
        let el = unsafe { std::ptr::read(ptr) };
        ptr = unsafe { ptr.add(1) };
        ptr = unsafe { realloc(ptr.cast(), Layout::array::<T>(cap).unwrap(), cap).cast() };
        Some((el, unsafe { Vec::from_raw_parts(ptr, len, cap) }))
    }
}*/

use regulus::Runner;

fn main() {
    /*let h = vec![0, 1];
    split_first(h); */
    dbg!(
        Runner::new()
            .code(
                r##"
def(range, start, end, _(
    while(false, _()),
)),

range(0, 4), 
    "##
            )
            .run()
            .0
    );
}
