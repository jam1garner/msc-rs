use super::MscsbFile;
use super::super::{Cmd, Command, Script};
use byteorder::{LittleEndian, BigEndian, WriteBytesExt};

impl MscsbFile {
    pub fn write(&self, f: &mut Vec<u8>) {
        // Little Endian
        macro_rules! write {
            ($e:expr) => {
                WriteImpl::write($e, f, false);
            }
        }
        let max_str_len = self.get_max_string_size();
        let mut script_data: Vec<u8> = vec![];
        let script_offsets = self.generate_script_data(&mut script_data);
        // Write magic
        write!(&b"\xB2\xAC\xBC\xBA\xE6\x90\x32\x01\xFD\x02\x00\x00\x00\x00\x00\x00"[..]);
        write!(0u32);
        write!(script_data.len() as u32);
        write!(self.entrypoint);
        write!(self.scripts.len() as u32);
        write!(0x16u32); // oof ouch magic number
        write!(max_str_len);
        write!(self.strings.len() as u32);
        write!(vec![0u8; 8]);
        write!(&script_data[..]);
        write!(&[0u8; 0xF][..(0x10 - (f.len() % 0x10)) & 0xF]); // Pad to 0x10
        write!(script_offsets);
        write!(&[0u8; 0xF][..(0x10 - (f.len() % 0x10)) & 0xF]); // Pad to 0x10
        for string in self.strings.iter() {
            write!(string.as_bytes());
            write!(0u8); // null terminate
            // Pad strings to max length
            write!(vec![0u8; max_str_len as usize - (1 + string.as_bytes().len())]);
        }
    }

    fn generate_script_data(&self, f: &mut Vec<u8>) -> Vec<u32> {
        // Big Endian
        macro_rules! write {
            ($e:expr) => {
                WriteImpl::write($e, f, true);
            }
        }
        let mut script_offsets = vec![];
        
        write!(vec![0u8; 0x10]);
        for script in self.scripts.iter() {
            script_offsets.push(f.len() as u32);
            for command in script.commands.iter() {
                write!(command);
            }
        }

        script_offsets
    }

    fn get_max_string_size(&self) -> u32 {
        self.strings
            .iter()
            .map(|s| (s.len() + 0x10) & 0xF)
            .max()
            .unwrap() as u32
    }
}

impl WriteImpl for &Command {
    fn write(self, f: &mut Vec<u8>, endian: bool) {
        // Big Endian
        macro_rules! write {
            ($e:expr) => {
                WriteImpl::write($e, f, endian);
            }
        }
        write!(self.cmd.value() & (if self.push_bit {0x80u8} else {0x0u8}));
        match self.cmd {
            Cmd::Begin { arg_count, var_count } => {
                write!(arg_count);
                write!(var_count);
            },
            Cmd::Jump { loc } => {
                write!(loc);
            },
            Cmd::Jump5 { loc } => {
                write!(loc);
            },
            Cmd::PushInt { val } => {
                write!(val);
            },
            Cmd::PushVar { var_type, var_num } => {
                write!(var_type);
                write!(var_num);
            },
            Cmd::PushShort { val } => {
                write!(val);
            },
            Cmd::IncI { var_type, var_num } => {
                write!(var_type);
                write!(var_num);
            },
            Cmd::DecI { var_type, var_num } => {
                write!(var_type);
                write!(var_num);
            },
            Cmd::SetVar { var_type, var_num } => {
                write!(var_type);
                write!(var_num);
            },
            Cmd::AddVarBy { var_type, var_num } => {
                write!(var_type);
                write!(var_num);
            },
            Cmd::SubVarBy { var_type, var_num } => {
                write!(var_type);
                write!(var_num);
            },
            Cmd::MultVarBy { var_type, var_num } => {
                write!(var_type);
                write!(var_num);
            },
            Cmd::DivVarBy { var_type, var_num } => {
                write!(var_type);
                write!(var_num);
            },
            Cmd::ModVarBy { var_type, var_num } => {
                write!(var_type);
                write!(var_num);
            },
            Cmd::AndVarBy { var_type, var_num } => {
                write!(var_type);
                write!(var_num);
            },
            Cmd::OrVarBy { var_type, var_num } => {
                write!(var_type);
                write!(var_num);
            },
            Cmd::XorVarBy { var_type, var_num } => {
                write!(var_type);
                write!(var_num);
            },
            Cmd::PrintF { arg_count } => {
                write!(arg_count);
            },
            Cmd::Sys { arg_count, sys_num } => {
                write!(arg_count);
                write!(sys_num);
            },
            Cmd::Try { loc } => {
                write!(loc);
            },
            Cmd::CallFunc { arg_count } => {
                write!(arg_count);
            },
            Cmd::CallFunc2 { arg_count } => {
                write!(arg_count);
            },
            Cmd::CallFunc3 { arg_count } => {
                write!(arg_count);
            },
            Cmd::If { loc } => {
                write!(loc);
            },
            Cmd::IfNot { loc } => {
                write!(loc);
            },
            Cmd::Else { loc } => {
                write!(loc);
            },
            Cmd::IntToFloat { stack_pos } => {
                write!(stack_pos);
            },
            Cmd::FloatToInt { stack_pos } => {
                write!(stack_pos);
            },
            Cmd::IncF { var_type, var_num } => {
                write!(var_type);
                write!(var_num);
            },
            Cmd::DecF { var_type, var_num } => {
                write!(var_type);
                write!(var_num);
            },
            Cmd::VarSetF { var_type, var_num } => {
                write!(var_type);
                write!(var_num);
            },
            Cmd::AddVarByF { var_type, var_num } => {
                write!(var_type);
                write!(var_num);
            },
            Cmd::SubVarByF { var_type, var_num } => {
                write!(var_type);
                write!(var_num);
            },
            Cmd::MultVarByF { var_type, var_num } => {
                write!(var_type);
                write!(var_num);
            },
            Cmd::DivVarByF { var_type, var_num } => {
                write!(var_type);
                write!(var_num);
            },
            _ => {}
        }
    }
}

