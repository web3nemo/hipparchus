use crate::value::Fp;
use float_cmp::approx_eq;

/// compute the Lp norm of a vector
pub trait LpNorm<'a, T>
where
    T: Fp + 'a,
    Self: Iterator<Item = &'a T>
{
    /// Compute the Lp norm of a vector with the specified `p` prameter (p=0, 1, 2, ..., Inf)
    fn lpnorm(self, p:f32) -> Option<T>;

    /// Compute the Lp norm of a vector with p = Inf
    fn lpnorm_inf(self) -> Option<T>;

    /// Compute the L2 norm of a vector
    fn l2norm(self) -> Option<T>;

    /// Compute the L1 norm of a vector
    fn l1norm(self) -> Option<T>;

    /// Compute the L0 norm of a vector
    fn l0norm(self) -> Option<T>;
}

impl<'a,T,I> LpNorm<'a,T> for I
where
    T: Fp + 'a,
    I: Iterator<Item = &'a T>,
{
    fn lpnorm(self, p:f32) -> Option<T>
    {
        match p
        {
            p if p.is_infinite() && p.is_sign_positive() => self.lpnorm_inf(),
            p if approx_eq!(f32, p, 2.0) => self.l2norm(),
            p if approx_eq!(f32, p, 1.0) => self.l1norm(),
            p if approx_eq!(f32, p, 0.0) => self.l0norm(),
            p if p >= 0.0 => 
            {
                let p = T::from_f32(p).unwrap();
                let mut empty = true;
                let sum = self.fold(T::zero(), |s,&x|
                {
                    empty = false;
                    s + x.abs().powf(p)
                });
    
                if empty { None } else { Some(sum.powf(T::one()/p)) }
            }
            _ => None
        }
    }

    fn lpnorm_inf(self) -> Option<T>
    {
        let agg = self.max_by(|a, b| a.partial_cmp(b).unwrap());
        match agg
        {
            None => None,
            _ => Some(*(agg.unwrap()))
        }
    }

    fn l2norm(self) -> Option<T>
    {
        let mut empty = true;
        let sum = self.fold(T::zero(), |s,&x|
        {
            empty = false;
            s + x * x
        });

        if empty { None } else { Some(sum.sqrt()) }
    }

    fn l1norm(self) -> Option<T>
    {
        let mut empty = true;
        let sum = self.fold(T::zero(), |s,&x|
        {
            empty = false;
            s + x.abs()
        });

        if empty { None } else { Some(sum) }
    }

    fn l0norm(self) -> Option<T>
    {
        let mut empty = true;
        let sum = self.fold(0usize, |s,&x|
        {
            empty = false;
            if x.is_zero() { s } else { s + 1 }
        });
        
        if empty { None } else { T::from_usize(sum) }
    }
}

#[cfg(test)]
mod tests 
{
    use super::*;
    use rstest::*;
    use float_cmp::assert_approx_eq;

    #[rstest]
    #[case(vec![0.0, 1.0, 2.0, 3.0, 4.0], 0.0, 4.0)]
    #[case(vec![1.0, 2.0, 3.0, 4.0, 5.0], 1.0, 15.0)]
    #[case(vec![3.0, 4.0], 2.0, 5.0)]
    #[case(vec![1.0, 2.0, 3.0, 4.0, 5.0], f32::INFINITY, 5.0)]
    #[case(vec![1.0, 1.0, 1.0, 2.0, 2.0, 2.0], 3.0, 3.0)]
    fn test_lpnorm(#[case] v: Vec<f32>, #[case] p: f32, #[case] expected: f32)
    {
        assert_approx_eq!(f32, expected, v.iter().lpnorm(p).unwrap());
    }

    #[rstest]
    #[case(0.0)]
    #[case(1.0)]
    #[case(2.0)]
    #[case(3.0)]
    #[case(f32::INFINITY)]
    fn test_lpnorm_empty(#[case] p: f32,)
    {
        let e = vec![] as Vec<f32>;
        assert_eq!(None, e.iter().lpnorm(p));
    }

    #[rstest]
    #[case(-1.0)]
    #[case(f32::NEG_INFINITY)]
    fn test_lpnorm_invalid(#[case] p: f32)
    {
        let v = vec![3.0, 4.0];
        assert_eq!(None, v.iter().lpnorm(p));
    }
}
