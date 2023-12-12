/// Implement binary operator for wrapped type.
///  The macro supports the following 12 combinations:
///     - $raw: Generic or n/a (concrete)
///     - $fn: mut or non-mut (with Output=Self)
///     - $rhs: n/a (Rhs=Self), with Rhs (Rhs in generic type) or with Rhs=$rhs:ty (Rhs in concrete type)
#[macro_export]
macro_rules! op_binary
{
    // Implement binary operator & trait for wrapped new type: non-mut with Rhs=Self, Output=Self
    (impl $trait:ident: fn $fn:ident for $wrap:ident$(<$raw:ident>)?) => 
    {
        impl$(<$raw>)? $trait for $wrap$(<$raw>)? $(where $raw:$trait<Output=$raw>)?
        {
            type Output = Self;
            
            fn $fn(self, rhs:Self) -> Self::Output
            {
                Self(self.0.$fn(rhs.0))
            }
        }
    };

    // Implement binary operator & trait for wrapped new type: mut with Rhs=Self
    (impl $trait:ident : mut fn $fn:ident for $wrap:ident$(<$raw:ident>)?) => 
    {
        impl$(<$raw>)? $trait for $wrap$(<$raw>)? $(where $raw:$trait)?
        {
            fn $fn(&mut self, rhs:Self)
            {
                self.0.$fn(rhs.0)
            }
        }
    };

    // Implement binary operator & trait for wrapped new type: non-mut with Rhs=Generic & Output=Self)
    (impl $trait:ident<Rhs> : fn $fn:ident for $wrap:ident$(<$raw:ident>)?) => 
    {
        impl<$($raw,)?Rhs> $trait<Rhs> for $wrap$(<$raw>)? $(where $raw:$trait<Rhs,Output=$raw>)?
        {
            type Output = Self;
            
            fn $fn(self, rhs:Rhs) -> Self::Output
            {
                Self(self.0.$fn(rhs))
            }
        }
    };

    // Implement binary operator & trait for wrapped new type: mut with Rhs=Generic
    (impl $trait:ident<Rhs> : mut fn $fn:ident for $wrap:ident$(<$raw:ident>)?) => 
    {
        impl<$($raw,)?Rhs> $trait<Rhs> for $wrap$(<$raw>)? $(where $raw:$trait<Rhs>)?
        {
            fn $fn(&mut self, rhs:Rhs)
            {
                self.0.$fn(rhs)
            }
        }
    };

    // Implement binary operator & trait for wrapped new type: non-mut with Rhs=Concrete & Output=Self
    (impl $trait:ident<Rhs=$rhs:ty> : fn $fn:ident for $wrap:ident$(<$raw:ident>)?) => 
    {
        impl$(<$raw>)? $trait<$rhs> for $wrap$(<$raw>)? $(where $raw:$trait<$rhs,Output=$raw>)?
        {
            type Output = Self;
            
            fn $fn(self, rhs:$rhs) -> Self::Output
            {
                Self(self.0.$fn(rhs))
            }
        }
    };

    // Implement binary operator & trait for wrapped new type: non-mut with Rhs=Concrete & Output=Self
    (impl $trait:ident<Rhs=$rhs:ty> : mut fn $fn:ident for $wrap:ident$(<$raw:ident>)?) => 
    {
        impl$(<$raw>)? $trait<$rhs> for $wrap$(<$raw>)? $(where $raw:$trait<$rhs>)?
        {
            fn $fn(&mut self, rhs:$rhs)
            {
                self.0.$fn(rhs)
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
    op_binary!(impl Add : fn add for GenericNewType<T>);
    op_binary!(impl AddAssign : mut fn add_assign for GenericNewType<T>);
    op_binary!(impl Mul<Rhs> : fn mul for GenericNewType<T>);
    op_binary!(impl MulAssign<Rhs> : mut fn mul_assign for GenericNewType<T>);

    pub struct ConcreteNewType(i32);
    impl_newtype!(ConcreteNewType(i32));
    op_binary!(impl Add : fn add for ConcreteNewType);
    op_binary!(impl AddAssign : mut fn add_assign for ConcreteNewType);
    op_binary!(impl Mul<Rhs=i32> : fn mul for ConcreteNewType);
    op_binary!(impl MulAssign<Rhs=i32> : mut fn mul_assign for ConcreteNewType);

    #[rstest]
    #[case(1, 2, 3)]
    fn test_binary_generic(#[case] raw: i32, #[case] rhs: i32, #[case] expected: i32)
    {
        let nt = GenericNewType::new(raw);
        let res = nt + GenericNewType::new(rhs);
        assert_eq!(expected, res.unwrap());
    }

    #[rstest]
    #[case(1, 2, 3)]
    fn test_binary_generic_mut(#[case] raw: i32, #[case] rhs: i32, #[case] expected: i32)
    {
        let mut nt = GenericNewType::new(raw);
        nt += GenericNewType::new(rhs);
        assert_eq!(expected, nt.unwrap());
    }

    #[rstest]
    #[case(2, 3, 6)]
    fn test_binary_generic_with(#[case] raw: i32, #[case] rhs: i32, #[case] expected: i32)
    {
        let nt = GenericNewType::new(raw);
        let res = nt * rhs;
        assert_eq!(expected, res.unwrap());
    }

    #[rstest]
    #[case(2, 3, 6)]
    fn test_binary_generic_mut_with(#[case] raw: i32, #[case] rhs: i32, #[case] expected: i32)
    {
        let mut wrap = GenericNewType::new(raw);
        wrap *= rhs;
        assert_eq!(expected, wrap.unwrap());
    }

    #[rstest]
    #[case(1, 2, 3)]
    fn test_binary_concrete(#[case] raw: i32, #[case] rhs: i32, #[case] expected: i32)
    {
        let nt = ConcreteNewType::new(raw);
        let res = nt + ConcreteNewType::new(rhs);
        assert_eq!(expected, res.unwrap());
    }

    #[rstest]
    #[case(1, 2, 3)]
    fn test_binary_concrete_mut(#[case] raw: i32, #[case] rhs: i32, #[case] expected: i32)
    {
        let mut nt = ConcreteNewType::new(raw);
        nt += ConcreteNewType::new(rhs);
        assert_eq!(expected, nt.unwrap());
    }

    #[rstest]
    #[case(2, 3, 6)]
    fn test_binary_concrete_with(#[case] raw: i32, #[case] rhs: i32, #[case] expected: i32)
    {
        let nt = ConcreteNewType::new(raw);
        let res = nt * rhs;
        assert_eq!(expected, res.unwrap());
    }

    #[rstest]
    #[case(2, 3, 6)]
    fn test_binary_concrete_mut_with(#[case] raw: i32, #[case] rhs: i32, #[case] expected: i32)
    {
        let mut wrap = ConcreteNewType::new(raw);
        wrap *= rhs;
        assert_eq!(expected, wrap.unwrap());
    }
}
