mod utils;
use std::io;

fn main() -> io::Result<()> {
    // Configure TUN device
    let fd = utils::configure_tun_device("100.0.0.10/24")?;
    let mut buffer = [0; 1504]; // Adjust the buffer size as needed

    loop {
        // Read data from the TUN interface
        let bytes_read = utils::read_from_tun(fd, &mut buffer)?;
        if bytes_read == 0 {
            continue; // No data read, continue loop
        }

        // Process or handle the read data as needed
        println!("Read {} bytes from TUN interface", bytes_read);

        // Write data to the TUN interface (for testing purposes, echo the read data back)
        let bytes_written = utils::write_to_tun(fd, &buffer[0..bytes_read])?;
        println!("Wrote {} bytes to TUN interface", bytes_written);
    }
}
