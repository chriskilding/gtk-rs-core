// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits and essential types intended for blanket imports.

pub use crate::{
    closure::{IntoClosureReturnValue, TryFromClosureReturnValue},
    collections::{ptr_slice::IntoPtrSlice, strv::IntoStrV},
    error::ErrorDomain,
    gstring::{IntoGStr, IntoOptionalGStr},
    object::{Cast, CastNone, IsA, ObjectExt, ObjectType},
    param_spec::{HasParamSpec, ParamSpecBuilderExt, ParamSpecType},
    types::{StaticType, StaticTypeExt},
    value::{
        ToSendValue, ToValue, ToValueOptional, ValueType, ValueTypeChecker, ValueTypeOptional,
    },
    variant::{FixedSizeVariantType, FromVariant, StaticVariantType, ToVariant},
    Continue,
};
