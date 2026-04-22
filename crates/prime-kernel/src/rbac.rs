use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Capability {
    ReadFiles,
    WriteFiles,
    ExecuteShell,
    NetworkAccess,
    SpawnAgents,
    ManageMemory,
    ManageSkills,
    UseBrowser,
    MakePurchases,
    AccessSecrets,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub name: String,
    pub capabilities: HashSet<Capability>,
}

#[derive(Debug, Default)]
pub struct RbacEngine {
    pub roles: HashMap<String, Role>,
    pub agent_roles: HashMap<String, String>,
}

impl RbacEngine {
    pub fn new() -> Self {
        let mut engine = Self::default();
        engine.register_defaults();
        engine
    }

    fn register_defaults(&mut self) {
        self.roles.insert("researcher".into(), Role {
            name: "researcher".into(),
            capabilities: [
                Capability::ReadFiles,
                Capability::NetworkAccess,
                Capability::ManageMemory,
                Capability::ManageSkills,
            ].into(),
        });
        self.roles.insert("coder".into(), Role {
            name: "coder".into(),
            capabilities: [
                Capability::ReadFiles,
                Capability::WriteFiles,
                Capability::ExecuteShell,
                Capability::NetworkAccess,
                Capability::ManageMemory,
                Capability::ManageSkills,
            ].into(),
        });
        self.roles.insert("orchestrator".into(), Role {
            name: "orchestrator".into(),
            capabilities: [
                Capability::ReadFiles,
                Capability::WriteFiles,
                Capability::NetworkAccess,
                Capability::SpawnAgents,
                Capability::ManageMemory,
                Capability::ManageSkills,
            ].into(),
        });
    }

    pub fn assign(&mut self, agent_id: &str, role: &str) {
        self.agent_roles.insert(agent_id.into(), role.into());
    }

    pub fn can(&self, agent_id: &str, cap: &Capability) -> bool {
        if let Some(role_name) = self.agent_roles.get(agent_id) {
            if let Some(role) = self.roles.get(role_name) {
                return role.capabilities.contains(cap);
            }
        }
        false
    }
}
