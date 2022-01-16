use std::convert::TryFrom;

#[derive(Debug)]
pub enum PIOInstruction {
    JMP {
        delay_sideset: u8,
        condition: JmpCondition,
        address: u8,
    },
    WAIT {
        delay_sideset: u8,
        polarity: WaitPolarity,
        source: WaitSource,
        index: u8,
    },
    IN {
        delay_sideset: u8,
        source: InSource,
        bit_count: u8,
    },
    OUT {
        delay_sideset: u8,
        destination: OutDestination,
        bit_count: u8,
    },
    PUSH {
        delay_sideset: u8,
        if_full: bool,
        block: bool,
    },
    PULL {
        delay_sideset: u8,
        if_empty: bool,
        block: bool,
    },
    MOV {
        delay_sideset: u8,
        destination: MovDestination,
        op: MovOp,
        source: MovSource,
    },
    IRQ {
        delay_sideset: u8,
        clear: bool,
        wait: bool,
        index: u8,
    },
    SET {
        delay_sideset: u8,
        destination: SetDestination,
        data: u8,
    },
}

impl PIOInstruction {
    pub fn decode(instr_data: u32) -> Result<Self, std::string::String> {
        println!("Decoding : 0x{:X}", instr_data);

        let op_code = (instr_data >> 13) & 7;
        let delay_sideset = ((instr_data >> 8) & 0x1f) as u8;

        match op_code {
            0 => {
                let address = (instr_data & 0x1f) as u8;
                let condition = JmpCondition::try_from(((instr_data >> 5) & 7) as u8)?;
                Ok(Self::JMP {
                    delay_sideset,
                    condition,
                    address,
                })
            }
            1 => {
                let polarity = WaitPolarity::try_from(((instr_data >> 7) & 1) as u8)?;
                let source = WaitSource::try_from(((instr_data >> 5) & 3) as u8)?;
                let index = (instr_data & 0x1f) as u8;
                Ok(Self::WAIT {
                    delay_sideset,
                    polarity,
                    source,
                    index,
                })
            }
            2 => {
                let source = InSource::try_from(((instr_data >> 5) & 7) as u8)?;
                let bit_count = (instr_data & 0x1f) as u8;
                Ok(Self::IN {
                    delay_sideset,
                    source,
                    bit_count,
                })
            }
            3 => {
                let destination = OutDestination::try_from(((instr_data >> 5) & 7) as u8)?;
                let bit_count = (instr_data & 0x1f) as u8;
                Ok(Self::OUT {
                    delay_sideset,
                    destination,
                    bit_count,
                })
            }
            4 => {
                if (instr_data >> 7) & 1 == 1 {
                    let if_empty = (instr_data >> 6) & 1 == 1;
                    let block = (instr_data >> 5) & 1 == 1;
                    Ok(Self::PULL {
                        delay_sideset,
                        if_empty,
                        block,
                    })
                } else {
                    let if_full = (instr_data >> 6) & 1 == 1;
                    let block = (instr_data >> 5) & 1 == 1;
                    Ok(Self::PUSH {
                        delay_sideset,
                        if_full,
                        block,
                    })
                }
            }
            5 => {
                let destination = MovDestination::try_from(((instr_data >> 5) & 7) as u8)?;
                let op = MovOp::try_from(((instr_data >> 3) & 3) as u8)?;
                let source = MovSource::try_from(((instr_data) & 7) as u8)?;
                Ok(Self::MOV {
                    delay_sideset,
                    destination,
                    op,
                    source,
                })
            }
            6 => {
                let clear = (instr_data >> 6) & 1 == 1;
                let wait = (instr_data >> 5) & 1 == 1;
                let index = (instr_data & 0x1f) as u8;
                Ok(Self::IRQ {
                    delay_sideset,
                    clear,
                    wait,
                    index,
                })
            }
            7 => {
                let destination = SetDestination::try_from(((instr_data >> 5) & 7) as u8)?;
                let data = (instr_data & 0x1f) as u8;
                Ok(Self::SET {
                    delay_sideset,
                    destination,
                    data,
                })
            }
            _ => Err(format!("Invalid instruction op code : {}", op_code)),
        }
    }
}

#[repr(u8)]
#[derive(Debug)]
pub enum JmpCondition {
    Always = 0,
    ScratchXZero = 1,
    ScratchXNonZeroPostDecrement = 2,
    ScratchYZero = 3,
    ScratchYNonZeroPostDecrement = 4,
    ScratchXNotEqualScratchY = 5,
    BranchOnInputPin = 6,
    OutputShiftRegisterNotEmpty = 7,
}

impl TryFrom<u8> for JmpCondition {
    type Error = std::string::String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Always),
            1 => Ok(Self::ScratchXZero),
            2 => Ok(Self::ScratchXNonZeroPostDecrement),
            3 => Ok(Self::ScratchYZero),
            4 => Ok(Self::ScratchYNonZeroPostDecrement),
            5 => Ok(Self::ScratchXNotEqualScratchY),
            6 => Ok(Self::BranchOnInputPin),
            7 => Ok(Self::OutputShiftRegisterNotEmpty),
            _ => Err(format!("Invalid JmpCondition : {}", value)),
        }
    }
}

