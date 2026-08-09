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

pub(crate) struct Unzip5<A, B, C, D, E>(pub A, pub B, pub C, pub D, pub E);

impl<A, B, C, D, E, T, U, V, W, X> FromIterator<(T, U, V, W, X)> for Unzip5<A, B, C, D, E>
where
    A: Default + Extend<T>,
    B: Default + Extend<U>,
    C: Default + Extend<V>,
    D: Default + Extend<W>,
    E: Default + Extend<X>,
{
    fn from_iter<I: IntoIterator<Item = (T, U, V, W, X)>>(iter: I) -> Self {
        let (mut a, mut b, mut c, mut d, mut e): (A, B, C, D, E) = Default::default();

        for (t, u, v, w, x) in iter {
            a.extend([t]);
            b.extend([u]);
            c.extend([v]);
            d.extend([w]);
            e.extend([x]);
        }

        Unzip5(a, b, c, d, e)
    }
}
