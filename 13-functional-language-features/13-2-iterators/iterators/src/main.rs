fn main() {
    let v = vec![1, 2, 3];
    let mut v_iter = v.iter();
    assert_eq!(v_iter.next(), Some(&1));
    assert_eq!(v_iter.next(), Some(&2));
    assert_eq!(v_iter.next(), Some(&3));
    assert_eq!(v_iter.next(), None);
    println!("{:?}", v_iter.next());
}
