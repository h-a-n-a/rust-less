use crate::extend::time::wastetime;
use crate::new_less::file::path_resolve;
use crate::new_less::fileinfo::FileInfo;
use crate::new_less::option::ParseOption;

#[test]
fn test_less() {
    let record = wastetime("test_less");
    // 处理过程
    let filepath = path_resolve("assets/demo.less");
    let info = FileInfo::create_disklocation(filepath, Default::default()).unwrap();
    record();
    println!("{:#?}", info);
    let json = serde_json::to_string_pretty(&info.tojson()).unwrap();
    println!("{}", json);
}

#[test]
fn test_less_bench() {
    let record = wastetime("test_less");
    // 处理过程
    let filepath = path_resolve("assets/demo.less");
    let mut index = 0;
    while index < 100 {
        FileInfo::create_disklocation(filepath.clone(), Default::default()).unwrap();
        index += 1;
    }
    record();
}

#[test]
fn test_less_bench_without_sourcemap() {
    let record = wastetime("test_less");
    // 处理过程
    let filepath = path_resolve("assets/demo.less");
    let mut index = 0;
    while index < 100 {
        FileInfo::create_disklocation(
            filepath.clone(),
            ParseOption {
                include_path: None,
                sourcemap: false,
            },
        )
        .unwrap();
        index += 1;
    }
    record();
}