impl Cmd {
    pub fn value(&self) -> u8 {
        match self {
            Cmd::Nop => 0,
            Cmd::Unk1 => 1,
            Cmd::Begin { arg_count, var_count } => 2,
            Cmd::End => 3,
            Cmd::Jump { loc } => 4,
            Cmd::Jump5 { loc } => 5,
            Cmd::Return6 => 6,
            Cmd::Return7 => 7,
            Cmd::Return8 => 8,
            Cmd::Return9 => 9,
            Cmd::PushInt { val } => 0xA,
            Cmd::PushVar { var_type, var_num } => 0xB,
            Cmd::ErrorC => 0xC,
            Cmd::PushShort { val } => 0xD,
            Cmd::AddI => 0xE,
            Cmd::SubI => 0xF,
            Cmd::MultI => 0x10,
            Cmd::DivI => 0x11,
            Cmd::ModI => 0x12,
            Cmd::NegI => 0x13,
            Cmd::IncI { var_type, var_num } => 0x14,
            Cmd::DecI { var_type, var_num } => 0x15,
            Cmd::AndI => 0x16,
            Cmd::OrI => 0x17,
            Cmd::NotI => 0x18,
            Cmd::XorI => 0x19,
            Cmd::ShiftL => 0x1A,
            Cmd::ShiftR => 0x1B,
            Cmd::SetVar { var_type, var_num } => 0x1C,
            Cmd::AddVarBy { var_type, var_num } => 0x1D,
            Cmd::SubVarBy { var_type, var_num } => 0x1E,
            Cmd::MultVarBy { var_type, var_num } => 0x1F,
            Cmd::DivVarBy { var_type, var_num } => 0x20,
            Cmd::ModVarBy { var_type, var_num } => 0x21,
            Cmd::AndVarBy { var_type, var_num } => 0x22,
            Cmd::OrVarBy { var_type, var_num } => 0x23,
            Cmd::XorVarBy { var_type, var_num } => 0x24,
            Cmd::Equals => 0x25,
            Cmd::NotEquals => 0x26,
            Cmd::LessThan => 0x27,
            Cmd::LessOrEqual => 0x28,
            Cmd::Greater => 0x29,
            Cmd::GreaterOrEqual => 0x2A,
            Cmd::Not => 0x2B,
            Cmd::PrintF { arg_count } => 0x2C,
            Cmd::Sys { arg_count, sys_num } => 0x2D,
            Cmd::Try { loc } => 0x2E,
            Cmd::CallFunc { arg_count } => 0x2F,
            Cmd::CallFunc2 { arg_count } => 0x2F,
            Cmd::CallFunc3 { arg_count } => 0x2F,
            Cmd::Push => 0x32,
            Cmd::Pop => 0x33,
            Cmd::If { loc  } => 0x34,
            Cmd::IfNot { loc } => 0x35,
            Cmd::Else { loc } => 0x36,
            Cmd::Error37 => 0x37,
            Cmd::IntToFloat { stack_pos } => 0x38,
            Cmd::FloatToInt { stack_pos } => 0x39,
            Cmd::AddF => 0x3A,
            Cmd::SubF => 0x3B,
            Cmd::MultF => 0x3C,
            Cmd::DivF => 0x3D,
            Cmd::NegF => 0x3E,
            Cmd::IncF { var_type, var_num } => 0x3F,
            Cmd::DecF { var_type, var_num } => 0x40,
            Cmd::VarSetF { var_type, var_num } => 0x41,
            Cmd::AddVarByF { var_type, var_num } => 0x42,
            Cmd::SubVarByF { var_type, var_num } => 0x43,
            Cmd::MultVarByF { var_type, var_num } => 0x44,
            Cmd::DivVarByF { var_type, var_num } => 0x45,
            Cmd::EqualsF => 0x46,
            Cmd::NotEqualsF => 0x47,
            Cmd::LessThanF => 0x48,
            Cmd::LessOrEqualF => 0x49,
            Cmd::GreaterF => 0x4A,
            Cmd::GreaterOrEqualF => 0x4B,
            Cmd::Error4C => 0x4C,
            Cmd::Exit => 0x4D,
            _ => panic!("Cannot cast Cmd {:?} to u8", self),
        }
    }
}

