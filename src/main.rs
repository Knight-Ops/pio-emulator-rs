use std::cell::RefCell;
use std::rc::Rc;

mod instructions;
use instructions::*;
mod memory_backing;
use memory_backing::*;
mod gpio;
mod state_machine;
use tock_registers::interfaces::Readable;

#[derive(Debug, Default)]
pub struct PIO {
    sm0: state_machine::PIOStateMachine,
    sm1: state_machine::PIOStateMachine,
    sm2: state_machine::PIOStateMachine,
    sm3: state_machine::PIOStateMachine,
    mmio: memory_backing::PIOMemoryBacking,
    gpio: Rc<RefCell<gpio::GPIO>>,
    irq_flags: [u8; 8],
    delay_count: u32,
    sm_id: u32,
}

impl PIO {
    pub fn new(state_machine_idx: u32, gpio: Rc<RefCell<gpio::GPIO>>) -> Self {
        let pio = PIO {
            sm0: state_machine::PIOStateMachine::default(),
            sm1: state_machine::PIOStateMachine::default(),
            sm2: state_machine::PIOStateMachine::default(),
            sm3: state_machine::PIOStateMachine::default(),
            mmio: memory_backing::PIOMemoryBacking::default(),
            irq_flags: [0; 8],
            gpio: gpio.clone(),
            delay_count: 0,
            sm_id: state_machine_idx,
        };
        pio
    }

    pub fn run(&mut self) -> Result<(), std::string::String> {
        loop {
            self.step()?
        }

        Ok(())
    }

