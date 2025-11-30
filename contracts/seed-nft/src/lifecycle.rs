use soroban_sdk::contracttype;

#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LifecycleState {
    Seed = 0,
    Germinated = 1,
    PlantVegetative = 2,
    PlantFlowering = 3,
    PlantHarvested = 4,
    Processed = 5,
    Distributed = 6,
    Consumed = 7,
}

impl LifecycleState {
    pub fn from_u32(value: u32) -> Option<Self> {
        match value {
            0 => Some(LifecycleState::Seed),
            1 => Some(LifecycleState::Germinated),
            2 => Some(LifecycleState::PlantVegetative),
            3 => Some(LifecycleState::PlantFlowering),
            4 => Some(LifecycleState::PlantHarvested),
            5 => Some(LifecycleState::Processed),
            6 => Some(LifecycleState::Distributed),
            7 => Some(LifecycleState::Consumed),
            _ => None,
        }
    }

    pub fn to_u32(self) -> u32 {
        self as u32
    }

    pub fn can_transition_to(self, to: LifecycleState) -> bool {
        match (self, to) {
            (LifecycleState::Seed, LifecycleState::Germinated) => true,
            (LifecycleState::Germinated, LifecycleState::PlantVegetative) => true,
            (LifecycleState::PlantVegetative, LifecycleState::PlantFlowering) => true,
            (LifecycleState::PlantFlowering, LifecycleState::PlantHarvested) => true,
            (LifecycleState::PlantHarvested, LifecycleState::Processed) => true,
            (LifecycleState::Processed, LifecycleState::Distributed) => true,
            (LifecycleState::Distributed, LifecycleState::Consumed) => true,
            _ => false,
        }
    }
}

