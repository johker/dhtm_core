// Temporal Memory Module
//

pub struct Cell {
    pub state: CellState,
    pub synapses: Vec<Synapse>,
    pub proximal_segement: Segment,
    pub distal_segments: Vec<Segment>,
    pub column: Link,
}

pub struct Synapse {
    id: u64,
    permanence: f32,
    functional: bool,
    link: Link,
}

pub struct Segment {
    id: u64,
    active: bool,
    link: Link,
}

pub struct Link {
    address: std::string::String,
}

pub enum CellState {
    inactive,
    active,
    predictive,
}
