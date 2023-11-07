use crate::metrics::Metrics;

#[repr(i32)]
#[derive(Clone,PartialEq,Debug)]
pub enum TextDistance
{
    Hamming = 1,
    Levenshtein = 2,
}

impl Metrics<&str, f32> for TextDistance
{
    fn measure(self, t1:&str, t2:&str) -> f32
    {
        match self
        {
            TextDistance::Hamming => TextDistance::hamming(t1, t2) as f32,
            TextDistance::Levenshtein => TextDistance::levenshtein(t1, t2) as f32,
        }
    }
}

impl TextDistance
{
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
}

#[cfg(test)]
mod tests 
{
    use super::*;
    use rstest::*;
    use float_cmp::assert_approx_eq;

    #[rstest]
    #[case("ABCD", "ABCE", TextDistance::Hamming, 1.0)]
    #[case("ABCD", "AB", TextDistance::Hamming, 2.0)]
    #[case("ABCD", "CD", TextDistance::Hamming, 4.0)]
    #[case("Hello, world!", "hello, world", TextDistance::Hamming, 2.0)]
    #[case("ABCD", "ABCE", TextDistance::Levenshtein, 1.0)]
    #[case("ABCD", "AB", TextDistance::Levenshtein, 2.0)]
    #[case("ABCD", "CD", TextDistance::Levenshtein, 2.0)]
    #[case("Hello, world!", "hello, world", TextDistance::Levenshtein, 2.0)]
    fn test_text_distance(#[case] t1: &str, #[case] t2: &str, #[case] metrics: TextDistance, #[case] distance: f32)
    {
        assert_approx_eq!(f32, distance, metrics.measure(t1, t2));
    }

    #[rstest]
    #[case("", TextDistance::Hamming)]
    #[case("", TextDistance::Levenshtein)]
    #[case("Hello, world!", TextDistance::Hamming)]
    #[case("Hello, world!", TextDistance::Levenshtein)]
    fn test_text_distance_eq(#[case] t: &str, #[case] metrics: TextDistance)
    {
        assert_approx_eq!(f32, 0.0, metrics.measure(t, t));
    }

    #[rstest]
    #[case("Hello, world!", TextDistance::Hamming, 13.0)]
    #[case("ABCD", TextDistance::Levenshtein, 4.0)]
    fn test_text_distance_empty(#[case] t: &str, #[case] metrics: TextDistance, #[case] distance: f32)
    {
        assert_approx_eq!(f32, distance, metrics.measure(t, ""));
    }
}
