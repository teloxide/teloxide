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

#[macro_use]
macro_rules! req_future {
    (
        $v2:vis def: | $( $arg:ident: $ArgTy:ty ),* $(,)? | $body:block

        $(#[$($meta:tt)*])*
        $v:vis $i:ident<$T:ident> ($inner:ident) -> $Out:ty
        $(where $($wh:tt)*)?
    ) => {
        #[pin_project::pin_project]
        $v struct $i<$T>
        $(where $($wh)*)?
        {
            #[pin]
            inner: $inner::$i<$T>
        }

        impl<$T> $i<$T>
        $(where $($wh)*)?
        {
            $v2 fn new($( $arg: $ArgTy ),*) -> Self {
                Self { inner: $inner::def($( $arg ),*) }
            }
        }

        // HACK(waffle): workaround for https://github.com/rust-lang/rust/issues/55997
        mod $inner {
            #![allow(type_alias_bounds)]

            // Mostly to bring `use`s
            #[allow(unused_imports)]
            use super::{*, $i as _};

            #[cfg(feature = "nightly")]
            pub(crate) type $i<$T>
            $(where $($wh)*)? = impl ::core::future::Future<Output = $Out>;

            #[cfg(feature = "nightly")]
            pub(crate) fn def<$T>($( $arg: $ArgTy ),*) -> $i<$T>
            $(where $($wh)*)?
            {
                $body
            }

            #[cfg(not(feature = "nightly"))]
            pub(crate) type $i<$T>
            $(where $($wh)*)?  = ::core::pin::Pin<Box<dyn ::core::future::Future<Output = $Out> + ::core::marker::Send + 'static>>;

            #[cfg(not(feature = "nightly"))]
            pub(crate) fn def<$T>($( $arg: $ArgTy ),*) -> $i<$T>
            $(where $($wh)*)?
            {
                Box::pin($body)
            }
        }

        impl<$T> ::core::future::Future for $i<$T>
        $(where $($wh)*)?
        {
            type Output = $Out;

            fn poll(self: ::core::pin::Pin<&mut Self>, cx: &mut ::core::task::Context<'_>) -> ::core::task::Poll<Self::Output> {
                let this = self.project();
                this.inner.poll(cx)
            }
        }

    };
}

/// Declares an item with a doc attribute computed by some macro expression.
/// This allows documentation to be dynamically generated based on input.
/// Necessary to work around https://github.com/rust-lang/rust/issues/52607.
#[macro_use]
macro_rules! calculated_doc {
    (
        $(
            #[doc = $doc:expr]
            $thing:item
        )*
    ) => (
        $(
            #[doc = $doc]
            $thing
        )*
    );
}

/// Declare payload type, implement `Payload` trait amd ::new method for it, 
/// declare setters trait and implement it for all type which have payload.
#[macro_use]
macro_rules! impl_payload {
    (
        $(
            #[ $($method_meta:tt)* ]
        )*
        $vi:vis $Method:ident ($Setters:ident) => $Ret:ty {
            $(
                required {
                    $(
                        $(
                            #[ $($field_meta:tt)* ]
                        )*
                        $v:vis $fields:ident : $FTy:ty $([$into:ident])?
                        ,
                    )*
                }
            )?

            $(
                optional {
                    $(
                        $(
                            #[ $($opt_field_meta:tt)* ]
                        )*
                        $opt_v:vis $opt_fields:ident : $OptFTy:ty $([$opt_into:ident])?
                    ),*
                    $(,)?
                }
            )?
        }
    ) => {
        $(
            #[ $($method_meta)* ]
        )*
        $vi struct $Method {
            $(
                $(
                    $(
                        #[ $($field_meta)* ]
                    )*
                    $v $fields : $FTy,
                )*
            )?
            $(
                $(
                    $(
                        #[ $($opt_field_meta)* ]
                    )*
                    $opt_v $opt_fields : core::option::Option<$OptFTy>,
                )*
            )?
        }

        impl $Method {
            $vi fn new($($($fields : impl_payload!(@into? $FTy $([$into])?)),*)?) -> Self {
                Self {
                    $(
                        $(
                            $fields: $fields $(.$into())?,
                        )*
                    )?
                    $(
                        $(
                            $opt_fields: None,
                        )*
                    )?
                }
            }
        }

        impl $crate::requests::Payload for $Method {
            type Output = $Ret;

            const NAME: &'static str = stringify!($Method);
        }

        calculated_doc! {
            #[doc = concat!(
                "Setters for fields of [`",
                stringify!($Method),
                "`]"
            )]
            $vi trait $Setters: $crate::requests::HasPayload<Payload = $Method> + ::core::marker::Sized {
                $(
                    $(
                        impl_payload! { @setter $Method $fields : $FTy $([$into])? }
                    )*
                )?
                $(
                    $(
                        impl_payload! { @setter_opt $Method $opt_fields : $OptFTy $([$opt_into])? }
                    )*
                )?
            }
        }

        impl<P> $Setters for P where P: crate::requests::HasPayload<Payload = $Method> {}
    };
    (@setter_opt $Method:ident $field:ident : $FTy:ty [into]) => {
        calculated_doc! {
            #[doc = concat!(
                "Setter for [`",
                stringify!($field),
                "`](",
                stringify!($Method),
                "::",
                stringify!($field),
                ") field."
            )]
            fn $field<T>(mut self, value: T) -> Self
            where
                T: Into<$FTy>,
            {
                self.payload_mut().$field = Some(value.into());
                self
            }
        }
    };
    (@setter_opt $Method:ident $field:ident : $FTy:ty) => {
        calculated_doc! {
            #[doc = concat!(
                "Setter for [`",
                stringify!($field),
                "`](",
                stringify!($Method),
                "::",
                stringify!($field),
                ") field."
            )]
            fn $field(mut self, value: $FTy) -> Self {
                self.payload_mut().$field = Some(value);
                self
            }
        }
    };
    (@setter $Method:ident $field:ident : $FTy:ty [into]) => {
        calculated_doc! {
            #[doc = concat!(
                "Setter for [`",
                stringify!($field),
                "`](",
                stringify!($Method),
                "::",
                stringify!($field),
                ") field."
            )]
            fn $field<T>(mut self, value: T) -> Self
            where
                T: Into<$FTy>,
            {
                self.payload_mut().$field = value.into();
                self
            }
        }
    };
    (@setter $Method:ident $field:ident : $FTy:ty) => {
        calculated_doc! {
            #[doc = concat!(
                "Setter for [`",
                stringify!($field),
                "`](",
                stringify!($Method),
                "::",
                stringify!($field),
                ") field."
            )]
            fn $field(mut self, value: $FTy) -> Self {
                self.payload_mut().$field = value;
                self
            }
        }
    };
    (@into? $T:ty [into]) => {
        impl ::core::convert::Into<$T>
    };
    (@into? $T:ty) => {
        $T
    };
}

