extern crate libc;

use std::ffi::{CString, CStr};
use std::io::{self};
use std::time::Duration;
use std::thread;

const TUNSETIFF: libc::c_ulong = 0x400454ca;

pub fn configure_tun_device(ass_ip: &str) -> io::Result<libc::c_int> {
    // Open TUN device
    let dev_name = CString::new("/dev/net/tun")?;
    let fd = unsafe { libc::open(dev_name.as_ptr(), libc::O_RDWR | libc::O_NONBLOCK) };
    if fd < 0 {
        return Err(io::Error::last_os_error());
    }

    // Set up TUN parameters
    let mut ifr: libc::ifreq = unsafe { std::mem::zeroed() };
    let flags = (libc::IFF_TUN | libc::IFF_NO_PI) as i16;
    unsafe {
        libc::strcpy(ifr.ifr_name.as_mut_ptr(), b"tun%d\0".as_ptr() as *const libc::c_char);
        ifr.ifr_ifru.ifru_flags = flags;
    }

    let ret = unsafe { libc::ioctl(fd, TUNSETIFF, &mut ifr) };
    if ret < 0 {
        return Err(io::Error::last_os_error());
    }

    // Extract the device name
    let name_ptr = ifr.ifr_name.as_ptr();
    let name_cstr = unsafe { CStr::from_ptr(name_ptr) };
    let name_str = name_cstr.to_str().unwrap();
    println!("TUN device created: {}", name_str);

    // Configure IP address using system call
    let ip_command = format!("ip addr add {} dev {}", ass_ip, name_str);
    let ip_command_cstr = CString::new(ip_command)?;
    let ret = unsafe { libc::system(ip_command_cstr.as_ptr()) };
    if ret < 0 {
        return Err(io::Error::last_os_error());
    }

    // Bring up the interface
    let up_command = format!("ip link set {} up", name_str);
    let up_command_cstr = CString::new(up_command)?;
    let ret = unsafe { libc::system(up_command_cstr.as_ptr()) };
    if ret < 0 {
        return Err(io::Error::last_os_error());
    }

    Ok(fd)
}

pub fn read_from_tun(fd: libc::c_int, buffer: &mut [u8]) -> io::Result<usize> {
    // Read data from the TUN interface
    let bytes_read = unsafe { libc::read(fd, buffer.as_mut_ptr() as *mut libc::c_void, buffer.len()) };
    if bytes_read < 0 {
        let errno = io::Error::last_os_error().raw_os_error().unwrap();
        if errno == libc::EAGAIN {
            // No data available at the moment, sleep for a short duration
            thread::sleep(Duration::from_millis(10));
            return Ok(0); // Returning Ok(0) to indicate no error and no bytes read
        } else {
            return Err(io::Error::last_os_error());
        }
    }
    Ok(bytes_read as usize)
}

pub fn write_to_tun(fd: libc::c_int, data: &[u8]) -> io::Result<usize> {
    // Write data to the TUN interface
    let bytes_written = unsafe { libc::write(fd, data.as_ptr() as *const libc::c_void, data.len()) };
    if bytes_written < 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(bytes_written as usize)
}
