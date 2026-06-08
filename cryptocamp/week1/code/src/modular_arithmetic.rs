use num_bigint::{self, BigUint, ToBigUint};
pub fn look_up_creator(
    value: &num_bigint::BigUint,
    prime: &num_bigint::BigUint,
    table_size: u64,
) -> Vec<BigUint> {
    let mut table: Vec<BigUint> = Vec::new();
    table.push(value.clone());
    for index in 1..table_size {
        let prev = table[index as usize - 1].clone();
        table.push((prev.clone() * prev) % prime);
    }
    return table;
}
pub fn mod_power(
    value: &num_bigint::BigUint,
    prime: &num_bigint::BigUint,
    exp: &num_bigint::BigUint,
) -> num_bigint::BigUint {
    let size_in_bits = exp.bits();
    let lookup_table = look_up_creator(&value, &prime, size_in_bits);
    let mut result = 1.to_biguint().unwrap();
    for index in 0..size_in_bits {
        if exp.bit(index) {
            result *= &lookup_table[index as usize];
        }
    }
    return result % prime;
}
pub fn mod_inv(value: &num_bigint::BigUint, prime: &num_bigint::BigUint) -> num_bigint::BigUint {
    // According to Fermat's little theorem, the modular inverse of an element "g"
    //      in the group is simply "g^(p-2)".
    let exponent = prime - 2.to_biguint().unwrap();
    mod_power(value, prime, &exponent)
}

#[cfg(test)]
mod tests {
    use num_bigint::ToBigUint;

    use crate::modular_arithmetic::{mod_inv, mod_power};

    #[test]
    fn test_power() {
        //  Largest prime fitting in u64
        let p = 18446744073709551557_u64.to_biguint().unwrap();
        assert!(
            mod_power(
                &2.to_biguint().unwrap(),
                &p,
                &(p.clone() - 1.to_biguint().unwrap())
            ) == 1.to_biguint().unwrap()
        );
        assert!(
            mod_power(
                &3.to_biguint().unwrap(),
                &p,
                &(p.clone() - 1.to_biguint().unwrap())
            ) == 1.to_biguint().unwrap()
        );
        assert!(
            mod_power(
                &(p.clone() - 1.to_biguint().unwrap()),
                &p.clone(),
                &3.to_biguint().unwrap()
            ) == p.clone() - 1.to_biguint().unwrap()
        );
        assert!(
            ((mod_power(
                &18446744073709551615_u64.to_biguint().unwrap(),
                &p.clone(),
                &(p.clone() - 2.to_biguint().unwrap())
            ) * 18446744073709551615_u64.to_biguint().unwrap())
                % p)
                == 1.to_biguint().unwrap()
        );
    }
    #[test]
    fn test_inv() {
        //  Largest prime fitting in u64
        let p = 18446744073709551557_u64.to_biguint().unwrap();
        assert!(mod_inv(&1.to_biguint().unwrap(), &(p.clone())) == 1.to_biguint().unwrap());
        assert!(
            mod_inv(&(p.clone() - 1.to_biguint().unwrap()), &(p.clone()))
                == p.clone() - 1.to_biguint().unwrap()
        );
        assert!(
            mod_inv(&2.to_biguint().unwrap(), &p.clone())
                == (p + 1.to_biguint().unwrap()) / 2.to_biguint().unwrap()
        );
    }
}
