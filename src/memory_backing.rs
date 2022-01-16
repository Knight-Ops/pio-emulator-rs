#[macro_use]
use tock_registers;
use tock_registers::{register_bitfields, register_structs};

use tock_registers::interfaces::{ReadWriteable, Readable, Writeable};
use tock_registers::registers::{ReadOnly, ReadWrite, WriteOnly};

use std::fmt::Debug;

// We are making everything `ReadWrite` so we can back this ourselves instead of only being able to use
// it as a pointer to memory
register_structs! {
    pub PIOMemoryBacking {
        (0x0 => pub CTRL: ReadWrite<u32, CTRL::Register>),
        (0x4 => pub FSTAT: ReadWrite<u32, FSTAT::Register>),
        (0x8 => pub FDEBUG: ReadWrite<u32, FDEBUG::Register>),
        (0xC => pub FLEVEL: ReadWrite<u32, FLEVEL::Register>),
        (0x10 => pub TXF0: ReadWrite<u32>),
        (0x14 => pub TXF1: ReadWrite<u32>),
        (0x18 => pub TXF2: ReadWrite<u32>),
        (0x1C => pub TXF3: ReadWrite<u32>),
        (0x20 => pub RXF0: ReadWrite<u32>),
        (0x24 => pub RXF1: ReadWrite<u32>),
        (0x28 => pub RXF2: ReadWrite<u32>),
        (0x2C => pub RXF3: ReadWrite<u32>),
        (0x30 => pub IRQ: ReadWrite<u32, IRQ::Register>),
        (0x34 => pub IRQ_FORCE: ReadWrite<u32, IRQ_FORCE::Register>),
        (0x38 => pub INPUT_SYNC_BYPASS: ReadWrite<u32>),
        (0x3C => pub DBG_PADOUT: ReadWrite<u32>),
        (0x40 => pub DBG_PADOE: ReadWrite<u32>),
        (0x44 => pub DBG_CFGINFO: ReadWrite<u32, DBG_CFGINFO::Register>),
        (0x48 => pub INSTR_MEM0: ReadWrite<u32>),
        (0x4C => pub INSTR_MEM1: ReadWrite<u32>),
        (0x50 => pub INSTR_MEM2: ReadWrite<u32>),
        (0x54 => pub INSTR_MEM3: ReadWrite<u32>),
        (0x58 => pub INSTR_MEM4: ReadWrite<u32>),
        (0x5C => pub INSTR_MEM5: ReadWrite<u32>),
        (0x60 => pub INSTR_MEM6: ReadWrite<u32>),
        (0x64 => pub INSTR_MEM7: ReadWrite<u32>),
        (0x68 => pub INSTR_MEM8: ReadWrite<u32>),
        (0x6C => pub INSTR_MEM9: ReadWrite<u32>),
        (0x70 => pub INSTR_MEM10: ReadWrite<u32>),
        (0x74 => pub INSTR_MEM11: ReadWrite<u32>),
        (0x78 => pub INSTR_MEM12: ReadWrite<u32>),
        (0x7C => pub INSTR_MEM13: ReadWrite<u32>),
        (0x80 => pub INSTR_MEM14: ReadWrite<u32>),
        (0x84 => pub INSTR_MEM15: ReadWrite<u32>),
        (0x88 => pub INSTR_MEM16: ReadWrite<u32>),
        (0x8C => pub INSTR_MEM17: ReadWrite<u32>),
        (0x90 => pub INSTR_MEM18: ReadWrite<u32>),
        (0x94 => pub INSTR_MEM19: ReadWrite<u32>),
        (0x98 => pub INSTR_MEM20: ReadWrite<u32>),
        (0x9C => pub INSTR_MEM21: ReadWrite<u32>),
        (0xA0 => pub INSTR_MEM22: ReadWrite<u32>),
        (0xA4 => pub INSTR_MEM23: ReadWrite<u32>),
        (0xA8 => pub INSTR_MEM24: ReadWrite<u32>),
        (0xAC => pub INSTR_MEM25: ReadWrite<u32>),
        (0xB0 => pub INSTR_MEM26: ReadWrite<u32>),
        (0xB4 => pub INSTR_MEM27: ReadWrite<u32>),
        (0xB8 => pub INSTR_MEM28: ReadWrite<u32>),
        (0xBC => pub INSTR_MEM29: ReadWrite<u32>),
        (0xC0 => pub INSTR_MEM30: ReadWrite<u32>),
        (0xC4 => pub INSTR_MEM31: ReadWrite<u32>),
        (0xC8 => pub SM0_CLKDIV: ReadWrite<u32, SM_CLKDIV::Register>),
        (0xCC => pub SM0_EXECCTRL: ReadWrite<u32, SM_EXECCTRL::Register>),
        (0xD0 => pub SM0_SHIFTCTRL: ReadWrite<u32, SM_SHIFTCTRL::Register>),
        (0xD4 => pub SM0_ADDR: ReadWrite<u32, SM_ADDR::Register>),
        (0xD8 => pub SM0_INSTR: ReadWrite<u32, SM_INSTR::Register>),
        (0xDC => pub SM0_PINCTRL: ReadWrite<u32, SM_PINCTRL::Register>),
        (0xE0 => pub SM1_CLKDIV: ReadWrite<u32, SM_CLKDIV::Register>),
        (0xE4 => pub SM1_EXECCTRL: ReadWrite<u32, SM_EXECCTRL::Register>),
        (0xE8 => pub SM1_SHIFTCTRL: ReadWrite<u32, SM_SHIFTCTRL::Register>),
        (0xEC => pub SM1_ADDR: ReadWrite<u32, SM_ADDR::Register>),
        (0xF0 => pub SM1_INSTR: ReadWrite<u32, SM_INSTR::Register>),
        (0xF4 => pub SM1_PINCTRL: ReadWrite<u32, SM_PINCTRL::Register>),
        (0xF8 => pub SM2_CLKDIV: ReadWrite<u32, SM_CLKDIV::Register>),
        (0xFC => pub SM2_EXECCTRL: ReadWrite<u32, SM_EXECCTRL::Register>),
        (0x100 => pub SM2_SHIFTCTRL: ReadWrite<u32, SM_SHIFTCTRL::Register>),
        (0x104 => pub SM2_ADDR: ReadWrite<u32, SM_ADDR::Register>),
        (0x108 => pub SM2_INSTR: ReadWrite<u32, SM_INSTR::Register>),
        (0x10C => pub SM2_PINCTRL: ReadWrite<u32, SM_PINCTRL::Register>),
        (0x110 => pub SM3_CLKDIV: ReadWrite<u32, SM_CLKDIV::Register>),
        (0x114 => pub SM3_EXECCTRL: ReadWrite<u32, SM_EXECCTRL::Register>),
        (0x118 => pub SM3_SHIFTCTRL: ReadWrite<u32, SM_SHIFTCTRL::Register>),
        (0x11C => pub SM3_ADDR: ReadWrite<u32, SM_ADDR::Register>),
        (0x120 => pub SM3_INSTR: ReadWrite<u32, SM_INSTR::Register>),
        (0x124 => pub SM3_PINCTRL: ReadWrite<u32, SM_PINCTRL::Register>),
        (0x128 => pub INTR: ReadWrite<u32, INTR::Register>),
        (0x12C => pub IRQ0_INTE: ReadWrite<u32, IRQ0_INTE::Register>),
        (0x130 => pub IRQ0_INTF: ReadWrite<u32, IRQ0_INTF::Register>),
        (0x134 => pub IRQ0_INTS: ReadWrite<u32, IRQ0_INTS::Register>),
        (0x138 => pub IRQ1_INTE: ReadWrite<u32, IRQ1_INTE::Register>),
        (0x13C => pub IRQ1_INTF: ReadWrite<u32, IRQ1_INTF::Register>),
        (0x140 => pub IRQ1_INTS: ReadWrite<u32, IRQ1_INTS::Register>),
        (0x144 => @END),
    }
}

