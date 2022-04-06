use test::Bencher;

#[bench]
fn parse_number_bench(bench: &mut Bencher) {
  bench.iter(|| {
    let is_number = |char: char| -> bool { char.to_string().parse::<i32>().is_ok() };
    let list = vec!['a', '1'];
    list.iter().for_each(|x| {
      is_number(*x);
    });
  });
}

#[bench]
fn parse_number_compare_bench(bench: &mut Bencher) {
  bench.iter(|| {
    let is_number = |char: char| -> bool {
      vec!['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'].contains(&char)
    };
    let list = vec!['a', '1'];
    list.iter().for_each(|x| {
      is_number(*x);
    });
  });
}

#[bench]
fn parse_str_u32_bench(bench: &mut Bencher) {
  bench.iter(|| {
    let char = "0123456789";
    let charlist = char.chars().collect::<Vec<char>>();
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
