use std::marker::PhantomData;

use crate::PipelineFn;

pub struct Pipeline<T, F>(pub(crate) F, PhantomData<T>);

impl<T1, T2, F1> From<F1> for Pipeline<(T1, T2), F1>
where
    F1: PipelineFn<T1, T2>,
{
    fn from(f: F1) -> Self {
        Self(f, PhantomData)
    }
}

impl<T1, T2, T3, F1, F2> From<(F1, F2)> for Pipeline<(T1, T2, T3), (F1, F2)>
where
    F1: PipelineFn<T1, T2>,
    F2: PipelineFn<T2, T3>,
{
    fn from((f1, f2): (F1, F2)) -> Self {
        Self((f1, f2), PhantomData)
    }
}

impl<T1, T2, T3, T4, F1, F2, F3> From<(F1, F2, F3)> for Pipeline<(T1, T2, T3, T4), (F1, F2, F3)>
where
    F1: PipelineFn<T1, T2>,
    F2: PipelineFn<T2, T3>,
    F3: PipelineFn<T3, T4>,
{
    fn from((f1, f2, f3): (F1, F2, F3)) -> Self {
        Self((f1, f2, f3), PhantomData)
    }
}

impl<T1, T2, T3, T4, T5, F1, F2, F3, F4> From<(F1, F2, F3, F4)>
    for Pipeline<(T1, T2, T3, T4, T5), (F1, F2, F3, F4)>
where
    F1: PipelineFn<T1, T2>,
    F2: PipelineFn<T2, T3>,
    F3: PipelineFn<T3, T4>,
    F4: PipelineFn<T4, T5>,
{
    fn from((f1, f2, f3, f4): (F1, F2, F3, F4)) -> Self {
        Self((f1, f2, f3, f4), PhantomData)
    }
}
