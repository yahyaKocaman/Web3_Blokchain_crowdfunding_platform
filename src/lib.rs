#![no_std]
use soroban_sdk::{contractimpl, symbol_short, vec, Env, Symbol, Vec};

pub struct CrowdfundingPlatform;

#[contractimpl]
impl CrowdfundingPlatform {
    pub fn init(env: Env) {
        let projects: Vec<Symbol> = vec![&env];
        env.storage().instance().set(&symbol_short!("projects"), &projects);
    }

    pub fn create_project(env: Env, name: Symbol, goal: i128, deadline: u64) -> Symbol {
        let mut projects: Vec<Symbol> = env.storage().instance().get(&symbol_short!("projects")).unwrap();
        projects.push_back(name.clone());
        env.storage().instance().set(&symbol_short!("projects"), &projects);

        env.storage().instance().set(&name, &(goal, deadline, 0_i128));
        name
    }

    pub fn fund_project(env: Env, project: Symbol, amount: i128) {
        let (goal, deadline, current_amount) = env.storage().instance().get(&project).unwrap();
        if env.ledger().timestamp() > deadline {
            panic!("Project deadline has passed");
        }
        let new_amount = current_amount + amount;
        env.storage().instance().set(&project, &(goal, deadline, new_amount));
    }

    pub fn get_project_status(env: Env, project: Symbol) -> (i128, u64, i128) {
        env.storage().instance().get(&project).unwrap()
    }

    pub fn list_projects(env: Env) -> Vec<Symbol> {
        env.storage().instance().get(&symbol_short!("projects")).unwrap()
    }
}
