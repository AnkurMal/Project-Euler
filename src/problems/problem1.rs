pub fn solve() {
    let vec = (1..1000).filter(|x| x%3==0 || x%5==0).collect::<Vec<i32>>();
    println!("{}", vec.iter().sum::<i32>())
}