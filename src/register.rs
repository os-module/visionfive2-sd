use alloc::format;
use alloc::string::{String, ToString};
use bit_field::BitField;
use bit_struct::{*};
use crate::cmd::Cmd;

pub const SDIO_BASE:usize = 0x16020000;
pub const CTRL_REG:usize = SDIO_BASE + 0x00;
pub const POWER_REG:usize = SDIO_BASE + 0x04;
pub const BLK_SIZE_REG:usize = SDIO_BASE + 0x1c;
pub const BYTE_CNT_REG:usize = SDIO_BASE + 0x20;
pub const CMD_REG:usize = SDIO_BASE + 0x2c;
pub const ARG_REG:usize = SDIO_BASE + 0x28;
pub const RESP0_REG:usize = SDIO_BASE + 0x30;
pub const RESP1_REG:usize = SDIO_BASE + 0x34;
pub const RESP2_REG:usize = SDIO_BASE + 0x38;
pub const RESP3_REG:usize = SDIO_BASE + 0x3c;
pub const STATUS_REG:usize = SDIO_BASE + 0x48;
pub const CDETECT_REG:usize = SDIO_BASE + 0x50;
pub const BUS_MODE_REG:usize = SDIO_BASE + 0x80;
pub const CTYPE_REG:usize = SDIO_BASE + 0x18;
pub const CLOCK_ENABLE_REG:usize = SDIO_BASE + 0x10;
pub const DBADDRL_REG:usize = SDIO_BASE + 0x88; // DMA DES Address Lower
pub const DBADDRU_REG:usize = SDIO_BASE + 0x8c; // DMA DES Address Upper
pub const CLK_DIVIDER_REG:usize = SDIO_BASE + 0x08;
pub const RAW_INT_STATUS_REG:usize = SDIO_BASE + 0x44;
pub const FIFO_DATA_REG:usize = SDIO_BASE + 0x600;


