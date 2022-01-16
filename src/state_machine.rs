use std::collections::VecDeque;

#[derive(Default, Debug)]
pub struct PIOStateMachine {
    osr: u32,
    isr: u32,
    output_shift_counter: u32,
    input_shift_counter: u32,
    x: u32,
    y: u32,
    tx_fifo: VecDeque<u32>,
    rx_fifo: VecDeque<u32>,
    pc: u32,
    clock_divider: u32,
}

impl PIOStateMachine {
    pub fn get_pc(&self) -> u32 {
        self.pc
    }

    pub fn set_pc(&mut self, new_pc: u32) -> Result<(), std::string::String> {
        if new_pc > 31 {
            return Err(format!("Invalid new PC provided : {}", new_pc));
        }
        self.pc = new_pc;
        Ok(())
    }

    pub fn inc_pc(&mut self) -> Result<(), std::string::String> {
        self.pc = (self.pc + 1) % 32;
        Ok(())
    }

    pub fn get_scratch_x(&self) -> u32 {
        self.x
    }

    pub fn decrement_x(&mut self) -> Result<(), std::string::String> {
        self.x -= 1;
        Ok(())
    }

    pub fn get_scratch_y(&self) -> u32 {
        self.y
    }

    pub fn decrement_y(&mut self) -> Result<(), std::string::String> {
        self.y -= 1;
        Ok(())
    }

    pub fn rx_fifo_full(&self) -> bool {
        if self.rx_fifo.len() >= 4 {
            true
        } else {
            false
        }
    }

    pub fn get_isr(&self) -> u32 {
        self.isr
    }

    pub fn push_to_rx_fifo(&mut self, value: u32) -> Result<(), std::string::String> {
        // TODO I don't actually know what happens when you push to a full FIFO for the PIO, this is an assumption to check
        if self.rx_fifo_full() {
            return Err("RX FIFO is full!".to_string());
        }

        self.rx_fifo.push_back(value);
        Ok(())
    }

    pub fn clear_isr(&mut self) -> Result<(), std::string::String> {
        self.isr = 0;
        Ok(())
    }

    pub fn tx_fifo_empty(&self) -> bool {
        self.tx_fifo.is_empty()
    }

    pub fn pop_from_tx_fifo(&mut self) -> Result<u32, std::string::String> {
        if let Some(val) = self.tx_fifo.pop_front() {
            Ok(val)
        } else {
            Err("Tried to pop data from an empty tx_fifo".to_string())
        }
    }

    pub fn set_osr(&mut self, value: u32) -> Result<(), std::string::String> {
        self.osr = value;
        Ok(())
    }

    pub fn get_osr_counter(&self) -> u32 {
        self.output_shift_counter
    }

    pub fn get_isr_counter(&self) -> u32 {
        self.input_shift_counter
    }

    pub fn set_scratch_x(&mut self, value: u32) -> Result<(), std::string::String> {
        self.x = value;
        Ok(())
    }

    pub fn set_scratch_y(&mut self, value: u32) -> Result<(), std::string::String> {
        self.y = value;
        Ok(())
    }
}
