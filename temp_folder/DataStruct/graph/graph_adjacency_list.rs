use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct Vertex {
    pub val: i32,
}

pub fn vals_to_vets(vals: Vec<i32>) -> Vec<Vertex> {
    vals.into_iter().map(|val| Vertex { val }).collect()
}

pub fn vets_to_vals(vets: Vec<Vertex>) -> Vec<i32> {
    vets.into_iter().map(|vet| vet.val).collect()
}

#[derive(Clone)]
pub struct GraphAdjList {
    pub adj_list: HashMap<Vertex, Vec<Vertex>>,
}

impl GraphAdjList {
    pub fn new(edges: Vec<[Vertex; 2]>) -> Self {
        let mut graph = GraphAdjList {
            adj_list: HashMap::new(),
        };
        for edge in edges {
            graph.add_vertex(edge[0]);
            graph.add_vertex(edge[1]);
            graph.add_edge(edge[0], edge[1]);
        }

        graph
    }

    #[allow(unused)]
    pub fn size(&self) -> usize {
        self.adj_list.len()
    }

    pub fn add_edge(&mut self, vet1: Vertex, vet2: Vertex) {
        if !self.adj_list.contains_key(&vet1) || !self.adj_list.contains_key(&vet2) || vet1 == vet2
        {
            panic!("value error");
        }
        self.adj_list.get_mut(&vet1).unwrap().push(vet2);
        self.adj_list.get_mut(&vet2).unwrap().push(vet1);
    }

    #[allow(unused)]
    pub fn remove_edge(&mut self, vet1: Vertex, vet2: Vertex) {
        if !self.adj_list.contains_key(&vet1) || !self.adj_list.contains_key(&vet2) || vet1 == vet2
        {
            panic!("value error");
        }
        self.adj_list
            .get_mut(&vet1)
            .unwrap()
            .retain(|&vet| vet != vet2);
        self.adj_list
            .get_mut(&vet2)
            .unwrap()
            .retain(|&vet| vet != vet1);
    }

    pub fn add_vertex(&mut self, vet: Vertex) {
        if self.adj_list.contains_key(&vet) {
            return;
        }
        self.adj_list.insert(vet, vec![]);
    }

    #[allow(unused)]
    pub fn remove_vertex(&mut self, vet: Vertex) {
        if !self.adj_list.contains_key(&vet) {
            panic!("value error");
        }
        self.adj_list.remove(&vet);
        for list in self.adj_list.values_mut() {
            list.retain(|&v| v != vet);
        }
    }

    pub fn print(&self) {
        println!("邻接表 =");
        for (vertex, list) in &self.adj_list {
            let list = list.iter().map(|vertex| vertex.val).collect::<Vec<i32>>();
            println!("{}: {:?}", vertex.val, list);
        }
    }
}

fn graph_bfs(graph: GraphAdjList, start_vet: Vertex) -> Vec<Vertex> {
    let mut res = vec![];
    let mut visited = HashSet::new();
    visited.insert(start_vet);
    let mut que = VecDeque::new();
    que.push_back(start_vet);
    while !que.is_empty() {
        let vet = que.pop_front().unwrap();
        res.push(vet);

        if let Some(adj_vets) = graph.adj_list.get(&vet) {
            for &adj_vet in adj_vets {
                if visited.contains(&adj_vet) {
                    continue;
                }
                que.push_back(adj_vet);
                visited.insert(adj_vet);
            }
        }
    }

    res
}

fn dfs(graph: &GraphAdjList, visited: &mut HashSet<Vertex>, res: &mut Vec<Vertex>, vet: Vertex) {
    res.push(vet);
    visited.insert(vet);

    if let Some(adj_vets) = graph.adj_list.get(&vet) {
        for &adj_vet in adj_vets {
            if visited.contains(&adj_vet) {
                continue;
            }
            dfs(graph, visited, res, adj_vet);
        }
    }
}

fn graph_dfs(graph: GraphAdjList, start_vet: Vertex) -> Vec<Vertex> {
    let mut res = vec![];
    let mut visited = HashSet::new();
    dfs(&graph, &mut visited, &mut res, start_vet);

    res
}

fn main() {
    let v = vals_to_vets(vec![1, 3, 2, 5, 4]);
    let edges = vec![
        [v[0], v[1]],
        [v[0], v[3]],
        [v[1], v[2]],
        [v[2], v[3]],
        [v[2], v[4]],
        [v[3], v[4]],
    ];

    let mut graph = GraphAdjList::new(edges);
    println!("\n初始化后，图为");
    graph.print();

    /* 添加边 */
    // 顶点 1, 2 即 v[0], v[2]
    graph.add_edge(v[0], v[2]);
    println!("\n添加边 1-2 后，图为");
    graph.print();

    // BFS
    let mut res = graph_bfs(graph.clone(), v[0].clone());
    println!("\n广度优先遍历（BFS）顶点序列为");
    println!("{:?}", vets_to_vals(res));

    /* 删除边 */
    // 顶点 1, 3 即 v[0], v[1]
    graph.remove_edge(v[0], v[1]);
    println!("\n删除边 1-3 后，图为");
    graph.print();

    /* 添加顶点 */
    let v5 = Vertex { val: 6 };
    graph.add_vertex(v5);
    println!("\n添加顶点 6 后，图为");
    graph.print();

    /* 删除顶点 */
    // 顶点 3 即 v[1]
    graph.remove_vertex(v[1]);
    println!("\n删除顶点 3 后，图为");
    graph.print();


    // DFS
    res = graph_dfs(graph.clone(), v[0].clone());
    println!("\n深度优先遍历（DFS）顶点序列为");
    println!("{:?}", vets_to_vals(res));
}
