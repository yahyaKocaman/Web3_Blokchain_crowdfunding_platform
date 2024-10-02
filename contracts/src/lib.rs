#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol, Vec};

#[contracttype]
pub enum DataKey {
    Projects,
    Project(Symbol),
}

#[contracttype]
pub struct ProjectInfo {
    creator: Address,
    goal: i128,
    deadline: u64,
    current_amount: i128,
}

#[contract]
pub struct CrowdfundingPlatform;

#[contractimpl]
impl CrowdfundingPlatform {
    pub fn init(env: Env) {
        let projects: Vec<Symbol> = Vec::new(&env);
        env.storage().instance().set(&DataKey::Projects, &projects);
    }

    pub fn create_project(env: Env, creator: Address, name: Symbol, goal: i128, deadline: u64) -> Symbol {
        creator.require_auth();

        let mut projects: Vec<Symbol> = env.storage().instance().get(&DataKey::Projects).unwrap_or_else(|| Vec::new(&env));
        projects.push_back(name.clone());
        env.storage().instance().set(&DataKey::Projects, &projects);

        let project_info = ProjectInfo {
            creator,
            goal,
            deadline,
            current_amount: 0,
        };
        env.storage().instance().set(&DataKey::Project(name.clone()), &project_info);
        name
    }

    pub fn fund_project(env: Env, funder: Address, project: Symbol, amount: i128) {
        funder.require_auth();

        let mut project_info: ProjectInfo = env.storage().instance().get(&DataKey::Project(project.clone())).unwrap();
        if env.ledger().timestamp() > project_info.deadline {
            panic!("Project deadline has passed");
        }
        project_info.current_amount += amount;
        env.storage().instance().set(&DataKey::Project(project), &project_info);
    }

    pub fn get_project_status(env: Env, project: Symbol) -> ProjectInfo {
        env.storage().instance().get(&DataKey::Project(project)).unwrap()
    }

    pub fn list_projects(env: Env) -> Vec<Symbol> {
        env.storage().instance().get(&DataKey::Projects).unwrap_or_else(|| Vec::new(&env))
    }

    pub fn finalize_project(env: Env, project: Symbol) {
        let project_info: ProjectInfo = env.storage().instance().get(&DataKey::Project(project.clone())).unwrap();
        
        if env.ledger().timestamp() <= project_info.deadline {
            panic!("Project deadline has not passed yet");
        }

        if project_info.current_amount >= project_info.goal {
            // üreticiye fon aktarı
            //  Stellar'ın yerel token transferi
        } else {
            // Tüm katkıda bulunanlara para iadesi 
            
        }

       
        let mut projects: Vec<Symbol> = env.storage().instance().get(&DataKey::Projects).unwrap();
        projects.remove(projects.binary_search(&project).unwrap());
        env.storage().instance().set(&DataKey::Projects, &projects);
        env.storage().instance().remove(&DataKey::Project(project));
    }
}

mod test;
