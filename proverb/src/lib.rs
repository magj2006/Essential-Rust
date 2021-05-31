pub fn build_proverb(list: &[&str]) -> String {
    /* first solution
    let len = list.len();

    if len == 0 {
        String::new()
    } else {
        let mut proverb = String::new();

        for i in 0..len - 1 {
            proverb.push_str(
                format!("For want of a {} the {} was lost.\n", list[i], list[i + 1]).as_str(),
            )
        }

        proverb.push_str(format!("And all for the want of a {}.", list[0]).as_str());

        proverb
    }
    */
    use std::iter;

    if list.is_empty() {
        String::new()
    } else {
        list.windows(2)
            .map(|x| format!("For want of a {} the {} was lost.\n", x[0], x[1]))
            .chain(iter::once(format!(
                "And all for the want of a {}.",
                list[0]
            )))
            .collect()
    }
}
