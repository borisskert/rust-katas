/// https://www.codewars.com/kata/526989a41034285187000de4/train/rust
///
/// # Arguments
///
/// * `start`: &str - the starting IP address
/// * `end`: &str - the ending IP address
pub fn ips_between(start: &str, end: &str) -> u32 {
    let start_ip = IpAddress::new(start);
    let end_ip = IpAddress::new(end);

    end_ip.to_u32() - start_ip.to_u32()
}

struct IpAddress {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
}

impl IpAddress {
    pub fn new(ip: &str) -> Self {
        let split: Vec<&str> = ip.split('.').collect();

        let a = split[0].parse().unwrap();
        let b = split[1].parse().unwrap();
        let c = split[2].parse().unwrap();
        let d = split[3].parse().unwrap();

        IpAddress { a, b, c, d }
    }

    pub fn to_u32(&self) -> u32 {
        let a = self.a as u32;
        let b = self.b as u32;
        let c = self.c as u32;

        (a << 24) | (b << 16) | (c << 8) | self.d as u32
    }
}
