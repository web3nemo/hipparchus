use crate::value::Fp;

pub fn l0norm<'a, T, I>(it: I) -> Option<T>
where
    T: Fp + 'a,
    I: Iterator<Item = &'a T>,
{
    let mut empty = true;
    let sum = it.fold(0usize, |s,&x|
    {
        empty = false;
        if T::zero() != x { s + 1 } else { s }
    });
    match empty
    {
        true => None,
        false => T::from_usize(sum),
    }
}

#[cfg(test)]
mod tests 
{
    use super::*;
    use float_cmp::assert_approx_eq;

    // Test L0 norm 
    #[test]
    fn test_l0norm()
    {
        assert_approx_eq!
        (
            f32, 5.0,
            l0norm(vec![1.0, 2.0, 3.0, 4.0, 5.0].iter()).unwrap()
        );
    }

    // Test L0 norm 
    #[test]
    fn test_l0norm_empty()
    {
        let e = vec![] as Vec<f32>;
        assert_eq!
        (
            Option::<f32>::None,
            l0norm(e.iter())
        );
    }
}
