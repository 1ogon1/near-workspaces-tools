use std::ops::{Div, Mul};

use crate::{TokenDecimals, U128Extensions};

impl U128Extensions for u128 {
    fn to_decimals(self, decimals: TokenDecimals) -> u128 {
        self.mul(10_u128.pow(decimals as _))
    }

    fn strip_decimals(self, decimals: TokenDecimals) -> u128 {
        self.div(10_u128.pow(decimals as _))
    }
}
