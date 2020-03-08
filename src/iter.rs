pub fn run() {
    let vec = vec![0, 1, 2, 3];

    vec.iter()/*.map(|x| x + 1).filter(|x| x > 1)*/.for_each(|x| println!("{}", x));

    let vec1 = vec![0, 1, 2, 3];
    for (i, v) in vec1.iter().chain(Some(42).iter()).enumerate() {
        println!("{}: {}", i, v);
    }

    let vec2: Vec<_> = vec![0, 1, 2, 3];
    vec2.iter().for_each(|v| println!("{}", v));
    //等同如下
    for v in &vec2 {
        println!("{}", v);
    }

    let vec3 = vec![0, 1, 2, 3];
    let mut iter = (&vec).into_iter();
    while let Some(v) = iter.next() {
        println!("{}", v);
    }
}