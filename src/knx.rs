use nom::{
    number::complete::{be_u16, be_u8},
};


#[derive(Debug)]
pub struct Frame {
    control: u8,
}


#[derive(Debug, PartialEq)]
pub enum FrameType {
    DataExtended,
    DataStandard,
    PollData,
}

pub fn get_frame_type(f: &Frame) -> Result<FrameType, String> {
    let b7 = f.control & 0b10000000;
    let b6 = f.control & 0b01000000;
    if b6 != 0 {
        if b7 != 0 {
            return Ok(FrameType::PollData);
        }
    } else {
        return Ok(if b7 != 0 { FrameType::DataStandard } else { FrameType::DataExtended });
    }
    return Err(format!("Invalid frame control field: {:08b}", f.control));
}

#[derive(Debug)]
pub struct Telegram {
    control: u8,
    source_address: Address,
}

#[derive(Debug)]
pub struct Address {
    type_: AddressType,
    area: u8,
    line: u8,
    bus: u8,
}

#[derive(Debug)]
pub enum AddressType {
    Individual,
    Group,
}


pub(crate) fn parse_tp(buff: &[u8]) -> Telegram {
    return Telegram {
        control: buff[0],
        source_address: Address {
            type_: AddressType::Individual,
            area: 0,
            line: 0,
            bus: 0,
        },
    };
}

#[cfg(test)]
mod tests {
    use crate::knx::{Frame, FrameType, get_frame_type};
    use crate::knx::FrameType::{DataExtended, DataStandard, PollData};

    #[test]
    fn t_control() {
        assert_frame_type(&[0b10000000, 0b10111111], &Ok(DataStandard));
        assert_frame_type(&[0b00000000, 0b00111111], &Ok(DataExtended));
        assert_frame_type(&[0b11000000, 0b11111111], &Ok(PollData));
        assert_frame_type_0(0b01000000, &Err("Invalid frame control field: 01000000".to_string()));
        assert_frame_type_0(0b01111111, &Err("Invalid frame control field: 01111111".to_string()));
    }

    fn assert_frame_type(controls: &[u8], expected: &Result<FrameType, String>) {
        for control in controls {
            assert_frame_type_0(*control, expected);
        }
    }

    fn assert_frame_type_0(control: u8, expected: &Result<FrameType, String>) {
        assert_eq!(*expected, get_frame_type(&Frame { control }))
    }
}