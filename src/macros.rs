#[macro_export]
macro_rules! impl_op {
    ($ops_trait:ident, $trait_meth:ident, $value_meth:ident, $operator:tt $(,)?) => {
        // impl $ops_trait for RGBValue {
        //     type Output = RGBValue;
        //     fn $trait_meth(self, rhs: RGBValue) -> Self::Output {
        //         RGBValue(self.$value_meth() $operator rhs.$value_meth())
        //     }
        // }
        // impl $ops_trait<f32> for RGBValue {
        //     type Output = RGBValue;
        //     fn $trait_meth(self, rhs: f32) -> Self::Output {
        //         RGBValue(self.$value_meth() $operator rhs)
        //     }
        // }
        impl<T> $ops_trait<T> for RGBValue where T: Into<RGBValue> {
            type Output = RGBValue;
            fn $trait_meth(self, rhs: T) -> Self::Output {
                let rhs_value = RGBValue(*rhs.into());
                RGBValue(self.$value_meth() $operator *rhs_value)
            }
        }
    };
}
