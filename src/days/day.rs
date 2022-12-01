pub trait Day {

    fn part_one(&self, input: &Input) -> String;

    fn part_two(&self, input: &Input) -> String;
}

pub struct Input {

    pub(crate) as_string: String,

    pub(crate) as_list: Vec<String>,
}