// WriteImpl trait for ezpz clean file writing
trait WriteImpl {
    fn write(self, f: &mut Vec<u8>, endian: bool);
}

impl WriteImpl for u32 {
    fn write(self, f: &mut Vec<u8>, endian: bool) {
        if endian {
            f.write_u32::<BigEndian>(self).unwrap();
        } else {
            f.write_u32::<LittleEndian>(self).unwrap();
        }
    }
}

impl WriteImpl for u16 {
    fn write(self, f: &mut Vec<u8>, endian: bool) {
        if endian {
            f.write_u16::<BigEndian>(self).unwrap();
        } else {
            f.write_u16::<LittleEndian>(self).unwrap();
        }
    }
}

impl WriteImpl for u8 {
    fn write(self, f: &mut Vec<u8>, endian: bool) {
        f.push(self);
    }
}

impl WriteImpl for &[u8] {
    fn write(self, f: &mut Vec<u8>, endian: bool) {
        f.extend_from_slice(self);
    }
}

impl WriteImpl for &str {
    fn write(self, f: &mut Vec<u8>, endian: bool) {
        f.extend_from_slice(self.as_bytes());
    }
}

impl<T> WriteImpl for Vec<T> where T: WriteImpl + Copy, {
    fn write(self, f: &mut Vec<u8>, endian: bool) {
        for i in self {
            WriteImpl::write(i, f, endian);
        }
    }
}

impl<T> WriteImpl for &mut Iterator<Item=T> where T: WriteImpl, {
    fn write(self, f: &mut Vec<u8>, endian: bool) {
        loop {
            match self.next() {
                Some(b) => WriteImpl::write(b, f, endian),
                None => break
            }
        }
    }
}

impl<T> WriteImpl for std::slice::Iter<'_, T> where T: WriteImpl + Clone, {
    fn write(self, f: &mut Vec<u8>, endian: bool) {
        for i in self {
            WriteImpl::write(i.clone(), f, endian);
        }        
    }
}

impl<T, T2> WriteImpl for (T, T2)
    where T: WriteImpl, T2: WriteImpl {
    fn write(self, f: &mut Vec<u8>, endian: bool) {
        WriteImpl::write(self.0, f, endian);
        WriteImpl::write(self.1, f, endian);
    }
}
