pub fn hamming(x: &str, y: &str) -> usize
{
    let mut total = 0;
    let mut xchars = x.chars();
    let mut ychars = y.chars();
    loop
    {
        let xchar = xchars.next();
        let ychar = ychars.next();
        if xchar == None || ychar == None
        {
            let xdelta = if xchar != None { xchars.count() + 1 } else { 0 };
            let ydelta = if ychar != None { ychars.count() + 1 } else { 0 };
            total += xdelta + ydelta;
            break;
        }
        if xchar.unwrap() != ychar.unwrap()
        {
            total += 1
        }
    }
    total
}

#[cfg(test)]
mod tests 
{
    use super::hamming;

    // Test hamming distance
    #[test]
    fn test_hamming()
    {
        assert_eq!
        (
            1,
            hamming
            (
                "ABCD",
                "ABCE"
            )
        );
    }

    // Test hamming distance
    #[test]
    fn test_levenshtein_equal()
    {
        assert_eq!
        (
            0,
            hamming
            (
                "ABCD",
                "ABCD"
            )
        );
    }

    // Test hamming distance
    #[test]
    fn test_hamming_empty()
    {
        assert_eq!
        (
            4,
            hamming
            (
                "ABCD",
                ""
            )
        );

        assert_eq!
        (
            4,
            hamming
            (
                "",
                "ABCD"
            )
        );

    }
}