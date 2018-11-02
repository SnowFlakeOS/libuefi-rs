// =======================================================================
//  Copyleft SnowFlakeOS Team 2018-∞.
//  Distributed under the terms of the 3-Clause BSD License.
//  (See accompanying file LICENSE or copy at
//   https://opensource.org/licenses/BSD-3-Clause)
// =======================================================================

//! Some code was borrowed from [uefi-rs](https://github.com/GabrielMajeri/uefi-rs)

// See also http://wiki.phoenix.com/wiki/index.php/EFI_BOOT_SERVICES

//* Use from external library *//
use core::{mem, ptr};

//* Use from local library *//
use super::TableHeader;
use protocols::Protocol;
use protocols::device_path::DevicePath;
use status::{Result, Status};
use utility::{Void,
              PoolPointer,
              EventNotifyFcn,
              Event,
              Guid,
              Handle,
              PhysAddress, 
              MemoryType, 
              MemoryDescriptor};

#[repr(C)]
pub struct BootServices {
    pub hdr: TableHeader,
    
    /* --- Tpl --- */
    /// Raises a task’s priority level and returns its previous level
    raise_tpl: efi_fcn! { fn(new_tpl: Tpl) -> Tpl },
    /// Restores a task’s priority level to its previous value
    restore_tpl: efi_fcn! { fn(old_tpl: Tpl) -> Tpl },

    /* --- Memory Allocate --- */
    /// Allocates memory pages from the system
    pub allocate_pages: efi_fcn! { fn(alloc_type: AllocType, mem_type: MemoryType, count: usize, addr: &mut PoolPointer<Void>) -> Status },
    /// Frees memory pages
    pub free_pages: efi_fcn! { fn(addr: PhysAddress, count: usize) -> Status },
    /// Returns the current memory map
    pub get_memory_map: efi_fcn! { fn(size: &mut usize, map: &mut MemoryDescriptor, key: &mut usize, desc_size: &mut usize, desc_version: &mut usize) -> Status },
    /// Allocates pool memory
    pub allocate_pool: efi_fcn! { fn(pool_type: MemoryType, size: usize, addr: &mut PhysAddress) -> Status },
    /// Returns pool memory to the system
    pub free_pool: efi_fcn! { fn(addr: &mut PhysAddress) -> Status },
    
    /* --- Event & Timer --- */
    /// Creates an event
    pub create_event: efi_fcn! { fn(event_type: u32, notify_tpl: Tpl, notify_function: Option<EventNotifyFcn>, notify_context: Option<PoolPointer<Void>>, event: &mut Event) -> Status },
    /// Sets the type of timer and the trigger time for a timer event
    pub set_timer: efi_fcn! { fn(event: Event, delay_type: TimerDelay, trigger_time: u64) -> Status },
    /// Stops execution until an event is signaled
    pub wait_for_event: efi_fcn! { fn(number_of_events: usize, event: *mut Event, index: &mut usize) -> Status },
    // TODO: Add description
    pub signal_event: efi_fcn! { fn(event: Event) -> Status },
    pub close_event: efi_fcn! { fn(event: Event) -> Status },
    pub check_event: efi_fcn! { fn(event: Event) -> Status },

    /* --- Protocol --- */
    pub install_protocol_interface: efi_fcn! { fn(handle: &mut Handle, protocol: &Guid, interface_type: InterfaceType, interface: PoolPointer<Void>) -> Status },
    pub reinstall_protocol_interface: efi_fcn! { fn(handle: Handle, protocol: &Guid, old_interface: PoolPointer<Void>, new_interface: PoolPointer<Void>) -> Status },
    pub uninstall_protocol_interface: efi_fcn! { fn(handle: Handle, protocol: &Guid, interface: PoolPointer<Void>) -> Status },
    /// Queries a handle to determine if it supports a specified protocol
    pub handle_protocol: efi_fcn! { fn(handle: Handle, protocol: &Guid, interface: &mut usize) -> Status },
    _reserved: usize,
    /// Creates an event that is to be signaled whenever an interface is installed for a specified protocol
    pub register_protocol_notify: efi_fcn! { fn(protocol: &Guid, event: Event, registration: PoolPointer<Void>) -> Status },
    /// Returns an array of handles that support a specified protocol
    pub locate_handle: efi_fcn! { fn(search_type: i32, protocol: &Guid, search_key: PoolPointer<Void>, buffer_size: &mut usize, buffer: *mut Handle) -> Status },
    pub locate_device_path: usize,
    pub install_configuration_table: usize,

