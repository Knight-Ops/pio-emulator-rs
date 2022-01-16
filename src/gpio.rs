#[derive(Debug, Default)]
pub struct GPIO {
    inner: [u8; 32],
    inner_enable: [u8; 32],
}

impl GPIO {
    pub fn new() -> Self {
        GPIO {
            inner: [0; 32],
            inner_enable: [0; 32],
        }
    }

    pub fn get_idx(&self, idx: usize) -> u8 {
        self.inner[idx]
    }

    pub fn get_idx_enable(&self, idx: usize) -> u8 {
        self.inner_enable[idx]
    }

    pub fn set_idx_continuous(
        &mut self,
        base: u32,
        count: u32,
        value: u8,
    ) -> Result<(), std::string::String> {
        for x in 0..count {
            let current_bit = (value >> x) & 1;
            let current_idx = (base + x) % 32;

            self.inner[current_idx as usize] = current_bit;
        }
        Ok(())
    }

    pub fn set_idx_enable_continuous(
        &mut self,
        base: u32,
        count: u32,
        value: u8,
    ) -> Result<(), std::string::String> {
        for x in 0..count {
            let current_bit = (value >> x) & 1;
            let current_idx = (base + x) % 32;

            self.inner_enable[current_idx as usize] = current_bit;
        }
        Ok(())
    }
}
