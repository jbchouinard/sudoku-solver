use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub enum ThreadMode {
    SingleThreaded,
    MultiThreaded(usize),
}

impl fmt::Display for ThreadMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let n: usize = self.into();
        write!(f, "{}", n)
    }
}

impl ThreadMode {
    pub fn new(n: usize) -> Self {
        if n > 1 {
            Self::MultiThreaded(n)
        } else {
            Self::SingleThreaded
        }
    }
}

impl Default for ThreadMode {
    fn default() -> Self {
        Self::MultiThreaded(num_cpus::get())
    }
}

impl FromStr for ThreadMode {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        Ok(ThreadMode::new(s.parse::<usize>()?))
    }
}

impl From<&ThreadMode> for usize {
    fn from(tm: &ThreadMode) -> Self {
        match tm {
            ThreadMode::SingleThreaded => 1,
            ThreadMode::MultiThreaded(n) => *n,
        }
    }
}

impl From<usize> for ThreadMode {
    fn from(n: usize) -> Self {
        Self::new(n)
    }
}