bit_struct! {
    pub struct PowerReg(u32){
        reserved:u2,
        power_enable:u30,
    }
    pub struct CmdArg(u32){
        arg:u32,
    }
    pub struct BlkSizeReg(u32){
        reserved:u16,
        block_size:u16,
    }
    /// Number of bytes to be transferred; should be integer multiple of Block Size for block transfers.
    pub struct ByteCountReg(u32){
        byte_count:u32,
    }
    pub struct CmdReg(u32){
        start_cmd:u1,
        reserved:u1,
        // Use Hold Register
        //
        // 0 - CMD and DATA sent to card bypassing HOLD Register
        //
        // 1 - CMD and DATA sent to card through the HOLD Register For more information,
        // refer to “Host Controller Output Path Timing” on page 320.
        use_hold_reg:u1,
        volt_switch:u1,
        boot_mode:u1,
        disable_boot:u1,
        expect_boot_ack:u1,
        enable_boot:u1,
        ccs_expected:u1,
        read_ceata_device:u1,
        // 0 - Normal command sequence
        // 1 - Do not send commands, just update clock register value into card clock domain
        update_clock_registers_only:u1,
        card_number:u5,
        send_initialization:u1,
        stop_abort_cmd:u1,
        //0 - Send command at once, even if previous data transfer has not completed
        //
        // 1 - Wait for previous data transfer completion before sending command
        //
        // The wait_prvdata_complete = 0 option typically used to query status of card
        // during data transfer or to stop current data transfer; card_number should be same as in previous command.
        wait_prvdata_complete:u1,
        //
        // 0 - No stop command sent at end of data transfer
        //
        // 1 - Send stop command at end of data transfer
        // Don't care if no data expected from card.
        send_auto_stop:u1,
        //
        // 0 - Block data transfer command
        //
        // 1 - Stream data transfer command Don’t care if no data expected.
        transfer_mode:u1,
        // 0 - Read from card
        //
        // 1 - Write to card
        //
        // Don’t care if no data expected from card.
        transfer_dir:u1,
        // 	0 - No data transfer expected (read/write) 1 - Data transfer expected (read/write)
        data_expected:u1,
        // 0 - Do not check response CRC
        //
        // 1 - Check response CRC
        //
        // Some of command responses do not return valid CRC bits.
        //
        // Software should disable CRC checks for those commands in order to disable CRC checking by controller.
        check_response_crc:u1,
        // 0 - Short response expected from card 1 - Long response expected from card
        response_length:u1,
        // 0 - No response expected from card 1 - Response expected from card
        response_expect:u1,
        // Command index
        cmd_index:u6,
    }
    pub struct CDetectReg(u32){
        reserved:u2,
        card_detect_n:u30,
    }
    pub struct ClockEnableReg(u32){
        // Low-power control for up to 16 SD card clocks and one MMC card clock supported.
        //
        // 0 - Non-low-power mode
        //
        // 1 - Low-power mode; stop clock when card in IDLE (should be normally set to only
        // MMC and SD memory cards; for SDIO cards, if interrupts must be detected, clock should not be stopped).
        cclk_low_power:u16,
        //
        // Clock-enable control for up to 16 SD card clocks and one MMC card clock supported.
        //
        // 0 - Clock disabled
        //
        // 1 - Clock enabled
        clk_enable:u16,
    }

    pub struct CardTypeReg(u32){
        // One bit per card indicates if card is 8-bit:
        // 0 - Non 8-bit mode
        //
        // 1 - 8-bit mode
        //
        // Bit[31] corresponds to card[15]; bit[16] corresponds to card[0].
        card_width8:u16,
        // One bit per card indicates if card is 1-bit or 4-bit:
        // 0 - 1-bit mode
        //
        // 1 - 4-bit mode
        //
        // Bit[15] corresponds to card[15], bit[0] corresponds to card[0].
        //
        // Only NUM_CARDS*2 number of bits are implemented.
        card_width4_1:u16,
    }
    pub struct ControlReg(u32){
        reserved:u6,
        // Present only for the Internal DMAC configuration; else, it is reserved.
        // 0– The host performs data transfers through the slave interface
        // 1– Internal DMAC used for data transfer
        use_internal_dmac:u1,
        // External open-drain pullup:
        //
        // 0- Disable
        // 1 - Enable
        // Inverted value of this bit is output to ccmd_od_pullup_en_n port.
        // When bit is set, command output always driven in open-drive mode;
        // that is, DWC_mobile_storage drives either 0 or high impedance, and does not drive hard 1.
        enable_od_pullup:u1,
        // Card regulator-B voltage setting; output to card_volt_b port.
        //
        // Optional feature; ports can be used as general-purpose outputs.
        card_voltage_b:u4,
        // Card regulator-A voltage setting; output to card_volt_a port.
        //
        // Optional feature; ports can be used as general-purpose outputs.
        card_voltage_a:u4,
        reserved1:u4,
        // 0 - Interrupts not enabled in CE-ATA device (nIEN = 1 in ATA control register)
        // 1 - Interrupts are enabled in CE-ATA device (nIEN = 0 in ATA control register)
        // Software should appropriately write to this bit after power-on reset or any other reset to CE-ATA device.
        // After reset, usually CE-ATA device interrupt is disabled (nIEN = 1).
        // If the host enables CE-ATA device interrupt, then software should set this bit.
        ceata_device_interrupt:u1,
        // 0 - Clear bit if DWC_mobile_storage does not reset the bit.
        // 1 - Send internally generated STOP after sending CCSD to CE-ATA device.
        send_auto_stop_ccsd:u1,
        //
        // 0 - Clear bit if DWC_mobile_storage does not reset the bit.
        //
        // 1 - Send Command Completion Signal Disable (CCSD) to CE-ATA device
        send_ccsd:u1,
        // 0 - No change
        //
        // 1 - After suspend command is issued during read-transfer, software polls card to
        // find when suspend happened. Once suspend occurs, software sets bit to reset data state-machine,
        // which is waiting for next block of data. Bit automatically clears once data state­machine resets to idle.
        //
        // Used in SDIO card suspend sequence.
        abort_read_data:u1,
        //
        // 0 - No change
        //
        // 1 - Send auto IRQ response
        //
        // Bit automatically clears once response is sent.
        //
        // To wait for MMC card interrupts, host issues CMD40, and DWC_mobile_storage waits for
        // interrupt response from MMC card(s). In meantime, if host wants DWC_mobile_storage
        // to exit waiting for interrupt state, it can set this bit, at which time DWC_mobile_storage
        // command state-machine sends CMD40 response on bus and returns to idle state.
        send_irq_response:u1,
        //
        // 0 - Clear read wait
        //
        // 1 - Assert read wait For sending read-wait to SDIO cards.
        read_wait:u1,
        //
        // 0 - Disable DMA transfer mode
        //
        // 1 - Enable DMA transfer mode
        dma_enable:u1,
        //
        // Global interrupt enable/disable bit:
        //
        // 0 - Disable interrupts
        //
        // 1 - Enable interrupts
        //
        // The int port is 1 only when this bit is 1 and one or more unmasked interrupts are set.
        int_enable:u1,
        reserved2:u1,
        // 0 - No change
        //
        // 1 - Reset internal DMA interface control logic
        //
        // To reset DMA interface, firmware should set bit to 1. This bit is auto-cleared after two AHB clocks.
        dma_reset:u1,
        // 0 - No change
        //
        // 1 - Reset to data FIFO To reset FIFO pointers
        //
        // To reset FIFO, firmware should set bit to 1. This bit is auto-cleared after completion of reset operation.
        fifo_reset:u1,
        //
        // 0 - No change
        //
        // 1 - Reset DWC_mobile_storage controller
        controller_reset:u1,
    }


    pub struct BusModeReg(u32){
        reserved:u21,
        // Programmable Burst Length. These bits indicate the maximum number of beats to be performed
        // in one IDMAC transaction. The IDMAC will always attempt to burst as specified in PBL
        // each time it starts a Burst transfer on the host bus.
        // The permissible values are 1,4, 8, 16, 32, 64, 128 and 256.
        // This value is the mirror of MSIZE of FIFOTH register. In order to change this value,
        // write the required value to FIFOTH register. This is an encode value as follows.
        pbl:u3,
        // IDMAC Enable. When set, the IDMAC is enabled.
        // DE is read/write.
        de:u1,
        // 	Descriptor Skip Length. Specifies the number of HWord/Word/Dword (depending on 16/32/64-bit bus)
        // to skip between two unchained descriptors. This is applicable only for dual buffer structure.
        // DSL is read/write.
        dsl:u5,
        fd:u1,
        // Software Reset. When set, the DMA Controller resets all its internal registers.
        // SWR is read/write. It is automatically cleared after 1 clock cycle.
        swr:u1,
    }



    pub struct  ClockDividerReg(u32){
        clk_divider3:u8,
        clk_divider2:u8,
        clk_divider1:u8,
        // Clock divider-0 value. Clock division is 2* n. For example, value of 0 means
        //
        // divide by 2*0 = 0 (no division, bypass), value of 1 means divide by 2*1 = 2,
        // value of “ff” means divide by 2*255 = 510, and so on.
        clk_divider0:u8,
    }


    pub struct StatusReg(u32){
        // DMA request signal state; either dw_dma_req or ge_dma_req, depending on DW-DMA or Generic-DMA selection.
        dma_req:u1,
        //DMA acknowledge signal state; either dw_dma_ack or ge_dma_ack, depending on DW-DMA or Generic-DMA selection.
        dma_ack:u1,
        // FIFO count - Number of filled locations in FIFO
        fifo_count:u13,
        // Index of previous response, including any auto-stop sent by core.
        response_index:u6,
        // Data transmit or receive state-machine is busy
        data_state_mc_busy:u1,
        // Inverted version of raw selected card_data[0] 0 - card data not busy 1 - card data busy
        data_busy:u1,
        // Raw selected card_data[3]; checks whether card is present 0 - card not present
        //
        // 1 - card present
        data_3_status:u1,
        command_fsm_states:u4,
        //  	FIFO is full status
        fifo_full:u1,
        fifo_empty:u1,
        // FIFO reached Transmit watermark level; not qualified with data
        //
        // transfer.
        fifo_tx_watermark:u1,
        //
        // FIFO reached Receive watermark level; not qualified with data
        //
        // transfer.
        fifo_rx_watermark:u1,
    }

    pub struct RawInterruptStatusReg(u32){
        // Interrupt from SDIO card; one bit for each card. Bit[31] corresponds to Card[15],
        // and bit[16] is for Card[0]. Writes to these bits clear them. Value of 1 clears bit and 0 leaves bit intact.
        //
        // 0 - No SDIO interrupt from card
        //
        // 1 - SDIO interrupt from card
        sdiojnterrupt:u16,
        // Writes to bits clear status bit. Value of 1 clears status bit, and value of 0 leaves bit intact.
        // Bits are logged regardless of interrupt mask status.
        int_status:u16,
    }

    pub struct RawInterrupt(u16){
        // End-bit error (read)/write no CRC (EBE)
        ebe:u1,
        // Auto command done (ACD)
        acd:u1,
        // Start-bit error (SBE) /Busy Clear Interrupt (BCI)
        sbe:u1,
        // Hardware locked write error (HLE)
        hle:u1,
        // FIFO underrun/overrun error (FRUN)
        frun:u1,
        // Data starvation-by-host timeout (HTO) /Volt_switch_int
        hto:u1,
        // Data read timeout (DRTO)/Boot Data Start (BDS)
        drto:u1,
        // Response timeout (RTO)/Boot Ack Received (BAR)
        rto:u1,
        // Data CRC error (DCRC)
        dcrc:u1,
        // Response CRC error (RCRC)
        rcrc:u1,
        // Receive FIFO data request (RxDR)
        rxdr:u1,
        // Transmit FIFO data request (TXDR)
        txdr:u1,
        // Data transfer over (DtO)
        dto:u1,
        // Command done (CD)
        command_done:u1,
        // Response error (RE)
        response_err:u1,
        // Card detect (Cd)
        card_dectect:u1,
    }

}


