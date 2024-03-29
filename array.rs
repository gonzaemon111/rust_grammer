fn main() {
    let a: [isize;3] = [1, 2 , 3];
    // `&配列` でスライスが作れる。
    let b: &[isize] = &a;
    // スライスをフォーマットするにはプレースホルダが`{:?}`になる。
    println!("{:?}", b); // => [1, 2, 3]
    for elm in b {
      println!("{}", elm);
    }
    // => 1
    //    2
    //    3

    // あるいは`(スライス/配列)[インデックス]`で要素にアクセスできる。
    println!("{:?}", b[0]); // => 1
}