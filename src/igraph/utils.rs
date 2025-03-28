use std::mem::MaybeUninit;
use std::os::raw::c_char;
use std::slice;
use crate::igraph::ffi::*;

pub const SUCCESS: igraph_error_type_t = igraph_error_type_t_IGRAPH_SUCCESS;

pub fn add_edge_weight(graph: &mut igraph_t, edge_id: igraph_integer_t, weight: igraph_real_t) -> Result<(), String>{
    unsafe {
        let attr_name = std::ffi::CString::new("weight").map_err(|err| err.to_string())?;

        match igraph_cattribute_EAN_set(graph, attr_name.as_ptr(), edge_id, weight)
        {
            SUCCESS => Ok(()),
            code => Err(format!("Failed to set weight {}", code))
        }
    }
}

pub fn get_edge_id(graph: &mut igraph_t, from: i64, to: i64) -> Result<igraph_integer_t, String>{
    unsafe {
        let mut eid = 0;
        match igraph_get_eid(graph, &mut eid, from, to, true, true){
            SUCCESS => Ok(eid),
            code =>  Err(format!("Failed to get edge ID {}", code))
        }
    }
}

pub fn create_vector_int() -> Result<igraph_vector_int_t, String> {
    unsafe {
        let mut vec = MaybeUninit::<igraph_vector_int_t>::uninit();
        if igraph_vector_int_init(vec.as_mut_ptr(), 0) != SUCCESS {
            return Err("Failed to initialize vector".to_string());
        }
        let vec = vec.assume_init();
        Ok(vec)
    }
}

impl Drop for igraph_vector_int_t {
    fn drop(&mut self) {
        unsafe {
            igraph_vector_int_destroy(self);
        }
    }
}

impl Drop for igraph_t{
    fn drop(&mut self) {
        unsafe {
            igraph_destroy(self)
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