// mid:u8,
// oid:u16,
// pnm:u32,
// prv:u8,
// psn:u32,
// reserved:u4,
// mdt:u12,
// crc:u7,
// zero:u1,
pub struct Cid(u128);
impl Cid{
    pub fn new(value:u128) -> Self {
        Cid(value)
    }
    pub fn fmt(&self) -> String {
        let mid = self.0.get_bits(120..=127) as u8;
        let oid = self.0.get_bits(104..=119) as u16;// 2char
        let oid = core::str::from_utf8(&oid.to_be_bytes()).unwrap().to_string();
        let pnm = self.0.get_bits(64..=103) as u64;// 5char
        let pnm = core::str::from_utf8(&pnm.to_be_bytes()[0..5]).unwrap().to_string();
        let prv_big = self.0.get_bits(60..=63) as u8;//
        let prv_small = self.0.get_bits(56..=59) as u8;//
        let prv = format!("{}.{}",prv_big,prv_small);
        let psn = self.0.get_bits(24..=55) as u32;//
        let year = self.0.get_bits(12..=19) as u8;//
        let month = self.0.get_bits(8..=11) as u8;//
        let mdt = format!("{}-{}",year as usize + 2000,month);
        let res = format!("mid:{} oid:{} pnm:{} prv:{} psn:{} mdt:{}",mid,oid,pnm,prv,psn,mdt);
        res
    }

