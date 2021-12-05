pub trait Solution {
    fn part_one(&self);
    fn part_two(&self);
    fn result(&self) {
        self.part_one();
        self.part_two();
    }
}
