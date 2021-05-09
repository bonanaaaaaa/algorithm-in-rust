use std::cmp::max;
use std::collections::HashMap;

pub fn dynamic_programming(items: Vec<(i32, i32)>, limit: i32) -> i32 {
    let mut value_table: HashMap<(i32, i32), i32> = HashMap::new();
    let mut max_value = 0;
    for (u, item) in items.iter().enumerate() {
        let i = 1 + u as i32;
        for w in 0..(limit + 1) {
            let (weight, value) = item;
            let previous_value = *value_table.get(&(w, i - 1)).unwrap_or(&0);
            if w - weight >= 0 {
                let new_value = value + *value_table
                    .get(&(w - weight, i - 1))
                    .unwrap_or(&0);

                max_value = max(previous_value, new_value);
                value_table.insert((w, i), max_value);
            } else {
                value_table.insert((w, i), previous_value);
            }
        }
    }
    max_value
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_knapsack() {
    let limit = 20;
    let items = vec![(10, 8), (15, 6), (8, 11), (5, 5)];
    let actual = dynamic_programming(items, limit);

    assert_eq!(actual, 19);
  }
}