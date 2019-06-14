use std::ptr;
use std::mem;
use std::fs::File;
use winapi::um::winioctl::{
    FSCTL_QUERY_USN_JOURNAL,
    FSCTL_READ_USN_JOURNAL,
};
use winapi::ctypes::c_void;
use winapi::shared::ntstatus;
use std::os::windows::io::AsRawHandle;
use byteorder::{ByteOrder, LittleEndian};
use winapi::um::ioapiset::DeviceIoControl;
use crate::listener::error::Error;


#[derive(Debug, Clone)]
pub enum UsnJournalData {
    V0(UsnJournalDataV0),
    V1(UsnJournalDataV1),
    V2(UsnJournalDataV2)
}
impl UsnJournalData {
    fn new(buffer: &[u8]) -> Result<UsnJournalData, Error> {
        match buffer.len() {
            56 => {
                return Ok(
                    UsnJournalData::V0(
                        UsnJournalDataV0::new(&buffer)
                    )
                );
            },
            60 => {
                return Ok(
                    UsnJournalData::V1(
                        UsnJournalDataV1::new(&buffer)
                    )
                );
            },
            80 => {
                return Ok(
                    UsnJournalData::V2(
                        UsnJournalDataV2::new(&buffer)
                    )
                );
            },
            other => {
                return Err(
                    Error::invalid_usn_journal_data(other)
                );
            }
        }
    }

    pub fn get_next_usn(&self) -> u64 {
        match self {
            UsnJournalData::V0(jd) => jd.next_usn,
            UsnJournalData::V1(jd) => jd.next_usn,
            UsnJournalData::V2(jd) => jd.next_usn,
        }
    }
}

// Size 56
#[derive(Debug, Clone)]
pub struct UsnJournalDataV0 {
    usn_jounral_id: u64,
    first_usn: u64,
    next_usn: u64,
    lowest_valid_usn: u64,
    max_usn: u64,
    maximum_size: u64,
    allocation_delta: u64,
}
impl UsnJournalDataV0 {
    fn new(buffer: &[u8]) -> UsnJournalDataV0 {
        let usn_jounral_id = LittleEndian::read_u64(&buffer[0..8]);
        let first_usn = LittleEndian::read_u64(&buffer[8..16]);
        let next_usn = LittleEndian::read_u64(&buffer[16..24]);
        let lowest_valid_usn = LittleEndian::read_u64(&buffer[24..32]);
        let max_usn = LittleEndian::read_u64(&buffer[32..40]);
        let maximum_size = LittleEndian::read_u64(&buffer[40..48]);
        let allocation_delta = LittleEndian::read_u64(&buffer[48..56]);

        return UsnJournalDataV0 {
            usn_jounral_id,
            first_usn,
            next_usn,
            lowest_valid_usn,
            max_usn,
            maximum_size,
            allocation_delta,
        }
    }
}

