use cwrapper_test::{add_edge, add_vertices, clean_graph, empty_graph, find_cycle, get_attr_table, is_directed, neighbours, num_edges, num_vertices, version_string};

fn main() -> Result<(), String> {
    println!("igraph version: {}", version_string());

    println!("Enable attr");
    let mut attr_table = get_attr_table();

    println!("creating graph");
    let mut graph = empty_graph(true, &mut attr_table)?;
    println!("created graph directed={}", is_directed(&mut graph));

    println!("adding nodes 0 - 4");
    add_vertices(&mut graph, 5)?;
    println!("added nodes");

    println!("adding edge 0 -> 1");
    add_edge(&mut graph, 0, 1, Some(0.5))?;

    println!("adding edge 1 -> 0");
    add_edge(&mut graph, 1, 0, Some(0.3))?;

    println!("adding edge 2 -> 0");
    add_edge(&mut graph, 2, 0, Some(1.0))?;

    println!("adding edge 0 -> 4");
    add_edge(&mut graph, 0, 4, Some(5.8))?;

    println!("adding edge 4 -> 2");
    add_edge(&mut graph, 4, 2, Some(9.0))?;

    let v_count = num_vertices(&graph);
    let e_count = num_edges(&graph);


    for i in 0..v_count {
        let n = neighbours(&graph, 0)?;
        println!("n{i} {:?}", n);
    }

    let (cycle_v, _) = find_cycle(&graph)?;

    println!("Result: \n \
    Number of Vertices({v_count}), \n \
    Number of Edges ({e_count}) \n \
    Has Cycle v({:?})", cycle_v);


    clean_graph(&mut graph);
    Ok(())
}