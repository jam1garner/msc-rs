use nom::{be_u8, be_u16, be_u32, le_u32, IResult};
use super::MscsbFile;
use super::super::{Cmd, Command, Script};

fn get_nom_position(input: &[u8], input_size: usize) -> IResult<&[u8], usize> {
    let remaining = input.len();
    do_parse!(input, (input_size - remaining))
}

fn take_script(input: &[u8], position: usize) -> IResult<&[u8], Script> {
    do_parse!(
        input,
        commands: many0!(complete!(
                do_parse!(
                    pos: apply!(get_nom_position, input.len()) >>
                    cmd: apply!(take_cmd, pos) >>
                    (cmd)
                ))) >>
        size: apply!(get_nom_position, input.len()) >>
        (Script {
            bounds: (position as u32, (position + size) as u32),
            commands
        })
    )
}

pub fn str_from_u8_nul_utf8(utf8_src: &[u8]) -> Result<&str, std::str::Utf8Error> {
    let nul_range_end = utf8_src.iter()
        .position(|&c| c == b'\0')
        .unwrap_or(utf8_src.len()); // default to length if no `\0` present
    ::std::str::from_utf8(&utf8_src[0..nul_range_end])
}

pub fn take_file(input: &[u8]) -> IResult<&[u8], MscsbFile> {
    do_parse!(
        input,
        tag!(b"\xB2\xAC\xBC\xBA\xE6\x90\x32\x01\xFD\x02\x00\x00\x00\x00\x00\x00") >>
        script_data_size: le_u32 >>
        entrypoint: le_u32 >>
        script_count: le_u32 >>
        _unk: le_u32 >>
        string_size: tap!(string_size: le_u32 => {println!("string size- {}", string_size)}) >>
        string_count: le_u32 >>
        _padding: take!(8) >>
        script_data: take!(script_data_size) >>
        _padding: take!((0x10 - (script_data_size & 0xF)) & 0xF) >> // pad to 0x10
        script_offsets: count!(le_u32, script_count as usize) >>
        _padding: take!((0x10 - ((script_count * 4) & 0xF)) & 0xF) >> // pad to 0x10
        strings: count!(take!(string_size), string_count as usize) >>
        ({
            let mut script_offsets = script_offsets.clone();
            script_offsets.sort();
            script_offsets.push(script_data_size);
            let scripts =
                (0..script_offsets.len() - 1)
                .filter_map(|i| {
                    Some(do_parse!(
                        &script_data[script_offsets[i] as usize..script_offsets[i+1] as usize],
                        script: complete!(apply!(take_script, script_offsets[i] as usize)) >>
                        (script)
                    ).unwrap().1)
                })
                .collect();
            let strings =
                strings
                .iter()
                .map(|s|
                    String::from(
                        str_from_u8_nul_utf8(s).unwrap_or("[UTF-8 Error]")
                )).collect();
            MscsbFile {
                scripts,
                strings,
                entrypoint
            }
        })
    )
}

