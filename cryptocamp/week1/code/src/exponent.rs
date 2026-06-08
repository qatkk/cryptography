pub struct GroupElement {
    pub p: u64,
    pub g: u64,
}

impl GroupElement{
    pub fn look_up_creator(&self, size: u32) -> Vec<u64> {
        let mut table = Vec::new();
        table.push(self.g);
        for index in 1..size {
            table.push(((table[index as usize - 1] as u128 )*(table[index as usize - 1] as u128 ) % self.p as u128) as u64);
        }
        return table;
    }
    pub fn mod_power(&self, exp: u64) -> u64 {
        let size_in_bits = exp.ilog2() + 1;
        let lookup_table = &self.look_up_creator(size_in_bits);
        let mut result: u64 = 1;
        let mut bit: bool;
        for index in 0..size_in_bits {
            bit = ((exp >> index)  & 1 ) != 0;
            if bit {
                result = (result as u128 * lookup_table[index as usize] as u128 % self.p as u128) as u64;
            }
        }
        return result % self.p;
    }
    pub fn mod_inv(&self) -> u64 {
        // According to Fermat's little theorem, the modular inverse of an element "g"
        //      in the group is simply "g^(p-2)".
        self.mod_power(self.p - 2)
    }
}
#[cfg(test)]
mod tests {
    use crate::exponent::GroupElement;

    #[test]
    fn test_power() {
        //  Largest prime fitting in u64
        let mut test_element = GroupElement{
            p: 18446744073709551557,
            g: 2,            
        };
        assert!(test_element.mod_power(test_element.p-1) == 1);
        test_element.g = 3;
        assert!(test_element.mod_power(test_element.p-1) == 1);
        test_element.g = test_element.p-1; 
        assert!(test_element.mod_power(3) == test_element.p-1);
        test_element.g = 18446744073709551615;
        assert!(((test_element.mod_power(test_element.p-2) as u128 * test_element.g as u128) % test_element.p as u128) == 1);
    }
    #[test]
    fn test_inv(){
        //  Largest prime fitting in u64
        let mut test_element = GroupElement{
            p: 18446744073709551557,
            g: 1,            
        };
        assert!(test_element.mod_inv() == 1);
        test_element.g = test_element.p - 1; 
        assert!(test_element.mod_inv() == test_element.p -1);
        test_element.g = 2;
        assert!(test_element.mod_inv() == (test_element.p + 1) /2);
    }
}