    /* --- Image services --- */
    /// Loads an EFI image into memory
    pub load_image: efi_fcn! { fn(boot_policy: bool, parent_image_handle: Handle, device_path: &mut DevicePath, source_buffer: PoolPointer<Void>, source_size: usize, image_handle: &mut Handle) -> Status },
    /// Transfers control to a loaded image’s entry point
    pub start_image: efi_fcn! { fn(image_handle: Handle, exit_data_size: usize, exit_data: &mut PoolPointer<u16>) -> Status },
    pub exit: efi_fcn! { fn(image_handle: Handle, exit_status: Status, exit_data_size: usize, exit_data: &u16) -> Status },
    pub unload_image: efi_fcn! { fn(image_handle: Handle) -> Status },
    pub exit_boot_services: efi_fcn! { fn(image_handle: Handle, map_key: usize) -> Status },

    /* --- Other services --- */
    /// Returns a monotonically increasing count for the platform
    pub get_next_monotonic_count: efi_fcn! { fn(count: &mut u64) -> Status },
    /// Induces a fine-grained stall
    pub stall: efi_fcn! { fn(microseconds: usize) -> Status },
    pub set_watchdog_timer: efi_fcn! { fn(timeout: usize, watchdog_code: u64, data_size: usize, watchdog_data: &u16) -> Status },
    
    /* --- Driver support services --- */
    /// Connects one or more drivers to a controller
    pub connect_controller: efi_fcn! { fn(controller_handle: Handle, driver_image_handle: &Handle, remaining_device_path: &DevicePath, recursive: bool) -> Status },
    /// Disconnects one or more drivers from a controller
    pub disconnect_controller: efi_fcn! { fn(controller_handle: Handle, driver_image_handle: Handle, child_handle: Handle) -> Status },
    
    /* --- Open and Close Protocol Services --- */
    pub open_protocol: efi_fcn! { fn(handle: Handle, protocol: &Guid, interface: PoolPointer<Void>, agent_handle: Handle, controller_handle: Handle, attributes: ProtocolAttribute) -> Status },
    pub close_protocol: efi_fcn! { fn(handle: Handle, protocol: &Guid, agent_handle: Handle, controller_handle: Handle) -> Status },

    /* --- Library Services --- */
    pub protocol_per_handle: efi_fcn! { fn(handle: Handle, protocol_buffer: &mut Guid, protocol_buffer_count: &mut usize) -> Status },
    pub locate_handle_buffer: efi_fcn! { fn(search_type: i32, protocol: &Guid, search_key: *const Void, no_handles: &mut *mut usize, buffer: &mut *mut Handle) -> Status },
    pub locate_protocol: efi_fcn! { fn(protocol: *const Guid, registration: *const Void, interface: *mut Void) -> Status },
    pub install_multiple_protocol_interfaces: efi_fcn! { fn() -> Status },
    pub uninstall_multiple_protocol_interfaces: efi_fcn! { fn() -> Status },

    /* --- 32-bit CRC Services --- */
    pub calculate_crc32: efi_fcn! { fn(data: *const Void, data_size: usize, crc32: &mut u32) -> Status },
    
    /* --- Other Services --- */
    pub copy_mem: efi_fcn! { fn(destination: *const Void, source: *const Void, lengh: usize) -> Status },
    pub set_mem: efi_fcn! { fn(buffer: *const Void, size: usize, value: u8) -> Status },
    pub create_event_ex: efi_fcn! { fn(ty: u32, notify_tpl: Tpl, notify_function: Option<EventNotifyFcn>, notify_content: Option<*const Void>, event_group: Option<*const Guid>, event: *mut Event) -> Status }
}

impl BootServices {
    /* --- Tpl --- */
    /// Raises a task’s priority level and returns its previous level
    pub unsafe fn raise_tpl(&self, new_tpl: Tpl) -> Tpl {
        (self.raise_tpl)(new_tpl)
    }

    /// Restores a task’s priority level to its previous value
    pub unsafe fn restore_tpl(&self, old_tpl: Tpl) -> Tpl {
        (self.restore_tpl)(old_tpl)
    }

    /* --- Memory Allocate --- */
    pub unsafe fn allocate_pool(&self, pool_type: MemoryType, size: usize) -> Result<PhysAddress> {
        let mut buffer = PhysAddress::new(0);
        (self.allocate_pool)(pool_type, size, &mut buffer).into_with(|| buffer)
    }

    pub unsafe fn free_pool(&self, address: PhysAddress) -> Result<()> {
        let mut addr = address.clone();
        (self.free_pool)(&mut addr).into()
    }

