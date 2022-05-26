use crate::register::Register;
use crate::VM;

impl VM {
    /// Run Pre Inc/Dec on register
    /// Also check and read bytes for index addressing
    /// Returns address offset and cycle cost
    pub fn pre_process(&mut self, reg: &Register) -> (i16, usize) {
        self.process_arg(reg, false);
        if reg.is_offset_num {
            (self.read_arg_word() as i16, 0)
        } else if reg.is_offset_reg {
            let offset_reg = self.read_arg_register();
            let (num, cost) = self.read_byte_reg_value(&offset_reg);
            (num as i16, cost)
        } else if reg.is_offset_ext_reg {
            let offset_reg = self.read_arg_register();
            let (num, cost) = self.read_word_reg_value(&offset_reg);
            (num as i16, cost)
        } else {
            (0, 0)
        }
    }

    #[inline(always)]
    pub fn post_process(&mut self, reg: &Register) {
        self.process_arg(reg, true)
    }

    fn process_arg(&mut self, reg: &Register, is_post: bool) {
        if reg.is_calc && reg.is_post == is_post {
            if reg.is_inc {
                match reg.size {
                    1 => {
                        let (value, _) = self.read_byte_reg_value(reg);
                        self.write_byte_reg_value(reg, value.wrapping_add(1));
                    }
                    2 => {
                        let (value, _) = self.read_word_reg_value(reg);
                        self.write_word_reg_value(reg, value.wrapping_add(2));
                    }
                    _ => self.fail(format!("Invalid register size: {}", reg.addr)),
                }
            } else {
                match reg.size {
                    1 => {
                        let (value, _) = self.read_byte_reg_value(reg);
                        self.write_byte_reg_value(reg, value.wrapping_sub(1));
                    }
                    2 => {
                        let (value, _) = self.read_word_reg_value(reg);
                        self.write_word_reg_value(reg, value.wrapping_sub(2));
                    }
                    _ => self.fail(format!("Invalid register size: {}", reg.addr)),
                }
            }
        }
    }
}
