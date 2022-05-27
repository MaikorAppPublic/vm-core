use crate::VM;

impl VM {
    /// Jump based on flag state
    /// If flag state matches required then jump
    pub fn jb_reg_reg(&mut self, required: bool) -> (bool, usize) {
        let dst = self.read_arg_register();
        let mask = self.read_arg_register();
        let (mask_offset, mask_offset_cost) = self.pre_process(&mask);
        let (dst_offset, dst_offset_cost) = self.pre_process(&dst);
        let (mask_value, mask_cost) = self.read_byte_reg(&mask, mask_offset);
        let (dst_value, dst_cost) = self.read_word_reg(&dst, dst_offset);
        let cost = mask_offset_cost
            + dst_offset_cost
            + mask_cost
            + dst_cost
            + self.post_process(&mask)
            + self.post_process(&dst);
        if required == self.check_flag(mask_value) {
            self.pc = dst_value;
            return (true, 1 + cost);
        }
        (false, cost)
    }

    /// Jump based on flag state
    /// If flag state matches required then jump
    pub fn jb_addr_reg(&mut self, required: bool) -> (bool, usize) {
        let dst = self.read_arg_word();
        let mask = self.read_arg_register();
        let (offset, offset_cost) = self.pre_process(&mask);
        let (mask_value, read_cost) = self.read_byte_reg(&mask, offset);
        let cost = read_cost + offset_cost + self.post_process(&mask);
        if required == self.check_flag(mask_value) {
            self.pc = dst;
            return (true, 1 + cost);
        }
        (false, cost)
    }

    /// Jump based on flag state
    /// If flag state matches required then jump
    pub fn jb_reg_num(&mut self, required: bool) -> (bool, usize) {
        let dst = self.read_arg_register();
        let mask = self.read_arg_byte();
        let (offset, offset_cost) = self.pre_process(&dst);
        let (addr, read_cost) = self.read_word_reg(&dst, offset);
        let cost = self.post_process(&dst) + read_cost + offset_cost;
        if required == self.check_flag(mask) {
            self.pc = addr;
            return (true, 1 + cost);
        }
        (false, cost)
    }

    /// Jump based on flag state
    /// If flag state matches required then jump
    pub fn jb_addr_num(&mut self, required: bool) -> (bool, usize) {
        let addr = self.read_arg_word();
        let mask = self.read_arg_byte();
        if required == self.check_flag(mask) {
            self.pc = addr;
            return (true, 2);
        }
        (false, 0)
    }
}
