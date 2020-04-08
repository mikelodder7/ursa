#[macro_use]
extern crate criterion;
extern crate bulletproofs_amcl;
extern crate zmix;

use zmix::amcl_wrapper::group_elem::GroupElement;
use criterion::Criterion;
use bulletproofs_amcl as bulletproofs;

fn small_set_benchmark(c: &mut Criterion) {

}

criterion_group!(
    name = bench_merkle_trees;
    config = Criterion::default();
    targets = small_set_benchmark
);

criterion_main!(bench_merkle_trees);

