
enum IntValue {
    Signed(i32),
    Unsigned(usize),
}

trait GetWithAnyInt<T> {
    fn getWithAnyInt(&self, value: &IntValue) -> Option<T>;
}

impl<T: Copy> GetWithAnyInt<T> for Vec<T> {
    fn getWithAnyInt(&self, value: &IntValue) -> Option<T> {
        match value {
            IntValue::Signed(index) => None,
            IntValue::Unsigned(index) => Some(self[*index])
        }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let index_values = vec![
        IntValue::Signed(3),
        IntValue::Unsigned(4),
        IntValue::Signed(-2),
        IntValue::Unsigned(2),
    ];

    // for i in index_values.iter() {
    //     println!("{}", number_list[i]);
    // }

    for i in index_values.iter() {
        println!("{}", number_list.getWithAnyInt(i).unwrap_or_else(|| -1));
    }
}