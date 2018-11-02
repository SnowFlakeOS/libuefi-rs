// =======================================================================
//  Copyleft SnowFlakeOS Team 2018-∞.
//  Distributed under the terms of the 3-Clause BSD License.
//  (See accompanying file LICENSE or copy at
//   https://opensource.org/licenses/BSD-3-Clause)
// =======================================================================

// See also http://wiki.phoenix.com/wiki/index.php/EFI_DEVICE_PATH_PROTOCOL

#[repr(C)]
pub struct DevicePath {
    ty: DevicePathType,
    sub_type: u8,
    len: [u8; 2]
}

#[repr(u8)]
pub enum DevicePathType {
    Hardware = 0x01,
    Acpi = 0x02,
    Messaging = 0x03,
    Media = 0x04,
    Bbs = 0x05,
    End = 0x7f
}

#[repr(u8)]
pub enum HardwareSubType {
    HwPCIDP = 0x01,
    HwPcCardDP = 0x02,
    HwMemMapDP = 0x03,
    HWVendorDP = 0x04,
    HwControllerDP = 0x05
}

#[repr(u8)]
pub enum AcpiSubType {
    AcpiDP = 0x01,
    AcpiExtendedDP = 0x02,
    AcpiAdrDP = 0x03
}

#[repr(u8)]
pub enum MsgSubType {
    MsgATAPIDP = 0x01,
    MsgSCSIDP = 0x02,
    MsgFibreChannelDP = 0x03,
    Msg1394DP = 0x04,
    MsgUSBDP = 0x05,
    MsgI20DP = 0x06,
    MsgInfiniBandDP = 0x09,
    MsgVendorDP = 0x0a,
    MsgMacAddrDP = 0x0b,
    MsgIPv4DP = 0x0c,
    MsgIPv6DP = 0x0d,
    MsgUartDP = 0x0e,
    MsgUsbClassDP = 0x0f,
    MsgUsbWwidDP = 0x10,
    MsgDeviceLogicalUnitDP = 0x11,
    MsgSataDP = 0x12,
    MsgIscsiDP = 0x13
}

#[repr(u8)]
pub enum MediaSubType {
    MediaHardDriveDP = 0x01,
    MediaCdRomDP = 0x02,
    MediaVendorDP = 0x03,
    MediaFilePathDP = 0x04,
    MediaProtocolDP = 0x05
}

#[repr(u8)]
pub enum BbsSubType {
    BbsBbsDP = 0x01
}

#[repr(u8)]
pub enum EndSubType {
    EndEntireDPS = 0xFF,
    EndInstanceDPS = 0x01
}