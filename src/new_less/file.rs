use std::ffi::OsString;
use std::path::Path;

pub fn path_resolve(path: &str) -> String {
  let work_cwd = env!("CARGO_MANIFEST_DIR");
  let os_work_cwd = OsString::from(work_cwd);
  return Path::new(&os_work_cwd)
    .join(path)
    .into_os_string()
    .into_string()
    .unwrap();
}

pub fn readfile(path: String) -> Option<String> {
  let filepath = Path::new(&path);
  if filepath.exists() {
    match std::fs::read_to_string(filepath) {
      Ok(content) => { Some(content) }
      Err(_) => { None }
    }
  } else {
    None
  }
}