pub struct Player{
    pub name: String,
    pub score: u32,
}
// Define a trait to represent printable objects
trait Printable {
    fn to_string(&self) -> String;
}

impl Printable for Player {
    fn to_string(&self) -> String {
        format!("{} ({})", self.name, self.score)
    }
}