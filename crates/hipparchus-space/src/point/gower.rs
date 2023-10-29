use hipparchus_mean::{Fp, arithmetic};

pub fn gower<T:Fp>(x: &[T], y: &[T]) -> T
{
    let all:Vec<T> = x.iter()
        .zip(y.iter())
        .map(|(a, &b)| a.sub(b).abs())
        .collect();
    let it = all.iter();
    arithmetic(it).unwrap()
}

#[cfg(test)]
mod tests 
{
    use super::*;
    use float_cmp::assert_approx_eq;

    // Test gower distance calculation on f32 vectors 
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

    // Test gower distance calculation on f32 vectors 
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
