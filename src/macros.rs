#[macro_export]
macro_rules! impl_op {
    ($ops_trait:ident, $trait_meth:ident, $value_meth:ident, $operator:tt $(,)?) => {
        // impl $ops_trait for Value {
        //     type Output = Value;
        //     fn $trait_meth(self, rhs: Value) -> Self::Output {
        //         Value(self.$value_meth() $operator rhs.$value_meth())
        //     }
        // }
        // impl $ops_trait<f32> for Value {
        //     type Output = Value;
        //     fn $trait_meth(self, rhs: f32) -> Self::Output {
        //         Value(self.$value_meth() $operator rhs)
        //     }
        // }
        impl<T> $ops_trait<T> for Value where T: Into<Value> {
            type Output = Value;
            fn $trait_meth(self, rhs: T) -> Self::Output {
                let rhs_value = Value(*rhs.into());
                Value(self.$value_meth() $operator *rhs_value)
            }
        }
    };
}