    pub fn step(&mut self) -> Result<(), std::string::String> {
        if self.delay_count != 0 {
            self.delay_count -= 1;
            return Ok(());
        }

        let pc = self.get_current_sm()?.get_pc();
        let instr = instructions::PIOInstruction::decode(self.mmio.get_pc_data(pc)?)?;
        println!("{} : {:?}", self.get_current_sm()?.get_pc(), instr);

        match instr {
            PIOInstruction::JMP {
                delay_sideset,
                condition,
                address,
            } => {
                self.process_delay_sideset(delay_sideset)?;

                let condition_result = match condition {
                    JmpCondition::Always => true,
                    JmpCondition::ScratchXZero => {
                        if self.get_current_sm()?.get_scratch_x() == 0 {
                            true
                        } else {
                            false
                        }
                    }
                    JmpCondition::ScratchXNonZeroPostDecrement => {
                        let initial = self.get_current_sm()?.get_scratch_x();
                        self.get_current_sm()?.decrement_x()?;
                        if initial != 0 {
                            true
                        } else {
                            false
                        }
                    }
                    JmpCondition::ScratchYZero => {
                        if self.get_current_sm()?.get_scratch_y() == 0 {
                            true
                        } else {
                            false
                        }
                    }
                    JmpCondition::ScratchYNonZeroPostDecrement => {
                        let initial = self.get_current_sm()?.get_scratch_y();
                        self.get_current_sm()?.decrement_y()?;
                        if initial != 0 {
                            true
                        } else {
                            false
                        }
                    }
                    JmpCondition::ScratchXNotEqualScratchY => {
                        if self.get_current_sm()?.get_scratch_x()
                            != self.get_current_sm()?.get_scratch_y()
                        {
                            true
                        } else {
                            false
                        }
                    }
                    JmpCondition::BranchOnInputPin => {
                        let pin = match self.sm_id {
                            0 => self.mmio.SM0_EXECCTRL.read(SM_EXECCTRL::JMP_PIN),
                            1 => self.mmio.SM1_EXECCTRL.read(SM_EXECCTRL::JMP_PIN),
                            2 => self.mmio.SM2_EXECCTRL.read(SM_EXECCTRL::JMP_PIN),
                            3 => self.mmio.SM3_EXECCTRL.read(SM_EXECCTRL::JMP_PIN),
                            _ => return Err(format!("Invalid State Machine ID : {}", self.sm_id)),
                        };

                        let borrowed_gpio: &gpio::GPIO = &self.gpio.borrow();
                        if borrowed_gpio.get_idx(pin as usize) == 1 {
                            true
                        } else {
                            false
                        }
                    }
                    JmpCondition::OutputShiftRegisterNotEmpty => {
                        // TODO Verify this is a correct interpretation
                        let bits_pulled = self.get_current_sm()?.get_osr_counter();
                        let pull_threshold = match self.sm_id {
                            0 => self.mmio.SM0_SHIFTCTRL.read(SM_SHIFTCTRL::PULL_THRESH),
                            1 => self.mmio.SM1_SHIFTCTRL.read(SM_SHIFTCTRL::PULL_THRESH),
                            2 => self.mmio.SM2_SHIFTCTRL.read(SM_SHIFTCTRL::PULL_THRESH),
                            3 => self.mmio.SM3_SHIFTCTRL.read(SM_SHIFTCTRL::PULL_THRESH),
                            _ => return Err(format!("Invalid State Machine ID : {}", self.sm_id)),
                        };

                        if bits_pulled >= pull_threshold {
                            true
                        } else {
                            false
                        }
                    }
                };

                if condition_result {
                    self.get_current_sm()?.set_pc(address as u32)?
                } else {
                    self.get_current_sm()?.inc_pc()?
                }
            }
            PIOInstruction::WAIT {
                delay_sideset,
                polarity,
                source,
                index,
            } => {
                self.process_delay_sideset(delay_sideset)?;

                let polarity_value = match polarity {
                    WaitPolarity::One => 1,
                    WaitPolarity::Zero => 0,
                };

                loop {
                    match source {
                        WaitSource::GPIO => {
                            let borrowed_gpio: &gpio::GPIO = &self.gpio.borrow();
                            if borrowed_gpio.get_idx(index as usize) == polarity_value {
                                break;
                            }
                        }
                        WaitSource::Pin => {
                            let in_base = match self.sm_id {
                                0 => self.mmio.SM0_PINCTRL.read(SM_PINCTRL::IN_BASE),
                                1 => self.mmio.SM1_PINCTRL.read(SM_PINCTRL::IN_BASE),
                                2 => self.mmio.SM2_PINCTRL.read(SM_PINCTRL::IN_BASE),
                                3 => self.mmio.SM3_PINCTRL.read(SM_PINCTRL::IN_BASE),
                                _ => {
                                    return Err(format!(
                                        "Invalid State Machine ID : {}",
                                        self.sm_id
                                    ))
                                }
                            };

                            let borrowed_gpio: &gpio::GPIO = &self.gpio.borrow();
                            if borrowed_gpio.get_idx(((index as u32 + in_base) % 32) as usize)
                                == polarity_value
                            {
                                break;
                            }
                        }
                        WaitSource::IRQ => {
                            if index > self.irq_flags.len() as u8 {
                                return Err(format!(
                                    "IRQ Index longer than total flag length : {}",
                                    index
                                ));
                            } else {
                                if self.irq_flags[index as usize] == polarity_value {
                                    break;
                                }
                            }
                        }
                        WaitSource::Reserved => {
                            unimplemented!("WaitSource::Reserved!");
                        }
                    }
                }

                self.get_current_sm()?.inc_pc();
            }
            PIOInstruction::IN {
                delay_sideset,
                source,
                bit_count,
            } => {
                self.process_delay_sideset(delay_sideset)?;

                unimplemented!("IN unimplemented");
            }
            PIOInstruction::OUT {
                delay_sideset,
                destination,
                bit_count,
            } => {
                self.process_delay_sideset(delay_sideset)?;

                unimplemented!("OUT unimplemented");
            }
            PIOInstruction::PUSH {
                delay_sideset,
                if_full,
                block,
            } => {
                self.process_delay_sideset(delay_sideset)?;

                if if_full {
                    let bits_pushed = self.get_current_sm()?.get_isr_counter();
                    let push_threshold = match self.sm_id {
                        0 => self.mmio.SM0_SHIFTCTRL.read(SM_SHIFTCTRL::PUSH_THRESH),
                        1 => self.mmio.SM1_SHIFTCTRL.read(SM_SHIFTCTRL::PUSH_THRESH),
                        2 => self.mmio.SM2_SHIFTCTRL.read(SM_SHIFTCTRL::PUSH_THRESH),
                        3 => self.mmio.SM3_SHIFTCTRL.read(SM_SHIFTCTRL::PUSH_THRESH),
                        _ => return Err(format!("Invalid State Machine ID : {}", self.sm_id)),
                    };

                    // TODO Verify this is a correct interpretation
                    if bits_pushed < push_threshold {
                        //TODO I think you are supposed to INC PC here, but I am not sure what "Do nothing" means exactly in the docs
                        self.get_current_sm()?.inc_pc()?;
                        return Ok(());
                    }
                }

                if block {
                    if self.get_current_sm()?.rx_fifo_full() {
                        return Ok(());
                    }
                }

                let isr = self.get_current_sm()?.get_isr();
                self.get_current_sm()?.clear_isr()?;
                self.get_current_sm()?.push_to_rx_fifo(isr)?;
                self.get_current_sm()?.inc_pc()?
            }
            PIOInstruction::PULL {
                delay_sideset,
                if_empty,
                block,
            } => {
                self.process_delay_sideset(delay_sideset)?;

                if if_empty {
                    // TODO Verify this is a correct interpretation
                    let bits_pulled = self.get_current_sm()?.get_osr_counter();
                    let pull_threshold = match self.sm_id {
                        0 => self.mmio.SM0_SHIFTCTRL.read(SM_SHIFTCTRL::PULL_THRESH),
                        1 => self.mmio.SM1_SHIFTCTRL.read(SM_SHIFTCTRL::PULL_THRESH),
                        2 => self.mmio.SM2_SHIFTCTRL.read(SM_SHIFTCTRL::PULL_THRESH),
                        3 => self.mmio.SM3_SHIFTCTRL.read(SM_SHIFTCTRL::PULL_THRESH),
                        _ => return Err(format!("Invalid State Machine ID : {}", self.sm_id)),
                    };

                    if bits_pulled < pull_threshold {
                        //TODO I think you are supposed to INC PC here, but I am not sure what "Do nothing" means exactly in the docs
                        self.get_current_sm()?.inc_pc()?;
                        return Ok(());
                    }
                }

                if block {
                    if self.get_current_sm()?.tx_fifo_empty() {
                        return Ok(());
                    }
                }

                let osr = if self.get_current_sm()?.tx_fifo_empty() {
                    self.get_current_sm()?.get_scratch_x()
                } else {
                    self.get_current_sm()?.pop_from_tx_fifo()?
                };

                self.get_current_sm()?.set_osr(osr)?;

                self.get_current_sm()?.inc_pc()?
            }
            PIOInstruction::MOV {
                delay_sideset,
                destination,
                op,
                source,
            } => {
                self.process_delay_sideset(delay_sideset)?;

                unimplemented!("MOV unimplemented");
            }
            PIOInstruction::IRQ {
                delay_sideset,
                clear,
                wait,
                index,
            } => {
                self.process_delay_sideset(delay_sideset)?;

                unimplemented!("IRQ unimplemented");
            }
            PIOInstruction::SET {
                delay_sideset,
                destination,
                data,
            } => {
                self.process_delay_sideset(delay_sideset)?;

                match destination {
                    SetDestination::PINS => {
                        let set_count = match self.sm_id {
                            0 => self.mmio.SM0_PINCTRL.read(SM_PINCTRL::SET_COUNT),
                            1 => self.mmio.SM1_PINCTRL.read(SM_PINCTRL::SET_COUNT),
                            2 => self.mmio.SM2_PINCTRL.read(SM_PINCTRL::SET_COUNT),
                            3 => self.mmio.SM3_PINCTRL.read(SM_PINCTRL::SET_COUNT),
                            _ => return Err(format!("Invalid State Machine ID : {}", self.sm_id)),
                        };

                        let set_base = match self.sm_id {
                            0 => self.mmio.SM0_PINCTRL.read(SM_PINCTRL::SET_BASE),
                            1 => self.mmio.SM1_PINCTRL.read(SM_PINCTRL::SET_BASE),
                            2 => self.mmio.SM2_PINCTRL.read(SM_PINCTRL::SET_BASE),
                            3 => self.mmio.SM3_PINCTRL.read(SM_PINCTRL::SET_BASE),
                            _ => return Err(format!("Invalid State Machine ID : {}", self.sm_id)),
                        };

                        let borrowed_gpio: &mut gpio::GPIO = &mut self.gpio.borrow_mut();
                        borrowed_gpio.set_idx_continuous(set_base, set_count, data)?;
                    }
                    SetDestination::X => {
                        self.get_current_sm()?.set_scratch_x(data.into())?;
                    }
                    SetDestination::Y => {
                        self.get_current_sm()?.set_scratch_y(data.into())?;
                    }
                    SetDestination::PINDIRS => {
                        let set_count = match self.sm_id {
                            0 => self.mmio.SM0_PINCTRL.read(SM_PINCTRL::SET_COUNT),
                            1 => self.mmio.SM1_PINCTRL.read(SM_PINCTRL::SET_COUNT),
                            2 => self.mmio.SM2_PINCTRL.read(SM_PINCTRL::SET_COUNT),
                            3 => self.mmio.SM3_PINCTRL.read(SM_PINCTRL::SET_COUNT),
                            _ => return Err(format!("Invalid State Machine ID : {}", self.sm_id)),
                        };

                        let set_base = match self.sm_id {
                            0 => self.mmio.SM0_PINCTRL.read(SM_PINCTRL::SET_BASE),
                            1 => self.mmio.SM1_PINCTRL.read(SM_PINCTRL::SET_BASE),
                            2 => self.mmio.SM2_PINCTRL.read(SM_PINCTRL::SET_BASE),
                            3 => self.mmio.SM3_PINCTRL.read(SM_PINCTRL::SET_BASE),
                            _ => return Err(format!("Invalid State Machine ID : {}", self.sm_id)),
                        };

                        let borrowed_gpio: &mut gpio::GPIO = &mut self.gpio.borrow_mut();
                        borrowed_gpio.set_idx_enable_continuous(set_base, set_count, data)?;
                    }
                    SetDestination::Reserved0
                    | SetDestination::Reserved1
                    | SetDestination::Reserved2
                    | SetDestination::Reserved3 => {
                        return Err("SetDestination::Reserved".to_string())
                    }
                }
            }
            _ => unimplemented!("Instruction Unimplemented!"),
        }

        Ok(())
    }

