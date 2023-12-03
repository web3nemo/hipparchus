/// Trait for wrapping a type into a newtype.
pub trait NewType
{
    /// The raw type wrapped by the newtype
    type Raw;

    /// Create a new instance of the wrapped type
    fn new(v: Self::Raw) -> Self;

    /// Unwrap the wrapped type
    fn unwrap(&self) -> Self::Raw;
}

/// Create wrapper type and implement Wrapped trait
#[macro_export]
macro_rules! impl_newtype
{
    ($wrap:ident) =>
    {
        impl<T> $crate::NewType for $wrap<T> where T: Copy
        {
            type Raw = T;

            #[inline]
            fn new(v: Self::Raw) -> Self { Self(v) }
            
            #[inline]
            fn unwrap(&self) -> Self::Raw { self.0 }
        }
    };
    ($wrap:ident<$ty:ty>) =>
    {
        impl $crate::NewType for $wrap
        {
            type Raw = $ty;

            #[inline]
            fn new(v: Self::Raw) -> Self { Self(v) }
            
            #[inline]
            fn unwrap(&self) -> Self::Raw { self.0 }
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use rstest::*;

    #[derive(Debug, Copy, Clone)]
    pub struct GenericNewType<T>(T);
    impl_newtype!(GenericNewType);

    pub struct ConcreteNewType(i32);
    impl_newtype!(ConcreteNewType<i32>);

    #[rstest]
    #[case(1)]
    fn test_newtype_generic(#[case] raw: i64)
    {
        let wrap = GenericNewType::new(raw);
        assert_eq!(raw, wrap.unwrap());
        assert_eq!("i64", std::any::type_name::<<GenericNewType<i64> as NewType>::Raw>());
    }

    #[rstest]
    #[case(1)]
    fn test_newtype_concrete(#[case] raw: i32)
    {
        let wrap = ConcreteNewType::new(raw);
        assert_eq!(raw, wrap.unwrap());
        assert_eq!("i32", std::any::type_name::<<ConcreteNewType as NewType>::Raw>());
    }
}