register_bitfields! {
    u32,
    pub CTRL [
        CLKDIV_RESTART OFFSET(8) NUMBITS(4) [],
        SM_RESTART OFFSET(4) NUMBITS(4) [],
        SM_ENABLE OFFSET(0) NUMBITS(4) [],
    ],

    pub FSTAT [
        TXEMPTY OFFSET(24) NUMBITS(4) [],
        TXFULL OFFSET(16) NUMBITS(4) [],
        RXEMPTY OFFSET(8) NUMBITS(4) [],
        RXFULL OFFSET(0) NUMBITS(4) [],
    ],

    pub FDEBUG [
        TXSTALL OFFSET(24) NUMBITS(4) [],
        TXOVER OFFSET(16) NUMBITS(4) [],
        RXUNDER OFFSET(8) NUMBITS(4) [],
        RXSTALL OFFSET(0) NUMBITS(4) [],
    ],

    pub FLEVEL [
        RX3 OFFSET(28) NUMBITS(4) [],
        TX3 OFFSET(24) NUMBITS(4) [],
        RX2 OFFSET(20) NUMBITS(4) [],
        TX2 OFFSET(16) NUMBITS(4) [],
        RX1 OFFSET(12) NUMBITS(4) [],
        TX1 OFFSET(8) NUMBITS(4) [],
        RX0 OFFSET(4) NUMBITS(4) [],
        TX0 OFFSET(0) NUMBITS(4) [],
    ],

    pub IRQ [
        IRQFLAGS OFFSET(0) NUMBITS(8) [],
    ],

    pub IRQ_FORCE [
        IRQFLAGS OFFSET(0) NUMBITS(8) [],
    ],

    pub DBG_CFGINFO [
        IMEM_SIZE OFFSET(16) NUMBITS(6) [],
        SM_COUNT OFFSET(8) NUMBITS(4) [],
        FIFO_DEPTH OFFSET(0) NUMBITS(6) [],
    ],
    pub SM_CLKDIV [
        INT OFFSET(16) NUMBITS(16) [],
        FRAC OFFSET(8) NUMBITS(8) [],
    ],

    pub SM_EXECCTRL [
        EXECSTALLED OFFSET(31) NUMBITS(1) [],
        SIDE_EN OFFSET(30) NUMBITS(1) [],
        SIDE_PINDIR OFFSET(29) NUMBITS(1) [],
        JMP_PIN OFFSET(24) NUMBITS(5) [],
        OUT_EN_SEL OFFSET(19) NUMBITS(5) [],
        INLINE_OUT_SEL OFFSET(18) NUMBITS(1) [],
        OUT_STICKY OFFSET(17) NUMBITS(1) [],
        WRAP_TOP OFFSET(12) NUMBITS(5) [],
        WRAP_BOTTOM OFFSET(7) NUMBITS(5) [],
        STATUS_SEL OFFSET(4) NUMBITS(1) [],
        STATUS_N OFFSET(0) NUMBITS(4) [],
    ],

    pub SM_SHIFTCTRL [
        FJOIN_RX OFFSET(31) NUMBITS(1) [],
        FJOIN_TX OFFSET(30) NUMBITS(1) [],
        PULL_THRESH OFFSET(25) NUMBITS(5) [],
        PUSH_THRESH OFFSET(20) NUMBITS(5) [],
        OUT_SHIFTDIR OFFSET(19) NUMBITS(1) [],
        IN_SHIFTDIR OFFSET(18) NUMBITS(1) [],
        AUTOPULL OFFSET(17) NUMBITS(1) [],
        AUTOPUSH OFFSET(16) NUMBITS(1) [],
    ],

    pub SM_ADDR [
        CURRENT_ADDRESS OFFSET(0) NUMBITS(5) [],
    ],

    pub SM_INSTR [
        CUR_INSTR OFFSET(0) NUMBITS(16) [],
    ],

    pub SM_PINCTRL [
        SIDESET_COUNT OFFSET(29) NUMBITS(3) [],
        SET_COUNT OFFSET(26) NUMBITS(3) [],
        OUT_COUNT OFFSET(20) NUMBITS(6) [],
        IN_BASE OFFSET(15) NUMBITS(5) [],
        SIDESET_BASE OFFSET(10) NUMBITS(5) [],
        SET_BASE OFFSET(5) NUMBITS(5) [],
        OUT_BASE OFFSET(0) NUMBITS(5) [],
    ],

    pub INTR [
        SM3 OFFSET(11) NUMBITS(1) [],
        SM2 OFFSET(10) NUMBITS(1) [],
        SM1 OFFSET(9) NUMBITS(1) [],
        SM0 OFFSET(8) NUMBITS(1) [],
        SM3_TXNFULL OFFSET(7) NUMBITS(1) [],
        SM2_TXNFULL OFFSET(6) NUMBITS(1) [],
        SM1_TXNFULL OFFSET(5) NUMBITS(1) [],
        SM0_TXNFULL OFFSET(4) NUMBITS(1) [],
        SM3_RXNEMPTY OFFSET(3) NUMBITS(1) [],
        SM2_RXNEMPTY OFFSET(2) NUMBITS(1) [],
        SM1_RXNEMPTY OFFSET(1) NUMBITS(1) [],
        SM0_RXNEMPTY OFFSET(0) NUMBITS(1) []
    ],

    pub IRQ0_INTE [
        SM3 OFFSET(11) NUMBITS(1) [],
        SM2 OFFSET(10) NUMBITS(1) [],
        SM1 OFFSET(9) NUMBITS(1) [],
        SM0 OFFSET(8) NUMBITS(1) [],
        SM3_TXNFULL OFFSET(7) NUMBITS(1) [],
        SM2_TXNFULL OFFSET(6) NUMBITS(1) [],
        SM1_TXNFULL OFFSET(5) NUMBITS(1) [],
        SM0_TXNFULL OFFSET(4) NUMBITS(1) [],
        SM3_RXNEMPTY OFFSET(3) NUMBITS(1) [],
        SM2_RXNEMPTY OFFSET(2) NUMBITS(1) [],
        SM1_RXNEMPTY OFFSET(1) NUMBITS(1) [],
        SM0_RXNEMPTY OFFSET(0) NUMBITS(1) []
    ],

    pub IRQ0_INTF [
        SM3 OFFSET(11) NUMBITS(1) [],
        SM2 OFFSET(10) NUMBITS(1) [],
        SM1 OFFSET(9) NUMBITS(1) [],
        SM0 OFFSET(8) NUMBITS(1) [],
        SM3_TXNFULL OFFSET(7) NUMBITS(1) [],
        SM2_TXNFULL OFFSET(6) NUMBITS(1) [],
        SM1_TXNFULL OFFSET(5) NUMBITS(1) [],
        SM0_TXNFULL OFFSET(4) NUMBITS(1) [],
        SM3_RXNEMPTY OFFSET(3) NUMBITS(1) [],
        SM2_RXNEMPTY OFFSET(2) NUMBITS(1) [],
        SM1_RXNEMPTY OFFSET(1) NUMBITS(1) [],
        SM0_RXNEMPTY OFFSET(0) NUMBITS(1) []
    ],

    pub IRQ0_INTS [
        SM3 OFFSET(11) NUMBITS(1) [],
        SM2 OFFSET(10) NUMBITS(1) [],
        SM1 OFFSET(9) NUMBITS(1) [],
        SM0 OFFSET(8) NUMBITS(1) [],
        SM3_TXNFULL OFFSET(7) NUMBITS(1) [],
        SM2_TXNFULL OFFSET(6) NUMBITS(1) [],
        SM1_TXNFULL OFFSET(5) NUMBITS(1) [],
        SM0_TXNFULL OFFSET(4) NUMBITS(1) [],
        SM3_RXNEMPTY OFFSET(3) NUMBITS(1) [],
        SM2_RXNEMPTY OFFSET(2) NUMBITS(1) [],
        SM1_RXNEMPTY OFFSET(1) NUMBITS(1) [],
        SM0_RXNEMPTY OFFSET(0) NUMBITS(1) []
    ],

    pub IRQ1_INTE [
        SM3 OFFSET(11) NUMBITS(1) [],
        SM2 OFFSET(10) NUMBITS(1) [],
        SM1 OFFSET(9) NUMBITS(1) [],
        SM0 OFFSET(8) NUMBITS(1) [],
        SM3_TXNFULL OFFSET(7) NUMBITS(1) [],
        SM2_TXNFULL OFFSET(6) NUMBITS(1) [],
        SM1_TXNFULL OFFSET(5) NUMBITS(1) [],
        SM0_TXNFULL OFFSET(4) NUMBITS(1) [],
        SM3_RXNEMPTY OFFSET(3) NUMBITS(1) [],
        SM2_RXNEMPTY OFFSET(2) NUMBITS(1) [],
        SM1_RXNEMPTY OFFSET(1) NUMBITS(1) [],
        SM0_RXNEMPTY OFFSET(0) NUMBITS(1) []
    ],

    pub IRQ1_INTF [
        SM3 OFFSET(11) NUMBITS(1) [],
        SM2 OFFSET(10) NUMBITS(1) [],
        SM1 OFFSET(9) NUMBITS(1) [],
        SM0 OFFSET(8) NUMBITS(1) [],
        SM3_TXNFULL OFFSET(7) NUMBITS(1) [],
        SM2_TXNFULL OFFSET(6) NUMBITS(1) [],
        SM1_TXNFULL OFFSET(5) NUMBITS(1) [],
        SM0_TXNFULL OFFSET(4) NUMBITS(1) [],
        SM3_RXNEMPTY OFFSET(3) NUMBITS(1) [],
        SM2_RXNEMPTY OFFSET(2) NUMBITS(1) [],
        SM1_RXNEMPTY OFFSET(1) NUMBITS(1) [],
        SM0_RXNEMPTY OFFSET(0) NUMBITS(1) []
    ],

    pub IRQ1_INTS [
        SM3 OFFSET(11) NUMBITS(1) [],
        SM2 OFFSET(10) NUMBITS(1) [],
        SM1 OFFSET(9) NUMBITS(1) [],
        SM0 OFFSET(8) NUMBITS(1) [],
        SM3_TXNFULL OFFSET(7) NUMBITS(1) [],
        SM2_TXNFULL OFFSET(6) NUMBITS(1) [],
        SM1_TXNFULL OFFSET(5) NUMBITS(1) [],
        SM0_TXNFULL OFFSET(4) NUMBITS(1) [],
        SM3_RXNEMPTY OFFSET(3) NUMBITS(1) [],
        SM2_RXNEMPTY OFFSET(2) NUMBITS(1) [],
        SM1_RXNEMPTY OFFSET(1) NUMBITS(1) [],
        SM0_RXNEMPTY OFFSET(0) NUMBITS(1) []
    ],
}

