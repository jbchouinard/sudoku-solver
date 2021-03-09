use std::cmp;
use std::collections::HashMap;
use std::fmt;
use std::ops::{Add, Div};

pub trait Statistic<T> {
    fn name(&self) -> String;
    fn update(&mut self, value: T);
    fn value(&self) -> T;
}

impl<T: fmt::Display> fmt::Display for dyn Statistic<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        write!(f, "{}: {}", self.name(), self.value())
    }
}

pub struct Formatted<T> {
    unit: String,
    precision: usize,
    statistic: Box<dyn Statistic<T>>,
}

impl<T> Formatted<T> {
    pub fn new(statistic: Box<dyn Statistic<T>>, unit: &str, precision: usize) -> Self {
        Self {
            unit: unit.to_string(),
            precision,
            statistic,
        }
    }
}

impl<T> Statistic<T> for Formatted<T> {
    fn name(&self) -> std::string::String {
        self.statistic.name()
    }
    fn update(&mut self, v: T) {
        self.statistic.update(v);
    }
    fn value(&self) -> T {
        self.statistic.value()
    }
}

impl<T: fmt::Display> fmt::Display for Formatted<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        write!(
            f,
            "{0}: {1:.3$}{2}",
            self.statistic.name(),
            self.statistic.value(),
            self.unit,
            self.precision
        )
    }
}

pub struct Mean<T> {
    count: T,
    sum: T,
}

impl<T: Zero> Mean<T> {
    pub fn new() -> Self {
        Self {
            count: T::zero(),
            sum: T::zero(),
        }
    }
}

impl<T: Copy + One + Zero + Add<Output = T> + Div<Output = T>> Statistic<T> for Mean<T> {
    fn name(&self) -> String {
        "mean".to_string()
    }
    fn update(&mut self, value: T) {
        self.count = self.count + T::one();
        self.sum = self.sum + value;
    }

    fn value(&self) -> T {
        self.sum / self.count
    }
}

pub struct Sum<T> {
    sum: T,
}

impl<T: Zero> Sum<T> {
    pub fn new() -> Self {
        Self { sum: T::zero() }
    }
}

impl<T: Copy + Zero + Add<Output = T>> Statistic<T> for Sum<T> {
    fn update(&mut self, value: T) {
        self.sum = self.sum + value
    }
    fn value(&self) -> T {
        self.sum
    }
    fn name(&self) -> std::string::String {
        "sum".to_string()
    }
}

pub struct Count<T> {
    count: T,
}

impl<T: Zero> Count<T> {
    pub fn new() -> Self {
        Self { count: T::zero() }
    }
}

impl<T: Copy + Zero + One + Add<Output = T>> Statistic<T> for Count<T> {
    fn update(&mut self, _: T) {
        self.count = self.count + T::one();
    }
    fn value(&self) -> T {
        self.count
    }
    fn name(&self) -> std::string::String {
        "count".to_string()
    }
}

pub struct Maximum<T> {
    max: Option<T>,
}

impl<T> Maximum<T> {
    pub fn new() -> Self {
        Self { max: None }
    }
}

impl<T: Copy + Zero + cmp::PartialOrd> Statistic<T> for Maximum<T> {
    fn name(&self) -> std::string::String {
        "maximum".to_string()
    }
    fn update(&mut self, value: T) {
        self.max = match self.max {
            Some(max) => {
                if value > max {
                    Some(value)
                } else {
                    Some(max)
                }
            }
            None => Some(value),
        }
    }
    fn value(&self) -> T {
        match self.max {
            Some(max) => max,
            None => T::zero(),
        }
    }
}

pub struct Minimum<T> {
    min: Option<T>,
}

impl<T> Minimum<T> {
    pub fn new() -> Self {
        Self { min: None }
    }
}

impl<T: Copy + Zero + cmp::PartialOrd> Statistic<T> for Minimum<T> {
    fn name(&self) -> std::string::String {
        "minimum".to_string()
    }
    fn update(&mut self, value: T) {
        self.min = match self.min {
            Some(min) => {
                if value < min {
                    Some(value)
                } else {
                    Some(min)
                }
            }
            None => Some(value),
        }
    }
    fn value(&self) -> T {
        match self.min {
            Some(min) => min,
            None => T::zero(),
        }
    }
}

pub struct Report<T> {
    fields: HashMap<String, Vec<Formatted<T>>>,
    order: Vec<String>,
}

impl<T: Copy> Report<T> {
    pub fn update(&mut self, field: &str, value: T) {
        for stat in self
            .fields
            .entry(field.to_string())
            .or_insert_with(Vec::new)
        {
            stat.update(value);
        }
    }
}

impl<T: fmt::Display> fmt::Display for Report<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        for field in &self.order {
            writeln!(f, "{}", field)?;
            for stat in self.fields.get(field).unwrap() {
                writeln!(f, "  {}", stat)?;
            }
        }
        Ok(())
    }
}

pub struct ReportBuilder<T> {
    fields: HashMap<String, Vec<Formatted<T>>>,
    order: Vec<String>,
}

impl<T: Copy> ReportBuilder<T> {
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
            order: vec![],
        }
    }
    pub fn with(mut self, field: &str, stat: Formatted<T>) -> Self {
        self.fields
            .entry(field.to_string())
            .or_insert_with(Vec::new)
            .push(stat);
        let f = field.to_string();
        if !self.order.contains(&f) {
            self.order.push(f);
        }
        self
    }
    pub fn build(self) -> Report<T> {
        Report {
            fields: self.fields,
            order: self.order,
        }
    }
}

pub trait Zero {
    fn zero() -> Self;
}

pub trait One {
    fn one() -> Self;
}

impl Zero for f64 {
    fn zero() -> Self {
        0.0
    }
}

impl One for f64 {
    fn one() -> Self {
        1.0
    }
}

impl Zero for u64 {
    fn zero() -> Self {
        0
    }
}

impl One for u64 {
    fn one() -> Self {
        1
    }
}

impl Zero for u128 {
    fn zero() -> Self {
        0
    }
}

impl One for u128 {
    fn one() -> Self {
        1
    }
}

impl Zero for i64 {
    fn zero() -> Self {
        0
    }
}

impl One for i64 {
    fn one() -> Self {
        1
    }
}

impl Zero for i128 {
    fn zero() -> Self {
        0
    }
}

impl One for i128 {
    fn one() -> Self {
        1
    }
}
