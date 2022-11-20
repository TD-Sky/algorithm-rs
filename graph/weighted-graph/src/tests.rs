use crate::{WeiEdge, WeiGraph};
use std::io::{stdout, Result as IOResult, Write};

fn sample() -> WeiGraph {
    let mut graph = WeiGraph::new();

    graph.add_edge(35, (4, 5), (), ());
    graph.add_edge(37, (4, 7), (), ());
    graph.add_edge(28, (5, 7), (), ());
    graph.add_edge(16, (0, 7), (), ());
    graph.add_edge(32, (1, 5), (), ());
    graph.add_edge(38, (0, 4), (), ());
    graph.add_edge(17, (2, 3), (), ());
    graph.add_edge(19, (1, 7), (), ());
    graph.add_edge(26, (0, 2), (), ());
    graph.add_edge(36, (1, 2), (), ());
    graph.add_edge(29, (1, 3), (), ());
    graph.add_edge(34, (2, 7), (), ());
    graph.add_edge(40, (6, 2), (), ());
    graph.add_edge(52, (3, 6), (), ());
    graph.add_edge(58, (6, 0), (), ());
    graph.add_edge(93, (6, 4), (), ());

    graph
}

fn print_mst(name: &'static str, mst: Vec<&WeiEdge>) -> IOResult<()> {
    let weight_sum = weight_sum(&mst);
    let mut stdout = stdout().lock();

    stdout.write_fmt(format_args!("\n{name}\n"))?;
    for edge in mst {
        stdout.write_fmt(format_args!("{edge:?}\n"))?;
    }
    stdout.write_fmt(format_args!("{weight_sum:?}\n"))?;

    Ok(())
}

fn weight_sum(mst: &[&WeiEdge]) -> i32 {
    mst.iter().map(|&edge| edge.weight).sum()
}

#[test]
fn lazy_prim_mst() -> IOResult<()> {
    let graph = sample();
    let mst = graph.lazy_prim_mst(0).unwrap();

    print_mst("lazy_prim_mst", mst)?;

    Ok(())
}

#[test]
fn prim_mst() -> IOResult<()> {
    let graph = sample();
    let mst = graph.prim_mst(0).unwrap();

    print_mst("prim_mst", mst)?;

    Ok(())
}

#[test]
fn kruskal_mst() -> IOResult<()> {
    let graph = sample();
    let mst = graph.kruskal_mst().unwrap();

    print_mst("kruskal_mst", mst)?;

    Ok(())
}
