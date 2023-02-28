pub trait With2<'a, T1, T2, T3> {
    fn with(self, x1: T1) -> Box<dyn FnOnce(T2) -> T3 + 'a>;
}

impl<'a, T1, T2, T3, F> With2<'a, T1, T2, T3> for F
where
    F: FnOnce(T1, T2) -> T3 + 'a,
    T1: 'a,
{
    fn with(self, x1: T1) -> Box<dyn FnOnce(T2) -> T3 + 'a> {
        Box::new(move |x2| self(x1, x2))
    }
}

pub trait With3<'a, T1, T2, T3, T4> {
    fn with(self, x1: T1, x2: T2) -> Box<dyn FnOnce(T3) -> T4 + 'a>;
}

impl<'a, T1, T2, T3, T4, F> With3<'a, T1, T2, T3, T4> for F
where
    F: FnOnce(T1, T2, T3) -> T4 + 'a,
    T1: 'a,
    T2: 'a,
{
    fn with(self, x1: T1, x2: T2) -> Box<dyn FnOnce(T3) -> T4 + 'a> {
        Box::new(move |x3| self(x1, x2, x3))
    }
}

pub trait With4<'a, T1, T2, T3, T4, T5> {
    fn with(self, x1: T1, x2: T2, x3: T3) -> Box<dyn FnOnce(T4) -> T5 + 'a>;
}

impl<'a, T1, T2, T3, T4, T5, F> With4<'a, T1, T2, T3, T4, T5> for F
where
    F: FnOnce(T1, T2, T3, T4) -> T5 + 'a,
    T1: 'a,
    T2: 'a,
    T3: 'a,
{
    fn with(self, x1: T1, x2: T2, x3: T3) -> Box<dyn FnOnce(T4) -> T5 + 'a> {
        Box::new(move |x4| self(x1, x2, x3, x4))
    }
}

pub trait With5<'a, T1, T2, T3, T4, T5, T6> {
    fn with(self, x1: T1, x2: T2, x3: T3, x4: T4) -> Box<dyn FnOnce(T5) -> T6 + 'a>;
}

impl<'a, T1, T2, T3, T4, T5, T6, F> With5<'a, T1, T2, T3, T4, T5, T6> for F
where
    F: FnOnce(T1, T2, T3, T4, T5) -> T6 + 'a,
    T1: 'a,
    T2: 'a,
    T3: 'a,
    T4: 'a,
{
    fn with(self, x1: T1, x2: T2, x3: T3, x4: T4) -> Box<dyn FnOnce(T5) -> T6 + 'a> {
        Box::new(move |x5| self(x1, x2, x3, x4, x5))
    }
}

pub trait With6<'a, T1, T2, T3, T4, T5, T6, T7> {
    fn with(self, x1: T1, x2: T2, x3: T3, x4: T4, x5: T5) -> Box<dyn FnOnce(T6) -> T7 + 'a>;
}

impl<'a, T1, T2, T3, T4, T5, T6, T7, F> With6<'a, T1, T2, T3, T4, T5, T6, T7> for F
where
    F: FnOnce(T1, T2, T3, T4, T5, T6) -> T7 + 'a,
    T1: 'a,
    T2: 'a,
    T3: 'a,
    T4: 'a,
    T5: 'a,
{
    fn with(self, x1: T1, x2: T2, x3: T3, x4: T4, x5: T5) -> Box<dyn FnOnce(T6) -> T7 + 'a> {
        Box::new(move |x6| self(x1, x2, x3, x4, x5, x6))
    }
}

pub trait With7<'a, T1, T2, T3, T4, T5, T6, T7, T8> {
    fn with(self, x1: T1, x2: T2, x3: T3, x4: T4, x5: T5, x6: T6)
        -> Box<dyn FnOnce(T7) -> T8 + 'a>;
}

impl<'a, T1, T2, T3, T4, T5, T6, T7, T8, F> With7<'a, T1, T2, T3, T4, T5, T6, T7, T8> for F
where
    F: FnOnce(T1, T2, T3, T4, T5, T6, T7) -> T8 + 'a,
    T1: 'a,
    T2: 'a,
    T3: 'a,
    T4: 'a,
    T5: 'a,
    T6: 'a,
{
    fn with(
        self,
        x1: T1,
        x2: T2,
        x3: T3,
        x4: T4,
        x5: T5,
        x6: T6,
    ) -> Box<dyn FnOnce(T7) -> T8 + 'a> {
        Box::new(move |x7| self(x1, x2, x3, x4, x5, x6, x7))
    }
}

pub trait With8<'a, T1, T2, T3, T4, T5, T6, T7, T8, T9> {
    #[allow(clippy::too_many_arguments)]
    fn with(
        self,
        x1: T1,
        x2: T2,
        x3: T3,
        x4: T4,
        x5: T5,
        x6: T6,
        x7: T7,
    ) -> Box<dyn FnOnce(T8) -> T9 + 'a>;
}

impl<'a, T1, T2, T3, T4, T5, T6, T7, T8, T9, F> With8<'a, T1, T2, T3, T4, T5, T6, T7, T8, T9> for F
where
    F: FnOnce(T1, T2, T3, T4, T5, T6, T7, T8) -> T9 + 'a,
    T1: 'a,
    T2: 'a,
    T3: 'a,
    T4: 'a,
    T5: 'a,
    T6: 'a,
    T7: 'a,
{
    fn with(
        self,
        x1: T1,
        x2: T2,
        x3: T3,
        x4: T4,
        x5: T5,
        x6: T6,
        x7: T7,
    ) -> Box<dyn FnOnce(T8) -> T9 + 'a> {
        Box::new(move |x8| self(x1, x2, x3, x4, x5, x6, x7, x8))
    }
}
