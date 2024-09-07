#![cfg(test)]
use super::*;
use soroban_sdk::{symbol_short, testutils::Address as _, Address, Env};
use soroban_sdk::testutils::{Ledger, LedgerInfo};

#[test]
fn test_crowdfunding_platform() {
    let env = Env::default();
    let contract_id = env.register_contract(None, CrowdfundingPlatform);
    let client = CrowdfundingPlatformClient::new(&env, &contract_id);

    let creator = Address::generate(&env);
    let funder = Address::generate(&env);

    env.mock_all_auths();

    client.init();

    let project_name = symbol_short!("project1");
    let goal = 1000;
    let deadline = env.ledger().timestamp() + 86400; // 1 gün sonra sona erecek

    client.create_project(&creator, &project_name, &goal, &deadline);

    client.fund_project(&funder, &project_name, &500);

    let project_info = client.get_project_status(&project_name);
    assert_eq!(project_info.creator, creator);
    assert_eq!(project_info.goal, goal);
    assert_eq!(project_info.deadline, deadline);
    assert_eq!(project_info.current_amount, 500);

    let projects = client.list_projects();
    assert_eq!(projects.len(), 1);
    assert_eq!(projects.get(0).unwrap(), project_name);

    // test projesi sonlandırıldığında projeyi silme
    env.ledger().set(LedgerInfo {
        timestamp: deadline + 1,
        protocol_version: 0,
        sequence_number: 0,
        network_id: Default::default(),
        base_reserve: 0,
        min_temp_entry_ttl: 0,
        min_persistent_entry_ttl: 0,
        max_entry_ttl: 0,
    });
    client.finalize_project(&project_name);

    let projects = client.list_projects();
    assert_eq!(projects.len(), 0);
}
