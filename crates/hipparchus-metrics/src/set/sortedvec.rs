pub fn sorted_intersect<T:PartialOrd>(x:&[T], y:&[T]) -> usize
{
    let mut x = x.iter();
    let mut y = y.iter();
    let mut a = x.next();
    let mut b = y.next();
    let mut c = 0;
    while !a.is_none() && !b.is_none()
    {
        let m = a.unwrap();
        let n = b.unwrap();
        if m <= n
        {
            a = x.next();
        }
        if n <= m
        {
            b = y.next();
        }
        if m == n
        {
            c += 1;
        }
    }
    c
}

pub fn sorted_union<T:PartialOrd>(x:&[T], y:&[T]) -> usize
{
    let mut x = x.iter();
    let mut y = y.iter();
    let mut a = x.next();
    let mut b = y.next();
    let mut c = 0;
    while !a.is_none() && !b.is_none()
    {
        let m = a.unwrap();
        let n = b.unwrap();
        if m <= n
        {
            a = x.next();
            c += 1
        }
        if n <= m
        {
            b = y.next();
            c += 1
        }
        if m == n
        {
            c -= 1;
        }
    }
    if a.is_none()
    {
        c -= 1;
    }
    if b.is_none()
    {
        c -= 1;
    }
    c + (x.count() + 1) + (y.count() + 1) 
}

#[cfg(test)]
mod tests 
{
    use crate::set::sortedvec::{sorted_intersect, sorted_union};

    // Test sorensen distance
    #[test]
    fn test_sorted_intersect()
    {
        assert_eq!
        (
            2,
            sorted_intersect
            (
                &vec![1, 2, 3, 5, 7],
                &vec![1, 2, 4, 6, 8],
            )
        );

        assert_eq!
        (
            2,
            sorted_intersect
            (
                &vec![1, 3, 5, 7, 8],
                &vec![2, 4, 6, 7, 8],
            )
        );
   }

    #[test]
    fn test_sorted_union()
    {
        assert_eq!
        (
            8,
            sorted_union
            (
                &vec![1, 2, 3, 5, 7],
                &vec![1, 2, 4, 6, 8],
            )
        );

        assert_eq!
        (
            8,
            sorted_union
            (
                &vec![1, 3, 5, 7, 8],
                &vec![2, 4, 6, 7, 8],
            )
        );
    }
}