    pub fn mid(&self) -> u8 {
        self.0.get_bits(120..=127) as u8
    }
    pub fn oid(&self) -> String {
        let oid = self.0.get_bits(104..=119) as u16;// 2char
        let oid = core::str::from_utf8(&oid.to_be_bytes()).unwrap().to_string();
        oid
    }
    pub fn pnm(&self) -> String {
        let pnm = self.0.get_bits(64..=103) as u64;// 5char
        let pnm = core::str::from_utf8(&pnm.to_be_bytes()[0..5]).unwrap().to_string();
        pnm
    }
    pub fn prv(&self) -> String {
        let prv_big = self.0.get_bits(60..=63) as u8;//
        let prv_small = self.0.get_bits(56..=59) as u8;//
        let prv = format!("{}.{}",prv_big,prv_small);
        prv
    }
    pub fn psn(&self) -> u32 {
        self.0.get_bits(24..=55) as u32
    }
    pub fn mdt(&self) -> String {
        let year = self.0.get_bits(12..=19) as u8;//
        let month = self.0.get_bits(8..=11) as u8;//
        let mdt = format!("{}-{}",year as usize + 2000,month);
        mdt
    }
}

impl RawInterrupt{
    pub fn have_error(&mut self) -> bool {
        self.rto().get_raw() == 1
            || self.dcrc().get_raw() == 1
            || self.response_err().get_raw() == 1
            || self.drto().get_raw() ==1
            || self.sbe().get_raw() ==1
            || self.ebe().get_raw() ==1
    }
}


