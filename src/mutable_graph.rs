mod mutable_graph {
    use std::rc::Rc;
    use std::borrow::{Borrow};
    use std::cell::RefCell;
    use std::any::TypeId;
    use std::hash::Hash;
    use std::fmt::Debug;

    type NodeList<T> = Vec<Rc<Node<T>>>;

    #[derive(Debug)]
    pub struct Node<T> {
        pub data :T,
        weight: Option<i32>,
        next: RefCell<NodeList<T>>
    }

    #[derive(Debug)]
    pub struct Graph<T> {
        nodes: NodeList<T>,
        n_vertices :i32,
        n_edges :i32,
    }

    impl <T> Node<T> {
        pub fn get_connected(&self) -> NodeList<T> {
            return self.next.borrow().clone();
        }
    }

    impl <T> Graph<T> {
        pub fn new_undirected() -> Graph<T> {
            Graph {
                nodes: vec![],
                n_vertices: 0,
                n_edges: 0,
            }
        }

        pub fn new_node(&mut self, data :T) -> Rc<Node<T>> {
            let node = Rc::new(Node { data, weight: None, next:RefCell::new(vec![]) });
            self.nodes.push(node.clone());
            self.n_vertices += 1;
            return node;
        }

        pub fn new_directional_edge(&mut self, src: &Rc<Node<T>>, dst: &Rc<Node<T>>) {
            let node :&Node<T> = src.borrow();
            let mut next = node.next.borrow_mut();
            next.push(dst.clone());
            self.n_edges += 1;
        }

        pub fn new_bidirectional_edge(&mut self, n1: &Rc<Node<T>>, n2: &Rc<Node<T>>) {
            self.new_bidirectional_edge(n1, n2);
            self.new_bidirectional_edge(n2, n1);
        }

    }

    /*
    pub fn bfs<T :Hash + Eq + Debug>(graph :Graph<T>, start :&Rc<Node<T>>) {
        let mut processed = HashSet::new();
        let mut discovered = HashSet::new();

        let mut queue = VecDeque::new();

        queue.push_back(start);
        while !queue.is_empty() {
            let v_ref = queue.pop_front().unwrap();
            let v: &Node<T> = v_ref.borrow();
            process_vertex_early(&v.data);
            processed.insert(&v.data);
            let p = v.next.borrow().clone();
            for nb in p {
                let y = nb.data.borrow().clone();
                process_edge(&v.data, &y); // Always directed graph
                if !discovered.contains(&y) {
                    queue.push_back(&nb);
                    discovered.insert(y);
                    //parent
                }
            }
            process_vertex_late(&v);
        }
    }

    pub fn process_vertex_late<T :Debug>(item: &T) {
        println!("Processed late: {:?}", &item)
    }
    pub fn process_vertex_early<T :Debug>(item: &T) {
        println!("Processed vertex: {:?}", &item)
    }
    pub fn process_edge<T :Debug>(from: &T, to: &T) {
        println!("Processed edge: {:?} -> {:?}", from, to)
    }
    */

}


#[cfg(test)]
mod tests {
    use crate::mutable_graph::mutable_graph::Graph;

    #[test]
    fn test_simple() {
        let mut x = Graph::new_undirected();
        let a = x.new_node(1);
        let b = x.new_node(2);
        let c = x.new_node(3);
        let d = x.new_node(4);

        x.new_directional_edge(&a, &b);
        x.new_directional_edge(&b, &c);
        x.new_directional_edge(&d, &a);
        x.new_directional_edge(&a, &c);
        x.new_directional_edge(&a, &d);

//        println!("Graph: {:?}", &x);

        for connected in a.get_connected() {
            println!("Con: {}", connected.data);
        }
    }

}

