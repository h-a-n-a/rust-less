use crate::new_less::file_manger::FileManger;

#[test]
fn test_rel_path() {
  let a = "../test/a.txt".to_string();
  let b = "./test/a.txt".to_string();
  let c = "/test/a.txt".to_string();
  assert_eq!(FileManger::is_relative_path(&a), true);
  assert_eq!(FileManger::is_relative_path(&b), true);
  assert_eq!(FileManger::is_relative_path(&c), true);
}
