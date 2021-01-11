use std::cell::RefCell;
use std::fmt::{Arguments, Write};
use std::rc::Rc;

#[derive(Clone)]
pub struct DebugBuffer {
    buffer: Rc<RefCell<String>>,
}

impl Default for DebugBuffer {
    fn default() -> Self {
        DebugBuffer {
            buffer: Rc::new(RefCell::new(String::new())),
        }
    }
}

impl std::io::Write for DebugBuffer {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.write_all(buf)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        let buffer = &mut *self.buffer.borrow_mut();
        let s = std::str::from_utf8(buf).expect("invalid utf8");
        buffer.write_str(s).expect("can not write");
        Ok(())
    }

    fn write_fmt(&mut self, fmt: Arguments<'_>) -> std::io::Result<()> {
        let buffer = &mut *self.buffer.borrow_mut();
        buffer.write_fmt(fmt).expect("can not write");
        Ok(())
    }
}

impl std::io::Read for DebugBuffer {
    fn read(&mut self, _buf: &mut [u8]) -> std::io::Result<usize> {
        unimplemented!()
    }

    fn read_to_string(&mut self, buf: &mut String) -> std::io::Result<usize> {
        let buffer = &*self.buffer.borrow_mut();
        buf.write_str(buffer.as_str()).expect("can not read string");
        Ok(buffer.len())
    }
}
