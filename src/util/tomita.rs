use std::collections::HashSet;
use petgraph::graphmap::{GraphMap,NodeTrait};
use petgraph::Undirected;
use petgraph::EdgeType;

// https://github.com/horasal/clique-rust/blob/master/src/tomita.rs

/// Tomita, Etsuji, Akira Tanaka, and Haruhisa Takahashi. "The worst-case time complexity for
/// generating all maximal cliques and computational experiments." Theoretical Computer Science
/// 363.1 (2006): 28-42.
///
/// connected is a symmetrical bolean matrix, N the number of nodes in the graph,
/// values of the diagonal should be true.
pub struct Tomita<N: NodeTrait, E, Ty = Undirected> {
    graph: GraphMap<N, E, Ty>,
    max_cliques: Vec<HashSet<N>>
}

impl<N: NodeTrait, E, Ty: EdgeType> Tomita<N, E, Ty> {
    pub fn new(graphmap: GraphMap<N, E, Ty>) -> Tomita<N, E, Ty> {
        Tomita {
            graph: graphmap,
            max_cliques: Vec::new()
        }
    }

    pub fn compute(&mut self) {
        let p = self.graph.nodes().collect::<HashSet<N>>();
        let r = HashSet::new();
        let x = HashSet::new();
        self.tomita(p, r, x);
    }

    pub fn cliques(&self) -> &Vec<HashSet<N>> {
        &self.max_cliques
    }

    fn pivot(&mut self, p: HashSet<N>) -> N {
        let pvec : Vec<N> = p.clone().into_iter().collect::<Vec<_>>();
        return pvec[pvec.clone().iter().
            map(| v | self.graph.neighbors(v.clone()).collect::<HashSet<N>>().
                intersection(&p).cloned().collect::<Vec<_>>().len()).
                collect::<Vec<_>>().iter().enumerate().
                map(|(x, y)| (y, x)).max().unwrap().1 as usize]
    }

    fn tomita(&mut self, p: HashSet<N>, r: HashSet<N>, x: HashSet<N>) {
        let mut p_fp = p.clone();
        let mut x_fp = x.clone();

        if p.is_empty() {
            if x.is_empty() {
                self.max_cliques.push(r.clone());
            }
            return;
        }

        let u = self.pivot(p);
        let u_neighbours = self.graph.neighbors(u.clone()).collect::<HashSet<N>>();
        let real_p = p_fp.difference(&u_neighbours).cloned().collect::<HashSet<N>>();

        for v in real_p.iter() {
            let v_neighbours = self.graph.neighbors(v.clone()).collect::<HashSet<N>>();

            let p_intersect_v_neighbors = p_fp.intersection(&v_neighbours).cloned().collect();
            let mut r_union_v = r.clone();
            r_union_v.insert(v.clone());
            let x_intersect_v_neighbors = x_fp.intersection(&v_neighbours).cloned().collect();

            self.tomita(p_intersect_v_neighbors, r_union_v, x_intersect_v_neighbors);

            p_fp.remove(v);
            x_fp.insert(*v);
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_cliques() {
        let mut graph = GraphMap::<&str, (), Undirected>::new();
        
        // Add clique 1 edges
        graph.add_edge("a", "b", ());
        graph.add_edge("b", "c", ());
        graph.add_edge("c", "a", ());
        
        // Add clique 2 edges
        graph.add_edge("d", "e", ());
        graph.add_edge("d", "f", ());
        graph.add_edge("d", "g", ());
        graph.add_edge("e", "f", ());
        graph.add_edge("e", "g", ());
        graph.add_edge("f", "g", ());
        
        let mut tomita = Tomita::new(graph);
        tomita.compute();
        
        let cliques = tomita.cliques();
        
        // Should find both cliques
        assert_eq!(cliques.len(), 2);
        
        // Convert results to sets for easier comparison
        let clique_sets: Vec<HashSet<&str>> = cliques.iter()
            .map(|clique| clique.iter().copied().collect())
            .collect();
        
        // Check for clique 1
        let clique1: HashSet<_> = ["a", "b", "c"].iter().copied().collect();
        assert!(clique_sets.iter().any(|set| 
            set.len() == 3 && 
            set.iter().all(|&x| clique1.contains(x))
        ));
        
        // Check for clique 2
        let clique2: HashSet<_> = ["d", "e", "f", "g"].iter().copied().collect();
        assert!(clique_sets.iter().any(|set| 
            set.len() == 4 && 
            set.iter().all(|&x| clique2.contains(x))
        ));
    }

    #[test]
    fn test_single_clique() {
        let mut graph = GraphMap::<&str, (), Undirected>::new();
        
        // Create a single triangle clique
        graph.add_edge("x", "y", ());
        graph.add_edge("y", "z", ());
        graph.add_edge("z", "x", ());
        
        let mut tomita = Tomita::new(graph);
        tomita.compute();
        
        let cliques = tomita.cliques();
        
        // Should find one clique
        assert_eq!(cliques.len(), 1);
        
        // Check the clique contents
        let clique: HashSet<_> = cliques[0].iter().copied().collect();
        let expected: HashSet<_> = ["x", "y", "z"].iter().copied().collect();
        assert_eq!(clique.len(), 3);
        assert!(clique.iter().all(|&x| expected.contains(x)));
    }

    #[test]
    fn test_pivot_selection() {
        let mut graph = GraphMap::<&str, (), Undirected>::new();
        
        // Create a clique of 4 nodes with an extra node that only connects to one of them
        // a-b-c-d (all connected to each other)
        //   |
        //   e     (only connected to b)
        graph.add_edge("a", "b", ());
        graph.add_edge("a", "c", ());
        graph.add_edge("a", "d", ());
        graph.add_edge("b", "c", ());
        graph.add_edge("b", "d", ());
        graph.add_edge("b", "e", ()); // extra connection
        graph.add_edge("c", "d", ());
        
        let mut tomita = Tomita::new(graph);
        
        // Node 'b' should be selected as pivot as it has the most connections (4)
        let p: HashSet<_> = ["a", "b", "c", "d", "e"].iter().copied().collect();
        assert_eq!(tomita.pivot(p), "b");
        
        // Verify we find the correct maximal clique
        tomita.compute();
        let cliques = tomita.cliques();
        
        // Should find one maximal clique of size 4
        assert_eq!(cliques.len(), 1);
        let clique: HashSet<_> = cliques[0].iter().copied().collect();
        let expected: HashSet<_> = ["a", "b", "c", "d"].iter().copied().collect();
        assert_eq!(clique, expected);
    }
}