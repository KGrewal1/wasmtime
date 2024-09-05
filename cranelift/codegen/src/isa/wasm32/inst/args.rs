use waffle::ValueDef;

/// A newtype wrapper around `ValueDef`.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Value(ValueDef);

impl PartialEq<ValueDef> for Value {
    fn eq(&self, other: &ValueDef) -> bool {
        self.0 == *other
    }
}

impl From<Value> for ValueDef {
    fn from(r: Value) -> Self {
        r.0
    }
}

impl TryFrom<ValueDef> for Value {
    type Error = ();
    fn try_from(r: ValueDef) -> Result<Self, Self::Error> {
        Self::new(r).ok_or(())
    }
}

impl Value {
    pub fn new(value: ValueDef) -> Self {
        Self(value)
    }

    /// Get this newtype's underlying `ValueDef`.
    pub fn to_def(self) -> ValueDef {
        self.0
    }
}

// Convenience impl so that people working with this newtype can use it
// "just like" a plain `ValueDef`.
impl std::ops::Deref for Value {
    type Target = ValueDef;

    fn deref(&self) -> &ValueDef {
        &self.0
    }
}

/// If you know what you're doing, you can explicitly mutably borrow the
/// underlying `ValueDef`.
impl AsMut<ValueDef> for Value {
    fn as_mut(&mut self) -> &mut ValueDef {
        &mut self.0
    }
}