impl Default for PIOMemoryBacking {
    fn default() -> Self {
        let backing = [0 as u8; std::mem::size_of::<PIOMemoryBacking>()];

        let pio_mem_backing: PIOMemoryBacking = unsafe { std::mem::transmute(backing) };

        pio_mem_backing
            .FSTAT
            .write(FSTAT::TXEMPTY.val(0xF) + FSTAT::RXEMPTY.val(0xF));

        pio_mem_backing.SM0_CLKDIV.write(SM_CLKDIV::INT.val(1));
        pio_mem_backing.SM1_CLKDIV.write(SM_CLKDIV::INT.val(1));
        pio_mem_backing.SM2_CLKDIV.write(SM_CLKDIV::INT.val(1));
        pio_mem_backing.SM3_CLKDIV.write(SM_CLKDIV::INT.val(1));

        pio_mem_backing
            .SM0_EXECCTRL
            .write(SM_EXECCTRL::WRAP_TOP.val(0x1f));
        pio_mem_backing
            .SM1_EXECCTRL
            .write(SM_EXECCTRL::WRAP_TOP.val(0x1f));
        pio_mem_backing
            .SM2_EXECCTRL
            .write(SM_EXECCTRL::WRAP_TOP.val(0x1f));
        pio_mem_backing
            .SM3_EXECCTRL
            .write(SM_EXECCTRL::WRAP_TOP.val(0x1f));

        pio_mem_backing
            .SM0_SHIFTCTRL
            .write(SM_SHIFTCTRL::IN_SHIFTDIR.val(1) + SM_SHIFTCTRL::OUT_SHIFTDIR.val(1));
        pio_mem_backing
            .SM1_SHIFTCTRL
            .write(SM_SHIFTCTRL::IN_SHIFTDIR.val(1) + SM_SHIFTCTRL::OUT_SHIFTDIR.val(1));
        pio_mem_backing
            .SM2_SHIFTCTRL
            .write(SM_SHIFTCTRL::IN_SHIFTDIR.val(1) + SM_SHIFTCTRL::OUT_SHIFTDIR.val(1));
        pio_mem_backing
            .SM3_SHIFTCTRL
            .write(SM_SHIFTCTRL::IN_SHIFTDIR.val(1) + SM_SHIFTCTRL::OUT_SHIFTDIR.val(1));

        pio_mem_backing
            .SM0_PINCTRL
            .write(SM_PINCTRL::SET_COUNT.val(5));
        pio_mem_backing
            .SM1_PINCTRL
            .write(SM_PINCTRL::SET_COUNT.val(5));
        pio_mem_backing
            .SM2_PINCTRL
            .write(SM_PINCTRL::SET_COUNT.val(5));
        pio_mem_backing
            .SM3_PINCTRL
            .write(SM_PINCTRL::SET_COUNT.val(5));

        pio_mem_backing
    }
}

