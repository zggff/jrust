use crate::source::ByteStream;

#[derive(Debug, Clone)]
#[repr(u8)]
pub enum OpCode {
    IConstM1 = 0x2, // push -1 onto stack
    IConst0 = 0x3,  // push 0 onto stack
    IConst1 = 0x4,  // push 1 onto stack
    IConst2 = 0x5,  // push 2 onto stack
    IConst3 = 0x6,  // push 3 onto stack
    IConst4 = 0x7,  // push 4 onto stack
    IConst5 = 0x8,  // push 5 onto stack

    BiPush(isize) = 0x10, // push byte
    SiPush(isize) = 0x11, // push short
    Ldc(usize) = 0x12, // push constant pool index onto stack

    ILoad0 = 0x1a, // load int from local
    ILoad1 = 0x1b, // load int from local
    ILoad2 = 0x1c, // load int from local
    ILoad3 = 0x1d, // load int from local


    ALoad0 = 0x2a, // load reference from local
    ALoad1 = 0x2b, // load reference from local
    ALoad2 = 0x2c, // load reference from local
    ALoad3 = 0x2d, // load reference from local

    IStore0 = 0x3b, // store int into local
    IStore1 = 0x3c, // store int into local
    IStore2 = 0x3d, // store int into local
    IStore3 = 0x3e, // store int into local


    AStore0 = 0x4b, // load reference into local
    AStore1 = 0x4c, // load reference into local
    AStore2 = 0x4d, // load reference into local
    AStore3 = 0x4e, // load reference into local

    Dup = 0x59, // duplicate top of stack

    IAdd = 0x60,
    ISub = 0x64,
    IMul = 0x68,

    Iinc(usize, isize) = 0x84,
     
    IReturn = 0xac,

    Return = 0xb1, // return void
    GetStatic(usize) = 0xb2,
    InvokeVirtual(usize) = 0xb6,
    InvokeSpecial(usize) = 0xb7,
    InvokeStatic(usize) = 0xb8,
    New(usize) = 0xbb, // create new object

    IfEq(isize) = 0x99,
    IfNe(isize) = 0x9a,
    IfLt(isize) = 0x9b,
    IfGe(isize) = 0x9c,
    IfGt(isize) = 0x9d,
    IfLe(isize) = 0x9e,
    IfICmpEq(isize) = 0x9f,
    IfICmpNe(isize) = 0xa0,
    IfICmpLt(isize) = 0xa1,
    IfICmpGe(isize) = 0xa2,
    IfICmpGt(isize) = 0xa3,
    IfICmpLe(isize) = 0xa4,

    Goto(isize) = 0xa7

}

impl OpCode {
    pub fn parse(c: &mut ByteStream) -> Option<OpCode> {
        let opcode = match c.next()? {
            0x2 => OpCode::IConstM1,
            0x3 => OpCode::IConst0,
            0x4 => OpCode::IConst1,
            0x5 => OpCode::IConst2,
            0x6 => OpCode::IConst3,
            0x7 => OpCode::IConst4,
            0x8 => OpCode::IConst5,

            0x10 => OpCode::BiPush(c.next_u1()? as i8 as isize),
            0x11 => OpCode::SiPush(c.next_u2()? as i16 as isize),
            0x12 => OpCode::Ldc(c.next_u1()? as usize),

            0x1a => OpCode::ILoad0,
            0x1b => OpCode::ILoad1,
            0x1c => OpCode::ILoad2,
            0x1d => OpCode::ILoad3,


            0x2a => OpCode::ALoad0,
            0x2b => OpCode::ALoad1,
            0x2c => OpCode::ALoad2,
            0x2d => OpCode::ALoad3,

            0x3b => OpCode::IStore0,
            0x3c => OpCode::IStore1,
            0x3d => OpCode::IStore2,
            0x3e => OpCode::IStore3,

            0x4b => OpCode::AStore0,
            0x4c => OpCode::AStore0,
            0x4d => OpCode::AStore0,
            0x4e => OpCode::AStore0,

            0x59 => OpCode::Dup,

            0x60 => OpCode::IAdd,
            0x64 => OpCode::ISub,
            0x68 => OpCode::IMul,

            0x84 => OpCode::Iinc(c.next_u1()? as usize, c.next_u1()? as i8 as isize),

            0xac => OpCode::IReturn,


            0xb1 => OpCode::Return,
            0xb2 => OpCode::GetStatic(c.next_u2()?),
            0xb6 => OpCode::InvokeVirtual(c.next_u2()?),
            0xb7 => OpCode::InvokeSpecial(c.next_u2()?),
            0xb8 => OpCode::InvokeStatic(c.next_u2()?),
            0xbb => OpCode::New(c.next_u2()?),

            // 0x99 => OpCode::IfEq(c.next_u1()? as usize, c.next_u1()? as usize),
            0x99 => OpCode::IfEq((c.next_u2()?) as i16 as isize),
            0x9a => OpCode::IfNe((c.next_u2()?) as i16 as isize),
            0x9b => OpCode::IfLt((c.next_u2()?) as i16 as isize),
            0x9c => OpCode::IfGe((c.next_u2()?) as i16 as isize),
            0x9d => OpCode::IfGt((c.next_u2()?) as i16 as isize),
            0x9e => OpCode::IfLe((c.next_u2()?) as i16 as isize),


            0x9f => OpCode::IfICmpEq((c.next_u2()?) as i16 as isize),
            0xa0 => OpCode::IfICmpNe((c.next_u2()?) as i16 as isize),
            0xa1 => OpCode::IfICmpLt((c.next_u2()?) as i16 as isize),
            0xa2 => OpCode::IfICmpGe((c.next_u2()?) as i16 as isize),
            0xa3 => OpCode::IfICmpGt((c.next_u2()?) as i16 as isize),
            0xa4 => OpCode::IfICmpLe((c.next_u2()?) as i16 as isize),

            0xa7 => OpCode::Goto((c.next_u2()?) as i16 as isize),

            op => todo!("opcode not implemented: 0x{op:0X}"),
        };
        Some(opcode)
    }
}
