pub struct GroupElement {
    pub base: u32,
    pub value: u32,
}

impl GroupElement{
    pub fn look_up_creator(&self, size: u32) -> Vec<u32> {
        let mut table = Vec::new();
        table.push(self.value);
        for index in 1..size {
            table.push(&table[index as usize - 1] * &table[index as usize - 1] % self.base);
        }
        return table;
    }
    pub fn exponent(&self, exponent_value: u32) -> u32 {
        let size = exponent_value.ilog2() + 1;
        let lookup_table = &self.look_up_creator(size);
        let mut result: u32 = 1;
        let mut bit: bool;
        for index in 0..size {
            bit = ((exponent_value >> index)  & 1 ) != 0;
            print!("the bit in index {:} is {:} ", index, bit);
            if bit {
                result *= lookup_table[index as usize] % self.base;
            }
        }
        return result;
    }
}