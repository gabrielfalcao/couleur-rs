use std::fmt::{Debug, Display};

use clap::{ValueEnum, builder::PossibleValue};
use heck::{ToKebabCase, ToLowerCamelCase, ToPascalCase, ToSnakeCase, ToTrainCase};
use serde::{Deserialize, Serialize};

#[doc(hidden)] pub mod container;
#[doc(inline)] pub use container::PrefixContainer;

use crate::{AnsiRenderable, ToAnsiEscSuffix};

#[doc(hidden)] pub mod prefix;
#[doc(inline)] pub use prefix::Prefix;
