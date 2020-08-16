#[macro_use]
macro_rules! forward_to_unsuported_ty {
    (
        supported: $supported:expr;
        simple { $( $method:ident $arg:ty )* }
        unit { $( $method1:ident $ty:expr )* }
        compound {
            $( $method2:ident $( <$T:ident: ?Sized + Serialize> )? ( $( $args:tt )* ) -> $ret:ty => $message:expr )*
        }
    ) => {
        $(
            fn $method(self, _: $arg) -> Result<Self::Ok, Self::Error> {
                Err(Self::Error::UnsupportedType {
                    ty: stringify!($arg),
                    supported: $supported,
                })
            }
        )+

        $(
            fn $method1(self) -> Result<Self::Ok, Self::Error> {
                Err(Self::Error::UnsupportedType {
                    ty: $ty,
                    supported: $supported,
                })
            }
        )+

        $(
            fn $method2 $( <$T: ?Sized + Serialize> )? (self, $( $args )*) -> Result<$ret, Self::Error> {
                Err(Self::Error::UnsupportedType {
                    ty: $message,
                    supported: $supported,
                })
            }
        )+
    };
}
