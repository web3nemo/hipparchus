use num::FromPrimitive;

pub fn l0norm<'a, T, I>(it: I) -> Option<T>
where
    T: FromPrimitive + PartialEq + 'a,
    I: Iterator<Item = &'a T>,
{
    let zero = T::from_i32(0).unwrap();
    let mut agg = 0;
    it.for_each(|v| match v
    {
        z if z.eq(&zero) => {},
        _ => agg += 1,
    });
    match agg
    {
        0 => None,
        _ => Some(T::from_usize(agg).unwrap()),
    }
}

#[cfg(test)]
mod tests 
{
    use super::l0norm;

    // Test l0norm 
    #[test]
    fn test_l0norm()
    {
        assert_eq!
        (
            5,
            l0norm(vec![1, 2, 3, 4, 5].iter()).unwrap()
        );
    }

    // Test l0norm 
    #[test]
    fn test_l0norm_empty()
    {
        let e = vec![] as Vec<i32>;
        assert_eq!
        (
            Option::<i32>::None,
            l0norm(e.iter())
        );
    }
}
