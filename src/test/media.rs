use crate::new_less::media::MediaQuery;

#[test]
fn test_select_parse() {
    let demo_select_list = vec![
        r#"@media screen and ( max-width: 900px)"#.to_string(),
        r#"@media screen and ( max-width  : 900px)"#.to_string(),
    ];
    let mut haserror = 0;
    demo_select_list.into_iter().for_each(|tt| {
        match MediaQuery::new(tt, None, None) {
            Ok(_) => {
                haserror += 0;
            }
            Err(msg) => {
                haserror += 1;
                println!("{:?}", msg);
            }
        };
    });
    assert_eq!(haserror, 0);
}

#[test]
fn test_select_error_parse() {
    let mut haserror = 0;
    let demo_select_list = vec![r#"@media screen and ( a: 900px:)"#.to_string()];
    demo_select_list.into_iter().for_each(|tt| {
        match MediaQuery::new(tt, None, None) {
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
