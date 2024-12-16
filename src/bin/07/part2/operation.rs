#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Add,
    Mul,
    Concat,
}

pub fn generate_combinations(n: usize) -> Vec<Vec<Operation>> {
    let mut out: Vec<Vec<Operation>> = Vec::new();

    if n == 0 {
        out.push(vec![]);
        return out;
    };

    generate_combinations(n - 1).iter().for_each(|combo| {
        let mut add_combo = combo.clone();
        add_combo.push(Operation::Add);
        out.push(add_combo);

        let mut mul_combo = combo.clone();
        mul_combo.push(Operation::Mul);
        out.push(mul_combo);

        let mut concat_combo = combo.clone();
        concat_combo.push(Operation::Concat);
        out.push(concat_combo);
    });

    out
}
