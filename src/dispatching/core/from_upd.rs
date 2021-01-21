pub trait FromUpd<Upd> {
    fn from_upd(upd: &Upd) -> Self;
}

impl<T> FromUpd<T> for T
where
    T: Clone,
{
    fn from_upd(upd: &T) -> Self {
        upd.clone()
    }
}