// TODO: https://docs.microsoft.com/en-us/windows/desktop/api/winioctl/ns-winioctl-usn_journal_data_v1
// Size 60
#[derive(Debug, Clone)]
pub struct UsnJournalDataV1 {
    usn_jounral_id: u64,
    first_usn: u64,
    next_usn: u64,
    lowest_valid_usn: u64,
    max_usn: u64,
    maximum_size: u64,
    allocation_delta: u64,
    min_major_version: u16,
    max_major_version: u16,
}
impl UsnJournalDataV1 {
    fn new(buffer: &[u8]) -> UsnJournalDataV1 {
        let usn_jounral_id = LittleEndian::read_u64(&buffer[0..8]);
        let first_usn = LittleEndian::read_u64(&buffer[8..16]);
        let next_usn = LittleEndian::read_u64(&buffer[16..24]);
        let lowest_valid_usn = LittleEndian::read_u64(&buffer[24..32]);
        let max_usn = LittleEndian::read_u64(&buffer[32..40]);
        let maximum_size = LittleEndian::read_u64(&buffer[40..48]);
        let allocation_delta = LittleEndian::read_u64(&buffer[48..56]);
        let min_major_version = LittleEndian::read_u16(&buffer[56..58]);
        let max_major_version = LittleEndian::read_u16(&buffer[58..60]);

        return UsnJournalDataV1 {
            usn_jounral_id,
            first_usn,
            next_usn,
            lowest_valid_usn,
            max_usn,
            maximum_size,
            allocation_delta,
            min_major_version,
            max_major_version,
        }
    }
}
// TODO: https://docs.microsoft.com/en-us/windows/desktop/api/winioctl/ns-winioctl-usn_journal_data_v2
// Size 80
#[derive(Debug, Clone)]
pub struct UsnJournalDataV2 {
    usn_jounral_id: u64,
    first_usn: u64,
    next_usn: u64,
    lowest_valid_usn: u64,
    max_usn: u64,
    maximum_size: u64,
    allocation_delta: u64,
    min_major_version: u16,
    max_major_version: u16,
    flags: u32,
    range_track_chunk_size: u64,
    range_track_file_size_threshold: i64,
}
impl UsnJournalDataV2 {
    fn new(buffer: &[u8]) -> UsnJournalDataV2 {
        let usn_jounral_id = LittleEndian::read_u64(&buffer[0..8]);
        let first_usn = LittleEndian::read_u64(&buffer[8..16]);
        let next_usn = LittleEndian::read_u64(&buffer[16..24]);
        let lowest_valid_usn = LittleEndian::read_u64(&buffer[24..32]);
        let max_usn = LittleEndian::read_u64(&buffer[32..40]);
        let maximum_size = LittleEndian::read_u64(&buffer[40..48]);
        let allocation_delta = LittleEndian::read_u64(&buffer[48..56]);
        let min_major_version = LittleEndian::read_u16(&buffer[56..58]);
        let max_major_version = LittleEndian::read_u16(&buffer[58..60]);
        let flags = LittleEndian::read_u32(&buffer[60..64]);
        let range_track_chunk_size = LittleEndian::read_u64(&buffer[64..72]);
        let range_track_file_size_threshold = LittleEndian::read_i64(&buffer[72..80]);

        return UsnJournalDataV2 {
            usn_jounral_id,
            first_usn,
            next_usn,
            lowest_valid_usn,
            max_usn,
            maximum_size,
            allocation_delta,
            min_major_version,
            max_major_version,
            flags,
            range_track_chunk_size,
            range_track_file_size_threshold,
        }
    }
}

// https://docs.microsoft.com/en-us/windows/desktop/api/winioctl/ni-winioctl-fsctl_query_usn_journal
pub fn query_usn_journal(volume_handle: &File) -> Result<UsnJournalData, Error> {
    let mut output_buffer = [0u8; 80];
    let mut bytes_read = 0;

    let result = unsafe {
        DeviceIoControl(
            volume_handle.as_raw_handle(),
            FSCTL_QUERY_USN_JOURNAL,
            ptr::null_mut(),
            0,
            output_buffer.as_mut_ptr() as *mut _,
            output_buffer.len() as u32,
            &mut bytes_read,
            ptr::null_mut()
        )
    };

    if result == ntstatus::STATUS_SUCCESS {
        return Err(
            Error::from_windows_error_code(
                result as u32
            )
        );
    } else {
        return UsnJournalData::new(
            &output_buffer[..bytes_read as usize]
        );
    }
}

#[derive(Debug, Clone)]
pub enum ReadUsnJournalData {
    V0(ReadUsnJournalDataV0),
    V1(ReadUsnJournalDataV1),
}
impl ReadUsnJournalData {
    pub fn from_usn_journal_data(journal_data: UsnJournalData) -> ReadUsnJournalData {
        match journal_data {
            UsnJournalData::V0(journal_data_v0) => {
                return ReadUsnJournalData::V0(
                    ReadUsnJournalDataV0::new(
                        journal_data_v0.first_usn,
                        journal_data_v0.usn_jounral_id
                    )
                );
            },
            UsnJournalData::V1(journal_data_v1) => {
                return ReadUsnJournalData::V1(
                    ReadUsnJournalDataV1::new(
                        journal_data_v1.first_usn,
                        journal_data_v1.usn_jounral_id,
                        journal_data_v1.min_major_version,
                        journal_data_v1.max_major_version
                    )
                );
            },
            UsnJournalData::V2(journal_data_v2) => {
                return ReadUsnJournalData::V1(
                    ReadUsnJournalDataV1::new(
                        journal_data_v2.first_usn,
                        journal_data_v2.usn_jounral_id,
                        journal_data_v2.min_major_version,
                        journal_data_v2.max_major_version
                    )
                );
            }
        }
    }

