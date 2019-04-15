use nom::{be_u8, be_u16, be_u32, le_u32, IResult};
use super::MscbFile;
use super::super::{Cmd, Command, Script};

fn get_nom_position(input: &[u8], input_size: usize) -> IResult<&[u8], usize> {
    let remaining = input.len();
    do_parse!(input, (input_size - remaining))
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
            )
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

pub fn take_file(input: &[u8]) -> IResult<&[u8], MscbFile> {
    do_parse!(
        input,
        tag!(b"\xB2\xAC\xBC\xBA\xE6\x90\x32\x01\xFD\x02\x00\x00\x00\x00\x00\x00") >>
        script_data_size: le_u32 >>
        entrypoint: le_u32 >>
        script_count: le_u32 >>
        _unk: le_u32 >>
        string_size: le_u32 >>
        string_count: le_u32 >>
        _padding: take!(8) >>
        script_data: take!(script_data_size) >>
        _padding: take!((0x10 - (script_data_size & 0xF)) & 0xF) >> // pad to 0x10
        script_offsets: count!(le_u32, script_count as usize) >>
        _padding: take!((0x10 - (script_data_size & 0xF)) & 0xF) >> // pad to 0x10
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
                    ).ok()?.1)
                })
                .collect();
            let strings =
                strings
                .iter()
                .map(|s|
                    String::from(
                        str_from_u8_nul_utf8(s).unwrap_or("[UTF-8 Error]")
                )).collect();
            MscbFile {
                scripts,
                strings,
                entrypoint
            }
        })
    )
}
