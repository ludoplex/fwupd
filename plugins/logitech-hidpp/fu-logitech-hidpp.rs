// Copyright (C) 2023 Richard Hughes <richard@hughsie.com>
// SPDX-License-Identifier: LGPL-2.1+

#[derive(ToString)]
enum LogitechHidppFeature {
    Root                  = 0x0000,
    IFeatureSet           = 0x0001,
    IFirmwareInfo         = 0x0003,
    GetDeviceNameType     = 0x0005,
    DfuControl            = 0x00C1,
    DfuControlSigned      = 0x00C2,
    DfuControlBolt        = 0x00C3,
    Dfu                   = 0x00D0,
    BatteryLevelStatus    = 0x1000,
    UnifiedBattery        = 0x1004,
    KbdReprogrammableKeys = 0x1B00,
    SpecialKeysButtons    = 0x1B04,
    MousePointerBasic     = 0x2200,
    AdjustableDpi         = 0x2201,
    AdjustableReportRate  = 0x8060,
    ColorLedEffects       = 0x8070,
    OnboardProfiles       = 0x8100,
    MouseButtonSpy        = 0x8110,
}

enum LogitechHidppBootloaderCmd {
    General_error = 0x01,
    Read = 0x10,
    Write = 0x20,
    WriteInvalidAddr = 0x21,
    WriteVerifyFail = 0x22,
    WriteNonzeroStart = 0x23,
    WriteInvalidCrc = 0x24,
    ErasePage = 0x30,
    ErasePageInvalidAddr = 0x31,
    ErasePageNonzeroStart = 0x33,
    GetHwPlatformId = 0x40,
    GetFwVersion = 0x50,
    GetChecksum = 0x60,
    Reboot = 0x70,
    GetMeminfo = 0x80,
    GetBlVersion = 0x90,
    GetInitFwVersion = 0xa0,
    ReadSignature = 0xb0,
    WriteRamBuffer = 0xc0,
    WriteRamBufferInvalidAddr = 0xc1,
    WriteRamBufferOverflow = 0xc2,
    FlashRam = 0xd0,
    FlashRamInvalidAddr = 0xd1,
    FlashRamWrongCrc = 0xd2,
    FlashRamPage0Invalid = 0xd3,
    FlashRamInvalidOrder = 0xd4,
    WriteSignature = 0xe0,
}
