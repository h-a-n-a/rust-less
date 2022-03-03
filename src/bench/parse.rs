use crate::new_less::context::Context;
use crate::new_less::file::path_resolve;
use crate::new_less::option::ParseOption;
use test::Bencher;

#[bench]
fn parse_less_bench(bench: &mut Bencher) {
  bench.iter(|| {
    // 处理过程
    let filepath = path_resolve("assets/demo.less");
    let context = Context::new(Default::default(), Some(filepath.clone())).unwrap();
    context.render(filepath).unwrap();
  });
}

#[bench]
fn render_less_arco_pro_bench(bench: &mut Bencher) {
  bench.iter(|| {
    // 处理过程
    let filepath = path_resolve("assets/arco-pro/13.less");
    let context = Context::new(Default::default(), Some(filepath.clone())).unwrap();
    context.render(filepath).unwrap();
  });
}

#[bench]
fn parse_less_bench_without_sourcemap(bench: &mut Bencher) {
  bench.iter(|| {
    // 处理过程
    let filepath = path_resolve("assets/base.less");
    let context = Context::new(
      ParseOption {
        include_path: vec![],
        sourcemap: false,
        tabspaces: 2,
        hooks: Default::default(),
      },
      Some(filepath.clone()),
    )
      .unwrap();
    context.render(filepath).unwrap();
  });
}

async fn add(num: i32) -> i32 {
  let mut i = 0;
  let mut val = num;
  while i < 3 {
    val += 1;
    i += 1;
  }
  val
}

#[bench]
fn async_add_bench(bench: &mut Bencher) {
  bench.iter(|| {
    let num = 1;
    let rt = tokio::runtime::Runtime::new().unwrap();
    let res = rt.block_on(async {
      let exec_times: i32 = 5;
      let mut task_list = vec![];
      let mut index = 0;
      while index < exec_times {
        let task = add(num);
        task_list.push(task);
        index += 1;
      }
      let res = futures::future::join_all(task_list).await;
      res
    });
    println!("{:#?}", res);
  });
}

#[bench]
fn add_bench(bench: &mut Bencher) {
  let add = |num: i32| {
    let mut i = 0;
    let mut val = num;
    while i < 3 {
      val += 1;
      i += 1;
    }
    val
  };

  bench.iter(|| {
    let num = 1;
    let res = {
      let exec_times: i32 = 5;
      let mut list = vec![];
      let mut index = 0;
      while index < exec_times {
        let res = add(num);
        list.push(res);
        index += 1;
      }
      list
    };
    println!("{:#?}", res);
  });
}

#[bench]
fn add_thread_bench(bench: &mut Bencher) {
  let add = |num: i32| {
    let mut i = 0;
    let mut val = num;
    while i < 3 {
      val += 1;
      i += 1;
    }
    val
  };

  bench.iter(|| {
    let mut tasklist = vec![];
    let mut index = 0;
    while index < 5 {
      let task = std::thread::spawn(move || {
        let res = add(1);
        res
      });
      tasklist.push(task);
      index += 1;
    }
    for task in tasklist{
      task.join().unwrap();
    }
  });
}
