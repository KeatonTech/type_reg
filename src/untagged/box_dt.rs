use std::ops::{Deref, DerefMut};

use serde::Serialize;

use crate::{
    untagged::{BoxDataTypeDowncast, DataType, DataTypeWrapper, IntoBoxDataType},
    TypeNameLit,
};

/// Box of any type, with no additional trait constraints.
#[derive(Clone, Serialize)]
pub struct BoxDt(pub(crate) Box<dyn DataType>);

#[cfg(feature = "debug")]
impl std::fmt::Debug for BoxDt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        (*self.0).fmt(f)
    }
}

#[cfg(not(feature = "debug"))]
impl std::fmt::Debug for BoxDt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_tuple("BoxDt").field(&"..").finish()
    }
}

impl BoxDt {
    /// Returns a new `BoxDt` wrapper around the provided type.
    pub fn new<T>(t: T) -> Self
    where
        T: DataType,
    {
        Self(Box::new(t))
    }
}

impl Deref for BoxDt {
    type Target = dyn DataType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BoxDt {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> IntoBoxDataType<BoxDt> for T
where
    T: DataType,
{
    fn into(t: Self) -> BoxDt {
        BoxDt(Box::new(t))
    }
}

impl<T> BoxDataTypeDowncast<T> for BoxDt
where
    T: DataType,
{
    fn downcast_ref(&self) -> Option<&T> {
        self.0.downcast_ref::<T>()
    }

    fn downcast_mut(&mut self) -> Option<&mut T> {
        self.0.downcast_mut::<T>()
    }
}

impl DataTypeWrapper for BoxDt {
    fn type_name(&self) -> TypeNameLit {
        DataType::type_name(&*self.0)
    }

    fn clone(&self) -> Self {
        Self(self.0.clone())
    }

    #[cfg(feature = "debug")]
    fn debug(&self) -> &dyn std::fmt::Debug {
        &self.0
    }

    fn inner(&self) -> &dyn DataType {
        &self.0
    }

    fn inner_mut(&mut self) -> &mut dyn DataType {
        &mut self.0
    }
}
