use crate::new_less::context::Context;
use crate::new_less::var_node::VarNode;
use test::Bencher;

#[bench]
fn parse_value_bench(bench: &mut Bencher) {
  bench.iter(|| {
    let mut index = 0;
    while index < 1000 {
      let content = r#"@width:400px;"#.to_string();
      VarNode::new(content, None, None, None, Context::default());
      index += 1;
    }
  });
}
