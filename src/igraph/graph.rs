use std::mem::MaybeUninit;
use crate::igraph::ffi::*;
use crate::igraph::utils::*;

pub struct Graph {
    inner: igraph_t,
}

impl Graph {
    pub fn new(is_directed: bool) -> Result<Self, String> {
        unsafe {
            let mut graph = MaybeUninit::<igraph_t>::uninit();
            match igraph_empty(graph.as_mut_ptr(), 0, is_directed)
            {
                SUCCESS => {
                    let result = graph.assume_init();
                    Ok(Graph { inner: result })
                },
                code => Err(format!("Failed to initialize graph! {}", code))
            }
        }
    }

    pub fn is_directed(&self) -> bool {
        unsafe {
            igraph_is_directed(&self.inner)
        }
    }


    pub fn add_vertices(&mut self, nv: i64) -> Result<(), String> {
        unsafe {
            match igraph_add_vertices(&mut self.inner, nv, std::ptr::null_mut()) {
                SUCCESS => Ok(()),
                code => Err(format!("error code {}", code))
            }
        }
    }

    pub fn v_count(&mut self) -> i64 {
        unsafe {
            igraph_vcount(&mut self.inner)
        }
    }

    pub fn e_count(&mut self) -> i64 {
        unsafe {
            igraph_ecount(&mut self.inner)
        }
    }

    pub fn add_edge(&mut self, from: i64, to: i64, weight: Option<f64>) -> Result<(), String> {
        unsafe {
            let vcount= self.v_count();
            if from >= vcount || to >= vcount || from < 0 || to < 0 {
                return Err(format!("Vertex out of bounds: from={}, to={}, vcount={}", from, to, vcount));
            }

            let err = igraph_add_edge(&mut self.inner, from, to);
            match err {
                SUCCESS => {
                    match weight {
                        None => Ok(()),
                        Some(value) => {
                            let eid = get_edge_id(&mut self.inner, from, to)?;
                            println!("edge id {}", eid.to_string());

                            add_edge_weight(&mut self.inner, eid, value)?;
                            Ok(())
                        }
                    }
                },
                code => Err(format!("error code {}", code))
            }
        }
    }

    pub fn get_neighbours(&mut self, nv: igraph_integer_t) -> Result<Vec<i64>, String> {
        unsafe {
            let mut vec = create_vector_int()?;
            match igraph_neighbors(&mut self.inner, &mut vec ,nv, igraph_neimode_t_IGRAPH_OUT){
                SUCCESS => {
                    let result = try_vector_int_to_vec(&mut vec)?;
                    Ok(result)
                },
                code => Err(format!("Failed to get neighbours {}", code))
            }
        }
    }

    pub fn find_cycle(&mut self) -> Result<(Vec<i64>, Vec<i64>), String>{
        unsafe {
            let mut vec = create_vector_int()?;
            let mut edges = create_vector_int()?;
            match igraph_find_cycle(&mut self.inner, &mut vec, &mut edges, igraph_neimode_t_IGRAPH_OUT) {
                SUCCESS => {
                    let v = try_vector_int_to_vec(&mut vec)?;
                    let e = try_vector_int_to_vec(&mut edges)?;
                    Ok((v, e))
                },
                code => {
                    Err(format!("failed {}", code))
                }
            }
        }
    }
}

impl Drop for Graph {
    fn drop(&mut self) {
        unsafe {
            igraph_destroy(&mut self.inner);
        }
    }
}