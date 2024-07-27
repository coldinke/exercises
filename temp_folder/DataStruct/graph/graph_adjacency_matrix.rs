pub struct GraphAdjMat {
    pub vertices: Vec<i32>,
    pub adj_mat: Vec<Vec<i32>>,
}

impl GraphAdjMat {
    pub fn new(vertices: Vec<i32>, edges: Vec<[usize; 2]>) -> Self {
        let mut graph = GraphAdjMat {
            vertices: vec![],
            adj_mat: vec![],
        };
        for val in vertices {
            graph.add_vertex(val);
        }
        for edge in edges {
            graph.add_edge(edge[0], edge[1]);
        }

        graph
    }

    pub fn size(&self) -> usize {
        self.vertices.len()
    }

    pub fn add_vertex(&mut self, val: i32) {
        let n = self.size();
        self.vertices.push(val);
        self.adj_mat.push(vec![0; n]);
        for row in &mut self.adj_mat {
            row.push(0);
        }
    }

    pub fn remove_vertex(&mut self, index: usize) {
        if index >= self.size() {
            panic!("index error")
        }
        self.vertices.remove(index);
        self.adj_mat.remove(index);
        for row in &mut self.adj_mat {
            row.remove(index);
        }
    }

    pub fn add_edge(&mut self, i: usize, j: usize) {
        if i >= self.size() || j >= self.size() || i == j {
            panic!("index error")
        }
        self.adj_mat[i][j] = 1;
        self.adj_mat[j][i] = 1;
    }

    pub fn remove_edge(&mut self, i: usize, j: usize) {
        if i >= self.size() || j >= self.size() || i == j {
            panic!("index error")
        }
        self.adj_mat[i][j] = 0;
        self.adj_mat[j][i] = 0;
    }

    pub fn print(&self) {
        println!("顶点列表 = {:?}", self.vertices);
        println!("邻接矩阵 =");
        println!("[");
        for row in &self.adj_mat {
            println!("  {:?}", row);
        }
        println!("]");
    }
}

fn main() {
    let vertices = vec![1, 3, 2, 5, 4];
    let edges = vec![[0, 1], [0, 3], [1, 2], [2, 3], [2, 4], [3, 4]];
    let mut graph = GraphAdjMat::new(vertices, edges);
    println!("\n初始化后，图为");
    graph.print();

    graph.add_edge(0, 2);
    println!("\n添加边 1-2 后，图为");
    graph.print();

    graph.remove_edge(1, 3);
    println!("\n删除边 1-3 后，图为");
    graph.print();

    graph.add_vertex(6);
    println!("\n添加顶点 6 后，图为");
    graph.print();

    graph.remove_vertex(1);
    println!("\n删除顶点 3 后，图为");
    graph.print();
}
