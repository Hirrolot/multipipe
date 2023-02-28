mod pipeline;
mod pipeline_call;
mod with;

pub use pipeline::*;
pub use pipeline_call::*;
pub use with::*;

pub trait Pipe: Sized {
    fn pipe<Out, Types, F>(self, f: impl Into<Pipeline<Types, F>>) -> Out
    where
        Pipeline<Types, F>: PipelineCall<Self, Out>;
}

impl<T> Pipe for T
where
    T: Sized,
{
    fn pipe<Out, Types, F>(self, f: impl Into<Pipeline<Types, F>>) -> Out
    where
        Pipeline<Types, F>: PipelineCall<Self, Out>,
    {
        f.into().call(self)
    }
}

pub trait PipelineFn<In, Out> {
    fn call(self, x: In) -> Out;
}

impl<In, Out, F> PipelineFn<In, Out> for F
where
    F: FnOnce(In) -> Out,
{
    fn call(self, x: In) -> Out {
        self(x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Eq, PartialEq, Debug)]
    struct A;
    #[derive(Eq, PartialEq, Debug)]
    struct B;
    #[derive(Eq, PartialEq, Debug)]
    struct C;
    #[derive(Eq, PartialEq, Debug)]
    struct D;
    #[derive(Eq, PartialEq, Debug)]
    struct E;

    #[test]
    fn pipe() {
        fn a_to_b(_: A) -> B {
            B
        }
        fn b_to_c(_: B) -> C {
            C
        }
        fn c_to_d(_: C) -> D {
            D
        }
        fn d_to_e(_: D) -> E {
            E
        }

        let x: B = A.pipe(a_to_b);
        assert_eq!(x, B);

        let x: C = A.pipe((a_to_b, b_to_c));
        assert_eq!(x, C);

        let x: D = A.pipe((a_to_b, b_to_c, c_to_d));
        assert_eq!(x, D);

        let x: E = A.pipe((a_to_b, b_to_c, c_to_d, d_to_e));
        assert_eq!(x, E);
    }

    #[test]
    fn pipe_with() {
        fn ab_to_c(_: A, _: B) -> C {
            C
        }
        fn c_to_d(_: C) -> D {
            D
        }

        let x: C = B.pipe(ab_to_c.with(A));
        assert_eq!(x, C);

        let x: D = B.pipe((ab_to_c.with(A), c_to_d));
        assert_eq!(x, D);
    }

    #[test]
    fn with() {
        fn ab_to_c(_: A, _: B) -> C {
            C
        }
        fn abc_to_d(_: A, _: B, _: C) -> D {
            D
        }
        fn abcd_to_e(_: A, _: B, _: C, _: D) -> E {
            E
        }

        let b_to_c = ab_to_c.with(A);
        assert_eq!(b_to_c(B), C);

        let c_to_d = abc_to_d.with(A, B);
        assert_eq!(c_to_d(C), D);

        let d_to_e = abcd_to_e.with(A, B, C);
        assert_eq!(d_to_e(D), E);
    }
}
