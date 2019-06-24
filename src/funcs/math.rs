/// Subtracs two numbers, if it would underflow it returns 0 instead of panicing.
/// # Examples
///
/// ```rust
/// # use arena::funcs::math::sub_save;
/// assert_eq!(sub_save(20,10) , 10);
/// assert_eq!(sub_save(0,10)  , 0);
/// ```
pub fn sub_save(first: usize,second:usize) -> usize {
    if first <= second {
        0
    } else {
        first - second
    }
}
/// Subtracs two numbers, always subtracting the lowest from the highest one
/// # Examples
///
/// ```rust
/// # use arena::funcs::math::sub_from_highest;
/// assert_eq!(sub_from_highest(20,10) , 10);
/// assert_eq!(sub_from_highest(10,20) , 10);
/// ```
pub fn sub_from_highest(first:usize, second :usize) -> usize {
    if first < second {
        second - first
    } else {
        first - second
    }
}