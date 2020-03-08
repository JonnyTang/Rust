fn foo(input: Option<i32>) -> Option<i32> {
    if input.is_none() {
        return None;
    }

    let input = input.unwrap();
    if input < 0 {
        return None;
    }

    Some(input)
    /*
    等同于
    input.and_then(|i| {
        if i < 0 {
            None
        } else {
            Some(i)
        }
    })
     */
}

fn bar(input: Option<i32>) -> Result<i32, ErrNegative> {
    match foo(input) {
        Some(n) => Ok(n),
        None => Err(ErrNegative),
    }
    /*
    等同于
    foo(input).ok_or(ErrNegative)
     */
}