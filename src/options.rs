//! Options model.

#[derive(Debug, Clone, PartialEq)]
pub enum Mode { Default, Processes, Refresh, Kill(u32), Cpu, Memory, Disk }

#[derive(Debug, Clone)]
pub struct Options { pub mode: Mode }
impl Default for Options { fn default() -> Self { Self { mode: Mode::Default } } }
#[cfg(test)] mod tests { use super::*; #[test] fn o() { assert!(matches!(Options::default().mode, Mode::Default)); } }
