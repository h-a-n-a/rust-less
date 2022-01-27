use crate::extend::string::StringExtend;
use crate::new_less::fileinfo::FileInfo;

#[test]
fn test_error_var_check() {
    let code = r#"
@width:400px;
a{
  font-size:10px;

  .c{
    font-size:20px;
  }
}

dfkljaskdlfjadfjadlskfj

asldkfjak
    "#;
    let msg = FileInfo::create_txt_content(code.to_string(), Default::default(), None)
        .err()
        .unwrap();
    assert_eq!(
        msg.simple_compare(),
        "the word is not with endqueto -> dfkljaskdlfjadfjadlskfjasldkfjak"
            .to_string()
            .simple_compare()
    );
    println!("........");
}
