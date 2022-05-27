use crate::VM;
use std::ops::Not;

impl VM {
    pub fn not_reg_byte(&mut self) -> usize {
        let reg = self.read_arg_register();
        let (offset, offset_cost) = self.pre_process(&reg);
        let (num, read_cost) = self.read_byte_reg(&reg, offset);
        let write_cost = self.write_byte_reg(&reg, offset, num.not());
        offset_cost + read_cost + write_cost + self.post_process(&reg)
    }

    pub fn not_reg_word(&mut self) -> usize {
        let reg = self.read_arg_register();
        let (offset, offset_cost) = self.pre_process(&reg);
        let (num, read_cost) = self.read_word_reg(&reg, offset);
        let write_cost = self.write_word_reg(&reg, offset, num.not());
        offset_cost + read_cost + write_cost + self.post_process(&reg)
    }

    pub fn bl_reg_reg_byte(&mut self, method: fn(u8, u8) -> u8) -> usize {
        let dst = self.read_arg_register();
        let src = self.read_arg_register();
        let (dst_offset, dst_offset_cost) = self.pre_process(&dst);
        let (src_offset, src_offset_cost) = self.pre_process(&src);
        let (dst_value, dst_read_cost) = self.read_byte_reg(&dst, dst_offset);
        let (src_value, src_read_cost) = self.read_byte_reg(&src, src_offset);
        let value = method(dst_value, src_value);
        let write_cost = self.write_byte_reg(&dst, dst_offset, value);
        dst_offset_cost
            + src_offset_cost
            + dst_read_cost
            + src_read_cost
            + write_cost
            + self.post_process(&dst)
            + self.post_process(&src)
    }

    pub fn bl_reg_reg_word(&mut self, method: fn(u16, u16) -> u16) -> usize {
        let dst = self.read_arg_register();
        let src = self.read_arg_register();
        let (dst_offset, dst_offset_cost) = self.pre_process(&dst);
        let (src_offset, src_offset_cost) = self.pre_process(&src);
        let (dst_value, dst_read_cost) = self.read_word_reg(&dst, dst_offset);
        let (src_value, src_read_cost) = self.read_word_reg(&src, src_offset);
        let value = method(dst_value, src_value);
        let write_cost = self.write_word_reg(&dst, dst_offset, value);
        dst_offset_cost
            + src_offset_cost
            + dst_read_cost
            + src_read_cost
            + write_cost
            + self.post_process(&dst)
            + self.post_process(&src)
    }

    pub fn bl_reg_num_byte(&mut self, method: fn(u8, u8) -> u8) -> usize {
        let dst = self.read_arg_register();
        let src = self.read_arg_byte();
        let (dst_offset, dst_offset_cost) = self.pre_process(&dst);
        let (dst_value, dst_read_cost) = self.read_byte_reg(&dst, dst_offset);
        let value = method(dst_value, src);
        let write_cost = self.write_byte_reg(&dst, dst_offset, value);
        dst_offset_cost + dst_read_cost + write_cost + self.post_process(&dst)
    }

    pub fn bl_reg_num_word(&mut self, method: fn(u16, u16) -> u16) -> usize {
        let dst = self.read_arg_register();
        let src = self.read_arg_word();
        let (dst_offset, dst_offset_cost) = self.pre_process(&dst);
        let (dst_value, dst_read_cost) = self.read_word_reg(&dst, dst_offset);
        let value = method(dst_value, src);
        let write_cost = self.write_word_reg(&dst, dst_offset, value);
        dst_offset_cost + dst_read_cost + write_cost + self.post_process(&dst)
    }
}
