use crate::new_less::context::Context;
use crate::new_less::file::path_resolve;
use test::Bencher;

#[bench]
fn parse_less_bench(bench: &mut Bencher) {
  bench.iter(|| {
    let filepath = path_resolve("assets/arco-pro/2.less");
    let context = Context::new(Default::default(), Some(filepath.clone())).unwrap();
    context.render(filepath).unwrap();
  });
}
