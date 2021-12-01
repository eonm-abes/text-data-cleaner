use pyo3::prelude::*;
use pyo3::types::PyAny;

mod whitespaces;
use whitespaces::*;

mod clean;
mod normalize;

use clean::*;
use normalize::Normalize;



use pyo3::types::PyString;



#[pyclass]
struct DataCleaner {
    data: String,
}

// #[pymodule]
// fn trait_exposure(_py: Python, m: &PyModule) -> PyResult<()> {
//     m.add_class::<UserModel>()?;
//     Ok(())
// }

#[pymethods]
impl DataCleaner {
    #[new]
    pub fn new(data: String) -> Self {
        DataCleaner { data }
    }

    pub fn data(&self) -> &String {
        &self.data
    }

    pub fn trim(&mut self) -> PyResult<()> {
        SpaceTrimmer::new().clean(&mut self.data);
        // let data = self.data.borrow_value();
        // let data = data.to_string();
        // let data = clean_data(&data);
        
        Ok(())
    }

    
}

#[pymodule]
fn txdc(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<DataCleaner>()?;
    Ok(())
}