use std::{sync::atomic::Ordering, collections::HashMap};

#[derive(Debug, Clone)]
pub struct Thread {
    pub instructions: Vec<Instructions>,
    /// Threads have their own local memory (imagine core caches)
    /// This only synchronises with the main program memory under specific atomic actions
    pub memory: HashMap<Path, usize>,
    pub flags: Flags,
}

#[derive(Debug, Clone)]
pub struct Flags {
    /// Whether the last registered condition was successful.
    pub condition: bool,
}

#[derive(Debug, Clone)]
pub struct Program {
    /// An AVM program starts with a set of sync instructions that get the memory into
    /// some pre-determined state.
    pub init: Vec<SyncInstructions>,
    /// A program has many threads. These run the atomic operations and have their own unsync memory
    pub threads: Vec<Thread>,
    /// The main memory for the program. When atomic operations allow it, the thread memory syncs here
    pub sync_memory: HashMap<Path, usize>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Instructions {
    Atomic(AtomicInstructions),
    Sync(SyncInstructions),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum AtomicInstructions {
    CompareExchange(CompareExchange),
    Load(Load),
    Store(Store),
    ReadModifyWrite(ReadModifyWrite),
    Fence(Ordering),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum SyncInstructions {
    Assign(Assign),
    Operation(Operation),
    /// Define a jump label
    Label(String),
    /// go-to the label unconditionally
    Goto(String),
    /// go-to the label if the condition flag is true
    Branch(String),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Assign {
    pub from: Path,
    pub to: Path,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
/// An operation on two values
pub enum Operation {
    /// Calculates `to += from`
    AddAssign(Assign),
    /// Calculates `to -= from`
    SubAssign(Assign),
    /// Swaps `from` and `to`
    Swap(Assign),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
/// A compare exchange operation. If the `atomic` has a value equal to `old`
/// then the value of `new` is set. `old` is updated to be equal to the current `atomic` value if this fails.
/// The condition flag is set to `true` if this succeeds, and false otherwise.
pub struct CompareExchange {
    pub atomic: Path,
    pub old: Path,
    pub new: Path,
    pub success: Ordering,
    pub failure: Ordering,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
/// An atomic [`Operation`]. It reads the value atomically, then updates the value atomically - with no room for activity inbetween.
/// Almost equivalent to a Load + Operation + CompareExchange loop. `to` in this operation is always the atomic value
pub struct ReadModifyWrite {
    pub operation: Operation,
    pub ordering: Ordering,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Path {
    pub components: Vec<String>,
}

/// Atomically load the data in `from` and store it in `to`.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Load {
    pub assign: Assign,
    pub ordering: Ordering,
}

/// Load the data in `from` and store it atomically in `to`.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Store {
    pub assign: Assign,
    pub ordering: Ordering,
}
