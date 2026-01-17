use x86_64::instructions::port::{Port, PortReadOnly};

#[derive(Debug, Clone)]
pub struct AtaDeviceIdentity{
    pub model_number: [u8; 40],
    pub serial_number: [u8; 20]
}

impl AtaDeviceIdentity {
    pub fn from_buffer(buffer: &[u16; 256]) -> Result<Self, &'static str>{
        let mut model = [0u8; 40];

        let mut current_char = 0;
        for i in 27..=46{
            let word = buffer[i];

            let first_char = (word >> 8) as u8;
            let second_char = (word & 0x00FF) as u8;

            model[current_char] = first_char;
            model[current_char + 1] = second_char;
            current_char += 2;
        }

        let mut serial = [0u8; 20];
        current_char = 0;
        for i in 10..=19{
            let word = buffer[i];


            let first_char = (word >> 8) as u8;
            let second_char = (word & 0x00FF) as u8;

            serial[current_char] = first_char;
            serial[current_char + 1] = second_char;
            current_char += 2;
        }

        Ok(AtaDeviceIdentity{
            model_number: model,
            serial_number: serial,
        })
    }
}


pub fn identify() -> Result<AtaDeviceIdentity, &'static str>{
    // PSST! These comments may seem AI-generated, but these are comments are copy-pasted from OSDev Wiki!
    // Source: https://wiki.osdev.org/ATA_PIO_Mode#IDENTIFY_command

    unsafe {

        // To use the IDENTIFY command, select a target drive by sending 0xA0 for the master drive,
        // or 0xB0 for the slave, to the "drive select" IO port. On the Primary bus, this would be port 0x1F6
        let mut drive_sel_port = Port::<u8>::new(0x1F6);
        drive_sel_port.write(0xA0);

        // Then set the Sectorcount, LBAlo, LBAmid, and LBAhi IO ports to 0 (port 0x1F2 to 0x1F5).
        for i in 0x1F2..=0x1F5{
            let mut port = Port::<u8>::new(i);
            port.write(0);
        }

        // Then send the IDENTIFY command (0xEC) to the Command IO port (0x1F7).
        let mut cmd_port = Port::<u8>::new(0x1F7);
        cmd_port.write(0xEC);

        // Then read the Status port (0x1F7) again
        loop {
            let status = cmd_port.read();

            // If the value read is 0, the drive does not exist. For any other value:
            // poll the Status port (0x1F7) until bit 7 (BSY, value = 0x80) clears
            if status == 0 {
                return Err("No Drive found on Primary ATA Bus");
            }
            if (status & 0x80) == 0 { break }
        }

        // Because of some ATAPI drives that do not follow spec, at this point you need to check
        // the LBAmid and LBAhi ports (0x1F4 and 0x1F5) to see if they are non-zero. If so,
        // the drive is not ATA, and you should stop polling.
        for i in [0x1F4, 0x1F5]{
            let mut val = PortReadOnly::<u8>::new(i);
            if val.read() != 0 {
                return Err("Drive is not ATA(likely ATAPI)");
            }
        }

        // Otherwise, continue polling one of the Status ports until bit 3 (DRQ, value = 8) sets,
        // or until bit 0 (ERR, value = 1) sets.
        let mut status_port = Port::<u8>::new(0x1F7);
        loop{
            let status_info = status_port.read();
            if (status_info & 0x08) != 0  || (status_info & 0x01) != 0 { break; }
        }

        let final_status = status_port.read();
        if (final_status & 0x01) != 0 {
            return Err("Drive reported an error after IDENTIFY command.");
        }

        // At that point, if ERR is clear, the data is ready to read from the Data port (0x1F0).
        // Read 256 16-bit values and store them.
        let mut data_port = Port::<u16>::new(0x1F0);
        let mut buffer : [u16; 256] = [0; 256];

        for i in 0..256{
            buffer[i] = data_port.read();
        }

        status_port.read();

        AtaDeviceIdentity::from_buffer(&buffer)
    }
}