#[macro_export]
macro_rules! op_bitwise
{
    ($wrap:ident$(<$raw:ident>)? => All) =>
    {
        op_bitwise!($wrap$(<$raw>)? => BitAnd $(, BitAnd<Rhs=$raw>)?, BitOr $(, BitOr<Rhs=$raw>)?, BitXor $(, BitXor<Rhs=$raw>)?, Shl $(, Shl<Rhs=$raw>)?, Shr $(, Shr<Rhs=$raw>)?, Not);
    };

    ($wrap:ident$(<$raw:ident>)? => $op:ident$(<Rhs$(=$rhs:ident)?>)?) =>
    {
        op_bitwise!($wrap$(<$raw>)?, $op$(<Rhs$(=$rhs)?>)?);
    };

    ($wrap:ident$(<$raw:ident>)? => $op:ident$(<Rhs$(=$rhs:ident)?>)?, $($tail:tt)*) =>
    {
        #[allow(unused_imports)]        
        use std::ops::*;
        op_bitwise!($wrap$(<$raw>)?, $op$(<Rhs$(=$rhs)?>)?);
        op_bitwise!($wrap$(<$raw>)? => $($tail)*);
    };

    ($wrap:ident$(<$raw:ident>)?, BitAnd$(<Rhs$(=$rhs:ident)?>)?) =>
    {
        op_binary!(impl BitAnd$(<Rhs$(=$rhs)?>)? : fn bitand for $wrap$(<$raw>)?);
        op_binary!(impl BitAndAssign$(<Rhs$(=$rhs)?>)? : mut fn bitand_assign for $wrap$(<$raw>)?);
    };

    ($wrap:ident$(<$raw:ident>)?, BitOr$(<Rhs$(=$rhs:ident)?>)?) =>
    {
        op_binary!(impl BitOr$(<Rhs$(=$rhs)?>)? : fn bitor for $wrap$(<$raw>)?);
        op_binary!(impl BitOrAssign$(<Rhs$(=$rhs)?>)? : mut fn bitor_assign for $wrap$(<$raw>)?);
    };

    ($wrap:ident$(<$raw:ident>)?, BitXor$(<Rhs$(=$rhs:ident)?>)?) =>
    {
        op_binary!(impl BitXor$(<Rhs$(=$rhs)?>)? : fn bitxor for $wrap$(<$raw>)?);
        op_binary!(impl BitXorAssign$(<Rhs$(=$rhs)?>)? : mut fn bitxor_assign for $wrap$(<$raw>)?);
    };

    ($wrap:ident$(<$raw:ident>)?, Shl$(<Rhs$(=$rhs:ident)?>)?) =>
    {
        op_binary!(impl Shl$(<Rhs$(=$rhs)?>)? : fn shl for $wrap$(<$raw>)?);
        op_binary!(impl ShlAssign$(<Rhs$(=$rhs)?>)? : mut fn shl_assign for $wrap$(<$raw>)?);
    };

    ($wrap:ident$(<$raw:ident>)?, Shr$(<Rhs$(=$rhs:ident)?>)?) =>
    {
        op_binary!(impl Shr$(<Rhs$(=$rhs)?>)? : fn shr for $wrap$(<$raw>)?);
        op_binary!(impl ShrAssign$(<Rhs$(=$rhs)?>)? : mut fn shr_assign for $wrap$(<$raw>)?);
    };

    ($wrap:ident$(<$raw:ident>)?, Not) =>
    {
        op_unary!(impl Not : fn not for $wrap$(<$raw>)?);
    };
}

#[cfg(test)]
mod tests
{
    use crate::*;
    use rstest::*;

    pub struct GenericNewType<T>(T);
    impl_newtype!(GenericNewType<T>);
    op_bitwise!(GenericNewType<T> => All);

    #[rstest]
    #[case(1, 2, 0)]
    fn test_bitwise_bitand(#[case] raw: i32, #[case] rhs:i32, #[case] expected: i32)
    {
        let wrap = GenericNewType::new(raw);
        let res = wrap & GenericNewType::new(rhs); 
        assert_eq!(expected, res.unwrap());
    }

    #[rstest]
    #[case(1, 2, 0)]
    fn test_bitwise_bitand_assign(#[case] raw: i32, #[case] rhs:i32, #[case] expected: i32)
    {
        let mut wrap = GenericNewType::new(raw);
        wrap &= GenericNewType::new(rhs); 
        assert_eq!(expected, wrap.unwrap());
    }

    #[rstest]
    #[case(1, 2, 3)]
    fn test_bitwise_bitor(#[case] raw: i32, #[case] rhs:i32, #[case] expected: i32)
    {
        let wrap = GenericNewType::new(raw);
        let res = wrap | GenericNewType::new(rhs); 
        assert_eq!(expected, res.unwrap());
    }

    #[rstest]
    #[case(1, 2, 3)]
    fn test_bitwise_bitor_assign(#[case] raw: i32, #[case] rhs:i32, #[case] expected: i32)
    {
        let mut wrap = GenericNewType::new(raw);
        wrap |= GenericNewType::new(rhs); 
        assert_eq!(expected, wrap.unwrap());
    }

    #[rstest]
    #[case(1, 2, 3)]
    fn test_bitwise_bitxor(#[case] raw: i32, #[case] rhs:i32, #[case] expected: i32)
    {
        let wrap = GenericNewType::new(raw);
        let res = wrap ^ GenericNewType::new(rhs); 
        assert_eq!(expected, res.unwrap());
    }

    #[rstest]
    #[case(1, 2, 3)]
    fn test_bitwise_bitxor_assign(#[case] raw: i32, #[case] rhs:i32, #[case] expected: i32)
    {
        let mut wrap = GenericNewType::new(raw);
        wrap ^= GenericNewType::new(rhs); 
        assert_eq!(expected, wrap.unwrap());
    }

    #[rstest]
    #[case(1, 2, 4)]
    fn test_bitwise_shl(#[case] raw: i32, #[case] rhs:i32, #[case] expected: i32)
    {
        let wrap = GenericNewType::new(raw);
        let res = wrap << GenericNewType::new(rhs); 
        assert_eq!(expected, res.unwrap());
    }

    #[rstest]
    #[case(1, 2, 4)]
    fn test_bitwise_shl_assign(#[case] raw: i32, #[case] rhs:i32, #[case] expected: i32)
    {
        let mut wrap = GenericNewType::new(raw);
        wrap <<= GenericNewType::new(rhs); 
        assert_eq!(expected, wrap.unwrap());
    }

    #[rstest]
    #[case(4, 2, 1)]
    fn test_bitwise_shr(#[case] raw: i32, #[case] rhs:i32, #[case] expected: i32)
    {
        let wrap = GenericNewType::new(raw);
        let res = wrap >> GenericNewType::new(rhs); 
        assert_eq!(expected, res.unwrap());
    }

    #[rstest]
    #[case(4, 2, 1)]
    fn test_bitwise_shr_assign(#[case] raw: i32, #[case] rhs:i32, #[case] expected: i32)
    {
        let mut wrap = GenericNewType::new(raw);
        wrap >>= GenericNewType::new(rhs); 
        assert_eq!(expected, wrap.unwrap());
    }

    #[rstest]
    #[case(0, -1)]
    fn test_bitwise_not(#[case] raw: i32, #[case] expected: i32)
    {
        let wrap = GenericNewType::new(raw);
        let res = !wrap; 
        assert_eq!(expected, res.unwrap());
    }
}
