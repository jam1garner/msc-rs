//
// msc
//  L Command
//  L Script
//  L MscbFile
//

#[macro_use] extern crate nom;

mod mscb_file;
pub use mscb_file::MscbFile;

#[derive(Debug)]
pub enum Cmd {
    Nop, // 0
    Unk1, // 1
    Begin {
        arg_count: u16,
        var_count: u16
    }, // 2
    End, // 3
    Jump {
        loc: u32
    }, // 4
    Jump5 {
        loc: u32
    }, // 5
    Return6, // 6
    Return7, // 7
    Return8, // 8
    Return9, // 9
    PushInt {
        val: u32,
    }, // 0xA
    PushVar {
        var_type: u8,
        var_num: u16,
    }, // 0xB
    ErrorC,
    PushShort {
        val: u16
    }, // 0xD
    AddI, // 0xE
    SubI, // 0xF
    MultI,// 0x10
    DivI, // 0x11
    ModI, // 0x12
    NegI, // 0x13
    IncI {
        var_type: u8,
        var_num: u16,
    }, // 0x14
    DecI{
        var_type: u8,
        var_num: u16,
    }, // 0x15
    AndI, // 0x16
    OrI,  // 0x17
    NotI, // 0x18
    XorI, // 0x19
    ShiftL,//0x1A
    ShiftR,//0x1B
    SetVar {
        var_type: u8,
        var_num: u16,
    },   //0x1C
    AddVarBy{
        var_type: u8,
        var_num: u16,
    }, //0x1D
    SubVarBy{
        var_type: u8,
        var_num: u16,
    }, //0x1E
    MultVarBy{
        var_type: u8,
        var_num: u16,
    },//0x1F
    DivVarBy{
        var_type: u8,
        var_num: u16,
    }, //0x20
    ModVarBy{
        var_type: u8,
        var_num: u16,
    }, //0x21
    AndVarBy{
        var_type: u8,
        var_num: u16,
    }, //0x22
    OrVarBy{
        var_type: u8,
        var_num: u16,
    },  //0x23
    XorVarBy{
        var_type: u8,
        var_num: u16,
    }, //0x24
    Equals,    // 0x25
    NotEquals, // 0x26
    LessThan,  // 0x27
    LessOrEqual,//0x28
    Greater,   // 0x29
    GreaterOrEqual,//0x2A
    Not,           // 0x2B
    PrintF {
        arg_count: u8
    },    // 0x2C
    Sys {
        arg_count: u8,
        sys_num: u8,
    },   // 0x2D
    Try {
        loc: u32,
    },      // 0x2E
    CallFunc {
        arg_count: u8,
    },  // 0x2F
    CallFunc2 {
        arg_count: u8,
    }, // 0x2F
    CallFunc3 {
        arg_count: u8,
    }, // 0x2F
    Push,    // 0x32
    Pop,     // 0x33
    If {
        loc: u32,
    },   // 0x34
    IfNot {
        loc: u32,
    },// 0x35
    Else {
        loc: u32,
    }, // 0x36
    Error37, // 0x37
    IntToFloat {
        stack_pos: u8,
    }, // 0x38
    FloatToInt {
        stack_pos: u8,
    }, // 0x39
    AddF, // 0x3A
    SubF, // 0x3B
    MultF,// 0x3C
    DivF, // 0x3D
    NegF, // 0x3E
    IncF{
        var_type: u8,
        var_num: u16,
    }, // 0x3F
    DecF{
        var_type: u8,
        var_num: u16,
    }, // 0x40
    VarSetF{
        var_type: u8,
        var_num: u16,
    }, // 0x41
    AddVarByF{
        var_type: u8,
        var_num: u16,
    }, //0x42
    SubVarByF{
        var_type: u8,
        var_num: u16,
    }, //0x43
    MultVarByF{
        var_type: u8,
        var_num: u16,
    },//0x44
    DivVarByF{
        var_type: u8,
        var_num: u16,
    }, //0x45
    EqualsF,    // 0x46
    NotEqualsF, // 0x47
    LessThanF,  // 0x48
    LessOrEqualF,//0x49
    GreaterF,   // 0x4A
    GreaterOrEqualF,//0x4B
    Error4C, // 0x4C
    Exit, // 0x4D
}

pub struct Command {
    cmd: Cmd,
    push_bit: bool,
    position: u32,
}

pub struct Script {
    commands: Vec<Command>,
    bounds: (u32, u32),
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

