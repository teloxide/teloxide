pub trait Guard<Upd: ?Sized> {
    fn check(&self, update: &Upd) -> bool;
}

impl<F, Upd> Guard<Upd> for F
where
    Upd: ?Sized,
    F: Fn(&Upd) -> bool,
{
    fn check(&self, update: &Upd) -> bool {
        self(update)
    }
}

impl<Upd> Guard<Upd> for Box<dyn Guard<Upd>> {
    fn check(&self, update: &Upd) -> bool {
        (**self).check(update)
    }
}

pub struct Guards<Upd> {
    guards: Vec<Box<dyn Guard<Upd>>>,
}

impl<Upd> Guards<Upd> {
    pub fn new() -> Self {
        Guards { guards: Vec::new() }
    }

    pub fn add<T>(mut self, data: T) -> Self
    where
        T: Guard<Upd> + 'static,
    {
        self.guards.push(Box::new(data));
        self
    }

    pub fn add_guard<T>(&mut self, data: T)
    where
        T: Guard<Upd> + 'static,
    {
        self.add_boxed_guard(Box::new(data));
    }

    pub fn add_boxed_guard(&mut self, data: Box<dyn Guard<Upd>>) {
        self.guards.push(data);
    }

    pub fn check(&self, update: &Upd) -> bool {
        Guard::check(self, update)
    }

    pub fn with(mut self, other: Self) -> Self {
        self.guards.extend(other.guards.into_iter());
        self
    }

    pub fn is_empty(&self) -> bool {
        self.guards.is_empty()
    }
}

impl<Upd> Guard<Upd> for Guards<Upd> {
    fn check(&self, update: &Upd) -> bool {
        self.guards.iter().all(|guard| guard.check(update))
    }
}

pub struct OrGuard<Left, Right>(Left, Right);

impl<Left, Right> OrGuard<Left, Right> {
    pub fn new(left: Left, right: Right) -> Self {
        OrGuard(left, right)
    }
}

impl<Left, Right, Upd> Guard<Upd> for OrGuard<Left, Right>
where
    Left: Guard<Upd>,
    Right: Guard<Upd>,
{
    fn check(&self, update: &Upd) -> bool {
        self.0.check(update) || self.1.check(update)
    }
}
