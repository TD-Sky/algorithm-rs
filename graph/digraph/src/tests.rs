use crate::DiGraph;

fn sample() -> DiGraph<()> {
    let mut digraph: DiGraph<()> = DiGraph::new();

    digraph.add_edge((0, 2), (), ());
    digraph.add_edge((0, 1), (), ());
    digraph.add_edge((0, 5), (), ());
    digraph.add_edge((3, 5), (), ());
    digraph.add_edge((3, 4), (), ());
    digraph.add_edge((2, 4), (), ());
    digraph.add_edge((2, 1), (), ());
    digraph.add_edge((2, 3), (), ());

    digraph
}

#[test]
fn shortest_path() {
    let digraph = sample();
    let path = vec![0, 5];

    assert_eq!(digraph.shortest_path(0, 5), Ok(path));
}