    pub fn step_n(&mut self, steps: usize) -> Result<(), std::string::String> {
        for _ in 0..steps {
            self.step()?
        }
        Ok(())
    }

    pub fn set_instruction_data(
        &mut self,
        index: u8,
        data: u16,
    ) -> Result<(), std::string::String> {
        self.mmio.set_instruction_data(index, data)
    }

    fn process_delay_sideset(&mut self, delay_sideset: u8) -> Result<(), std::string::String> {
        let sideset_count = match self.sm_id {
            0 => self.mmio.SM0_PINCTRL.read(SM_PINCTRL::SIDESET_COUNT),
            1 => self.mmio.SM1_PINCTRL.read(SM_PINCTRL::SIDESET_COUNT),
            2 => self.mmio.SM2_PINCTRL.read(SM_PINCTRL::SIDESET_COUNT),
            3 => self.mmio.SM3_PINCTRL.read(SM_PINCTRL::SIDESET_COUNT),
            _ => return Err(format!("Invalid State Machine ID : {}", self.sm_id)),
        };
        let delay_count = 5 - sideset_count;

        if delay_count != 0 {
            let mask = (2 << (delay_count - 1)) - 1;
            self.delay_count = delay_sideset as u32 & mask;
        }

        if sideset_count != 0 {
            unimplemented!("Sidesetting is unimplemented!");
        }

        Ok(())
    }

    fn get_current_sm(
        &mut self,
    ) -> Result<&mut state_machine::PIOStateMachine, std::string::String> {
        match &self.sm_id {
            0 => Ok(&mut self.sm0),
            1 => Ok(&mut self.sm1),
            2 => Ok(&mut self.sm2),
            3 => Ok(&mut self.sm3),
            _ => Err(format!("Invalid State Machine ID : {}", self.sm_id)),
        }
    }
}

fn main() {
    let gpio = Rc::new(RefCell::new(gpio::GPIO::default()));
    let mut pio = PIO::new(0, gpio);

    pio.set_instruction_data(0, 0xffff);

    let run_result = pio.run();
    println!("Run result : {:?}", run_result);
}
