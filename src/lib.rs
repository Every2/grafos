struct IncidenceMatrix {
    vertices: i32,
    edges: i32,
}

impl IncidenceMatrix {
    fn new(vertices: i32, edges: i32) -> Vec<Vec<i32>> {
        let mut v: Vec<i32> = Vec::new();
        let mut e: Vec<i32> = Vec::new();
        let mut m: Vec<Vec<i32>> = Vec::new();

        for i in 0..=vertices {
            v.push(i);
        }

        for i in 0..=edges {
            e.push(i);
        }

        m.push(v);
        m.push(e);

        m
    }
}