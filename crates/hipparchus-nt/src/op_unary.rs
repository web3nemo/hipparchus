/// Implement unary operator for wrapped type
#[macro_export]
macro_rules! op_unary
{
    // Implement unary operator & trait for wrapped new type (non-mut with Output=Self)
    (impl $trait:ident : fn $fn:ident for $wrap:ident$(<$raw:ident>)?) => 
    {
        impl$(<$raw>)? $trait for $wrap$(<$raw>)? $(where $raw:$trait<Output=$raw>)?
        {
            type Output = Self;
            
            fn $fn(self) -> Self::Output
            {
                Self(self.0.$fn())
            }
        }
    };

    // Implement unary operator & trait for wrapped new type (mut)
    (impl $trait:ident : mut fn $fn:ident for $wrap:ident$(<$raw:ident>)?) => 
    {
        impl$(<$raw>)? $trait for $wrap$(<$raw>)? $(where $raw:$trait)?
        {
            fn $fn(&mut self)
            {
                self.0.$fn()
            }
        }
    };
}

#[cfg(test)]
mod tests
{
    use crate::*;
    use rstest::*;
    use std::ops::*;

    #[derive(Debug, Copy, Clone)]
    pub struct GenericNewType<T>(T);
    impl_newtype!(GenericNewType<T>);
    op_unary!(impl Neg : fn neg for GenericNewType<T>);
    
    pub struct ConcreteNewType(i32);
    impl_newtype!(ConcreteNewType(i32));
    op_unary!(impl Neg : fn neg for ConcreteNewType);

    #[rstest]
    #[case(1, -1)]
    fn test_unary_generic(#[case] raw: i32, #[case] expected: i32)
    {
        let nt = GenericNewType::new(raw);
        let res = -nt;
        assert_eq!(expected, res.unwrap());
    }

    #[rstest]
    #[case(1, -1)]
    fn test_unary_concrete(#[case] raw: i32, #[case] expected: i32)
    {
        let nt = ConcreteNewType::new(raw);
        let res = -nt;
        assert_eq!(expected, res.unwrap());
    }
}