    /* --- Protocol --- */
    /// Queries a handle to determine if it supports a specified protocol
    pub unsafe fn handle_protocol<P: Protocol>(&self, handle: Handle) -> Option<ptr::NonNull<P>> {
        let mut ptr = 0usize;
        match (self.handle_protocol)(handle, &P::GUID, &mut ptr) {
            Status::Success => ptr::NonNull::new(ptr as *mut P),
            _ => None,
        }
    }

    /// Returns an array of handles that support a specified protocol
    pub unsafe fn locate_handle(&self, search_type: LocateSearchType, output: Option<&mut [Handle]>) -> Result<usize> {
        let handle_size = mem::size_of::<Handle>();

        const NULL_BUFFER: *mut Handle = ptr::null_mut();

        let (mut buffer_size, buffer) = match output {
            Some(buffer) => (buffer.len() * handle_size, buffer.as_mut_ptr()),
            None => (0, NULL_BUFFER),
        };

        // Obtain the needed data from the parameters.
        let (search_type, guid, key) = match search_type {
            LocateSearchType::ByProtocol(guid) => (2, guid as *const _, ptr::null_mut()),
            _ => (0, ptr::null(), ptr::null_mut()),
        };

        let status = (self.locate_handle)(search_type, &*guid, key, &mut buffer_size, buffer);

        // Must convert the returned size (in bytes) to length (number of elements).
        let buffer_len = buffer_size / handle_size;

        match (buffer, status) {
            (NULL_BUFFER, Status::BufferTooSmall) => Ok(buffer_len.into()),
            (_, other_status) => other_status.into_with(|| buffer_len),
        }
    }

    /* --- Other services --- */
    pub unsafe fn stall(&self, microseconds: usize) -> Result<()> {
        (self.stall)(microseconds).into()
    }
}

#[repr(u32)]
pub enum ProtocolAttribute {
    /// Used in the implementation of HandleProtocol(). 
    /// Since OpenProtocol() performs the same function as HandleProtocol() with additional functionality, 
    /// HandleProtocol() can simply call OpenProtocol() with this Attributes value. 
    ByHandleProtocol = 0x00000001,
    ///	Specifies the protocol to search by. 
    /// This parameter is only valid if SearchType is ByProtocol. 
    GetProtocol = 0x00000002,
    /// Used by a driver to test for the existence of a protocol interface on a handle. 
    /// Interface is optional for this attribute value, so it is ignored,
    /// and the caller should only use the return status code. 
    /// The caller is also not required to close the protocol interface with CloseProtocol().
    TestProtocol = 0x00000004,
    /// Used by bus drivers to show that a protocol interface is being used by one of the child controllers of a bus. 
    /// This information is used by the boot service EFI_BOOT_SERVICES.ConnectController() to recursively connect 
    /// all child controllers and by the boot service EFI_BOOT_SERVICES.DisconnectController() to get the list of child controllers that a bus driver created.
    ByChildProtocol = 0x00000008,
    /// Used by a driver to gain access to a protocol interface. 
    /// When this mode is used, the driver’s Stop() function will be called by EFI_BOOT_SERVICES.DisconnectController() 
    /// if the protocol interface is reinstalled or uninstalled. 
    /// Once a protocol interface is opened by a driver with this attribute, no other drivers will be allowed to open the same protocol interface with the BY_DRIVER attribute.
    ByDriver = 0x00000010,
    /// Used by applications to gain exclusive access to a protocol interface.
    /// If any drivers have the protocol interface opened with an attribute of BY_DRIVER, then an attempt will be made to remove them by calling the driver’s Stop() function.
    Exclusive = 0x00000020
}

#[repr(C)]
pub enum TimerDelay {
	Cancel,
	Periodic,
	Relative
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub enum LocateSearchType<'a> {
    AllHandles,
    ByRegisterNotify,
    ByProtocol(&'a Guid)
}

impl<'a> LocateSearchType<'a> {
    /// Constructs a new search type for a specified protocol.
    pub fn from_proto<P: Protocol>() -> Self {
        LocateSearchType::ByProtocol(&P::GUID)
    }
}

#[repr(C)]
pub enum InterfaceType {
    Native
}

#[repr(C)]
pub enum AllocType {
	AnyPages,
	MaxAddress,
	Address
}

#[derive(Debug, Copy, Clone)]
#[repr(usize)]
pub enum Tpl {
    /// Normal task execution level.
    Application = 4,
    /// Async interrupt-style callbacks run at this TPL.
    Callback = 8,
    /// Notifications are masked at this level.
    ///
    /// This is used in critical sections of code.
    Notify = 16,
    /// Highest priority level.
    ///
    /// Even processor interrupts are disable at this level.
    HighLevel = 31,
}