impl Debug for PIOMemoryBacking {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PIOMemoryBacking")
            .field("CTRL", &self.CTRL.get())
            .field("FSTAT", &self.FSTAT.get())
            .field("FDEBUG", &self.FDEBUG.get())
            .field("FLEVEL", &self.FLEVEL.get())
            .field("TXF0", &self.TXF0.get())
            .field("TXF1", &self.TXF1.get())
            .field("TXF2", &self.TXF2.get())
            .field("TXF3", &self.TXF3.get())
            .field("RXF0", &self.RXF0.get())
            .field("RXF1", &self.RXF1.get())
            .field("RXF2", &self.RXF2.get())
            .field("RXF3", &self.RXF3.get())
            .field("IRQ", &self.IRQ.get())
            .field("IRQ_FORCE", &self.IRQ_FORCE.get())
            .field("INPUT_SYNC_BYPASS", &self.INPUT_SYNC_BYPASS.get())
            .field("DBG_PADOUT", &self.DBG_PADOUT.get())
            .field("DBG_PADOE", &self.DBG_PADOE.get())
            .field("DBG_CFGINFO", &self.DBG_CFGINFO.get())
            .field("INSTR_MEM0", &self.INSTR_MEM0.get())
            .field("INSTR_MEM1", &self.INSTR_MEM1.get())
            .field("INSTR_MEM2", &self.INSTR_MEM2.get())
            .field("INSTR_MEM3", &self.INSTR_MEM3.get())
            .field("INSTR_MEM4", &self.INSTR_MEM4.get())
            .field("INSTR_MEM5", &self.INSTR_MEM5.get())
            .field("INSTR_MEM6", &self.INSTR_MEM6.get())
            .field("INSTR_MEM7", &self.INSTR_MEM7.get())
            .field("INSTR_MEM8", &self.INSTR_MEM8.get())
            .field("INSTR_MEM9", &self.INSTR_MEM9.get())
            .field("INSTR_MEM10", &self.INSTR_MEM10.get())
            .field("INSTR_MEM11", &self.INSTR_MEM11.get())
            .field("INSTR_MEM12", &self.INSTR_MEM12.get())
            .field("INSTR_MEM13", &self.INSTR_MEM13.get())
            .field("INSTR_MEM14", &self.INSTR_MEM14.get())
            .field("INSTR_MEM15", &self.INSTR_MEM15.get())
            .field("INSTR_MEM16", &self.INSTR_MEM16.get())
            .field("INSTR_MEM17", &self.INSTR_MEM17.get())
            .field("INSTR_MEM18", &self.INSTR_MEM18.get())
            .field("INSTR_MEM19", &self.INSTR_MEM19.get())
            .field("INSTR_MEM20", &self.INSTR_MEM20.get())
            .field("INSTR_MEM21", &self.INSTR_MEM21.get())
            .field("INSTR_MEM22", &self.INSTR_MEM22.get())
            .field("INSTR_MEM23", &self.INSTR_MEM23.get())
            .field("INSTR_MEM24", &self.INSTR_MEM24.get())
            .field("INSTR_MEM25", &self.INSTR_MEM25.get())
            .field("INSTR_MEM26", &self.INSTR_MEM26.get())
            .field("INSTR_MEM27", &self.INSTR_MEM27.get())
            .field("INSTR_MEM28", &self.INSTR_MEM28.get())
            .field("INSTR_MEM29", &self.INSTR_MEM29.get())
            .field("INSTR_MEM30", &self.INSTR_MEM30.get())
            .field("INSTR_MEM31", &self.INSTR_MEM31.get())
            .field("SM0_CLKDIV", &self.SM0_CLKDIV.get())
            .field("SM0_EXECCTRL", &self.SM0_EXECCTRL.get())
            .field("SM0_SHIFTCTRL", &self.SM0_SHIFTCTRL.get())
            .field("SM0_ADDR", &self.SM0_ADDR.get())
            .field("SM0_INSTR", &self.SM0_INSTR.get())
            .field("SM0_PINCTRL", &self.SM0_PINCTRL.get())
            .field("SM1_CLKDIV", &self.SM1_CLKDIV.get())
            .field("SM1_EXECCTRL", &self.SM1_EXECCTRL.get())
            .field("SM1_SHIFTCTRL", &self.SM1_SHIFTCTRL.get())
            .field("SM1_ADDR", &self.SM1_ADDR.get())
            .field("SM1_INSTR", &self.SM1_INSTR.get())
            .field("SM1_PINCTRL", &self.SM1_PINCTRL.get())
            .field("SM2_CLKDIV", &self.SM2_CLKDIV.get())
            .field("SM2_EXECCTRL", &self.SM2_EXECCTRL.get())
            .field("SM2_SHIFTCTRL", &self.SM2_SHIFTCTRL.get())
            .field("SM2_ADDR", &self.SM2_ADDR.get())
            .field("SM2_INSTR", &self.SM2_INSTR.get())
            .field("SM2_PINCTRL", &self.SM2_PINCTRL.get())
            .field("SM3_CLKDIV", &self.SM3_CLKDIV.get())
            .field("SM3_EXECCTRL", &self.SM3_EXECCTRL.get())
            .field("SM3_SHIFTCTRL", &self.SM3_SHIFTCTRL.get())
            .field("SM3_ADDR", &self.SM3_ADDR.get())
            .field("SM3_INSTR", &self.SM3_INSTR.get())
            .field("SM3_PINCTRL", &self.SM3_PINCTRL.get())
            .field("INTR", &self.INTR.get())
            .field("IRQ0_INTE", &self.IRQ0_INTE.get())
            .field("IRQ0_INTF", &self.IRQ0_INTF.get())
            .field("IRQ0_INTS", &self.IRQ0_INTS.get())
            .field("IRQ1_INTE", &self.IRQ1_INTE.get())
            .field("IRQ1_INTF", &self.IRQ1_INTF.get())
            .field("IRQ1_INTS", &self.IRQ1_INTS.get())
            .finish()
    }
}

