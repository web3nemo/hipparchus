use crate::value::Fp;
use float_cmp::approx_eq;

pub trait LpNorm<'a, T>
where
    T: Fp + 'a,
    Self: Iterator<Item = &'a T>
{
    fn lpnorm(self, p:f32) -> Option<T>;
    fn l0norm(self) -> Option<T>;
    fn l1norm(self) -> Option<T>;
    fn l2norm(self) -> Option<T>;
    fn lpnorm_inf(self) -> Option<T>;
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
            p if approx_eq!(f32, p, 0.0) => self.l0norm(),
            p if approx_eq!(f32, p, 1.0) => self.l1norm(),
            p if approx_eq!(f32, p, 2.0) => self.l2norm(),
            p if p.is_infinite() => self.lpnorm_inf(),
            _ =>
            {
                let k = T::from_f32(p).unwrap();
                let mut empty = true;
                let sum = self.fold(T::zero(), |s,&x|
                {
                    empty = false;
                    s + x.powf(k)
                });
    
                if empty { None } else { Some(sum.powf(T::one()/k)) }
            }
        }
    }

    fn l0norm(self) -> Option<T>
    {
        let mut empty = true;
        let sum = self.fold(0usize, |s,&x|
        {
            empty = false;
            if T::zero() != x { s + 1 } else { s }
        });
        
        if empty { None } else { T::from_usize(sum) }
    }

    fn l1norm(self) -> Option<T>
    {
        let mut empty = true;
        let sum = self.fold(T::zero(), |s,&x|
        {
            empty = false;
            s + x
        });

        if empty { None } else { Some(sum) }
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

    fn lpnorm_inf(self) -> Option<T>
    {
        let agg = self.max_by(|a, b| a.partial_cmp(b).unwrap());
        match agg
        {
            None => None,
            _ => Some(*(agg.unwrap()))
        }
    }
}

#[cfg(test)]
mod tests 
{
    use super::*;
    use float_cmp::assert_approx_eq;

    #[test]
    fn test_l0norm()
    {
        let v = vec![0.0, 1.0, 2.0, 3.0, 4.0];
        assert_approx_eq!
        (
            f32, 4.0,
            v.iter().lpnorm(0.0).unwrap()
        );
    }

    #[test]
    fn test_l1norm()
    {
        let v = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_approx_eq!
        (
            f32, 15.0,
            v.iter().lpnorm(1.0).unwrap()
        );
    }

    #[test]
    fn test_l2norm()
    {
        let v = vec![3.0, 4.0];
        assert_approx_eq!
        (
            f32, 5.0,
            v.iter().lpnorm(2.0).unwrap()
        );
    }    // Test Lp norm 

    #[test]
    fn test_lpnorm_inf()
    {
        let v = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_approx_eq!
        (
            f32, 5.0,
            v.iter().lpnorm(f32::INFINITY).unwrap()
        );
    }

    #[test]
    fn test_lpnorm()
    {
        let v = vec![1.0, 1.0, 1.0, 2.0, 2.0, 2.0];
        assert_approx_eq!
        (
            f32, 3.0,
            v.iter().lpnorm(3.0).unwrap()
        );
    }

    #[test]
    fn test_lpnorm_empty()
    {
        let e = vec![] as Vec<f32>;
        assert_eq!(None, e.iter().l0norm());
        assert_eq!(None, e.iter().l1norm());
        assert_eq!(None, e.iter().l2norm());
        assert_eq!(None, e.iter().lpnorm_inf());
        assert_eq!(None, e.iter().lpnorm(3.0));
    }
}
