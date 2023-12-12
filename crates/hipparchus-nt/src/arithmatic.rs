#[macro_export]
macro_rules! op_arithmatic
{
    ($wrap:ident$(<$raw:ident>)? => All) =>
    {
        op_arithmatic!($wrap$(<$raw>)? => Add $(, Add<Rhs=$raw>)?, Sub $(, Sub<Rhs=$raw>)?, Mul $(, Mul<Rhs=$raw>)?, Div $(, Div<Rhs=$raw>)?, Rem $(, Rem<Rhs=$raw>)?, Neg);
    };

    ($wrap:ident$(<$raw:ident>)? => $op:ident$(<Rhs$(=$rhs:ident)?>)?) =>
    {
        op_arithmatic!($wrap$(<$raw>)?, $op$(<Rhs$(=$rhs)?>)?);
    };

    ($wrap:ident$(<$raw:ident>)? => $op:ident$(<Rhs$(=$rhs:ident)?>)?, $($tail:tt)*) =>
    {
        #[allow(unused_imports)]        
        use std::ops::*;
        #[allow(unused_imports)]        
        use $crate::*;
        
        op_arithmatic!($wrap$(<$raw>)?, $op$(<Rhs$(=$rhs)?>)?);
        op_arithmatic!($wrap$(<$raw>)? => $($tail)*);
    };

    ($wrap:ident$(<$raw:ident>)?, Add$(<Rhs$(=$rhs:ident)?>)?) =>
    {
        op_binary!(impl Add$(<Rhs$(=$rhs)?>)? : fn add for $wrap$(<$raw>)?);
        op_binary!(impl AddAssign$(<Rhs$(=$rhs)?>)? : mut fn add_assign for $wrap$(<$raw>)?);
    };

    ($wrap:ident$(<$raw:ident>)?, Sub$(<Rhs$(=$rhs:ident)?>)?) =>
    {
        op_binary!(impl Sub$(<Rhs$(=$rhs)?>)? : fn sub for $wrap$(<$raw>)?);
        op_binary!(impl SubAssign$(<Rhs$(=$rhs)?>)? : mut fn sub_assign for $wrap$(<$raw>)?);
    };

    ($wrap:ident$(<$raw:ident>)?, Mul$(<Rhs$(=$rhs:ident)?>)?) =>
    {
        op_binary!(impl Mul$(<Rhs$(=$rhs)?>)? : fn mul for $wrap$(<$raw>)?);
        op_binary!(impl MulAssign$(<Rhs$(=$rhs)?>)? : mut fn mul_assign for $wrap$(<$raw>)?);
    };

    ($wrap:ident$(<$raw:ident>)?, Div$(<Rhs$(=$rhs:ident)?>)?) =>
    {
        op_binary!(impl Div$(<Rhs$(=$rhs)?>)? : fn div for $wrap$(<$raw>)?);
        op_binary!(impl DivAssign$(<Rhs$(=$rhs)?>)? : mut fn div_assign for $wrap$(<$raw>)?);
    };

    ($wrap:ident$(<$raw:ident>)?, Rem$(<Rhs$(=$rhs:ident)?>)?) =>
    {
        op_binary!(impl Rem$(<Rhs$(=$rhs)?>)? : fn rem for $wrap$(<$raw>)?);
        op_binary!(impl RemAssign$(<Rhs$(=$rhs)?>)? : mut fn rem_assign for $wrap$(<$raw>)?);
    };

    ($wrap:ident$(<$raw:ident>)?, Neg) =>
    {
        op_unary!(impl Neg : fn neg for $wrap$(<$raw>)?);
    };
}

#[cfg(test)]
mod tests
{
    use crate::*;
    use rstest::*;

    #[derive(Debug, Copy, Clone)]
    pub struct GenericNewType<T>(T);
    impl_newtype!(GenericNewType<T>);
    op_arithmatic!(GenericNewType<T> => All);

