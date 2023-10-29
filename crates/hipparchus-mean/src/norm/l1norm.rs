use crate::value::Fp;

pub fn l1norm<'a, T, I>(it: I) -> Option<T>
where
    T: Fp + 'a,
    I: Iterator<Item = &'a T>,
{
    let mut empty = true;
    let sum = it.fold(T::zero(), |s,&x|
    {
        empty = false;
        s + x
    });
    match empty
    {
        true => None,
        false => Some(sum),
    }
}

#[cfg(test)]
mod tests 
{
    use super::*;
    use float_cmp::assert_approx_eq;

    // Test L1 norm 
    #[test]
    fn test_l1norm()
    {
        assert_approx_eq!
        (
            f32, 15.0,
            l1norm(vec![1.0, 2.0, 3.0, 4.0, 5.0].iter()).unwrap()
        );
    }

    // Test L1 norm 
    #[test]
    fn test_l1norm_empty()
    {
        let e = vec![] as Vec<f32>;
        assert_eq!
        (
            Option::<f32>::None,
            l1norm(e.iter())
        );
    }
}
