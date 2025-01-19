#[allow(clippy::enum_variant_names)]
#[derive(Debug, Copy, Clone)]
pub enum Cmd {
    GoIdleState,
    AllSendCid,
    SendRelativeAddr,
    SetDSR,
    SelectCard,
    SendIfCond,
    SendCsd,
    SendCid,
    StopTransmission,
    SendStatus,
    GoInactiveState,
    SetBlockLen,
    ReadSingleBlock,
    ReadMultipleBlock,
    WriteSingleBlock,
    WriteMultipleBlock,
    EraseWrBlkStart,
    EraseWrBlkEnd,
    Erase,
    AppCmd,
    GenCmd,
    SetBusWidth,
    SdStatus,
    SendNumWrBlocks,
    SetWrBlkEraseCnt,
    SdSendOpCond,
    SetClrCardDetect,
    SendScr,
    // Private
    ResetClock,
}
impl From<Cmd> for u8 {
    fn from(val: Cmd) -> Self {
        match val {
            Cmd::GoIdleState => 0,
            Cmd::AllSendCid => 2,
            Cmd::SendRelativeAddr => 3,
            Cmd::SetDSR => 4,
            Cmd::SelectCard => 7,
            Cmd::SendIfCond => 8,
            Cmd::SendCsd => 9,
            Cmd::SendCid => 10,
            Cmd::StopTransmission => 12,
            Cmd::SendStatus => 13,
            Cmd::GoInactiveState => 15,
            Cmd::SetBlockLen => 16,
            Cmd::ReadSingleBlock => 17,
            Cmd::ReadMultipleBlock => 18,
            Cmd::WriteSingleBlock => 24,
            Cmd::WriteMultipleBlock => 25,
            Cmd::EraseWrBlkStart => 32,
            Cmd::EraseWrBlkEnd => 33,
            Cmd::Erase => 38,
            Cmd::AppCmd => 55,
            Cmd::GenCmd => 56,
            Cmd::SetBusWidth => 6,
            Cmd::SdStatus => 13,
            Cmd::SendNumWrBlocks => 22,
            Cmd::SetWrBlkEraseCnt => 23,
            Cmd::SdSendOpCond => 41,
            Cmd::SetClrCardDetect => 42,
            Cmd::SendScr => 51,
            _ => {
                panic!("Not implemented for cmd {:?}", val);
            }
        }
    }
}
