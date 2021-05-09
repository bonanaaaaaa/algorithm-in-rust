mod knapsack;

fn main() {
    let limit = 20;
    let items = vec![(10, 8), (15, 6), (8, 11), (5, 5)];
    let answer = knapsack::dynamic_programming(items, limit);

    println!("{}", answer);
}
