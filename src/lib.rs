use pyo3::prelude::*;

/// A Python module implemented in Rust
#[pymodule]
mod pyo3_test {
    use pyo3::prelude::*;
    use uuid::Uuid;

    #[pyfunction]
    fn hello(name: String) -> String {
        format!("Hello, {}!", name)
    }

    /// Example of calling externalRust library.
    #[pyfunction]
    fn generate_uuid() -> String {
        Uuid::new_v4().to_string()
    }
}