#[macro_use]
macro_rules! serde_or_unknown {
    (
        #[unknown_mod = $mod_:ident]
        #[unknown_kind = $Kind:ident]
        #[unknown_path = $path:literal]
        $(
            #[ $($meta:tt)* ]
        )*
        $v:vis enum $Name:ident {
            #[unknown]
            $(
                #[ $($unknown_meta:tt)* ]
            )*
            $Unknown:ident(String)

            $(
                ,
                $(
                    #[ $($var_meta:tt)* ]
                )*
                $Var:ident
            )*
            $(,)?
        }
    ) => {
        mod $mod_ {
            #[allow(unused_imports)]
            use super::{*, $Name as _};

            $(
                #[ $($meta)* ]
            )*
            $v enum $Name {
                $(
                    $(
                        #[ $($var_meta)* ]
                    )*
                    $Var,
                )*
            }

            #[derive(::serde::Deserialize)]
            #[serde(untagged)]
            $v enum $Kind {
                Known($Name),
                Unknown(String),
            }

            impl ::core::convert::From<$Kind> for super::$Name {
                fn from(kind: $Kind) -> Self {
                    match kind {
                        $Kind::Unknown(string) => Self::Unknown(string),
                        $Kind::Known(known) => match known {
                            $(
                                $Name::$Var => Self::$Var,
                            )*
                        }
                    }
                }
            }
        }

        $(
            #[ $($meta)* ]
        )*
        #[serde(from = $path)]
        $v enum $Name {
            $(
                $(
                    #[ $($var_meta)* ]
                )*
                $Var,
            )*
            $(
                #[ $($unknown_meta)* ]
            )*
            $Unknown(String),
        }
    }
}