    pub fn with_reason_mask(mut self, reason_mask: u32) -> Self {
        match self {
            ReadUsnJournalData::V0(ref mut read_data_v0) => {
                read_data_v0.reason_mask = reason_mask
            },
            ReadUsnJournalData::V1(ref mut read_data_v1) => {
                read_data_v1.reason_mask = reason_mask
            }
        }

        self
    }

    pub fn with_start_usn(mut self, start_usn: u64) -> Self {
        match self {
            ReadUsnJournalData::V0(ref mut read_data_v0) => {
                read_data_v0.start_usn = start_usn
            },
            ReadUsnJournalData::V1(ref mut read_data_v1) => {
                read_data_v1.start_usn = start_usn
            }
        }

        self
    }
}

// https://docs.microsoft.com/en-us/windows/desktop/api/winioctl/ns-winioctl-read_usn_journal_data_v0
#[derive(Debug, Clone)]
#[repr(C)]
pub struct ReadUsnJournalDataV0 {
    start_usn: u64,
    reason_mask: u32,
    return_only_on_close: u32,
    timeout: u64,
    bytes_to_wait_for: u64,
    usn_journal_id: u64,
}
impl ReadUsnJournalDataV0 {
    fn new(start_usn: u64, usn_journal_id: u64) -> ReadUsnJournalDataV0 {
        let reason_mask = 0xffffffff;
        let return_only_on_close = 0;
        let timeout = 0;
        let bytes_to_wait_for = 0;

        return ReadUsnJournalDataV0 {
            start_usn,
            reason_mask,
            return_only_on_close,
            timeout,
            bytes_to_wait_for,
            usn_journal_id,
        }
    }
}

// TODO: https://docs.microsoft.com/en-us/windows/desktop/api/winioctl/ns-winioctl-read_usn_journal_data_v1
#[derive(Debug, Clone)]
#[repr(C)]
pub struct ReadUsnJournalDataV1 {
    start_usn: u64,
    reason_mask: u32,
    return_only_on_close: u32,
    timeout: u64,
    bytes_to_wait_for: u64,
    usn_journal_id: u64,
    min_major_version: u16,
    max_major_version: u16,
}
impl ReadUsnJournalDataV1 {
    fn new(
        start_usn: u64, usn_journal_id: u64, 
        min_major_version: u16, max_major_version: u16
    ) -> ReadUsnJournalDataV1 {
        let reason_mask = 0xffffffff;
        let return_only_on_close = 0;
        let timeout = 0;
        let bytes_to_wait_for = 0;

        return ReadUsnJournalDataV1 {
            start_usn,
            reason_mask,
            return_only_on_close,
            timeout,
            bytes_to_wait_for,
            usn_journal_id,
            min_major_version,
            max_major_version,
        }
    }
}

pub fn read_usn_journal<'a> (
    volume_handle: &File, 
    read_jounral_data: ReadUsnJournalData, 
    record_buffer: &'a mut [u8]
) -> Result<&'a [u8], Error> {
    let mut bytes_read: u32 = 0;

    let result = match read_jounral_data {
        ReadUsnJournalData::V0(mut read_data_v0) => {
            unsafe {
                DeviceIoControl(
                    volume_handle.as_raw_handle(),
                    FSCTL_READ_USN_JOURNAL,
                    &mut read_data_v0 as *mut _ as *mut c_void,
                    mem::size_of::<ReadUsnJournalDataV0>() as u32,
                    record_buffer.as_mut_ptr() as *mut _,
                    record_buffer.len() as u32,
                    &mut bytes_read,
                    ptr::null_mut()
                )
            }
        },
        ReadUsnJournalData::V1(mut read_data_v1) => {
            unsafe {
                DeviceIoControl(
                    volume_handle.as_raw_handle(),
                    FSCTL_READ_USN_JOURNAL,
                    &mut read_data_v1 as *mut _ as *mut c_void,
                    mem::size_of::<ReadUsnJournalDataV0>() as u32,
                    record_buffer.as_mut_ptr() as *mut _,
                    record_buffer.len() as u32,
                    &mut bytes_read,
                    ptr::null_mut()
                )
            }
        },
    };

    if result == 0 {
        return Err(
            Error::from_windows_last_error()
        );
    } else {
        return Ok(
            &record_buffer[..bytes_read as usize]
        )
    }
}