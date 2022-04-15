//! A stack-based executor with customizable state.
//! A memory-based state is provided, but can replaced by a custom
//! implementation, for exemple one interacting with a database.

mod executor;
mod memory;

pub use self::executor::Erc20Backend;
pub use self::executor::StackSubstateMetadata;
pub use self::executor::StackState;
pub use self::executor::StackExitKind;
pub use self::executor::StackExecutor;
pub use self::executor::PrecompileSet;
pub use self::executor::PrecompileOutput;
pub use self::executor::PrecompileFn;
pub use self::executor::PrecompileFailure;
pub use self::executor::Accessed;
pub use self::executor::StatefulPrecompileSet;

pub use self::memory::MemoryStackAccount;
pub use self::memory::MemoryStackState;
pub use self::memory::MemoryStackSubstate;

pub use ethereum::Log;