fn main() {
    let v = vec![1, 2, 3];
    let mut v_iter = v.iter();
    assert_eq!(v_iter.next(), Some(&1));
    assert_eq!(v_iter.next(), Some(&2));
    assert_eq!(v_iter.next(), Some(&3));
    assert_eq!(v_iter.next(), None);

    let v1 = vec![1, 2, 3];
    let mut v_iter = v1.into_iter();
    println!("{:?}", v1);
    assert_eq!(v_iter.next(), Some(1));
    assert_eq!(v_iter.next(), Some(2));
    assert_eq!(v_iter.next(), Some(3));
    assert_eq!(v_iter.next(), None);

    let mut v = vec![1, 2, 3];
    let mut v_iter = v.iter_mut();
    assert_eq!(v_iter.next(), Some(&mut 1));
    assert_eq!(v_iter.next(), Some(&mut 2));
    assert_eq!(v_iter.next(), Some(&mut 3));
    assert_eq!(v_iter.next(), None);
}
