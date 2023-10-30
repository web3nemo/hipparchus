use num::Float;
use super::sortedvec::sorted_intersect;

pub fn sorensen<T:PartialOrd, U:Float>(x:&[T], y:&[T]) -> U
{
    let total_x = x.len();
    let total_y = y.len();

    match (total_x, total_y)
    {
        (0, 0) => U::zero(),
        (0, _) => U::one(),
        (_, 0) => U::one(),
        (_, _) =>
        {
            let i = sorted_intersect(&x, &y);
            U::one() - U::from(2).unwrap() * U::from(i).unwrap() / U::from(total_x + total_y).unwrap()
        }
    }
}

#[cfg(test)]
mod tests 
{
    use super::sorensen;
    use float_cmp::assert_approx_eq;

    // Test sorensen distance
    #[test]
    fn test_sorensen()
    {
        assert_approx_eq!
        (
            f32,
            0.6,
            sorensen::<i32,f32>
            (
                &vec![1, 2, 3, 5, 7],
                &vec![1, 2, 4, 6, 8],
            )
        );

        assert_approx_eq!
        (
            f32,
            0.6,
            sorensen::<i32,f32>
            (
                &vec![1, 3, 5, 7, 8],
                &vec![2, 4, 6, 7, 8],
            )
        );
    }

    // Test sorensen distance
    #[test]
    fn test_sorensen_diffsize()
    {
        assert_approx_eq!
        (
            f32,
            0.6,
            sorensen::<i32,f32>
            (
                &vec![1, 2, 3, 5],
                &vec![1, 2, 4, 6, 7, 8],
            )
        );

        assert_approx_eq!
        (
            f32,
            0.6,
            sorensen::<i32,f32>
            (
                &vec![1, 3, 7, 8],
                &vec![2, 4, 5, 6, 7, 8],
            )
        );
    }

    // Test sorensen distance
    #[test]
    fn test_jsorensen_empty()
    {
        assert_approx_eq!
        (
            f32,
            1.0,
            sorensen::<i32,f32>
            (
                &vec![],
                &vec![1, 2, 3],
            )
        );

        assert_approx_eq!
        (
            f32,
            1.0,
            sorensen::<i32,f32>
            (
                &vec![1, 2, 3],
                &vec![],
            )
        );
    }
    
    // Test sorensen distance
    #[test]
    fn test_sorensen_allempty()
    {
        assert_approx_eq!
        (
            f32,
            0.0,
            sorensen::<i32,f32>
            (
                &vec![],
                &vec![],
            )
        );
    }
}