impl CmdReg {
    pub fn default(card_number:usize,cmd_number:u8)->Self{
        let mut cmd = CmdReg::try_from(0).unwrap();
        cmd.start_cmd().set(u1!(1));
        cmd.use_hold_reg().set(u1!(1));
        cmd.response_expect().set(u1!(1));
        cmd.wait_prvdata_complete().set(u1!(1));
        cmd.check_response_crc().set(u1!(1));
        let c_number = u5::new(card_number as u8).unwrap();
        cmd.card_number().set(c_number);
        let cmd_index = u6::new(cmd_number).unwrap();
        cmd.cmd_index().set(cmd_index);
        cmd
    }
    pub fn with_no_data(card_number:usize,cmd_number:u8) -> Self {
        let cmd = CmdReg::default(card_number,cmd_number);
        cmd
    }

    pub fn with_data(card_number:usize,cmd_number:u8)->Self{
        let mut cmd = CmdReg::default(card_number,cmd_number);
        cmd.data_expected().set(u1!(1));
        cmd
    }
}

impl From<Cmd> for CmdReg {
    fn from(value: Cmd) -> Self {
        match value {
            Cmd::GoIdleState => {
                let mut cmd0 = CmdReg::with_no_data(0, value.into());
                cmd0.send_initialization().set(u1!(1));
                cmd0
            }
            Cmd::SendIfCond
            |Cmd::AppCmd
            |Cmd::SendRelativeAddr
            |Cmd::SelectCard=> {
                let cmd = CmdReg::with_no_data(0, value.into());
                cmd
            }
            Cmd::SdSendOpCond => {
                let mut cmd41 = CmdReg::with_no_data(0, value.into());
                cmd41.check_response_crc().set(u1!(0));
                cmd41
            }
            Cmd::SendCsd => {
                let mut cmd9 = CmdReg::with_no_data(0, value.into());
                cmd9.check_response_crc().set(u1!(0));
                cmd9
            }
            Cmd::AllSendCid => {
                let mut cmd2 = CmdReg::with_no_data(0, value.into());
                cmd2.check_response_crc().set(u1!(0));
                cmd2.response_length().set(u1!(1)); // long response
                cmd2
            }
            Cmd::SendScr
            |Cmd::ReadSingleBlock=> {
                let cmd = CmdReg::with_data(0, value.into());
                cmd
            }
            Cmd::WriteSingleBlock => {
                let mut cmd = CmdReg::with_data(0, value.into());
                cmd.transfer_dir().set(u1!(1)); // write to card
                cmd
            }
            _ => {
                panic!("Not implemented")
            }
        }
    }
}