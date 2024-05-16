use revm::primitives::U256;

pub struct IntegerDecimal {
    pub number: U256,
    pub decimals: U256
}

impl IntegerDecimal {
    pub fn new(number: U256, decimals: U256) -> Self {
        Self {
            number,
            decimals
        }
    }

    pub fn integer(&self) -> U256 {
        self.number.checked_div(U256::from(10).checked_pow(self.decimals).unwrap()).unwrap()
    }

    pub fn decimal(&self) -> U256 {
        if self.decimals == U256::from(0) {
            return U256::from(0);
            
        }
        let divisor = U256::from(10).checked_pow(self.decimals).unwrap();
        self.number.checked_rem(divisor).unwrap()
    }

    pub fn divide(&self, divisor: &IntegerDecimal) -> f64 {
        if divisor.number.is_zero(){
            panic!("Cannot divide by zero");
        }
        let dividend = self.to_string().parse::<f64>().unwrap();
        let divisor = divisor.to_string().parse::<f64>().unwrap();
        dividend / divisor
    }

    pub fn to_string(&self) -> String {
        format!("{}.{}", self.integer(), self.decimal())
    }
}

#[cfg(test)]
mod tests {
    use crate::integer_decimal::IntegerDecimal;

    #[test]
    fn test_integer() {
        let integer_decimal = IntegerDecimal {
            number: revm::primitives::U256::from(123456),
            decimals: revm::primitives::U256::from(3)
        };
        assert_eq!(integer_decimal.integer(), revm::primitives::U256::from(123));
        let zero_decimals = IntegerDecimal {
            number: revm::primitives::U256::from(123456),
            decimals: revm::primitives::U256::from(0)
        };
        assert_eq!(zero_decimals.integer(), revm::primitives::U256::from(123456));

        let zero_integer = IntegerDecimal {
            number: revm::primitives::U256::from(123),
            decimals: revm::primitives::U256::from(3)
        };
        assert_eq!(zero_integer.integer(), revm::primitives::U256::from(0));
    }

    #[test]
    fn test_decimal() {
        let integer_decimal = IntegerDecimal {
            number: revm::primitives::U256::from(123456),
            decimals: revm::primitives::U256::from(3)
        };
        assert_eq!(integer_decimal.decimal(), revm::primitives::U256::from(456));


        let zero_decimals = IntegerDecimal {
            number: revm::primitives::U256::from(123456),
            decimals: revm::primitives::U256::from(0)
        };

        assert_eq!(zero_decimals.decimal(), revm::primitives::U256::from(0));

        let zero_integer = IntegerDecimal {
            number: revm::primitives::U256::from(123),
            decimals: revm::primitives::U256::from(3)
        };

        assert_eq!(zero_integer.decimal(), revm::primitives::U256::from(123));
        
        let zero_integer = IntegerDecimal {
            number: revm::primitives::U256::from(123),
            decimals: revm::primitives::U256::from(4)
        };

        assert_eq!(zero_integer.decimal(), revm::primitives::U256::from(123));
    }

    #[test]
    fn test_one_divided_by_two() {
        let one_integer = IntegerDecimal {
            number: revm::primitives::U256::from(1),
            decimals: revm::primitives::U256::from(0)
        };
        let two_integer = IntegerDecimal {
            number: revm::primitives::U256::from(2),
            decimals: revm::primitives::U256::from(0)
        };
        let one_divided_by_two = one_integer.divide(&two_integer);
        assert_eq!(one_divided_by_two, 0.5);
    }

    #[test]
    fn test_two_divided_by_one() {
        let one_integer = IntegerDecimal {
            number: revm::primitives::U256::from(1),
            decimals: revm::primitives::U256::from(0)
        };
        let two_integer = IntegerDecimal {
            number: revm::primitives::U256::from(2),
            decimals: revm::primitives::U256::from(0)
        };
        let two_divided_by_one = two_integer.divide(&one_integer);
        assert_eq!(two_divided_by_one, 2.0);
    }
}