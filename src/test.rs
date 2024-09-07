#![cfg(test)]

use super::*;
use soroban_sdk::{symbol_short, Env};

#[test]
fn test_crowdfunding_platform() {
    let env = Env::default();
    let contract_id = env.register_contract(None, CrowdfundingPlatform);
    let client = CrowdfundingPlatformClient::new(&env, &contract_id);

    client.init();

    let project_name = symbol_short!("project1");
    let goal = 1000;
    let deadline = env.ledger().timestamp() + 86400; 

    client.create_project(&project_name, &goal, &deadline);

    client.fund_project(&project_name, &500);

    let (project_goal, project_deadline, current_amount) = client.get_project_status(&project_name);
    assert_eq!(project_goal, goal);
    assert_eq!(project_deadline, deadline);
    assert_eq!(current_amount, 500);

    let projects = client.list_projects();
    assert_eq!(projects.len(), 1);
    assert_eq!(projects.get(0).unwrap(), project_name);
}
