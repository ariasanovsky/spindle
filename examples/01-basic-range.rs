#[spindle::basic_range]
fn square_over_two(x: i32) -> i32 {
    (x * x) / 2
}

fn main() -> Result<(), spindle::range::Error> {
    let foo = square_over_two(5);
    assert_eq!(foo, 12);
    let bar = unsafe { 10.square_over_two() }?;
    assert_eq!(bar, [0, 0, 2, 4, 8, 12, 18, 24, 32, 40]);
    let baz: Box<[i32; 10]> = unsafe { _square_over_two() }?;
    assert_eq!(baz, Box::new([0, 0, 2, 4, 8, 12, 18, 24, 32, 40]));
    Ok(())
}
