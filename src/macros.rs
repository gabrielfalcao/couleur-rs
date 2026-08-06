#[doc(hidden)]
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

#[doc(hidden)]
#[macro_export]
macro_rules! impl_struct_representative_of_color {
    ($struct_name:ident, $color_attribute_name:ident $(,)?) => {
        use crate::Color;

        impl std::ops::Deref for $struct_name {
            type Target = Color;

            fn deref(&self) -> &Self::Target {
                &self.$color_attribute_name
            }
        }
        impl std::fmt::Display for $struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{color}", color = &self.$color_attribute_name)
            }
        }

        impl From<Color> for $struct_name {
            fn from($color_attribute_name: Color) -> $struct_name {
                $struct_name { $color_attribute_name }
            }
        }
        impl Into<Color> for $struct_name {
            fn into(self) -> Color {
                self.$color_attribute_name
            }
        }
    };
}
