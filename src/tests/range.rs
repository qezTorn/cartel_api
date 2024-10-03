use crate::model::Range;

#[test]
fn test_range(){
    let r = Range::new().build();
    dbg!(&r);
    println!("Empty range is: {}", r.use_range());

    let f = Range::new().from(1727631659).build();
    println!("Range with just from is: {}", f.use_range());
    assert!(f.use_range() == "&from=1727631659");

    let t = Range::new().to(1727631659).build();
    println!("Range with just to is: {}", t.use_range());
    assert!(t.use_range() == "&to=1727631659");

    let ft = Range::new().from(1727631659).to(1727631659).build();
    println!("Range with both is: {}", ft.use_range());
    assert!(ft.use_range() == "&from=1727631659&to=1727631659");
    ()
}
