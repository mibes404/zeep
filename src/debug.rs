use std::fmt::{Arguments, Error, Write};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct DebugBuffer {
    buffer: Arc<Mutex<String>>,
}

impl Default for DebugBuffer {
    fn default() -> Self {
        DebugBuffer {
            buffer: Arc::new(Mutex::new(String::new())),
        }
    }
}

impl std::io::Write for DebugBuffer {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.write_all(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    fn write_all(&mut self, mut buf: &[u8]) -> std::io::Result<()> {
        let mut w_buf = self.buffer.lock().expect("can not obtain lock");
        let s = std::str::from_utf8(buf).expect("invalid utf8");
        w_buf.write_str(s).expect("can not write");
        Ok(())
    }

    fn write_fmt(&mut self, fmt: Arguments<'_>) -> std::io::Result<()> {
        let mut w_buf = self.buffer.lock().expect("can not obtain lock");
        w_buf.write_fmt(fmt).expect("can not write");
        Ok(())
    }
}

impl std::io::Read for DebugBuffer {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        unimplemented!()
    }

    fn read_to_string(&mut self, buf: &mut String) -> std::io::Result<usize> {
        let w_buf = self.buffer.lock().expect("can not obtain lock");
        buf.write_str(w_buf.as_str());
        Ok(w_buf.len())
    }
}
