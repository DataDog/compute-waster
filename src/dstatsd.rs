use std::io;
use std::net::{Ipv4Addr, UdpSocket};
use std::os::unix::net::UnixDatagram;
use std::time;

enum Socket {
    UnixDatagram(UnixDatagram),
    UdpSocket(UdpSocket),
}

impl Socket {
    fn connect(addr: &str) -> io::Result<Self> {
        if let Some(path) = addr.strip_prefix("unix:") {
            return Self::from_unix(path);
        }
        Self::from_ip(addr)
    }

    fn from_unix(path: &str) -> io::Result<Self> {
        let s = UnixDatagram::unbound()?;
        s.connect(path)?;
        Ok(Self::UnixDatagram(s))
    }

    fn from_ip(addr: &str) -> io::Result<Self> {
        let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0))?;
        socket.connect(addr)?;
        Ok(Self::UdpSocket(socket))
    }

    fn send(&self, buf: &[u8]) -> io::Result<()> {
        match self {
            Self::UdpSocket(s) => s.send(buf)?,
            Self::UnixDatagram(s) => s.send(buf)?,
        };
        Ok(())
    }

    fn set_write_timeout(&mut self, timeout: Option<time::Duration>) -> io::Result<()> {
        match self {
            Self::UdpSocket(s) => s.set_write_timeout(timeout)?,
            Self::UnixDatagram(s) => s.set_write_timeout(timeout)?,
        };
        Ok(())
    }
}

pub struct Metric {
    pub name: String,
    pub tags: String,
}

pub struct Hist<'a> {
    metric: &'a Metric,
    value: f64,
}

impl Metric {
    pub fn with_value<'a>(&'a self, value: f64) -> Hist<'a> {
        Hist {
            metric: self,
            value,
        }
    }
}

impl<'a> Hist<'a> {
    fn serialize_into<T: io::Write>(&self, mut buf: T) -> io::Result<()> {
        write!(
            buf,
            "{}:{}|h|#{}\n",
            self.metric.name, self.value, self.metric.tags
        )?;
        Ok(())
    }
}

pub struct Client {
    addr: String,
    socket: Socket,
    buffer: Vec<u8>,
}

impl Client {
    pub fn new(addr: String) -> io::Result<Self> {
        let mut socket = Socket::connect(&addr)?;
        socket
            .set_write_timeout(Some(time::Duration::new(1, 0)))
            .expect("wasn't able to set write timeout on socket");
        Ok(Self {
            addr,
            socket,
            buffer: Vec::new(),
        })
    }

    fn reconnect(&mut self) -> io::Result<()> {
        let mut socket = Socket::connect(&self.addr)?;
        socket
            .set_write_timeout(Some(time::Duration::new(1, 0)))
            .expect("wasn't able to set write timeout on socket");
        self.socket = socket;
        Ok(())
    }

    pub fn send(&mut self, metric: Hist) -> io::Result<()> {
        self.buffer.truncate(0);
        metric.serialize_into(&mut self.buffer)?;
        // use std::io::Write;
        // std::io::stdout().write_all(&self.buffer).unwrap();
        let mut errors = 0;
        loop {
            match self.socket.send(&self.buffer) {
                Ok(_) => break,
                Err(e) => {
                    errors += 1;
                    if errors > 3 {
                        return Err(e);
                    }
                    let _ignored = self.reconnect();
                }
            }
        }
        Ok(())
    }

}
