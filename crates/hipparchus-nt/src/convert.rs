// Implementations of From and Into traits for a generic or concrete newtype.
#[macro_export]
macro_rules! impl_newtype_from
{
    ($wrap:ident<$p:ident>, char) =>
    {
        impl_newtype_from!($wrap<$p>, [u8] => <char> => [u32 u64 u128 String]);
        impl_newtype_from!($wrap<$p>, [u32] => try<char> => [u8 u16]);
    };

    ($wrap:ident<$p:ident>, bool) =>
    {
        impl_newtype_from!($wrap<$p>, [] => <bool> => [i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize]);
        impl_newtype_from!($wrap<$p>, [] => try<i8> => []);
    };

    ($wrap:ident<$p:ident>, f32) =>
    {
        impl_newtype_from!($wrap<$p>, [i8 u8 i16 u16 ] => <f32> => [f64]);
        impl_newtype_from!($wrap<$p>, [] => try<bool> => []);
    };

    ($wrap:ident<$p:ident>, f64) =>
    {
        impl_newtype_from!($wrap<$p>, [i8 u8 i16 u16 i32 u32] => <f64> => []);
        impl_newtype_from!($wrap<$p>, [] => try<f64> => []);
    };

    ($wrap:ident<$p:ident>, i8) =>
    {
        impl_newtype_from!($wrap<$p>, [] => <bool i8> => [f32 f64 i16 i32 i64 i128 isize]);
        impl_newtype_from!($wrap<$p>, [u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize] => try<i8> => [u8 u16 u32 u64 u128 usize]);
    };

    ($wrap:ident<$p:ident>, u8) =>
    {
        impl_newtype_from!($wrap<$p>, [] => <u8> => [bool f32 f64 i16 u16 i32 u32 i64 u64 i128 u128 isize usize]);
        impl_newtype_from!($wrap<$p>, [char i8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize] => try<u8> => [i8]);
    };

    ($wrap:ident<$p:ident>, i16) =>
    {
        impl_newtype_from!($wrap<$p>, [bool i8 u8] => <i16> => [f32 f64 i32 i64 i128]);
        impl_newtype_from!($wrap<$p>, [u16 i32 u32 i64 u64 i128 u128 isize usize] => try<i16> => [i8 u8 u16 u32 u64 u128]);
    };

    ($wrap:ident<$p:ident>, u16) =>
    {
        impl_newtype_from!($wrap<$p>, [bool u8] => <u16> => [f32 f64 i64 u64 i128 u128]);
        impl_newtype_from!($wrap<$p>, [char i8 i16 i32 u32 i64 u64 i128 u128 isize usize] => try<u16> => [i8 u8 i16 u16 i32]);
    };

    ($wrap:ident<$p:ident>, i32) =>
    {
        impl_newtype_from!($wrap<$p>, [bool i8 u8 i16 u16] => <i32> => [f64 i64 i128]);
        impl_newtype_from!($wrap<$p>, [u32 i64 u64 i128 u128 isize usize] => try<i32> => [i8 u8 i16 u16 u32 u64 u128]);
    };

    ($wrap:ident<$p:ident>, u32) =>
    {
        impl_newtype_from!($wrap<$p>, [bool u8 u16] => <u32> => [f64 i64 u64 i128 u128]);
        impl_newtype_from!($wrap<$p>, [i8 i16 i32 i64 u64 i128 u128 isize usize] => try<u32> => [i8 u8 i16 u16 i32]);
    };

    ($wrap:ident<$p:ident>, i64) =>
    {
        impl_newtype_from!($wrap<$p>, [bool i8 u8 i16 u16 i32 u32] => <i64> => [i128]);
        impl_newtype_from!($wrap<$p>, [u64 i128 u128 isize usize] => try<i64> => [i8 u8 i16 u16 i32 u32 u64 u128]);
    };

    ($wrap:ident<$p:ident>, u64) =>
    {
        impl_newtype_from!($wrap<$p>, [bool u8 u16 u32] => <u64> => [i128 u128]);
        impl_newtype_from!($wrap<$p>, [i8 i16 i32 i64 i128 u128 isize usize] => try<u64> => [i8 u8 i16 u16 i32 u32 i64]);
    };

    ($wrap:ident<$p:ident>, i128) =>
    {
        impl_newtype_from!($wrap<$p>, [bool i8 u8 i16 u16 i32 u32 i64 u64] => <i128> => []);
        impl_newtype_from!($wrap<$p>, [u128 isize usize] => try<i128> => [i8 u8 i16 u16 i32 u32 i64 u64 u128]);
    };

    ($wrap:ident<$p:ident>, u128) =>
    {
        impl_newtype_from!($wrap<$p>, [bool u8 u16 u32 u64] => <u128> => []);
        impl_newtype_from!($wrap<$p>, [i8 i16 i32 i64 i128 isize usize] => try<u128> => [i8 u8 i16 u16 i32 u32 i64 u64 i128]);
    };

    ($wrap:ident<$p:ident>, isize) =>
    {
        impl_newtype_from!($wrap<$p>, [bool i8 u8 i16] => <isize> => []);
        impl_newtype_from!($wrap<$p>, [u16 i32 u32 i64 u64 i128 u128 usize] => try<isize> => [i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 usize]);
    };

    ($wrap:ident<$p:ident>, usize) =>
    {
        impl_newtype_from!($wrap<$p>, [bool u8 u16] => <usize> => []);
        impl_newtype_from!($wrap<$p>, [i8 i16 i32 i64 i128 u128 isize] => try<usize> => [i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize]);
    };

    ($wrap:ident<$p:ident>: <$raw:tt>, $($tail:tt)*) =>
    {
        impl_newtype_from!($wrap<$p>, $raw);
        impl_newtype_from!($wrap<$p>: $($tail)*);
    };

    ($wrap:ident<$p:ident>: <$raw:tt>) =>
    {
        impl_newtype_from!($wrap<$p>, $raw);
        impl_newtype_from!($wrap<$p>);
    };

    ($wrap:ident<$p:ident>: [$($from:tt)*] => <$raw:ty> => [$($into:tt)*], $($tail:tt)*) =>
    {
        impl_newtype_from!($wrap<$p>, [$($from)*] => <$raw> => [$($into)*]);
        impl_newtype_from!($wrap<$p>: $($tail)*);
    };

    ($wrap:ident<$p:ident>: [$($from:ty)*]=> <$raw:ty> => [$($into:ty)*]) =>
    {
        impl_newtype_from!($wrap<$p>, [$($from)*] => <$raw> => [$($into)*]);
        impl_newtype_from!($wrap<$p>);
    };

    ($wrap:ident<$p:ident>: [$($from:ty)*] => try<$raw:ty> => [$($into:ty)*], $($tail:tt)*) =>
    {
        impl_newtype_from!($wrap<$p>, [$($from)*] => try<$raw> => [$($into)*]);
        impl_newtype_from!($wrap<$p>: $($tail)*);
    };

    ($wrap:ident<$p:ident>: [$($from:ty)*] => try<$raw:ty> => [$($into:ty)*]) =>
    {
        impl_newtype_from!($wrap<$p>, [$($from)*] => try<$raw> => [$($into)*]);
        impl_newtype_from!($wrap<$p>);
    };

    ($wrap:ident<$p:ident>) =>
    {
        impl<$p> From<$p> for $wrap<$p>
        {
            #[inline]
            fn from(v: $p) -> Self { Self(v) }
        }
    };

    ($wrap:ident<$p:ident>, [$($from:ty)*] => <$raw:ty> => [$($into:ty)*]) =>
    {
        impl From<$wrap<$raw>> for $raw
        {
            #[inline]
            fn from(v: $wrap<$raw>) -> $raw { v.0 }
        }

        $(
            impl_newtype_from!($wrap<$p>, <$raw> <= $from);
        )*
        
        $(
            impl_newtype_from!($wrap<$p>, <$raw> => $into);
        )*
    };

    ($wrap:ident<$p:ident>, [$($from:ty)*] => try<$raw:ty> => [$($into:ty)*]) =>
    {
        $(
            impl_newtype_from!($wrap<$p>, try<$raw> <= $from);
        )*
        
        $(
            impl_newtype_from!($wrap<$p>, try<$raw> => $into);
        )*
    };

    ($wrap:ident<$p:ident>, <$raw:ty> <= $from:ty) =>
    {
        impl From<$from> for $wrap<$raw>
        {
            #[inline]
            fn from(v: $from) -> Self { Self(v.into()) }
        }
    };

    ($wrap:ident<$p:ident>, <$raw:ty> => $into:ty) =>
    {
        impl From<$wrap<$raw>> for $into
        {
            #[inline]
            fn from(v: $wrap<$raw>) -> $into { v.0.into() }
        }
    };

    ($wrap:ident<$p:ident>, try<$raw:ty> <= $from:ty) =>
    {
        impl TryFrom<$from> for $wrap<$raw>
        {
            type Error = <$raw as TryFrom<$from>>::Error;

            #[inline]
            fn try_from(v: $from) -> std::result::Result<Self, Self::Error>
            {
                Ok(Self(v.try_into()?))
            }
        }
    };

    ($wrap:ident<$p:ident>, try<$raw:ty> => $into:ty) =>
    {
        impl TryFrom<$wrap<$raw>> for $into
        {
            type Error = <$into as TryFrom<$raw>>::Error;

            #[inline]
            fn try_from(v: $wrap<$raw>) -> std::result::Result<Self, Self::Error>
            {
                Ok(v.0.try_into()?)
            }
        }
    };

    ($wrap:ident(char): *) =>
    {
        impl_newtype_from!($wrap(char));
        impl_newtype_from!($wrap(char) <= [u8 u32?]);
        impl_newtype_from!($wrap(char) => [u32 u64 u128 String u8? u16?]);
    };

    ($wrap:ident(bool): *) =>
    {
        impl_newtype_from!($wrap(bool));
        impl_newtype_from!($wrap(bool) <= []);
        impl_newtype_from!($wrap(bool) => [i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize]);
    };

    ($wrap:ident(f32): *) =>
    {
        impl_newtype_from!($wrap(f32));
        impl_newtype_from!($wrap(f32) <= [i8 u8 i16 u16]);
        impl_newtype_from!($wrap(f32) => [f64]);
    };

    ($wrap:ident(f64): *) =>
    {
        impl_newtype_from!($wrap(f64));
        impl_newtype_from!($wrap(f64) <= [i8 u8 i16 u16 i32 u32]);
        impl_newtype_from!($wrap(f64) => []);
    };

    ($wrap:ident(i8): *) =>
    {
        impl_newtype_from!($wrap(i8));
        impl_newtype_from!($wrap(i8) <= [u8? i16? u16? i32? u32? i64? u64? i128? u128? isize? usize?]);
        impl_newtype_from!($wrap(i8) => [f32 f64 i16 i32 i64 i128 isize u8? u16? u32? u64? u128? usize?]);
    };

    ($wrap:ident(u8): *) =>
    {
        impl_newtype_from!($wrap(u8));
        impl_newtype_from!($wrap(u8) <= [char? i8? i16? u16? i32? u32? i64? u64? i128? u128? isize? usize?]);
        impl_newtype_from!($wrap(u8) => [bool f32 f64 i16 u16 i32 u32 i64 u64 i128 u128 isize usize i8?]);
    };

    ($wrap:ident(i16): *) =>
    {
        impl_newtype_from!($wrap(i16));
        impl_newtype_from!($wrap(i16) <= [bool i8 u8 u16? i32? u32? i64? u64? i128? u128? isize? usize?]);
        impl_newtype_from!($wrap(i16) => [f32 f64 i32 i64 i128 i8? u8? u16? u32? u64? u128?]);
    };

    ($wrap:ident(u16): *) =>
    {
        impl_newtype_from!($wrap(u16));
        impl_newtype_from!($wrap <= [bool u8 char? i8? i16? i32? u32? i64? u64? i128? u128? isize? usize?]);
        impl_newtype_from!($wrap => [f32 f64 i64 u64 i128 u128 i8? u8? i16? u16? i32?]);
    };

    ($wrap:ident(i32): *) =>
    {
        impl_newtype_from!($wrap(i32));
        impl_newtype_from!($wrap(i32) <= [bool i8 u8 i16 u16 u32? i64? u64? i128? u128? isize? usize?]);
        impl_newtype_from!($wrap(i32) => [f64 i64 i128 i8? u8? i16? u16? u32? u64? u128?]);
    };

    ($wrap:ident(u32): *) =>
    {
        impl_newtype_from!($wrap(u32));
        impl_newtype_from!($wrap(u32) <= [bool u8 u16 i8? i16? i32? i64? u64? i128? u128? isize? usize?]);
        impl_newtype_from!($wrap(u32) => [f64 i64 u64 i128 u128 i8? u8? i16? u16? i32?]);
    };

    ($wrap:ident(i64): *) =>
    {
        impl_newtype_from!($wrap(i64));
        impl_newtype_from!($wrap(i64) <= [bool i8 u8 i16 u16 i32 u32 u64? i128? u128? isize? usize?]);
        impl_newtype_from!($wrap(i64) => [i128 i8? u8? i16? u16? i32? u32? u64? u128?]);
    };

    ($wrap:ident(u64): *) =>
    {
        impl_newtype_from!($wrap(u64));
        impl_newtype_from!($wrap(u64) <= [bool u8 u16 u32 i8? i16? i32? i64? i128? u128? isize? usize?]);
        impl_newtype_from!($wrap(u64) => [i128 u128 i8? u8? i16? u16? i32? u32? i64?]);
    };

    ($wrap:ident(i128): *) =>
    {
        impl_newtype_from!($wrap(i128));
        impl_newtype_from!($wrap(i128) <= [bool i8 u8 i16 u16 i32 u32 i64 u64 u128? isize? usize?]);
        impl_newtype_from!($wrap(i128) => [i8? u8? i16? u16? i32? u32? i64? u64? u128?]);
    };

    ($wrap:ident(u128): *) =>
    {
        impl_newtype_from!($wrap(u128));
        impl_newtype_from!($wrap(u128) <= [bool u8 u16 u32 u64 i8? i16? i32? i64? i128? isize? usize?]);
        impl_newtype_from!($wrap(u128) => [i8? u8? i16? u16? i32? u32? i64? u64? i128?]);
    };

    ($wrap:ident(isize): *) =>
    {
        impl_newtype_from!($wrap(isize));
        impl_newtype_from!($wrap(isize) <= [bool i8 u8 i16 u16? i32? u32? i64? u64? i128? u128? usize?]);
        impl_newtype_from!($wrap(isize) => [i8? u8? i16? u16? i32? u32? i64? u64? i128? u128? usize?]);
    };

    ($wrap:ident(usize): *) =>
    {
        impl_newtype_from!($wrap(usize));
        impl_newtype_from!($wrap(usize) <= [bool u8 u16] [i8? i16? i32? i64? i128? u128? isize?]);
        impl_newtype_from!($wrap(usize) => [i8? u8? i16? u16? i32? u32? i64? u64? i128? u128? isize?]);
    };

    ($wrap:ident($ty:ty)) =>
    {
        impl From<$ty> for $wrap
        {
            #[inline]
            fn from(v: $ty) -> Self { Self(v) }
        }

        impl From<$wrap> for $ty
        {
            #[inline]
            fn from(v: $wrap) -> Self { v.0 }
        }
    };

    ($wrap:ident($ty:ty) <= [$from:ident? $($tail:tt)*]) =>
    {
        impl TryFrom<$from> for $wrap
        {
            type Error = <$ty as TryFrom<$from>>::Error;

            #[inline]
            fn try_from(v: $from) -> std::result::Result<Self, Self::Error>
            {
                Ok(Self(v.try_into()?))
            }
        }

        impl_newtype_from!($wrap($ty) <= [$($tail)*]);
    };

    ($wrap:ident($ty:ty) <= [$from:ident $($tail:tt)*]) =>
    {
        impl From<$from> for $wrap
        {
            #[inline]
            fn from(v: $from) -> Self { Self(v.into()) }
        }

        impl_newtype_from!($wrap($ty) <= [$($tail)*]);
    };

    ($wrap:ident($ty:ty) <= []) =>
    {
    };

    ($wrap:ident($ty:ty) => [$into:ident? $($tail:tt)*]) =>
    {
        impl TryFrom<$wrap> for $into
        {
            type Error = <$into as TryFrom<$ty>>::Error;

            #[inline]
            fn try_from(v: $wrap) -> std::result::Result<Self, Self::Error>
            {
                Ok(v.0.try_into()?)
            }
        }
        impl_newtype_from!($wrap($ty) => [$($tail)*]);
    };

    ($wrap:ident($ty:ty) => [$into:ident $($tail:tt)*]) =>
    {
        impl From<$wrap> for $into
        {
            #[inline]
            fn from(v: $wrap) -> $into { v.0.into() }
        }
        impl_newtype_from!($wrap($ty) => [$($tail)*]);
    };

    ($wrap:ident($ty:ty) => []) =>
    {
    };

    ($wrap:ident($ty:ty): [$($from:tt)*] => [$($into:tt)*]) =>
    {
        impl_newtype_from!($wrap($ty));
        impl_newtype_from!($wrap($ty) <= [$($from)*]);
        impl_newtype_from!($wrap($ty) => [$($into)*]);
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
    impl_newtype_from!(GenericNewType<T>: <i32>, <i64>);

    #[derive(Debug, Copy, Clone)]
    pub struct ConcreteNewType(i32);
    impl_newtype!(ConcreteNewType(i32));
    impl_newtype_from!(ConcreteNewType(i32): *);

    #[rstest]
    #[case(1)]
    fn test_generic_from_raw2nt(#[case] raw: i32)
    {
        let wrap = GenericNewType::<i32>::from(raw);
        assert_eq!(raw, wrap.unwrap());
    }

    #[rstest]
    #[case(1)]
    fn test_generic_from_raw2nt_diff(#[case] input:i16)
    {
        let wrap = GenericNewType::<i32>::from(input);
        assert_eq!(input as i32, wrap.unwrap());
    }

    #[rstest]
    #[case(1)]
    fn test_generic_from_nt2raw(#[case] raw: i32)
    {
        let wrap = GenericNewType::new(raw);
        let from = i32::from(wrap);
        assert_eq!(raw, from);
    }

    #[rstest]
    #[case(1)]
    fn test_generic_from_nt2raw_diff(#[case] raw: i32)
    {
        let wrap = GenericNewType::new(raw);
        let from = i64::from(wrap);
        assert_eq!(raw as i64, from);
    }

    #[rstest]
    #[case(1)]
    fn test_generic_into_raw2nt(#[case] raw: i32)
    {
        let wrap:GenericNewType<i32> = raw.into();
        assert_eq!(raw, wrap.unwrap());
    }

    #[rstest]
    #[case(1)]
    fn test_generic_into_raw2nt_diff(#[case] raw: i16)
    {
        let wrap:GenericNewType<i32> = raw.into();
        assert_eq!(raw as i32, wrap.unwrap());
    }

    #[rstest]
    #[case(1)]
    fn test_generic_into_nt2raw(#[case] raw: i32)
    {
        let wrap = GenericNewType::new(raw);
        let into:i32 = wrap.into();
        assert_eq!(raw, into);
    }

    #[rstest]
    #[case(1)]
    fn test_generic_into_nt2raw_diff(#[case] raw: i32)
    {
        let wrap = GenericNewType::new(raw);
        let into:i64 = wrap.into();
        assert_eq!(raw as i64, into);
    }

    #[rstest]
    #[case(1)]
    fn test_generic_tryfrom_raw2nt(#[case] raw: u32)
    {
        let wrap = GenericNewType::<i32>::try_from(raw);
        assert!(wrap.is_ok());
        assert_eq!(raw as i32, wrap.unwrap().unwrap());
    }

    #[rstest]
    #[case(u32::MAX)]
    fn test_generic_tryfrom_raw2nt_err(#[case] raw: u32)
    {
        let wrap = GenericNewType::<i32>::try_from(raw);
        assert!(wrap.is_err());
    }

    #[rstest]
    #[case(1)]
    fn test_generic_tryinto_raw2nt(#[case] raw: u32)
    {
        let wrap: Result<GenericNewType::<i32>, _> = raw.try_into();
        assert!(wrap.is_ok());
        assert_eq!(raw as i32, wrap.unwrap().unwrap());
    }

    #[rstest]
    #[case(u32::MAX)]
    fn test_generic_tryinto_raw2nt_err(#[case] raw: u32)
    {
        let wrap: Result<GenericNewType::<i32>, _> = raw.try_into();
        assert!(wrap.is_err());
    }

    #[rstest]
    #[case(1)]
    fn test_generic_tryfrom_nt2raw(#[case] raw: i32)
    {
        let wrap = GenericNewType::new(raw);
        let result = u32::try_from(wrap);
        assert!(result.is_ok());
        assert_eq!(raw as u32, result.unwrap());
    }

    #[rstest]
    #[case(-1)]
    fn test_generic_tryfrom_nt2raw_err(#[case] raw: i32)
    {
        let wrap = GenericNewType::new(raw);
        let result = u32::try_from(wrap);
        assert!(result.is_err());
    }

    #[rstest]
    #[case(1)]
    fn test_generic_tryinto_nt2raw(#[case] raw: i32)
    {
        let wrap = GenericNewType::new(raw);
        let result = wrap.try_into();
        assert!(result.is_ok());
        assert_eq!(raw as u32, result.unwrap());
    }

    #[rstest]
    #[case(-1)]
    fn test_generic_tryinto_nt2raw_err(#[case] raw: i32)
    {
        let wrap = GenericNewType::new(raw);
        let result: Result<u32,_> = wrap.try_into();
        assert!(result.is_err());
    }

    #[rstest]
    #[case(1)]
    fn test_concrete_from_raw2nt(#[case] raw: i32)
    {
        let wrap = ConcreteNewType::from(raw);
        assert_eq!(raw, wrap.unwrap());
    }

    #[rstest]
    #[case(1)]
    fn test_concrete_from_raw2nt_diff(#[case] raw: i16)
    {
        let wrap = ConcreteNewType::from(raw);
        assert_eq!(raw as i32, wrap.unwrap());
    }

    #[rstest]
    #[case(1)]
    fn test_concrete_from_nt2raw(#[case] raw: i32)
    {
        let wrap = ConcreteNewType::new(raw);
        let from = i32::from(wrap);
        assert_eq!(raw, from);
    }

    #[rstest]
    #[case(1)]
    fn test_concrete_from_nt2raw_diff(#[case] raw: i32)
    {
        let wrap = ConcreteNewType::new(raw);
        let from = i64::from(wrap);
        assert_eq!(raw as i64, from);
    }

    #[rstest]
    #[case(1)]
    fn test_concrete_into_raw2nt(#[case] raw: i32)
    {
        let wrap:ConcreteNewType = raw.into();
        assert_eq!(raw, wrap.unwrap());
    }

    #[rstest]
    #[case(1)]
    fn test_concrete_into_raw2nt_diff(#[case] raw: i16)
    {
        let wrap:ConcreteNewType = raw.into();
        assert_eq!(raw as i32, wrap.unwrap());
    }

    #[rstest]
    #[case(1)]
    fn test_concrete_into_nt2raw(#[case] raw: i32)
    {
        let wrap = ConcreteNewType::from(raw);
        let into:i32 = wrap.into();
        assert_eq!(raw, into);
    }

    #[rstest]
    #[case(1)]
    fn test_concrete_into_nt2raw_diff(#[case] raw: i32)
    {
        let wrap = ConcreteNewType::from(raw);
        let into:i64 = wrap.into();
        assert_eq!(raw as i64, into);
    }
}
