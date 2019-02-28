// 
// msc
//  L Command
//  L Script
//  L MscbFile
//

#[derive(Debug)]
pub enum Cmd {
    Nop, // 0
    Begin(u16, u16), // 2
    End, // 3
    Jump(u32), // 4, 5
    Return(u8), // 6, 7, 8, 9 (store command as the u8)
    PushInt(u32), // 0xA
    PushVar(u8, u16), // 0xB
    ErrorC,
    PushShort(u16), // 0xD
    AddI, // 0xE
    SubI, // 0xF
    MultI,// 0x10
    DivI, // 0x11
    ModI, // 0x12
    NegI, // 0x13
    IncI(u8, u16), // 0x14
    DecI(u8, u16), // 0x15
    AndI, // 0x16
    OrI,  // 0x17
    NotI, // 0x18
    XorI, // 0x19
    ShiftL,//0x1A
    ShiftR,//0x1B
    SetVar(u8, u16),   //0x1C
    AddVarBy(u8, u16), //0x1D
    SubVarBy(u8, u16), //0x1E
    MultVarBy(u8, u16),//0x1F
    DivVarBy(u8, u16), //0x20
    ModVarBy(u8, u16), //0x21
    AndVarBy(u8, u16), //0x22
    OrVarBy(u8, u16),  //0x23
    XorVarBy(u8, u16), //0x24
    Equals,    // 0x25
    NotEquals, // 0x26
    LessThan,  // 0x27
    LessOrEqual,//0x28
    Greater,   // 0x29
    GreaterOrEqual,//0x2A
    Not,           // 0x2B
    PrintF(u8),    // 0x2C
    Sys(u8, u8),   // 0x2D
    Try(u32),      // 0x2E
    CallFunc(u8),  // 0x2F
    CallFunc2(u8), // 0x2F
    CallFunc3(u8), // 0x2F
    Push,    // 0x32
    Pop,     // 0x33
    If(u32),   // 0x34
    IfNot(u32),// 0x35
    Else(u32), // 0x36
    Error37, // 0x37
    IntToFloat(u8), // 0x38
    FloatToInt(u8), // 0x39
    AddF, // 0x3A
    SubF, // 0x3B
    MultF,// 0x3C
    DivF, // 0x3D
    NegF, // 0x3E
    IncF(u8, u16), // 0x3F
    DecF(u8, u16), // 0x40
    VarSetF(u8, u16), // 0x41
    AddVarByF(u8, u16), //0x42
    SubVarByF(u8, u16), //0x43
    MultVarByF(u8, u16),//0x44
    DivVarByF(u8, u16), //0x45
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
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

