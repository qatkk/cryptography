pub struct GroupElement {
    pub p: u32,
    pub g: u32,
}

impl GroupElement{
    pub fn look_up_creator(&self, size: u32) -> Vec<u32> {
        let mut table = Vec::new();
        table.push(self.g);
        for index in 1..size {
            table.push(&table[index as usize - 1].pow(2) % self.p);
        }
        return table;
    }
    pub fn mod_power(&self, exp: u32) -> u32 {
        let size_in_bits = exp.ilog2() + 1;
        let lookup_table = &self.look_up_creator(size_in_bits);
        let mut result: u32 = 1;
        let mut bit: bool;
        for index in 0..size_in_bits {
            bit = ((exp >> index)  & 1 ) != 0;
            if bit {
                result *= lookup_table[index as usize] % self.p;
            }
        }
        return result;
    }
}