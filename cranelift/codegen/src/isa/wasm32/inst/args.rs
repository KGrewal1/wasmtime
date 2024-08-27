macro_rules! newtype_of_reg {
    (
        $newtype_reg:ident,
        $newtype_writable_reg:ident,
        |$check_reg:ident| $check:expr
    ) => {
        /// A newtype wrapper around `Reg`.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $newtype_reg(Reg);

        impl PartialEq<Reg> for $newtype_reg {
            fn eq(&self, other: &Reg) -> bool {
                self.0 == *other
            }
        }

        impl From<$newtype_reg> for Reg {
            fn from(r: $newtype_reg) -> Self {
                r.0
            }
        }

        impl TryFrom<Reg> for $newtype_reg {
            type Error = ();
            fn try_from(r: Reg) -> Result<Self, Self::Error> {
                Self::new(r).ok_or(())
            }
        }

        impl $newtype_reg {
            /// Create this newtype from the given register, or return `None` if the register
            /// is not a valid instance of this newtype.
            pub fn new($check_reg: Reg) -> Option<Self> {
                if $check {
                    Some(Self($check_reg))
                } else {
                    None
                }
            }

            /// Get this newtype's underlying `Reg`.
            pub fn to_reg(self) -> Reg {
                self.0
            }
        }

        // Convenience impl so that people working with this newtype can use it
        // "just like" a plain `Reg`.
        //
        // NB: We cannot implement `DerefMut` because that would let people do
        // nasty stuff like `*my_xreg.deref_mut() = some_freg`, breaking the
        // invariants that `XReg` provides.
        impl std::ops::Deref for $newtype_reg {
            type Target = Reg;

            fn deref(&self) -> &Reg {
                &self.0
            }
        }

        /// If you know what you're doing, you can explicitly mutably borrow the
        /// underlying `Reg`. Don't make it point to the wrong type of register
        /// please.
        impl AsMut<Reg> for $newtype_reg {
            fn as_mut(&mut self) -> &mut Reg {
                &mut self.0
            }
        }

        /// Writable Reg.
        pub type $newtype_writable_reg = Writable<$newtype_reg>;

        impl From<$newtype_reg> for pulley_interpreter::regs::$newtype_reg {
            fn from(r: $newtype_reg) -> Self {
                Self::new(r.to_real_reg().unwrap().hw_enc()).unwrap()
            }
        }
        impl<'a> From<&'a $newtype_reg> for pulley_interpreter::regs::$newtype_reg {
            fn from(r: &'a $newtype_reg) -> Self {
                Self::new(r.to_real_reg().unwrap().hw_enc()).unwrap()
            }
        }
        impl From<$newtype_writable_reg> for pulley_interpreter::regs::$newtype_reg {
            fn from(r: $newtype_writable_reg) -> Self {
                Self::new(r.to_reg().to_real_reg().unwrap().hw_enc()).unwrap()
            }
        }
        impl<'a> From<&'a $newtype_writable_reg> for pulley_interpreter::regs::$newtype_reg {
            fn from(r: &'a $newtype_writable_reg) -> Self {
                Self::new(r.to_reg().to_real_reg().unwrap().hw_enc()).unwrap()
            }
        }

        impl TryFrom<Writable<Reg>> for $newtype_writable_reg {
            type Error = ();
            fn try_from(r: Writable<Reg>) -> Result<Self, Self::Error> {
                let r = r.to_reg();
                match $newtype_reg::new(r) {
                    Some(r) => Ok(Writable::from_reg(r)),
                    None => Err(()),
                }
            }
        }
    };
}

newtype_of_reg!(XReg, WritableXReg, |reg| reg.class() == RegClass::Int);
newtype_of_reg!(FReg, WritableFReg, |reg| reg.class() == RegClass::Float);
newtype_of_reg!(VReg, WritableVReg, |reg| reg.class() == RegClass::Vector);
