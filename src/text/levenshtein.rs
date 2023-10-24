pub fn levenshtein(x: &str, y: &str) -> usize
{
    if x.is_empty()
    {
        return y.chars().count();
    }
    if y.is_empty()
    {
        return x.chars().count();
    }
    if x == y
    {
        return 0;
    }

    let mut cache: Vec<usize> = (1..).take(x.chars().count()).collect();
    let mut distance_a;
    let mut distance_b;
    let mut result = 0;
    for (idxb, b) in y.chars().enumerate()
    {
        result = idxb;
        distance_a = idxb;

        for (idxa, a) in x.chars().enumerate()
        {
            distance_b = if a == b { distance_a } else { distance_a + 1 };
            distance_a = cache[idxa];
            result = 
                if distance_a > result
                {
                    if distance_b > result
                    {
                        result + 1
                    }
                    else
                    {
                        distance_b
                    }
                }
                else if distance_b > distance_a
                {
                    distance_a + 1
                }
                else
                {
                    distance_b
                };

            cache[idxa] = result;
        }
    }

    result
}

#[cfg(test)]
mod tests 
{
    use super::levenshtein;

    // Test levenshtein distance
    #[test]
    fn test_levenshtein()
    {
        assert_eq!
        (
            1,
            levenshtein
            (
                "ABCD",
                "ABCE"
            )
        );
    }

    // Test levenshtein distance
    #[test]
    fn test_levenshtein_equal()
    {
        assert_eq!
        (
            0,
            levenshtein
            (
                "ABCD",
                "ABCD"
            )
        );
    }

    // Test levenshtein distance
    #[test]
    fn test_levenshtein_empty()
    {
        assert_eq!
        (
            4,
            levenshtein
            (
                "ABCD",
                ""
            )
        );

        assert_eq!
        (
            4,
            levenshtein
            (
                "",
                "ABCD"
            )
        );

    }
}