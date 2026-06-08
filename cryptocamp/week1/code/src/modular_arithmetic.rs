pub fn look_up_creator(value: u64, prime: u64, table_size: u32) -> Vec<u64> {
    let mut table = Vec::new();
    table.push(value);
    for index in 1..table_size {
        table.push(((table[index as usize - 1] as u128 )*(table[index as usize - 1] as u128 ) % prime as u128) as u64);
    }
    return table;
}
pub fn mod_power(value: u64, prime: u64, exp: u64) -> u64 {
    let size_in_bits = exp.ilog2() + 1;
    let lookup_table = look_up_creator(value, prime, size_in_bits);
    let mut result: u64 = 1;
    let mut bit: bool;
    for index in 0..size_in_bits {
        bit = ((exp >> index)  & 1 ) != 0;
        if bit {
            result = (result as u128 * lookup_table[index as usize] as u128 % prime as u128) as u64;
        }
    }
    return result % prime;
}
pub fn mod_inv(value: u64, prime: u64) -> u64 {
    // According to Fermat's little theorem, the modular inverse of an element "g"
    //      in the group is simply "g^(p-2)".
    mod_power(value, prime, prime - 2)
}

#[cfg(test)]
mod tests {
    use crate::modular_arithmetic::{mod_inv, mod_power};

    #[test]
    fn test_power() {
        //  Largest prime fitting in u64
        let p = 18446744073709551557;
        assert!(mod_power(2, p, p-1) == 1);
        assert!(mod_power(3, p, p-1) == 1);
        assert!(mod_power(p-1, p, 3) == p-1);
        assert!(((mod_power(18446744073709551615, p, p-2) as u128 * 18446744073709551615 as u128) % p as u128) == 1);
    }
    #[test]
    fn test_inv(){
        //  Largest prime fitting in u64
        let p = 18446744073709551557;
        assert!(mod_inv(1, p) == 1);
        assert!(mod_inv(p-1, p) == p -1);
        assert!(mod_inv(2, p) == (p + 1) /2);
    }
}