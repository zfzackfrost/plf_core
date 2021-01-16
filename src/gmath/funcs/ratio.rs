use num_traits::Num;

pub trait RatioMethods {
    fn remap<T: Num + Sized + Clone>(
        value: T,
        lo_in: T,
        hi_in: T,
        lo_out: T,
        hi_out: T,
    ) -> T
    {
        lo_out.clone() + (value - lo_in.clone()) * (hi_out.clone() - lo_out) / (hi_in - lo_in)
    }
}

impl RatioMethods for f32 {}

impl RatioMethods for f64 {}

impl RatioMethods for i16 {}

impl RatioMethods for i32 {}

impl RatioMethods for i64 {}

