/* automatically generated by rust-bindgen */

pub const SI_USER: u32 = 65537;
pub const SI_QUEUE: u32 = 65538;
pub const SI_TIMER: u32 = 65539;
pub const SI_ASYNCIO: u32 = 65540;
pub const SI_MESGQ: u32 = 65541;
pub const SI_USB_VID: u32 = 4292;
pub const SI_USB_PID: u32 = 33097;
pub const SI_SUCCESS: u32 = 0;
pub const SI_DEVICE_NOT_FOUND: u32 = 255;
pub const SI_INVALID_HANDLE: u32 = 1;
pub const SI_READ_ERROR: u32 = 2;
pub const SI_RX_QUEUE_NOT_READY: u32 = 3;
pub const SI_WRITE_ERROR: u32 = 4;
pub const SI_RESET_ERROR: u32 = 5;
pub const SI_INVALID_PARAMETER: u32 = 6;
pub const SI_INVALID_REQUEST_LENGTH: u32 = 7;
pub const SI_DEVICE_IO_FAILED: u32 = 8;
pub const SI_INVALID_BAUDRATE: u32 = 9;
pub const SI_FUNCTION_NOT_SUPPORTED: u32 = 10;
pub const SI_GLOBAL_DATA_ERROR: u32 = 11;
pub const SI_SYSTEM_ERROR_CODE: u32 = 12;
pub const SI_READ_TIMED_OUT: u32 = 13;
pub const SI_WRITE_TIMED_OUT: u32 = 14;
pub const SI_IO_PENDING: u32 = 15;
pub const SI_RETURN_SERIAL_NUMBER: u32 = 0;
pub const SI_RETURN_DESCRIPTION: u32 = 1;
pub const SI_RETURN_LINK_NAME: u32 = 2;
pub const SI_RETURN_VID: u32 = 3;
pub const SI_RETURN_PID: u32 = 4;
pub const SI_RX_NO_OVERRUN: u32 = 0;
pub const SI_RX_EMPTY: u32 = 0;
pub const SI_RX_OVERRUN: u32 = 1;
pub const SI_RX_READY: u32 = 2;
pub const SI_MAX_DEVICE_STRLEN: u32 = 256;
pub const SI_MAX_READ_SIZE: u32 = 65536;
pub const SI_MAX_WRITE_SIZE: u32 = 4096;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UsbDevHandle {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SiPrivate {
    pub magic: ::std::os::raw::c_int,
    pub udev: *mut UsbDevHandle,
    pub interface: ::std::os::raw::c_int,
    pub ep_out: ::std::os::raw::c_int,
    pub ep_in: ::std::os::raw::c_int,
    pub bufsize: ::std::os::raw::c_int,
    pub buffer: [::std::os::raw::c_char; 4096usize],
}
#[test]
fn bindgen_test_layout_si_private() {
    assert_eq!(
        ::std::mem::size_of::<SiPrivate>(),
        4128usize,
        concat!("Size of: ", stringify!(SI_Private))
    );
    assert_eq!(
        ::std::mem::align_of::<SiPrivate>(),
        8usize,
        concat!("Alignment of ", stringify!(SI_Private))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SiPrivate>())).magic as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SI_Private),
            "::",
            stringify!(magic)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SiPrivate>())).udev as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(SI_Private),
            "::",
            stringify!(udev)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SiPrivate>())).interface as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(SI_Private),
            "::",
            stringify!(interface)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SiPrivate>())).ep_out as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(SI_Private),
            "::",
            stringify!(ep_out)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SiPrivate>())).ep_in as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(SI_Private),
            "::",
            stringify!(ep_in)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SiPrivate>())).bufsize as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(SI_Private),
            "::",
            stringify!(bufsize)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SiPrivate>())).buffer as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(SI_Private),
            "::",
            stringify!(buffer)
        )
    );
}
extern "C" {
    pub fn SI_GetNumDevices(num_devices: *mut ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SI_GetProductString(
        device_num: ::std::os::raw::c_int,
        device_string: *mut ::std::os::raw::c_char,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SI_Open(
        device_num: ::std::os::raw::c_int,
        p_handle: *mut *mut SiPrivate,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SI_Close(handle: *mut SiPrivate) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SI_Read(
        handle: *mut SiPrivate,
        buffer: *mut ::std::os::raw::c_char,
        bytes_to_read: ::std::os::raw::c_int,
        bytes_returned: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SI_Write(
        handle: *mut SiPrivate,
        buffer: *mut ::std::os::raw::c_char,
        bytes_to_write: ::std::os::raw::c_int,
        bytes_written: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SI_ResetDevice(handle: *mut SiPrivate) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SI_DeviceIOControl(
        handle: *mut SiPrivate,
        io_control_code: ::std::os::raw::c_int,
        in_buffer: *mut ::std::os::raw::c_char,
        bytes_to_read: ::std::os::raw::c_int,
        out_buffer: *mut ::std::os::raw::c_char,
        bytes_to_write: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SI_FlushBuffers(
        handle: *mut SiPrivate,
        flush_transmit: ::std::os::raw::c_char,
        flush_receive: ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SI_SetTimeouts(
        read_timeout: ::std::os::raw::c_int,
        write_timeout: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SI_GetTimeouts(
        read_timeout: *mut ::std::os::raw::c_int,
        write_timeout: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn SI_CheckRXQueue(
        handle: *mut SiPrivate,
        num_bytes_in_queue: *mut ::std::os::raw::c_int,
        queue_status: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}