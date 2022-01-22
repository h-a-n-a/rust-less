use crate::extend::string::StringExtend;
use crate::extend::vec_str::VecStrExtend;
use crate::new_less::select::Selector;

#[test]
fn test_select_parse() {
  let demo_select_list = vec![
    r#".a .b"#.to_string(),
    r#".a>.b"#.to_string(),
    r#"h1>.b"#.to_string(),
    r#"h1>#b1"#.to_string(),
    r#"h1~#b"#.to_string(),
    r#"h1~textarea"#.to_string(),
    r#"h1~*textarea"#.to_string(),
    r#"h1~img"#.to_string(),
    r#"*h1~*textarea"#.to_string(),
    r#".a.b"#.to_string(),
    r#"*.a+*.b"#.to_string(),
    r#">a"#.to_string(),
    r#">.b"#.to_string(),
    r#".b > a"#.to_string(),
    r#"p::first-line"#.to_string(),
    r#"selector:pseudo-class"#.to_string(),
    r#"\.a\\.b"#.to_string(),
    r#".a>.b,.c .d"#.to_string(),
    r#".\a>._b,.-c .d"#.to_string(),
    r#".a>"#.to_string(),
    r#".a~"#.to_string(),
    r#".a&c"#.to_string(),
    r#".a[id="xyz"]>.c"#.to_string(),
    r#".a[id*="xyz"]>.c"#.to_string(),
  ];
  let target = r#"
.a .b
.a > .b
h1 > .b
h1 > #b1
h1 ~ #b
h1 ~ textarea
h1 ~ *textarea
h1 ~ img
*h1 ~ *textarea
.a.b
*.a + *.b
> a
> .b
.b > a
p::first-line
selector:pseudo-class
\.a\\.b
.a > .b -> .c .d
.\a > ._b -> .-c .d
.a
.a
.a $(&)c
.a[id="xyz"] > .c
.a[id*="xyz"] > .c
  "#;
  let mut base = "".to_string();
  demo_select_list.into_iter().for_each(|tt| {
    let ss = Selector::new(tt).unwrap();
    let value;
    if ss.single_select_txt.len() < 2 {
      value = ss.single_select_txt.poly();
    } else {
      value = ss.single_select_txt.join(" -> ");
    }
    base += &value;
    println!("{:?}", value);
  });
  assert_eq!(base.simple_compare(), target.to_string().simple_compare());
}

#[test]
fn test_select_error_parse() {
  let mut haserror = 0;
  let demo_select_list = vec![
    r#"."#.to_string(),
    r#"$"#.to_string(),
    r#".b > > a"#.to_string(),
    r#".b$"#.to_string(),
    r#".b@"#.to_string(),
    r#".b.c!"#.to_string(),
    r#">&c"#.to_string(),
    r#"+&c"#.to_string(),
    r#".a[*id="xyz"]>.c"#.to_string(),
    r#".a[="xyz"]>.c"#.to_string(),
    r#".a[id="xyz">.c"#.to_string(),
    r#".a[id="xyz>.c"#.to_string(),
  ];
  demo_select_list.into_iter().for_each(|tt| {
    match Selector::new(tt) {
      Ok(_) => {
        haserror += 1;
      }
      Err(msg) => {
        haserror += 0;
        println!("{:?}", msg);
      }
    };
  });
  assert_eq!(haserror, 0)
}