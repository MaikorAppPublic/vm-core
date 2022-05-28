use crate::register::Register;
use crate::VM;

impl VM {
    /// Run Pre Inc/Dec on register
    /// Also check and read bytes for index addressing
    /// Returns address offset and cycle cost
    #[must_use]
    pub fn pre_process(&mut self, reg: &Register, op_size: u8) -> (i16, usize) {
        let process_cost = self.process_arg(reg, op_size, false);
        if reg.is_offset_num {
            (self.read_arg_word() as i16, process_cost)
        } else if reg.is_offset_reg {
            let offset_reg = self.read_arg_register();
            let (num, cost) = self.read_byte_reg_value(&offset_reg);
            (num as i16, cost + process_cost)
        } else if reg.is_offset_ext_reg {
            let offset_reg = self.read_arg_register();
            let (num, cost) = self.read_word_reg_value(&offset_reg);
            (num as i16, cost + process_cost)
        } else {
            (0, process_cost)
        }
    }

    #[inline(always)]
    #[must_use]
    pub fn post_process(&mut self, reg: &Register, op_size: u8) -> usize {
        self.process_arg(reg, op_size, true)
    }

    #[must_use]
    fn process_arg(&mut self, reg: &Register, op_size: u8, is_post: bool) -> usize {
        if reg.is_calc && reg.is_post == is_post {
            if reg.is_inc {
                match reg.size {
                    1 => {
                        let (value, _) = self.read_byte_reg_value(reg);
                        return self.write_byte_reg_value(reg, value.wrapping_add(op_size));
                    }
                    2 => {
                        let (value, _) = self.read_word_reg_value(reg);
                        return self.write_word_reg_value(reg, value.wrapping_add(op_size as u16));
                    }
                    _ => self.fail(format!("Invalid register size: {}", reg.addr)),
                }
            } else {
                match reg.size {
                    1 => {
                        let (value, _) = self.read_byte_reg_value(reg);
                        return self.write_byte_reg_value(reg, value.wrapping_sub(op_size));
                    }
                    2 => {
                        let (value, _) = self.read_word_reg_value(reg);
                        return self.write_word_reg_value(reg, value.wrapping_sub(op_size as u16));
                    }
                    _ => self.fail(format!("Invalid register size: {}", reg.addr)),
                }
            }
        }
        0
    }
}
