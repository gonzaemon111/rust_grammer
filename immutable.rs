fn main() {
    // let x - 1+2;だとエラーになる
    let mut x = 1 + 2;
    // 再代入できる。
    x = 5;
    println!("{}", x); // => 5
}

