pub type Asparagus = StalkPlant;
pub type Inches = u8;
pub type PercentFresh = i32;

#[derive(Debug)]
pub struct StalkPlant {
    pub length: Inches,
    pub ripeness: PercentFresh,
}