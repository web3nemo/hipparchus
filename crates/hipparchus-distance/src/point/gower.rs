use hipparchus_mean::{Fp, Mean};

pub fn gower<T:Fp>(x: &[T], y: &[T]) -> T
{
    let all:Vec<T> = x.iter()
        .zip(y.iter())
        .map(|(a, &b)| a.sub(b).abs())
        .collect();
    
    all.iter().arithmetic_mean().unwrap()
}

#[cfg(test)]
mod tests 
{
    use super::*;
    use float_cmp::assert_approx_eq;

    #[test]
    fn test_gower()
    {
        assert_approx_eq!
        (
            f32, 1.5,
            gower::<f32>
            (
                &[0.0, 1.0],
                &[1.0, -1.0],
            )
        );

        assert_approx_eq!
        (
            f32, 1.5,
            gower::<f32>
            (
                &[1.0, -1.0],
                &[0.0, 1.0],
            )
        );
    }

    #[test]
    fn test_gower_zero()
    {
        assert_approx_eq!
        (
            f32, 0.0,
            gower::<f32>
            (
                &[1.0, 1.0],
                &[1.0, 1.0],
            )
        );
    }
}