impl PIOMemoryBacking {
    pub fn get_pc_data(&self, pc: u32) -> Result<u32, std::string::String> {
        match pc {
            0 => Ok(self.INSTR_MEM0.get()),
            1 => Ok(self.INSTR_MEM1.get()),
            2 => Ok(self.INSTR_MEM2.get()),
            3 => Ok(self.INSTR_MEM3.get()),
            4 => Ok(self.INSTR_MEM4.get()),
            5 => Ok(self.INSTR_MEM5.get()),
            6 => Ok(self.INSTR_MEM6.get()),
            7 => Ok(self.INSTR_MEM7.get()),
            8 => Ok(self.INSTR_MEM8.get()),
            9 => Ok(self.INSTR_MEM9.get()),
            10 => Ok(self.INSTR_MEM10.get()),
            11 => Ok(self.INSTR_MEM11.get()),
            12 => Ok(self.INSTR_MEM12.get()),
            13 => Ok(self.INSTR_MEM13.get()),
            14 => Ok(self.INSTR_MEM14.get()),
            15 => Ok(self.INSTR_MEM15.get()),
            16 => Ok(self.INSTR_MEM16.get()),
            17 => Ok(self.INSTR_MEM17.get()),
            18 => Ok(self.INSTR_MEM18.get()),
            19 => Ok(self.INSTR_MEM19.get()),
            20 => Ok(self.INSTR_MEM20.get()),
            21 => Ok(self.INSTR_MEM21.get()),
            22 => Ok(self.INSTR_MEM22.get()),
            23 => Ok(self.INSTR_MEM23.get()),
            24 => Ok(self.INSTR_MEM24.get()),
            25 => Ok(self.INSTR_MEM25.get()),
            26 => Ok(self.INSTR_MEM26.get()),
            27 => Ok(self.INSTR_MEM27.get()),
            28 => Ok(self.INSTR_MEM28.get()),
            29 => Ok(self.INSTR_MEM29.get()),
            30 => Ok(self.INSTR_MEM30.get()),
            31 => Ok(self.INSTR_MEM31.get()),
            _ => Err(format!("Invalid PC provided : {}", pc)),
        }
    }