    #[rstest]
    #[case(1, 2, 3)]
    fn test_arithmatic_add(#[case] raw: i32, #[case] rhs:i32, #[case] expected: i32)
    {
        let wrap = GenericNewType::new(raw);
        let res = wrap + GenericNewType::new(rhs); 
        assert_eq!(expected, res.unwrap());
    }

    #[rstest]
    #[case(1, 2, 3)]
    fn test_arithmatic_addassign(#[case] raw: i32, #[case] rhs:i32, #[case] expected: i32)
    {
        let mut wrap = GenericNewType::new(raw);
        wrap += GenericNewType::new(rhs); 
        assert_eq!(expected, wrap.unwrap());
    }

    #[rstest]
    #[case(3, 2, 1)]
    fn test_arithmatic_sub(#[case] raw: i32, #[case] rhs:i32, #[case] expected: i32)
    {
        let wrap = GenericNewType::new(raw);
        let res = wrap - GenericNewType::new(rhs); 
        assert_eq!(expected, res.unwrap());
    }

    #[rstest]
    #[case(3, 2, 1)]
    fn test_arithmatic_subassign(#[case] raw: i32, #[case] rhs:i32, #[case] expected: i32)
    {
        let mut wrap = GenericNewType::new(raw);
        wrap -= GenericNewType::new(rhs); 
        assert_eq!(expected, wrap.unwrap());
    }

    #[rstest]
    #[case(2, 3, 6)]
    fn test_arithmatic_mul(#[case] raw: i32, #[case] rhs:i32, #[case] expected: i32)
    {
        let wrap = GenericNewType::new(raw);
        let res = wrap * GenericNewType::new(rhs); 
        assert_eq!(expected, res.unwrap());
    }

    #[rstest]
    #[case(2, 3, 6)]
    fn test_arithmatic_mulassign(#[case] raw: i32, #[case] rhs:i32, #[case] expected: i32)
    {
        let mut wrap = GenericNewType::new(raw);
        wrap *= GenericNewType::new(rhs); 
        assert_eq!(expected, wrap.unwrap());
    }

    #[rstest]
    #[case(6, 2, 3)]
    fn test_arithmatic_div(#[case] raw: i32, #[case] rhs:i32, #[case] expected: i32)
    {
        let wrap = GenericNewType::new(raw);
        let res = wrap / GenericNewType::new(rhs); 
        assert_eq!(expected, res.unwrap());
    }

    #[rstest]
    #[case(6, 2, 3)]
    fn test_arithmatic_divassign(#[case] raw: i32, #[case] rhs:i32, #[case] expected: i32)
    {
        let mut wrap = GenericNewType::new(raw);
        wrap /= GenericNewType::new(rhs); 
        assert_eq!(expected, wrap.unwrap());
    }

    #[rstest]
    #[case(1, 2, 3)]
    fn test_arithmatic_add_raw(#[case] raw: i32, #[case] rhs:i32, #[case] expected: i32)
    {
        let wrap = GenericNewType::new(raw);
        let res = wrap + rhs; 
        assert_eq!(expected, res.unwrap());
    }

    #[rstest]
    #[case(1, 2, 3)]
    fn test_arithmatic_addassign_raw(#[case] raw: i32, #[case] rhs:i32, #[case] expected: i32)
    {
        let mut wrap = GenericNewType::new(raw);
        wrap += rhs; 
        assert_eq!(expected, wrap.unwrap());
    }

    #[rstest]
    #[case(3, 2, 1)]
    fn test_arithmatic_sub_raw(#[case] raw: i32, #[case] rhs:i32, #[case] expected: i32)
    {
        let wrap = GenericNewType::new(raw);
        let res = wrap - rhs; 
        assert_eq!(expected, res.unwrap());
    }

    #[rstest]
    #[case(3, 2, 1)]
    fn test_arithmatic_subassign_raw(#[case] raw: i32, #[case] rhs:i32, #[case] expected: i32)
    {
        let mut wrap = GenericNewType::new(raw);
        wrap -= rhs; 
        assert_eq!(expected, wrap.unwrap());
    }

    #[rstest]
    #[case(2, 3, 6)]
    fn test_arithmatic_mul_raw(#[case] raw: i32, #[case] rhs:i32, #[case] expected: i32)
    {
        let wrap = GenericNewType::new(raw);
        let res = wrap * rhs; 
        assert_eq!(expected, res.unwrap());
    }

    #[rstest]
    #[case(2, 3, 6)]
    fn test_arithmatic_mulassign_raw(#[case] raw: i32, #[case] rhs:i32, #[case] expected: i32)
    {
        let mut wrap = GenericNewType::new(raw);
        wrap *= rhs; 
        assert_eq!(expected, wrap.unwrap());
    }

    #[rstest]
    #[case(6, 2, 3)]
    fn test_arithmaticwith_div(#[case] raw: i32, #[case] rhs:i32, #[case] expected: i32)
    {
        let wrap = GenericNewType::new(raw);
        let res = wrap / rhs; 
        assert_eq!(expected, res.unwrap());
    }

    #[rstest]
    #[case(6, 2, 3)]
    fn test_arithmaticwith_divassign(#[case] raw: i32, #[case] rhs:i32, #[case] expected: i32)
    {
        let mut wrap = GenericNewType::new(raw);
        wrap /= rhs; 
        assert_eq!(expected, wrap.unwrap());
    }
}
