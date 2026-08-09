#[doc(hidden)]
#[macro_export]
macro_rules! impl_op {
    ($ops_trait:ident, $trait_meth:ident, $value_meth:ident, $operator:tt $(,)?) => {
        impl<T> $ops_trait<T> for Value where T: Into<Value> {
            type Output = Value;
            fn $trait_meth(self, rhs: T) -> Self::Output {
                let rhs_value = Value(*rhs.into());
                Value(self.$value_meth() $operator *rhs_value)
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_getter_setters_and_with_method_builder_style {
    ($method_name:ident, $value_type:ident,Optional $(,)?) => {
        pub fn $method_name(&self) -> Option<$method_name> {
            self.$value_meth
        }
        pub fn set_$method_nme(&mut self, value: $method_name) {
            self.$value_meth = Some(value);
        }
        pub fn with_$method_name(mut self, value: $method_name) -> RenderableColor {
            self.set_$method_name(value);
            self
        }
    };
    ($method_name:ident, $value_type:ident $(,)?) => {
        pub fn $method_name(&self) -> $method_name {
            self.$value_meth
        }
        pub fn set_$method_nme(&mut self, value: $method_name) {
            self.$value_meth = value;
        }
        pub fn with_$method_name(mut self, value: $method_name) -> RenderableColor {
            self.set_$method_name(value);
            self
        }
    };
}
