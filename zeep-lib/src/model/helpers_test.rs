use crate::model::helpers_content::restrictions::{CheckRestrictions, Restrictions};
use std::rc::Rc;

#[test]
fn can_check_restrictions_on_string() {
    let r = Rc::new(Restrictions {
        min_length: Some(6),
        ..Default::default()
    });

    let s = "hello".to_string();
    assert!(s.check_restrictions(Some(r.clone())).is_err());

    let s = "hello world".to_string();
    assert!(s.check_restrictions(Some(r)).is_ok());
}
