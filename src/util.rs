//! ksysguard model — KDE system monitor.

#[derive(Debug, Clone)]
pub struct Process {
    pub pid: u32,
    pub name: String,
    pub cpu: f64,
    pub mem: f64,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub processes: Vec<Process>,
}

impl Default for Model {
    fn default() -> Self { Self { processes: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn refresh(&mut self, processes: Vec<Process>) { self.processes = processes; }

    pub fn kill(&mut self, pid: u32) -> bool {
        if let Some(pos) = self.processes.iter().position(|p| p.pid == pid) {
            self.processes.remove(pos);
            true
        } else { false }
    }

    pub fn list(&self) -> &[Process] { &self.processes }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn process_refresh_kill() {
        let mut k = Model::new();
        k.refresh(vec![Process { pid: 1, name: "init".into(), cpu: 0.0, mem: 0.1, status: "S".into() }]);
        assert!(k.kill(1));
    }
}
