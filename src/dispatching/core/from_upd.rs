pub trait FromUpd<Upd>: Sized {
    fn from_upd(upd: &Upd) -> Option<Self>;
}

impl<T> FromUpd<T> for T
where
    T: Clone,
{
    fn from_upd(upd: &T) -> Option<Self> {
        Some(upd.clone())
    }
}
