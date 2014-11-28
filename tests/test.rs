#![feature(phase)]
#![feature(unboxed_closures)]

#[phase(plugin)]
extern crate capture;

#[test]
fn test_basic() {
    let (x, y, z) = (1u32, 2u32, 3u32);
    let g = capture!(move x, ref y, clone z in move |:| x + *y + z);

    assert_eq!(g(), 6);
}
