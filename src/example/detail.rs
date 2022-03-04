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

#[bench]
fn parse_number_bench(bench: &mut Bencher) {
  bench.iter(|| {
    let is_number = |char: &str| -> bool { char.parse::<i32>().is_ok() };
    let list = vec!["a", "1"];
    list.iter().for_each(|x| {
      is_number(x);
    });
  });
}

#[bench]
fn parse_number_compare_bench(bench: &mut Bencher) {
  bench.iter(|| {
    let is_number = |char: &str| -> bool {
      vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "0"].contains(&char)
    };
    let list = vec!["a", "1"];
    list.iter().for_each(|x| {
      is_number(x);
    });
  });
}

#[bench]
fn parse_str_u32_bench(bench: &mut Bencher) {
  bench.iter(|| {
    let char = "0123456789";
    let charlist = char.chars().map(|x| x as u32).collect::<Vec<u32>>();
    for item in charlist {
      println!("{:#?}", item);
    }
  });
}

#[bench]
fn parse_str_bench(bench: &mut Bencher) {
  bench.iter(|| {
    let chars = "0123456789";
    let charlist = chars
      .chars()
      .map(|x| String::from(x))
      .collect::<Vec<String>>();
    for item in charlist {
      println!("{:#?}", item);
    }
  });
}
