use num::Float;
use super::sortedvec::sorted_intersect;

pub fn kumar_hassebrook<T:PartialOrd, U:Float>(x:&[T], y:&[T]) -> U
{
    let total_x = x.len();
    let total_y = y.len();

    match (total_x, total_y)
    {
        (0, 0) => U::one(),
        (0, _) => U::zero(),
        (_, 0) => U::zero(),
        (_, _) =>
        {
            let i = sorted_intersect(&x, &y);
            let u = total_x + total_y - i;
            U::from(i).unwrap() / U::from(u).unwrap()
        }
    }
}

pub fn jaccard<T:PartialOrd, U:Float>(x:&[T], y:&[T]) -> U
{
    U::one() - kumar_hassebrook(x, y)
}

#[cfg(test)]
mod tests
{
    use super::jaccard;
    use float_cmp::assert_approx_eq;

    // Test jaccard distance
    #[test]
    fn test_jaccard()
    {
        assert_approx_eq!
        (
            f32,
            0.75,
            jaccard::<i32,f32>
            (
                &vec![1, 2, 3, 5, 7],
                &vec![1, 2, 4, 6, 8],
            )
        );

        assert_approx_eq!
        (
            f32,
            0.75,
            jaccard::<i32,f32>
            (
                &vec![1, 3, 5, 7, 8],
                &vec![2, 4, 6, 7, 8],
            )
        );
    }

    // Test jaccard distance
    #[test]
    fn test_jaccard_diffsize()
    {
        assert_approx_eq!
        (
            f32,
            0.75,
            jaccard::<i32,f32>
            (
                &vec![1, 2, 3, 5],
                &vec![1, 2, 4, 6, 7, 8],
            )
        );

        assert_approx_eq!
        (
            f32,
            0.75,
            jaccard::<i32,f32>
            (
                &vec![1, 3, 7, 8],
                &vec![2, 4, 5, 6, 7, 8],
            )
        );
    }

    // Test jaccard distance
    #[test]
    fn test_jaccard_empty()
    {
        assert_approx_eq!
        (
            f32,
            1.0,
            jaccard::<i32,f32>
            (
                &vec![],
                &vec![1, 2, 3],
            )
        );

        assert_approx_eq!
        (
            f32,
            1.0,
            jaccard::<i32,f32>
            (
                &vec![1, 2, 3],
                &vec![],
            )
        );
    }
    
    // Test jaccard distance
    #[test]
    fn test_jaccard_allempty()
    {
        assert_approx_eq!
        (
            f32,
            0.0,
            jaccard::<i32,f32>
            (
                &vec![],
                &vec![],
            )
        );
    }
}