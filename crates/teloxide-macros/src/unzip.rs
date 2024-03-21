pub(crate) struct Unzip<A, B>(pub A, pub B);

impl<A, B, T, U> FromIterator<(T, U)> for Unzip<A, B>
where
    A: Default + Extend<T>,
    B: Default + Extend<U>,
{
    fn from_iter<I: IntoIterator<Item = (T, U)>>(iter: I) -> Self {
        let (mut a, mut b): (A, B) = Default::default();

        for (t, u) in iter {
            a.extend([t]);
            b.extend([u]);
        }

        Unzip(a, b)
    }
}
