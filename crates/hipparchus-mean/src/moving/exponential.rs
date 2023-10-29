use crate::value::Fp;

pub fn ema<T:Fp>(s: &[T], decay:T) -> Option<T>
{
    let one = T::from_i32(1).unwrap();
    let mut total = 0;
    let mut agg = s[0];
    s.iter().for_each(|v|
    {
        agg = decay * agg + (one - decay) * *v;
        total += 1;
    });

    match total
    {
        0 => None,
        _ => Some(agg)
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use float_cmp::assert_approx_eq;

    #[test]
    fn test_ema()
    {
        let mut s = Vec::<f32>::new();
        for i in 0..100
        {
            let v = 0.9f32.powi(i);
            s.push(v);
        }
        assert_approx_eq!
        (
            f32, 0.40670013,
            ema(&s, 0.99).unwrap()
        );
    }

    #[test]
    fn test_ema_eq()
    {
        let s = vec![1.0; 10];
        assert_approx_eq!
        (
            f32, 1.0,
            ema(&s, 0.9).unwrap()
        );
    }
}
