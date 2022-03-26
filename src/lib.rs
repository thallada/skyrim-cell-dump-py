use pyo3::prelude::*;
use seahash;
use skyrim_cell_dump as native_skyrim_cell_dump;

#[pyclass]
pub struct Plugin {
    #[pyo3(get, set)]
    pub header: PluginHeader,
    #[pyo3(get, set)]
    pub worlds: Vec<World>,
    #[pyo3(get, set)]
    pub cells: Vec<Cell>,
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct PluginHeader {
    #[pyo3(get, set)]
    pub version: f32,
    #[pyo3(get, set)]
    pub num_records_and_groups: i32,
    #[pyo3(get, set)]
    pub next_object_id: u32,
    #[pyo3(get, set)]
    pub author: Option<String>,
    #[pyo3(get, set)]
    pub description: Option<String>,
    #[pyo3(get, set)]
    pub masters: Vec<String>,
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct Cell {
    #[pyo3(get, set)]
    pub form_id: u32,
    #[pyo3(get, set)]
    pub editor_id: Option<String>,
    #[pyo3(get, set)]
    pub x: Option<i32>,
    #[pyo3(get, set)]
    pub y: Option<i32>,
    #[pyo3(get, set)]
    pub world_form_id: Option<u32>,
    #[pyo3(get, set)]
    pub is_persistent: bool,
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct World {
    #[pyo3(get, set)]
    pub form_id: u32,
    #[pyo3(get, set)]
    pub editor_id: String,
}

impl std::convert::From<native_skyrim_cell_dump::Plugin<'_>> for Plugin {
    fn from(plugin: native_skyrim_cell_dump::Plugin) -> Self {
        Plugin {
            header: PluginHeader {
                version: plugin.header.version,
                num_records_and_groups: plugin.header.num_records_and_groups,
                next_object_id: plugin.header.next_object_id,
                author: plugin.header.author.map(|s| s.to_string()),
                description: plugin.header.description.map(|s| s.to_string()),
                masters: plugin
                    .header
                    .masters
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect(),
            },
            worlds: plugin.worlds.into_iter().map(World::from).collect(),
            cells: plugin.cells.into_iter().map(Cell::from).collect(),
        }
    }
}

impl std::convert::From<native_skyrim_cell_dump::World> for World {
    fn from(world: native_skyrim_cell_dump::World) -> Self {
        World {
            form_id: world.form_id,
            editor_id: world.editor_id,
        }
    }
}

impl std::convert::From<native_skyrim_cell_dump::Cell> for Cell {
    fn from(cell: native_skyrim_cell_dump::Cell) -> Self {
        Cell {
            form_id: cell.form_id,
            editor_id: cell.editor_id,
            x: cell.x,
            y: cell.y,
            world_form_id: cell.world_form_id,
            is_persistent: cell.is_persistent,
        }
    }
}

// From: https://stackoverflow.com/a/50278316/6620612
fn format_radix(mut x: u64, radix: u32) -> String {
    let mut result = vec![];
    loop {
        let m = x % radix as u64;
        x /= radix as u64;

        // will panic if you use a bad radix (< 2 or > 36).
        result.push(std::char::from_digit(m as u32, radix).unwrap());
        if x == 0 {
            break;
        }
    }
    result.into_iter().rev().collect()
}

/// Parses the plugin into a Python Plugin class
#[pyfunction]
fn parse_plugin(input: &[u8]) -> PyResult<Py<Plugin>> {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let plugin: Plugin = native_skyrim_cell_dump::parse_plugin(&input)
        .unwrap()
        .into();

    Py::new(py, plugin)
}

/// Hashes bytes with seahash and returns the resulting number
#[pyfunction]
fn hash_plugin(input: &[u8]) -> PyResult<u64> {
    Ok(seahash::hash(&input))
}

/// Hashes bytes with seahash and returns the resulting base 36 encoded hash string
#[pyfunction]
fn hash_plugin_to_string(input: &[u8]) -> PyResult<String> {
    Ok(format_radix(seahash::hash(&input), 36))
}

/// A Python module implemented in Rust.
#[pymodule]
fn skyrim_cell_dump(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_plugin, m)?)?;
    m.add_function(wrap_pyfunction!(hash_plugin, m)?)?;
    m.add_function(wrap_pyfunction!(hash_plugin_to_string, m)?)?;
    Ok(())
}
