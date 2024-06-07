

pub struct BytePacketBuffer {
    pub buf : [u8;512],
    pub pos : usize //32bits on 32bit systems
}

impl BytePacketBuffer {

    //A fresh buffer for holding the packet contents
    pub fn new() -> BytePacketBuffer {
        BytePacketBuffer {
            buf : [0;512],
            pos : 0 //for keeping track of where we are in the buffer
        }
    }

    //Current position within buffer
    pub fn pos(&self) -> usize{
        self.pos
    }

    //Step the buffer position forward a specific number of steps
    pub fn step (&mut self , steps : usize) -> Result<(), Box<dyn std::error::Error>>{
        self.pos += steps;
        Ok(())
    }

    // change the buffer position
    fn seek(&mut self, pos : usize) -> Result<(), Box<dyn std::error::Error>>{
        self.pos = pos;
        Ok(())
    }

    //Read a single byte from the buffer and advance the position
    pub fn read(&mut self) -> Result<u8, Box<dyn std::error::Error>>{
        if self.pos >=512 {
            return Err("End of buffer".into());
        }
        let res = self.buf[self.pos];
        self.pos += 1;
    
        Ok(res)
    }

    // Get a single byte , without changing the buffer position
    pub fn get(&self , pos : usize) -> Result<u8, Box<dyn std::error::Error>>{
        if self.pos >= 512 {
            return Err("End of buffer".into());
        }
        Ok(self.buf[self.pos])
    }

    //Get a range of bytes from the buffer
    pub fn get_range(&self, start : usize, len : usize) -> Result<&[u8], Box<dyn std::error::Error>>{
        if start + len >= 512 {
            return Err("End of buffer".into());
        }
        Ok(&self.buf[start..start+len as usize])
    }

    //Read two bytes and interpret as a u16 in network byte order. stepping 2 steps forward
    pub fn read_u16(&mut self) -> Result<u16, Box<dyn std::error::Error>>{
        let res = u16::from(self.read()?) << 8 | u16::from(self.read()?);
        Ok(res)
    }

    //Read four bytes and interpret as a u32 in network byte order. stepping 4 steps forward
    pub fn read_u32(&mut self) -> Result<u32, Box<dyn std::error::Error>>{
        let res = u32::from(self.read()?) << 24 
        | u32::from(self.read()?) << 16 
        | u32::from(self.read()?) << 8 
        | u32::from(self.read()?);
        Ok(res)
    }

    //Read a domain name from the buffer
    //Taking lables into consideration
    //Input : something like [3]www[6]google[3]com[0]
    //Outputs : www.google.com in outstr
    pub fn read_qname (&mut self , outstr : &mut String) -> Result<(), Box<dyn std::error::Error>>{
        //to tackle the jumps in the domain name, we keep a separate buffer as well.
        //This allows us to move the shared positiom to a point past out current qname, while still being able to read the qname

        let mut pos = self.pos;

        let mut jumped = false;
        let max_jumps = 5; //to prevent infinite loops
        let mut jumps = 0;

        let mut delim = "";
        // our delimiter which we append for each label.Since we don't want a dot at the start of the domain name
        //we start with an empty string and then set it to . at the end of first itr

        loop {
            //DNS packets are kaafi problematic, hence to prevent someone from making a packet with 
            //a qname that loops forever, we have a max_jumps variable

            if jumps > max_jumps {
                return Err("Limit of jumps exceeded! Kuch toh Scam h!".into());
            }

            //Here we are the the beginning of a label. Label beings with a length byte
            let len = self.get(pos)?;

            //If the length byte has the two most significant bits set:
            //Iska matlab jump hai, and we need to follow the pointer

            if(len & 0xC0) == 0xC0 {

                //Update the buffer position to a point past the current label
                //Abhi isko touch nahi karna h , isliye we store the current position in pos
                if !jumped {
                    self.seek(pos + 2)?;
                }

                //Read another byte, and calculate the offset in the buffer
                //And hamara local wala position varibale mein dal do
                let b2 = self.get(pos + 1 )? as u16;
                let offset = (((len as u16) ^ 0xC0) << 8) | b2;
                pos = offset as usize;

                jumped = true;
                jumps += 1;

                continue;
            }

            //otherwise we are at the beginning of a normal label and appending it to the output string
            else{
                pos+=1;

                if len == 0 { //0 length label indicates the end of the qname
                    break;
                }

                //Append the delimiter to the output string
                outstr.push_str(delim);

                //Read the label into the output string
                let str_buffer = self.get_range(pos, len as usize)?;
                outstr.push_str(&String::from_utf8_lossy(str_buffer).to_lowercase());

                delim = "."; //After the first label, we want to append a dot before each label

                //Move to the next label
                pos += len as usize;
            }
        } 
        if !jumped {
            self.seek(pos)?;
        }
        Ok(())
    }
    
    //Write a single byte to the buffer and advance the position
    pub fn write(&mut self, val : u8) -> Result<(), Box<dyn std::error::Error>>{
        if self.pos >= 512 {
            return Err("End of buffer".into());
        }
        self.buf[self.pos] = val;
        self.pos += 1;
        Ok(())
    }

    //Write a u8 to the buffer and advance the position by 1
    pub fn write_u8(&mut self, val : u8) -> Result<(), Box<dyn std::error::Error>>{
        self.write(val)?;
        Ok(())
    }

    //Write a u16 to the buffer in network byte order and advance the position by 2
    pub fn write_u16(&mut self, val : u16) -> Result<(), Box<dyn std::error::Error>>{
        self.write(((val >> 8) & 0xFF) as u8)?;
        self.write((val & 0xFF) as u8)?;
        Ok(())
    }


    //Write a u32 to the buffer in network byte order and advance the position by 4
    pub fn write_u32(&mut self, val : u32) -> Result<(), Box<dyn std::error::Error>>{
        self.write(((val >> 24) & 0xFF) as u8)?;
        self.write(((val >> 16) & 0xFF) as u8)?;
        self.write(((val >> 8) & 0xFF) as u8)?;
        self.write((val & 0xFF) as u8)?;
        Ok(())
    }

    //Write a qname to the buffer
    pub fn write_qname(&mut self , qname : &str) -> Result<(), Box<dyn std::error::Error>>{
        for part in qname.split(".") {
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


}