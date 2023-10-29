use crate::value::Fp;

pub fn l2norm<'a, T, I>(it: I) -> Option<T>
where
    T: Fp + 'a,
    I: Iterator<Item = &'a T>,
{
    let mut empty = true;
    let sum = it.fold(T::zero(), |s,&x|
    {
        empty = false;
        s + x * x
    });
    match empty
    {
        true => None,
        false => Some(sum.sqrt()),
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use float_cmp::assert_approx_eq;

    // Test L2 norm 
    #[test]
    fn test_l2norm()
    {
        assert_approx_eq!
        (
            f32, 5.0,
            l2norm(vec![3.0, 4.0].iter()).unwrap()
        );
    }

    // Test L2 norm 
    #[test]
    fn test_l2norm_empty()
    {
        let e = vec![] as Vec<f32>;
        assert_eq!
        (
            Option::<f32>::None,
            l2norm(e.iter())
        );
    }
}
