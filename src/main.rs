use cwrapper_test::{add_edge, add_vertices, clean_graph, empty_graph, find_cycle, is_directed, neighbours, num_edges, num_vertices, version_string};

fn main() -> Result<(), String> {
    println!("igraph version: {}", version_string());
    println!("creating graph");
    let mut graph = empty_graph(true)?;
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

    let n_0 = neighbours(&graph, 0)?;
    println!("n0 {:?}", n_0);

    let n_1 = neighbours(&graph, 1)?;
    println!("n1 {:?}", n_1);

    let n_2 = neighbours(&graph, 2)?;
    println!("n2 {:?}", n_2);

    let n_3 = neighbours(&graph, 3)?;
    println!("n3 {:?}", n_3);

    let n_4 = neighbours(&graph, 4)?;
    println!("n4 {:?}", n_4);

    let (cycle_v, _) = find_cycle(&graph)?;

    println!("Result: \n \
    Number of Vertices({v_count}), \n \
    Number of Edges ({e_count}) \n \
    Has Cycle v({:?})", cycle_v);


    //clean_graph(&mut graph);
    Ok(())
}