    pub fn set_instruction_data(
        &mut self,
        index: u8,
        data: u16,
    ) -> Result<(), std::string::String> {
        match index {
            0 => {
                self.INSTR_MEM0.set(data as u32);
                Ok(())
            }
            1 => {
                self.INSTR_MEM1.set(data as u32);
                Ok(())
            }
            2 => {
                self.INSTR_MEM2.set(data as u32);
                Ok(())
            }
            3 => {
                self.INSTR_MEM3.set(data as u32);
                Ok(())
            }
            4 => {
                self.INSTR_MEM4.set(data as u32);
                Ok(())
            }
            5 => {
                self.INSTR_MEM5.set(data as u32);
                Ok(())
            }
            6 => {
                self.INSTR_MEM6.set(data as u32);
                Ok(())
            }
            7 => {
                self.INSTR_MEM7.set(data as u32);
                Ok(())
            }
            8 => {
                self.INSTR_MEM8.set(data as u32);
                Ok(())
            }
            9 => {
                self.INSTR_MEM9.set(data as u32);
                Ok(())
            }
            10 => {
                self.INSTR_MEM10.set(data as u32);
                Ok(())
            }
            11 => {
                self.INSTR_MEM11.set(data as u32);
                Ok(())
            }
            12 => {
                self.INSTR_MEM12.set(data as u32);
                Ok(())
            }
            13 => {
                self.INSTR_MEM13.set(data as u32);
                Ok(())
            }
            14 => {
                self.INSTR_MEM14.set(data as u32);
                Ok(())
            }
            15 => {
                self.INSTR_MEM15.set(data as u32);
                Ok(())
            }
            16 => {
                self.INSTR_MEM16.set(data as u32);
                Ok(())
            }
            17 => {
                self.INSTR_MEM17.set(data as u32);
                Ok(())
            }
            18 => {
                self.INSTR_MEM18.set(data as u32);
                Ok(())
            }
            19 => {
                self.INSTR_MEM19.set(data as u32);
                Ok(())
            }
            20 => {
                self.INSTR_MEM20.set(data as u32);
                Ok(())
            }
            21 => {
                self.INSTR_MEM21.set(data as u32);
                Ok(())
            }
            22 => {
                self.INSTR_MEM22.set(data as u32);
                Ok(())
            }
            23 => {
                self.INSTR_MEM23.set(data as u32);
                Ok(())
            }
            24 => {
                self.INSTR_MEM24.set(data as u32);
                Ok(())
            }
            25 => {
                self.INSTR_MEM25.set(data as u32);
                Ok(())
            }
            26 => {
                self.INSTR_MEM26.set(data as u32);
                Ok(())
            }
            27 => {
                self.INSTR_MEM27.set(data as u32);
                Ok(())
            }
            28 => {
                self.INSTR_MEM28.set(data as u32);
                Ok(())
            }
            29 => {
                self.INSTR_MEM29.set(data as u32);
                Ok(())
            }
            30 => {
                self.INSTR_MEM30.set(data as u32);
                Ok(())
            }
            31 => {
                self.INSTR_MEM31.set(data as u32);
                Ok(())
            }
            _ => Err(format!("Invalid index provided : {}", index)),
        }
    }
}