fn take_cmd(input: &[u8], position: usize) -> IResult<&[u8], Command> {
    do_parse!(
        input,
        cmd_num: be_u8 >>
        cmd: switch!(value!(cmd_num & 0x7F, take!(0)),
            0 => value!(Cmd::Nop, take!(0)) |
            1 => value!(Cmd::Unk1, take!(0)) |
            2 => do_parse!(
                arg_count: be_u16 >>
                var_count: be_u16 >>
                (Cmd::Begin {
                    arg_count,
                    var_count
                })
            ) |
            3 => value!(Cmd::End, take!(0)) |
            4 => do_parse!(
                loc: be_u32 >>
                (Cmd::Jump {
                    loc
                })
            ) |
            5 => do_parse!(
                loc: be_u32 >>
                (Cmd::Jump5 {
                    loc
                })
            ) |
            6 => value!(Cmd::Return6, take!(0)) |
            7 => value!(Cmd::Return7, take!(0)) |
            8 => value!(Cmd::Return8, take!(0)) |
            9 => value!(Cmd::Return9, take!(0)) |
            0xA => do_parse!(
                val: be_u32 >>
                (Cmd::PushInt {
                    val
                })
            ) |
            0xB => do_parse!(
                var_type: be_u8 >>
                var_num: be_u16 >>
                (Cmd::PushVar {
                    var_type,
                    var_num
                })
            ) |
            0xC => value!(Cmd::ErrorC, take!(0)) |
            0xD => do_parse!(
                val: be_u16 >>
                (Cmd::PushShort {
                    val
                })
            ) |
            0xE => value!(Cmd::AddI, take!(0)) |
            0xF => value!(Cmd::SubI, take!(0)) |
            0x10 => value!(Cmd::MultI, take!(0)) |
            0x11 => value!(Cmd::DivI, take!(0)) |
            0x12 => value!(Cmd::ModI, take!(0)) |
            0x13 => value!(Cmd::NegI, take!(0)) |
            0x14 => do_parse!(
                var_type: be_u8 >>
                var_num: be_u16 >>
                (Cmd::IncI {
                    var_type,
                    var_num
                })
            ) |
            0x15 => do_parse!(
                var_type: be_u8 >>
                var_num: be_u16 >>
                (Cmd::DecI {
                    var_type,
                    var_num
                })
            ) |
            0x16 => value!(Cmd::AndI, take!(0)) |
            0x17 => value!(Cmd::OrI, take!(0)) |
            0x18 => value!(Cmd::NotI, take!(0)) |
            0x19 => value!(Cmd::XorI, take!(0)) |
            0x1A => value!(Cmd::ShiftL, take!(0)) |
            0x1B => value!(Cmd::ShiftR, take!(0)) |
            0x1C => do_parse!(
                var_type: be_u8 >>
                var_num: be_u16 >>
                (Cmd::SetVar {
                    var_type,
                    var_num
                })
            ) |
            0x1D => do_parse!(
                var_type: be_u8 >>
                var_num: be_u16 >>
                (Cmd::AddVarBy {
                    var_type,
                    var_num
                })
            ) |
            0x1E => do_parse!(
                var_type: be_u8 >>
                var_num: be_u16 >>
                (Cmd::SubVarBy {
                    var_type,
                    var_num
                })
            ) |
            0x1F => do_parse!(
                var_type: be_u8 >>
                var_num: be_u16 >>
                (Cmd::MultVarBy {
                    var_type,
                    var_num
                })
            ) |
            0x20 => do_parse!(
                var_type: be_u8 >>
                var_num: be_u16 >>
                (Cmd::DivVarBy {
                    var_type,
                    var_num
                })
            ) |
            0x21 => do_parse!(
                var_type: be_u8 >>
                var_num: be_u16 >>
                (Cmd::ModVarBy {
                    var_type,
                    var_num
                })
            ) |
            0x22 => do_parse!(
                var_type: be_u8 >>
                var_num: be_u16 >>
                (Cmd::AndVarBy {
                    var_type,
                    var_num
                })
            ) |
            0x23 => do_parse!(
                var_type: be_u8 >>
                var_num: be_u16 >>
                (Cmd::OrVarBy {
                    var_type,
                    var_num
                })
            ) |
            0x24 => do_parse!(
                var_type: be_u8 >>
                var_num: be_u16 >>
                (Cmd::XorVarBy {
                    var_type,
                    var_num
                })
            ) |
            0x25 => value!(Cmd::Equals, take!(0)) |
            0x26 => value!(Cmd::NotEquals, take!(0)) |
            0x27 => value!(Cmd::LessThan, take!(0)) |
            0x28 => value!(Cmd::LessOrEqual, take!(0)) |
            0x29 => value!(Cmd::Greater, take!(0)) |
            0x2A => value!(Cmd::GreaterOrEqual, take!(0)) |
            0x2B => value!(Cmd::Not, take!(0)) |
            0x2C => do_parse!(
                arg_count: be_u8 >>
                (Cmd::PrintF {
                    arg_count
                })
            ) |
            0x2D => do_parse!(
                arg_count: be_u8 >>
                sys_num: be_u8 >>
                (Cmd::Sys {
                    arg_count,
                    sys_num
                })
            ) |
            0x2E => do_parse!(
                loc: be_u32 >>
                (Cmd::Try {
                    loc
                })
            ) |
            0x2F => do_parse!(
                arg_count: be_u8 >>
                (Cmd::CallFunc {
                    arg_count
                })
            ) |
            0x30 => do_parse!(
                arg_count: be_u8 >>
                (Cmd::CallFunc2 {
                    arg_count
                })
            ) |
            0x31 => do_parse!(
                arg_count: be_u8 >>
                (Cmd::CallFunc3 {
                    arg_count
                })
            ) |
            0x32 => value!(Cmd::Push, take!(0)) |
            0x33 => value!(Cmd::Pop, take!(0)) |
            0x34 => do_parse!(
                loc: be_u32 >>
                (Cmd::If {
                    loc
                })
            ) |
            0x35 => do_parse!(
                loc: be_u32 >>
                (Cmd::IfNot {
                    loc
                })
            ) |
            0x36 => do_parse!(
                loc: be_u32 >>
                (Cmd::Else {
                    loc
                })
            ) |
            0x37 => value!(Cmd::Error37, take!(0)) |
            0x38 => do_parse!(
                stack_pos: be_u8 >>
                (Cmd::IntToFloat {
                    stack_pos
                })
            ) |
            0x39 => do_parse!(
                stack_pos: be_u8 >>
                (Cmd::FloatToInt {
                    stack_pos
                })
            ) |
            0x3A => value!(Cmd::AddF, take!(0)) |
            0x3B => value!(Cmd::SubF, take!(0)) |
            0x3C => value!(Cmd::MultF, take!(0)) |
            0x3D => value!(Cmd::DivF, take!(0)) |
            0x3E => value!(Cmd::NegF, take!(0)) |
            0x3F => do_parse!(
                var_type: be_u8 >>
                var_num: be_u16 >>
                (Cmd::IncF {
                    var_type,
                    var_num
                })
            ) |
            0x40 => do_parse!(
                var_type: be_u8 >>
                var_num: be_u16 >>
                (Cmd::DecF {
                    var_type,
                    var_num
                })
            ) |
            0x41 => do_parse!(
                var_type: be_u8 >>
                var_num: be_u16 >>
                (Cmd::VarSetF {
                    var_type,
                    var_num
                })
            ) |
            0x42 => do_parse!(
                var_type: be_u8 >>
                var_num: be_u16 >>
                (Cmd::AddVarByF {
                    var_type,
                    var_num
                })
            ) |
            0x43 => do_parse!(
                var_type: be_u8 >>
                var_num: be_u16 >>
                (Cmd::SubVarByF {
                    var_type,
                    var_num
                })
            ) |
            0x44 => do_parse!(
                var_type: be_u8 >>
                var_num: be_u16 >>
                (Cmd::MultVarByF {
                    var_type,
                    var_num
                })
            ) |
            0x45 => do_parse!(
                var_type: be_u8 >>
                var_num: be_u16 >>
                (Cmd::DivVarByF {
                    var_type,
                    var_num
                })
            ) |
            0x46 => value!(Cmd::Equals, take!(0)) |
            0x47 => value!(Cmd::NotEquals, take!(0)) |
            0x48 => value!(Cmd::LessThan, take!(0)) |
            0x49 => value!(Cmd::LessOrEqual, take!(0)) |
            0x4A => value!(Cmd::Greater, take!(0)) |
            0x4B => value!(Cmd::GreaterOrEqual, take!(0)) |
            0x4C => value!(Cmd::Error4C, take!(0)) |
            0x4D => value!(Cmd::Exit, take!(0))
        ) >>
        ({
            Command {
                position: position as u32,
                push_bit: cmd_num & 0x80 == 0x80,
                cmd
            }
        })
    )
}
