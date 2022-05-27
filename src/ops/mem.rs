use crate::VM;

impl VM {
    pub fn copy_mem(&mut self, dst: u16, src: u16, count: usize) {
        unsafe {
            let dst_ptr = self.get_memory_mut(dst as usize, count).as_mut_ptr();
            let src_ptr = self.get_memory_mut(src as usize, count).as_mut_ptr();
            std::ptr::copy(src_ptr, dst_ptr, count);
        }
    }

    pub fn swap_mem(&mut self, dst: u16, src: u16, count: usize) {
        unsafe {
            let dst_ptr = self.get_memory_mut(dst as usize, count).as_mut_ptr();
            let src_ptr = self.get_memory_mut(src as usize, count).as_mut_ptr();
            std::ptr::swap(src_ptr, dst_ptr);
        }
    }

    pub fn mem_addr_addr_byte(&mut self, method: fn(&mut VM, u16, u16, usize)) -> usize {
        let dst = self.read_arg_word();
        let src = self.read_arg_word();
        let count = self.read_arg_byte();
        method(self, dst, src, count as usize);
        2
    }

    pub fn mem_addr_addr_reg(&mut self, method: fn(&mut VM, u16, u16, usize)) -> usize {
        let dst = self.read_arg_word();
        let src = self.read_arg_word();
        let count = self.read_arg_register();
        let (offset, offset_cost) = self.pre_process(&count);
        let (count_value, read_cost) = self.read_byte_reg(&count, offset);
        method(self, dst, src, count_value as usize);
        offset_cost + read_cost + 2 + self.post_process(&count)
    }

    pub fn mem_addr_reg_byte(&mut self, method: fn(&mut VM, u16, u16, usize)) -> usize {
        let dst = self.read_arg_word();
        let src = self.read_arg_register();
        let count = self.read_arg_byte();
        let (offset, offset_cost) = self.pre_process(&src);
        let (src_value, read_cost) = self.read_word_reg(&src, offset);
        method(self, dst, src_value, count as usize);
        offset_cost + read_cost + 2 + self.post_process(&src)
    }

    pub fn mem_addr_reg_reg(&mut self, method: fn(&mut VM, u16, u16, usize)) -> usize {
        let dst = self.read_arg_word();
        let src = self.read_arg_register();
        let count = self.read_arg_register();
        let (src_offset, src_offset_cost) = self.pre_process(&src);
        let (count_offset, count_offset_cost) = self.pre_process(&count);
        let (src_value, src_read_cost) = self.read_word_reg(&src, src_offset);
        let (count_value, count_read_cost) = self.read_word_reg(&count, count_offset);
        method(self, dst, src_value, count_value as usize);

        count_offset_cost
            + src_offset_cost
            + src_read_cost
            + count_read_cost
            + 2
            + self.post_process(&count)
            + self.post_process(&src)
    }

    pub fn mem_reg_addr_byte(&mut self, method: fn(&mut VM, u16, u16, usize)) -> usize {
        let dst = self.read_arg_register();
        let src = self.read_arg_word();
        let count = self.read_arg_byte();
        let (offset, offset_cost) = self.pre_process(&dst);
        let (dst_addr, read_cost) = self.read_word_reg(&dst, offset);
        method(self, dst_addr, src, count as usize);
        read_cost + offset_cost + 2 + self.post_process(&dst)
    }

    pub fn mem_reg_addr_reg(&mut self, method: fn(&mut VM, u16, u16, usize)) -> usize {
        let dst = self.read_arg_register();
        let src = self.read_arg_word();
        let count = self.read_arg_register();
        let (dst_offset, dst_offset_cost) = self.pre_process(&dst);
        let (count_offset, count_offset_cost) = self.pre_process(&count);
        let (count_value, count_read_cost) = self.read_byte_reg(&count, count_offset);
        let (dst_addr, dst_read_cost) = self.read_word_reg(&dst, dst_offset);
        method(self, dst_addr, src, count_value as usize);
        count_offset_cost
            + count_read_cost
            + 2
            + dst_offset_cost
            + dst_read_cost
            + self.post_process(&count)
    }

    pub fn mem_reg_reg_byte(&mut self, method: fn(&mut VM, u16, u16, usize)) -> usize {
        let dst = self.read_arg_register();
        let src = self.read_arg_register();
        let count = self.read_arg_byte();
        let (src_offset, src_offset_cost) = self.pre_process(&src);
        let (dst_offset, dst_offset_cost) = self.pre_process(&dst);
        let (src_addr, src_read_cost) = self.read_word_reg(&src, src_offset);
        let (dst_addr, dst_read_cost) = self.read_word_reg(&dst, dst_offset);
        method(self, dst_addr, src_addr, count as usize);
        src_offset_cost
            + src_read_cost
            + 2
            + dst_offset_cost
            + dst_read_cost
            + self.post_process(&src)
            + self.post_process(&dst)
    }

    pub fn mem_reg_reg_reg(&mut self, method: fn(&mut VM, u16, u16, usize)) -> usize {
        let dst = self.read_arg_register();
        let src = self.read_arg_register();
        let count = self.read_arg_register();
        let (src_offset, src_offset_cost) = self.pre_process(&src);
        let (dst_offset, dst_offset_cost) = self.pre_process(&dst);
        let (count_offset, count_offset_cost) = self.pre_process(&count);
        let (src_addr, src_read_cost) = self.read_word_reg(&src, src_offset);
        let (dst_addr, dst_read_cost) = self.read_word_reg(&dst, dst_offset);
        let (count_value, count_read_cost) = self.read_word_reg(&count, count_offset);
        method(self, dst_addr, src_addr, count_value as usize);
        self.post_process(&count)
            + self.post_process(&src)
            + self.post_process(&dst)
            + count_offset_cost
            + src_offset_cost
            + src_read_cost
            + count_read_cost
            + 2
            + dst_offset_cost
            + dst_read_cost
    }
}
