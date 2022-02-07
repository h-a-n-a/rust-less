use crate::new_less::file::path_resolve;
use crate::new_less::fileinfo::FileInfo;
use crate::new_less::option::ParseOption;
use test::Bencher;

#[bench]
fn parse_less_bench(bench: &mut Bencher) {
  bench.iter(|| {
    // 处理过程
    let filepath = path_resolve("assets/demo.less");
    FileInfo::create_disklocation(filepath.clone(), Default::default()).unwrap();
  });
}

#[bench]
fn parse_less_bench_without_sourcemap(bench: &mut Bencher) {
  bench.iter(|| {
    // 处理过程
    let filepath = path_resolve("assets/demo.less");
    FileInfo::create_disklocation(
      filepath.clone(),
      ParseOption {
        include_path: None,
        sourcemap: false,
        tabspaces: 4,
      },
    )
    .unwrap();
  });
}
