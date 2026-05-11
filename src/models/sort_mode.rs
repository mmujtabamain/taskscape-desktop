#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortMode {
    Manual,
    Newest,
    Oldest,
    DueDate,
    Priority,
    Alphabetical,
}

impl SortMode {
    pub const ALL: [Self; 6] = [
        Self::Manual,
        Self::Newest,
        Self::Oldest,
        Self::DueDate,
        Self::Priority,
        Self::Alphabetical,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::Manual => "Manual",
            Self::Newest => "Newest",
            Self::Oldest => "Oldest",
            Self::DueDate => "Due date",
            Self::Priority => "Priority",
            Self::Alphabetical => "A-Z",
        }
    }
}