#[repr(u8)]
#[derive(Debug)]
pub enum WaitPolarity {
    Zero = 0,
    One = 1,
}

impl TryFrom<u8> for WaitPolarity {
    type Error = std::string::String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Zero),
            1 => Ok(Self::One),
            _ => Err(format!("Invalid WaitPolarity : {}", value)),
        }
    }
}

#[repr(u8)]
#[derive(Debug)]
pub enum WaitSource {
    GPIO = 0,
    Pin = 1,
    IRQ = 2,
    Reserved = 3,
}

impl TryFrom<u8> for WaitSource {
    type Error = std::string::String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::GPIO),
            1 => Ok(Self::Pin),
            2 => Ok(Self::IRQ),
            3 => Ok(Self::Reserved),
            _ => Err(format!("Invalid WaitSource : {}", value)),
        }
    }
}

#[repr(u8)]
#[derive(Debug)]
pub enum InSource {
    PINS = 0,
    X = 1,
    Y = 2,
    NULL = 3,
    Reserved0 = 4,
    Reserved1 = 5,
    ISR = 6,
    OSR = 7,
}

impl TryFrom<u8> for InSource {
    type Error = std::string::String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::PINS),
            1 => Ok(Self::X),
            2 => Ok(Self::Y),
            3 => Ok(Self::NULL),
            4 => Ok(Self::Reserved0),
            5 => Ok(Self::Reserved1),
            6 => Ok(Self::ISR),
            7 => Ok(Self::OSR),
            _ => Err(format!("Invalid InSource : {}", value)),
        }
    }
}

#[repr(u8)]
#[derive(Debug)]
pub enum OutDestination {
    PINS = 0,
    X = 1,
    Y = 2,
    NULL = 3,
    PINDIRS = 4,
    PC = 5,
    ISR = 6,
    EXEC = 7,
}

impl TryFrom<u8> for OutDestination {
    type Error = std::string::String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::PINS),
            1 => Ok(Self::X),
            2 => Ok(Self::Y),
            3 => Ok(Self::NULL),
            4 => Ok(Self::PINDIRS),
            5 => Ok(Self::PC),
            6 => Ok(Self::ISR),
            7 => Ok(Self::EXEC),
            _ => Err(format!("Invalid OutDestination : {}", value)),
        }
    }
}

#[repr(u8)]
#[derive(Debug)]
pub enum MovDestination {
    PINS = 0,
    X = 1,
    Y = 2,
    Reserved = 3,
    EXEC = 4,
    PC = 5,
    ISR = 6,
    OSR = 7,
}

impl TryFrom<u8> for MovDestination {
    type Error = std::string::String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::PINS),
            1 => Ok(Self::X),
            2 => Ok(Self::Y),
            3 => Ok(Self::Reserved),
            4 => Ok(Self::EXEC),
            5 => Ok(Self::PC),
            6 => Ok(Self::ISR),
            7 => Ok(Self::OSR),
            _ => Err(format!("Invalid MovDestination : {}", value)),
        }
    }
}

#[repr(u8)]
#[derive(Debug)]
pub enum MovOp {
    None = 0,
    Invert = 1,
    BitReverse = 2,
    Reserved = 4,
}

impl TryFrom<u8> for MovOp {
    type Error = std::string::String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Invert),
            2 => Ok(Self::BitReverse),
            3 => Ok(Self::Reserved),
            _ => Err(format!("Invalid MovOp : {}", value)),
        }
    }
}

#[repr(u8)]
#[derive(Debug)]
pub enum MovSource {
    PINS = 0,
    X = 1,
    Y = 2,
    NULL = 3,
    Reserved = 4,
    STATUS = 5,
    ISR = 6,
    EXEC = 7,
}

impl TryFrom<u8> for MovSource {
    type Error = std::string::String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::PINS),
            1 => Ok(Self::X),
            2 => Ok(Self::Y),
            3 => Ok(Self::NULL),
            4 => Ok(Self::Reserved),
            5 => Ok(Self::STATUS),
            6 => Ok(Self::ISR),
            7 => Ok(Self::EXEC),
            _ => Err(format!("Invalid MovSource : {}", value)),
        }
    }
}

#[repr(u8)]
#[derive(Debug)]
pub enum SetDestination {
    PINS = 0,
    X = 1,
    Y = 2,
    Reserved0 = 3,
    PINDIRS = 4,
    Reserved1 = 5,
    Reserved2 = 6,
    Reserved3 = 7,
}

impl TryFrom<u8> for SetDestination {
    type Error = std::string::String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::PINS),
            1 => Ok(Self::X),
            2 => Ok(Self::Y),
            3 => Ok(Self::Reserved0),
            4 => Ok(Self::PINDIRS),
            5 => Ok(Self::Reserved1),
            6 => Ok(Self::Reserved2),
            7 => Ok(Self::Reserved3),
            _ => Err(format!("Invalid SetDestination : {}", value)),
        }
    }
}
