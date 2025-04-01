mod igraph;


fn main() -> Result<(), String> {

    println!("Version {}", igraph::global::version_string());

    println!("Enable Attributes!");
    let _ = igraph::global::enable_attr_table();

    println!("creating graph");
    let mut graph = igraph::graph::Graph::new(true)?;
    println!("created graph directed={}", graph.is_directed());

    println!("adding nodes 0 - 4");
    graph.add_vertices(5)?;
    println!("added nodes");

    let v_count = graph.v_count();
    println!("v_count {}", v_count);

    println!("adding edge 0 -> 1");
    graph.add_edge(0, 1, Some(0.5))?;

    println!("adding edge 1 -> 0");
    graph.add_edge(1, 0, Some(0.3))?;

    println!("adding edge 2 -> 0");
    graph.add_edge(2, 0, Some(1.0))?;

    println!("adding edge 0 -> 4");
    graph.add_edge(0, 4, Some(5.8))?;

    println!("adding edge 4 -> 2");
    graph.add_edge(4, 2, Some(9.0))?;


    let e_count = graph.e_count();


    for i in 0..v_count {
        let n = graph.get_neighbours(i)?;
        println!("n{i} {:?}", n);
    }

    graph.find_cycles(|e| {
        println!("Found Cycle {:?}", e)
    });

    let (cycle_v, _) = graph.find_cycle()?;

    println!("Result: \n \
        Number of Vertices({v_count}), \n \
        Number of Edges ({e_count}) \n \
        Has Cycle v({:?})", cycle_v);

    Ok(())
}