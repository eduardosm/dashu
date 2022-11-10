use core::ops::{Mul, Neg};
use dashu_base::Sign;

use crate::{
    rbig::{RBig, Relaxed},
    repr::Repr,
};

impl RBig {
    /// Get the sign of the number. Zero value has a positive sign.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dashu_base::Sign;
    /// # use dashu_ratio::RBig;
    /// assert_eq!(RBig::ZERO.sign(), Sign::Positive);
    /// assert_eq!(RBig::ONE.sign(), Sign::Positive);
    /// assert_eq!(RBig::NEG_ONE.sign(), Sign::Negative);
    /// ```
    #[inline]
    pub const fn sign(&self) -> Sign {
        self.0.numerator.sign()
    }
}

impl Relaxed {
    /// Get the sign of the number. Zero value has a positive sign.
    ///
    /// See [RBig::sign] for details.
    #[inline]
    pub const fn sign(&self) -> Sign {
        self.0.numerator.sign()
    }
}

impl Repr {
    #[inline]
    pub fn neg(mut self) -> Repr {
        self.numerator = -self.numerator;
        self
    }

    #[inline]
    pub fn abs(mut self) -> Repr {
        if self.numerator.sign() == Sign::Negative {
            self.numerator = -self.numerator
        }
        self
    }
}

impl Neg for RBig {
    type Output = RBig;
    #[inline]
    fn neg(self) -> Self::Output {
        RBig(self.0.neg())
    }
}

impl Neg for &RBig {
    type Output = RBig;
    #[inline]
    fn neg(self) -> Self::Output {
        RBig(self.0.clone().neg())
    }
}

impl Neg for Relaxed {
    type Output = Relaxed;
    #[inline]
    fn neg(self) -> Self::Output {
        Relaxed(self.0.neg())
    }
}

impl Neg for &Relaxed {
    type Output = Relaxed;
    #[inline]
    fn neg(self) -> Self::Output {
        Relaxed(self.0.clone().neg())
    }
}

impl Mul<Repr> for Sign {
    type Output = Repr;
    #[inline]
    fn mul(self, mut rhs: Repr) -> Repr {
        rhs.numerator *= self;
        rhs
    }
}
impl Mul<Sign> for Repr {
    type Output = Repr;
    #[inline]
    fn mul(mut self, rhs: Sign) -> Repr {
        self.numerator *= rhs;
        self
    }
}

impl Mul<Sign> for RBig {
    type Output = RBig;
    #[inline]
    fn mul(mut self, rhs: Sign) -> RBig {
        self.0.numerator *= rhs;
        self
    }
}

impl Mul<Sign> for Relaxed {
    type Output = Relaxed;
    #[inline]
    fn mul(mut self, rhs: Sign) -> Self::Output {
        self.0.numerator *= rhs;
        self
    }
}
