use std::path::Path;
use std::time::Instant;

use pyo3::prelude::*;

/// Tests for small data transmission overhead.
pub struct RandomNumberGenerator {
    code: String,
}

impl RandomNumberGenerator {
    pub fn load<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;

        Ok(Self { code: content })
    }

    pub fn generate(&self, py: Python) -> anyhow::Result<u16> {
        let module = PyModule::from_code(py, &self.code, "rand.py", "rand")?;
        let fun = module.getattr("generate")?;
        let result = fun.call0()?;
        Ok(result.extract()?)
    }
}

/// TODO
/// Tests for large data transmission overhead.
pub struct DataHasher {
    code: String,
}

impl DataHasher {
    pub fn load<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;

        Ok(Self { code: content })
    }

    pub fn hash(&self, py: Python, data: Vec<Vec<u8>>) -> anyhow::Result<()> {
        let module = PyModule::from_code(py, &self.code, "hash.py", "hash")?;
        let fun = module.getattr("hash_data")?;
        let _result = fun.call1((data,))?;
        Ok((/* todo */))
    }
}

fn bench(name: &str, iter: usize, f: impl Fn()) {
    let start = Instant::now();

    for _ in 0..iter {
        (f)();
    }

    let mean = start.elapsed() / iter as u32;

    println!("{}: {} us (mean)", name, mean.as_micros())
}

fn main() {
    // basic benchmarks

    let gen = RandomNumberGenerator::load("scripts/rand.py").unwrap();

    bench("RNG acq GIL", 10_000, || {
        Python::with_gil(|py| {
            gen.generate(py).unwrap();
        })
    });

    Python::with_gil(|py| {
        bench("RNG w GIL", 10_000, || {
            gen.generate(py).unwrap();
        })
    });
}
