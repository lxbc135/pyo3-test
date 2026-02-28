use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
mod pyo3_test {
    use pyo3::prelude::*;
    use uuid::Uuid;

    /// Formats the sum of two numbers as string.
    #[pyfunction]
    fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
        Ok((a + b).to_string())
    }

    /// Generates a random UUID v4.
    #[pyfunction]
    fn generate_uuid() -> String {
        Uuid::new_v4().to_string()
    }
}
