use std::ffi::{CStr, CString};
use std::mem::MaybeUninit;
use std::os::raw::{c_char};
use std::slice;

#[allow(
    unsafe_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    unused_variables,
    improper_ctypes,
    missing_debug_implementations,
    dead_code
)]
mod ffi {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use ffi::*;

pub fn version_string() -> String {
    unsafe {
        // Also get numeric components
        let mut version_ptr: *const c_char = std::ptr::null();
        let mut major = 0;
        let mut minor = 0;
        let mut patch = 0;
        igraph_version(&mut version_ptr, &mut major, &mut minor, &mut patch);


        let version_str = CStr::from_ptr(version_ptr)
            .to_string_lossy()
            .into_owned();
        format!("({}: {}.{}.{})",version_str, major, minor, patch)
    }
}

pub fn get_attr_table() -> igraph_attribute_table_t {
    unsafe {
        igraph_cattribute_table
    }
}

pub fn empty_graph(is_directed: bool, attr_table: &mut igraph_attribute_table_t) -> Result<igraph_t, String> {
    unsafe {
        igraph_set_attribute_table(attr_table);


        let mut graph = MaybeUninit::<igraph_t>::uninit();
        match igraph_empty(graph.as_mut_ptr(), 0, is_directed)
        {
            igraph_error_type_t_IGRAPH_SUCCESS => {
                let result = graph.assume_init();
                Ok(result)
            },
            code => Err(format!("Failed to initialize graph! {}", code))
        }
    }
}


pub fn is_directed(graph: &mut igraph_t) -> igraph_bool_t {
    unsafe  {
        igraph_is_directed(graph)
    }
}

pub fn add_vertices(graph: &mut igraph_t, nv: i64) -> Result<(), String> {
    unsafe {
        match igraph_add_vertices(graph, nv, std::ptr::null_mut()) {
            igraph_error_type_t_IGRAPH_SUCCESS => Ok(()),
            code => Err(format!("error code {}", code))
        }
    }
}

pub fn num_vertices(graph: &igraph_t) -> igraph_integer_t {
    unsafe {
        igraph_vcount(graph)
    }
}




pub fn num_edges(graph: &igraph_t) -> igraph_integer_t {
    unsafe {
        igraph_ecount(graph)
    }
}

pub fn add_edge(graph: &mut igraph_t, from: i64, to: i64, weight: Option<f64>) -> Result<(), String> {
    unsafe {
        let err = igraph_add_edge(graph, from, to);
        match err {
            igraph_error_type_t_IGRAPH_SUCCESS => {
                match weight {
                    None => Ok(()),
                    Some(value) =>{
                        let eid = get_edge_id(graph, from, to)?;
                        println!("edge id {}",eid.to_string());

                        //add_edge_weight(graph, eid, value)?;
                        Ok(())
                    }
                }
            },
            code => Err(format!("error code {}", code))
        }
    }
}

fn add_edge_weight(graph: &mut igraph_t, edge_id: igraph_integer_t, weight: igraph_real_t) -> Result<(), String>{
    unsafe {
        let attr_name = std::ffi::CString::new("weight").map_err(|err| err.to_string())?;

        /*// 2. Check if attribute exists, if not create it
        if igraph_cattribute_has_attr(graph, igraph_attribute_elemtype_t_IGRAPH_ATTRIBUTE_EDGE, attr_name.as_ptr()) == false {
            if igraph_cattribute_EAN_setv(graph, attr_name.as_ptr(), std::ptr::null()) != igraph_error_type_t_IGRAPH_SUCCESS {
                return Err("Failed to initialize weight attribute".to_string());
            }
        }
*/
        match igraph_cattribute_EAN_set(graph, attr_name.as_ptr(), edge_id, weight)
        {
            igraph_error_type_t_IGRAPH_SUCCESS => Ok(()),
            code => Err(format!("Failed to set weight {}", code))
        }
    }
}

fn get_edge_id(graph: &mut igraph_t, from: i64, to: i64) -> Result<igraph_integer_t, String>{
    unsafe {
        let mut eid = 0;
        match igraph_get_eid(graph, &mut eid, from, to, true, true){
            igraph_error_type_t_IGRAPH_SUCCESS => Ok(eid),
            code =>  Err(format!("Failed to get edge ID {}", code))
        }
    }
}

pub fn print_vector_int(vector: igraph_vector_int_t) -> igraph_error_type_t {
    unsafe {
        igraph_vector_int_print(&vector)
    }
}

fn create_vector_int() -> Result<igraph_vector_int_t, String> {
    unsafe {
        let mut vec = MaybeUninit::<igraph_vector_int_t>::uninit();
        if igraph_vector_int_init(vec.as_mut_ptr(), 0) != 0 {
            return Err("Failed to initialize vector".to_string());
        }
        let vec = vec.assume_init();
        Ok(vec)
    }
}
fn create_vector_t() -> Result<igraph_vector_t, String> {
    unsafe {
        let mut vec = MaybeUninit::<igraph_vector_t>::uninit();
        if igraph_vector_init(vec.as_mut_ptr(), 0) != 0 {
            return Err("Failed to initialize vector".to_string());
        }
        let vec = vec.assume_init();
        Ok(vec)
    }
}
fn create_vector_str() -> Result<igraph_strvector_t, String> {
    unsafe {
        let mut vec = MaybeUninit::<igraph_strvector_t>::uninit();
        if igraph_strvector_init(vec.as_mut_ptr(), 0) != 0 {
            return Err("Failed to initialize vector".to_string());
        }
        let vec = vec.assume_init();
        Ok(vec)
    }
}


fn create_attr_table() -> Result<igraph_attribute_combination_t, String> {
    unsafe {
        let mut table = MaybeUninit::<igraph_attribute_combination_t>::uninit();
        if igraph_attribute_combination_init(table.as_mut_ptr()) != 0 {
            return Err("Failed to initialize vector".to_string());
        }

        let mut table = table.assume_init();

        let weight = CString::new("weight").map_err(|e| e.to_string())?;
        if igraph_attribute_combination_add(&mut table, weight.as_ptr(), igraph_attribute_combination_type_t_IGRAPH_ATTRIBUTE_COMBINE_SUM, None) != 0 {
            return Err(format!("Failed to add weight attribute"));
        }
        Ok(table)
    }
}

pub fn neighbours(graph: &igraph_t, nv: igraph_integer_t) -> Result<Vec<i64>, String> {
    unsafe {
        let mut vec = create_vector_int()?;
        match igraph_neighbors(graph, &mut vec ,nv, igraph_neimode_t_IGRAPH_OUT){
            igraph_error_type_t_IGRAPH_SUCCESS => {
                //println!("using print_vector_int");
                //print_vector_int(vec);
                let result = try_vector_int_to_vec(&mut vec)?;
                igraph_vector_int_destroy(&mut vec);
                Ok(result)
            },
            code => Err(format!("Failed to get neighbours {}", code))
        }
    }
}
/*TODO pub fn shortest_path_dijkstra(graph: &igraph_t, from: i64, to: i64) -> Result<(f64, Vec<i64>), String> {
    unsafe {
        // Prepare output variables
        let mut edges = create_vector_int()?;
        let mut path = create_vector_int()?;

        // Get weight attribute
        let weights = {
            let attr_name = std::ffi::CString::new("weight").unwrap();
            igraph_cattribute_EANV(graph, attr_name.as_ptr(), 0)
        };

        // Calculate shortest path
        let res = igraph_get_shortest_path_dijkstra(
            graph,
            &mut path,
            &mut edges,
            from as _,
            to as _,
            weights,
            igraph_neimode_t_IGRAPH_OUT
        );

        // Process results
        match res {
            0 => {
                let vertices = try_vector_int_to_vec(&path)?;
                igraph_vector_int_destroy(&mut path);
                Ok((length, vertices))
            },
            code => {
                igraph_vector_int_destroy(&mut path);
                Err(format!("Failed to calculate path: error code {}", code))
            }
        }
    }
}
*/
pub fn find_cycle(graph: &igraph_t) -> Result<(Vec<i64>, Vec<i64>), String>{
    unsafe {
        let mut vec = create_vector_int()?;
        let mut edges = create_vector_int()?;
        match igraph_find_cycle(graph, &mut vec, &mut edges, igraph_neimode_t_IGRAPH_OUT) {
            igraph_error_type_t_IGRAPH_SUCCESS => Ok((try_vector_int_to_vec(&mut vec)?, try_vector_int_to_vec(&mut edges)?)),
            code => Err(format!("failed {}", code))
        }
    }
}

pub fn try_vector_int_to_vec(igraph_vec: &igraph_vector_int_t) -> Result<Vec<i64>, String> {
    unsafe {
        // 1. Null check - completely uninitialized vector
        if igraph_vec.stor_begin.is_null() {
            return Ok(Vec::new());
        }

        // 2. Use igraph's official size as primary source
        let official_len = igraph_vector_int_size(igraph_vec) as usize;

        // 3. Pointer arithmetic verification (debug only)
        #[cfg(debug_assertions)]
        {
            let calc_len = if igraph_vec.stor_end.is_null() {
                0
            } else {
                (igraph_vec.stor_end as usize - igraph_vec.stor_begin as usize)
                    / std::mem::size_of::<i64>()
            };

            if official_len != calc_len {
                // This explains why we see empty print output but storage exists
                /*eprintln!("Debug: Vector metadata - official_len: {}, calc_len: {}, stor_begin: {:?}, contents: {}",
                          official_len,
                          calc_len,
                          igraph_vec.stor_begin,
                          *igraph_vec.stor_begin
                );*/
            }
        }

        // 4. Convert to Rust vector
        if official_len == 0 {
            Ok(Vec::new())
        } else {
            Ok(slice::from_raw_parts(igraph_vec.stor_begin as *const i64, official_len).to_vec())
        }
    }
}


pub fn clean_graph(graph: &mut igraph_t,){
    unsafe {
        igraph_destroy(graph)
    }
}