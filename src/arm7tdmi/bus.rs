use super::Addr;
use std::io;
use std::ops::Add;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

pub enum MemoryAccessType {
    NonSeq,
    Seq,
}

pub enum MemoryAccessWidth {
    MemoryAccess8,
    MemoryAccess16,
    MemoryAccess32,
}

impl Add<MemoryAccessWidth> for MemoryAccessType {
    type Output = MemoryAccess;

    fn add(self, other: MemoryAccessWidth) -> Self::Output {
        MemoryAccess(self, other)
    }
}

pub struct MemoryAccess(pub MemoryAccessType, pub MemoryAccessWidth);

pub trait Bus {
    fn read_32(&self, addr: Addr) -> u32 {
        self.get_bytes(addr, 4).read_u32::<LittleEndian>().unwrap()
    }

    fn read_16(&self, addr: Addr) -> u16 {
        self.get_bytes(addr, 2).read_u16::<LittleEndian>().unwrap()
    }

    fn read_8(&self, addr: Addr) -> u8 {
        self.get_bytes(addr, 1)[0]
    }

    fn write_32(&mut self, addr: Addr, value: u32) -> Result<(), io::Error> {
        self.get_bytes_mut(addr, 4).write_u32::<LittleEndian>(value)
    }

    fn write_16(&mut self, addr: Addr, value: u16) -> Result<(), io::Error> {
        self.get_bytes_mut(addr, 2).write_u16::<LittleEndian>(value)
    }

    fn write_8(&mut self, addr: Addr, value: u8) -> Result<(), io::Error> {
        self.get_bytes_mut(addr, 1).write_u8(value)
    }
    /// Return a slice of bytes
    /// Will panic if requested range is out of bounds
    fn get_bytes(&self, addr: Addr, len: usize) -> &[u8];

    /// Return a mutable slice of bytes
    /// Will panic if requested range is out of bounds
    fn get_bytes_mut(&mut self, addr: Addr, len: usize) -> &mut [u8];

    /// returns the number of cycles needed for this memory access
    fn get_cycles(&self, addr: Addr, access: MemoryAccess) -> usize;
}