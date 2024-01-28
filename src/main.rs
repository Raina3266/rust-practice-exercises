mod numbers;
mod lists;

fn main() {
    let list = vec![1, 2, 3, 0, 9, 6, 8];
    let target = 0;
    let result = lists::question_6(&list, target);
    println!("{result:?}");
}
