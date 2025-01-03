pub struct BytePacketBuffer {
    pub buf: [u8; 512],
    pub pos: usize, // Current position in the buffer
}

impl Default for BytePacketBuffer {
    fn default() -> Self {
        BytePacketBuffer::new()
    }
}

impl BytePacketBuffer {
    // Initialize a new buffer
    pub fn new() -> BytePacketBuffer {
        BytePacketBuffer {
            buf: [0; 512],
            pos: 0,
        }
    }

    // Get the current position
    pub fn pos(&self) -> usize {
        self.pos
    }

    // Advance the buffer position by a specific number of steps
    pub fn step(&mut self, steps: usize) -> Result<(), Box<dyn std::error::Error>> {
        self.pos += steps;
        if self.pos >= 512 {
            return Err("End of buffer".into());
        }
        Ok(())
    }

    // Change the buffer position
    pub fn seek(&mut self, pos: usize) -> Result<(), Box<dyn std::error::Error>> {
        self.pos = pos;
        if self.pos >= 512 {
            return Err("End of buffer".into());
        }
        Ok(())
    }

    // Read a single byte from the buffer and advance the position
    pub fn read(&mut self) -> Result<u8, Box<dyn std::error::Error>> {
        if self.pos >= 512 {
            return Err("End of buffer".into());
        }
        let res = self.buf[self.pos];
        self.pos += 1;
        Ok(res)
    }

    // Get a single byte without changing the buffer position
    pub fn get(&self, pos: usize) -> Result<u8, Box<dyn std::error::Error>> {
        if pos >= 512 {
            return Err("End of buffer".into());
        }
        Ok(self.buf[pos])
    }

    // Get a range of bytes from the buffer
    pub fn get_range(&self, start: usize, len: usize) -> Result<&[u8], Box<dyn std::error::Error>> {
        if start + len > 512 {
            return Err("End of buffer".into());
        }
        Ok(&self.buf[start..start + len])
    }

    // Read two bytes and interpret as a u16 in network byte order
    pub fn read_u16(&mut self) -> Result<u16, Box<dyn std::error::Error>> {
        let res = u16::from(self.read()?) << 8 | u16::from(self.read()?);
        Ok(res)
    }

    // Read four bytes and interpret as a u32 in network byte order
    pub fn read_u32(&mut self) -> Result<u32, Box<dyn std::error::Error>> {
        let res = u32::from(self.read()?) << 24
            | u32::from(self.read()?) << 16
            | u32::from(self.read()?) << 8
            | u32::from(self.read()?);
        Ok(res)
    }

    // Read a domain name from the buffer
    pub fn read_qname(&mut self, outstr: &mut String) -> Result<(), Box<dyn std::error::Error>> {
        let mut pos = self.pos;
        let mut jumped = false;
        let max_jumps = 5;
        let mut jumps = 0;
        let mut delim = "";

        loop {
            if jumps > max_jumps {
                return Err("Limit of jumps exceeded! Possible loop detected.".into());
            }

            let len = self.get(pos)?;
            if (len & 0xC0) == 0xC0 {
                if !jumped {
                    self.seek(pos + 2)?;
                }
                let b2 = self.get(pos + 1)? as u16;
                let offset = (((len as u16) ^ 0xC0) << 8) | b2;
                pos = offset as usize;
                jumped = true;
                jumps += 1;
                continue;
            } else {
                pos += 1;
                if len == 0 {
                    break;
                }
                outstr.push_str(delim);
                let str_buffer = self.get_range(pos, len as usize)?;
                outstr.push_str(&String::from_utf8_lossy(str_buffer).to_lowercase());
                delim = ".";
                pos += len as usize;
            }
        }
        if !jumped {
            self.seek(pos)?;
        }
        Ok(())
    }

    // Write a single byte to the buffer and advance the position
    pub fn write(&mut self, val: u8) -> Result<(), Box<dyn std::error::Error>> {
        if self.pos >= 512 {
            return Err("End of buffer".into());
        }
        self.buf[self.pos] = val;
        self.pos += 1;
        Ok(())
    }

    // Write a u8 to the buffer
    pub fn write_u8(&mut self, val: u8) -> Result<(), Box<dyn std::error::Error>> {
        self.write(val)?;
        Ok(())
    }

    // Write a u16 to the buffer in network byte order
    pub fn write_u16(&mut self, val: u16) -> Result<(), Box<dyn std::error::Error>> {
        self.write(((val >> 8) & 0xFF) as u8)?;
        self.write((val & 0xFF) as u8)?;
        Ok(())
    }

    // Write a u32 to the buffer in network byte order
    pub fn write_u32(&mut self, val: u32) -> Result<(), Box<dyn std::error::Error>> {
        self.write(((val >> 24) & 0xFF) as u8)?;
        self.write(((val >> 16) & 0xFF) as u8)?;
        self.write(((val >> 8) & 0xFF) as u8)?;
        self.write((val & 0xFF) as u8)?;
        Ok(())
    }

    // Write a qname to the buffer
    pub fn write_qname(&mut self, qname: &str) -> Result<(), Box<dyn std::error::Error>> {
        for part in qname.split('.') {
            if part.len() > 63 {
                return Err("Label too long".into());
            }
            self.write(part.len() as u8)?;
            for c in part.chars() {
                self.write(c as u8)?;
            }
        }
        self.write(0)?;
        Ok(())
    }

    pub fn set(&mut self, pos: usize, val: u8) -> Result<(),Box<dyn std::error::Error>> {
        self.buf[pos] = val;

        Ok(())
    }

    pub fn set_u16(&mut self, pos: usize, val: u16) -> Result<(),Box<dyn std::error::Error>> {
        self.set(pos, (val >> 8) as u8)?;
        self.set(pos + 1, (val & 0xFF) as u8)?;

        Ok(())
    }
}
