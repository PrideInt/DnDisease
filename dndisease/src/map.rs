pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

pub struct Node {
    pub country: String,
    pub population: u32,
}

pub struct Edge {
    pub weight: u32,
    pub from: Node,
    pub to: Node,
}

// Test the graph module
pub fn new_graph() -> Graph {
    let mut nodes: Vec<Node> = Vec::new();
    let mut edges: Vec<Edge> = Vec::new();

    let us: Node = Node {
        country: "United States".to_string(),
        population: 331,
    };
    let can: Node = Node {
        country: "Canada".to_string(),
        population: 38,
    };
    let atl_ocean: Node = Node {
        country: "Atlantic Ocean".to_string(),
        population: 0,
    };
    let asia: Node = Node {
        country: "Asia".to_string(),
        population: 4641,
    };

    nodes.push(us);
    nodes.push(can);
    nodes.push(atl_ocean);
    nodes.push(asia);

    let us_can: Edge = Edge {
        weight: 1,
        from: us,
        to: can,
    };

    let us_atl_ocean: Edge = Edge {
        weight: 1,
        from: us,
        to: atl_ocean,
    };

    let us_asia: Edge = Edge {
        weight: 1,
        from: us,
        to: asia,
    };

    edges.push(us_can);
    edges.push(us_atl_ocean);
    edges.push(us_asia);

    return Graph {
        nodes: nodes,
        edges: edges,
    };
}