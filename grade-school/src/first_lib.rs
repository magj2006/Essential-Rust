// This annotation prevents Clippy from warning us that `School` has a
// `fn new()` with no arguments, but doesn't implement the `Default` trait.
//
// Normally, it's good practice to just do what Clippy tells you, but in this
// case, we want to keep things relatively simple. The `Default` trait is not the point
// of this exercise.

#[allow(clippy::new_without_default)]
#[derive(Clone)]
pub struct School {
    roster: Vec<(u32, String)>,
}

impl School {
    pub fn new() -> School {
        School { roster: Vec::new() }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.roster.push((grade, String::from(student)))
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grades = self.roster.iter().map(|&(g, _)| g).collect::<Vec<u32>>();
        grades.sort();

        grades.dedup();

        grades
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut grade = self
            .roster
            .iter()
            .filter(|&k| k.0 == grade)
            .map(|(_, s)| s.clone())
            .collect::<Vec<String>>();

        grade.sort_by(|a, b| a.cmp(b));

        grade
    }
}
