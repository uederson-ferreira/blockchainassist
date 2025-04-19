#![test_only]

use multiversx_sc::{
    types::{Address, BigUint, ManagedBuffer},
    storage::mappers::SingleValueMapper,
};
use multiversx_sc_scenario::{
    rust_biguint, DebugApi, testing_framework::*,
    scenario_model::*,
};

use erc20_token::*;

const WASM_PATH: &'static str = "../output/erc20_token.wasm";

struct ContractSetup<ContractObjBuilder>
where
    ContractObjBuilder: 'static + Copy + Fn() -> erc20_token::ContractObj<DebugApi>,
{
    pub blockchain_wrapper: BlockchainStateWrapper,
    pub owner_address: Address,
    pub user_address: Address,
    pub contract_wrapper: ContractObjWrapper<erc20_token::ContractObj<DebugApi>, ContractObjBuilder>,
}

fn setup_contract<ContractObjBuilder>(
    cf_builder: ContractObjBuilder,
) -> ContractSetup<ContractObjBuilder>
where
    ContractObjBuilder: 'static + Copy + Fn() -> erc20_token::ContractObj<DebugApi>,
{
    let rust_zero = rust_biguint!(0u64);
    let mut blockchain_wrapper = BlockchainStateWrapper::new();
    let owner_address = blockchain_wrapper.create_user_account(&rust_zero);
    let user_address = blockchain_wrapper.create_user_account(&rust_zero);
    
    let contract_wrapper = blockchain_wrapper.create_sc_account(
        &rust_zero,
        Some(&owner_address),
        cf_builder,
        WASM_PATH,
    );

    // Inicializa o contrato com valores padrão
    blockchain_wrapper.execute_tx(&owner_address, &contract_wrapper, &rust_zero, |sc| {
        // Nome: "Test Token", Ticker: "TEST", 18 casas decimais, supply inicial: 1.000.000 tokens
        let initial_supply = BigUint::from(1_000_000u64);
        let token_name = ManagedBuffer::from("Test Token");
        let token_ticker = ManagedBuffer::from("TEST");
        let token_decimals = 18u8;

        sc.init(
            initial_supply,
            token_name,
            token_ticker,
            token_decimals,
        );
    }).assert_ok();

    ContractSetup {
        blockchain_wrapper,
        owner_address,
        user_address,
        contract_wrapper,
    }
}

#[test]
fn test_init() {
    let mut setup = setup_contract(erc20_token::contract_obj);
    
    // Testa os valores iniciais do token
    setup.blockchain_wrapper.execute_query(&setup.contract_wrapper, |sc| {
        assert_eq!(
            ManagedBuffer::from("Test Token"),
            sc.get_name()
        );
        assert_eq!(
            ManagedBuffer::from("TEST"),
            sc.get_ticker()
        );
        assert_eq!(
            18u8,
            sc.get_decimals()
        );
        assert_eq!(
            BigUint::from(1_000_000u64),
            sc.total_supply()
        );
        
        // Verifica que o supply inicial foi atribuído ao criador do contrato
        assert_eq!(
            BigUint::from(1_000_000u64),
            sc.balance_of(&ManagedAddress::from_address(&setup.owner_address))
        );
        
        // Verifica que o contrato não está pausado inicialmente
        assert_eq!(false, sc.is_paused());
        
        // Verifica que a taxa inicial é 0%
        assert_eq!(0u64, sc.get_fee_percentage());
    });
}

#[test]
fn test_transfer() {
    let mut setup = setup_contract(erc20_token::contract_obj);
    let owner_addr = setup.owner_address.clone();
    let user_addr = setup.user_address.clone();
    
    // Owner transfere 1000 tokens para o usuário
    setup.blockchain_wrapper.execute_tx(&owner_addr, &setup.contract_wrapper, &rust_biguint!(0), |sc| {
        let user_managed_addr = ManagedAddress::from_address(&user_addr);
        let amount = BigUint::from(1000u64);
        
        sc.transfer(&user_managed_addr, &amount);
    }).assert_ok();
    
    // Verifica se os saldos foram atualizados corretamente
    setup.blockchain_wrapper.execute_query(&setup.contract_wrapper, |sc| {
        let owner_managed_addr = ManagedAddress::from_address(&owner_addr);
        let user_managed_addr = ManagedAddress::from_address(&user_addr);
        
        assert_eq!(
            BigUint::from(999_000u64),  // 1.000.000 - 1.000
            sc.balance_of(&owner_managed_addr)
        );
        assert_eq!(
            BigUint::from(1000u64),
            sc.balance_of(&user_managed_addr)
        );
    });
}

#[test]
fn test_transfer_with_fee() {
    let mut setup = setup_contract(erc20_token::contract_obj);
    let owner_addr = setup.owner_address.clone();
    let user_addr = setup.user_address.clone();
    
    // Configura uma taxa de 5% (500 basis points)
    setup.blockchain_wrapper.execute_tx(&owner_addr, &setup.contract_wrapper, &rust_biguint!(0), |sc| {
        sc.set_fee_percentage(500u64); // 5%
    }).assert_ok();
    
    // Owner transfere 1000 tokens para o usuário (com taxa de 5%)
    setup.blockchain_wrapper.execute_tx(&owner_addr, &setup.contract_wrapper, &rust_biguint!(0), |sc| {
        let user_managed_addr = ManagedAddress::from_address(&user_addr);
        let amount = BigUint::from(1000u64);
        
        sc.transfer(&user_managed_addr, &amount);
    }).assert_ok();
    
    // Verifica se os saldos foram atualizados corretamente considerando a taxa
    setup.blockchain_wrapper.execute_query(&setup.contract_wrapper, |sc| {
        let owner_managed_addr = ManagedAddress::from_address(&owner_addr);
        let user_managed_addr = ManagedAddress::from_address(&user_addr);
        
        // Owner deve ter: 1.000.000 - 1.000 + 50 (taxa) = 999.050
        assert_eq!(
            BigUint::from(999_050u64),
            sc.balance_of(&owner_managed_addr)
        );
        
        // Usuário deve ter: 1.000 - 50 (taxa) = 950
        assert_eq!(
            BigUint::from(950u64),
            sc.balance_of(&user_managed_addr)
        );
    });
}

#[test]
fn test_approve_and_transfer_from() {
    let mut setup = setup_contract(erc20_token::contract_obj);
    let owner_addr = setup.owner_address.clone();
    let user_addr = setup.user_address.clone();
    
    // Owner aprova o usuário a gastar 500 tokens
    setup.blockchain_wrapper.execute_tx(&owner_addr, &setup.contract_wrapper, &rust_biguint!(0), |sc| {
        let user_managed_addr = ManagedAddress::from_address(&user_addr);
        let amount = BigUint::from(500u64);
        
        sc.approve(&user_managed_addr, &amount);
    }).assert_ok();
    
    // Verifica se a allowance foi configurada corretamente
    setup.blockchain_wrapper.execute_query(&setup.contract_wrapper, |sc| {
        let owner_managed_addr = ManagedAddress::from_address(&owner_addr);
        let user_managed_addr = ManagedAddress::from_address(&user_addr);
        
        assert_eq!(
            BigUint::from(500u64),
            sc.allowance(&owner_managed_addr, &user_managed_addr)
        );
    });
    
    // Usuário transfere 300 tokens do owner para si mesmo usando transferFrom
    setup.blockchain_wrapper.execute_tx(&user_addr, &setup.contract_wrapper, &rust_biguint!(0), |sc| {
        let owner_managed_addr = ManagedAddress::from_address(&owner_addr);
        let user_managed_addr = ManagedAddress::from_address(&user_addr);
        let amount = BigUint::from(300u64);
        
        sc.transfer_from(&owner_managed_addr, &user_managed_addr, &amount);
    }).assert_ok();
    
    // Verifica os saldos e allowance após transferFrom
    setup.blockchain_wrapper.execute_query(&setup.contract_wrapper, |sc| {
        let owner_managed_addr = ManagedAddress::from_address(&owner_addr);
        let user_managed_addr = ManagedAddress::from_address(&user_addr);
        
        // Owner deve ter 999.700 tokens
        assert_eq!(
            BigUint::from(999_700u64),
            sc.balance_of(&owner_managed_addr)
        );
        
        // Usuário deve ter 300 tokens
        assert_eq!(
            BigUint::from(300u64),
            sc.balance_of(&user_managed_addr)
        );
        
        // Allowance restante deve ser 200 (500 - 300)
        assert_eq!(
            BigUint::from(200u64),
            sc.allowance(&owner_managed_addr, &user_managed_addr)
        );
    });
}

#[test]
fn test_mint_and_burn() {
    let mut setup = setup_contract(erc20_token::contract_obj);
    let owner_addr = setup.owner_address.clone();
    let user_addr = setup.user_address.clone();
    
    // Owner cria 5000 tokens para o usuário
    setup.blockchain_wrapper.execute_tx(&owner_addr, &setup.contract_wrapper, &rust_biguint!(0), |sc| {
        let user_managed_addr = ManagedAddress::from_address(&user_addr);
        let amount = BigUint::from(5000u64);
        
        sc.mint_endpoint(&user_managed_addr, &amount);
    }).assert_ok();
    
    // Verifica se o mint funcionou corretamente
    setup.blockchain_wrapper.execute_query(&setup.contract_wrapper, |sc| {
        let user_managed_addr = ManagedAddress::from_address(&user_addr);
        
        assert_eq!(
            BigUint::from(5000u64),
            sc.balance_of(&user_managed_addr)
        );
        
        // Supply total deve ser 1.005.000
        assert_eq!(
            BigUint::from(1_005_000u64),
            sc.total_supply()
        );
    });
    
    // Owner queima 2000 tokens do usuário
    setup.blockchain_wrapper.execute_tx(&owner_addr, &setup.contract_wrapper, &rust_biguint!(0), |sc| {
        let user_managed_addr = ManagedAddress::from_address(&user_addr);
        let amount = BigUint::from(2000u64);
        
        sc.burn_endpoint(&user_managed_addr, &amount);
    }).assert_ok();
    
    // Verifica se o burn funcionou corretamente
    setup.blockchain_wrapper.execute_query(&setup.contract_wrapper, |sc| {
        let user_managed_addr = ManagedAddress::from_address(&user_addr);
        
        assert_eq!(
            BigUint::from(3000u64),  // 5000 - 2000
            sc.balance_of(&user_managed_addr)
        );
        
        // Supply total deve ser 1.003.000
        assert_eq!(
            BigUint::from(1_003_000u64),
            sc.total_supply()
        );
    });
}

#[test]
fn test_public_mint() {
    let mut setup = setup_contract(erc20_token::contract_obj);
    let user_addr = setup.user_address.clone();
    
    // Usuário solicita o public_mint
    setup.blockchain_wrapper.execute_tx(&user_addr, &setup.contract_wrapper, &rust_biguint!(0), |sc| {
        sc.public_mint();
    }).assert_ok();
    
    // Verifica se o usuário recebeu 10 tokens
    setup.blockchain_wrapper.execute_query(&setup.contract_wrapper, |sc| {
        let user_managed_addr = ManagedAddress::from_address(&user_addr);
        
        assert_eq!(
            BigUint::from(10u64),
            sc.balance_of(&user_managed_addr)
        );
    });
    
    // Tenta solicitar novamente e deve falhar
    let result = setup.blockchain_wrapper.execute_tx(&user_addr, &setup.contract_wrapper, &rust_biguint!(0), |sc| {
        sc.public_mint();
    });
    
    assert!(result.result.is_err());
}

#[test]
fn test_pause_functionality() {
    let mut setup = setup_contract(erc20_token::contract_obj);
    let owner_addr = setup.owner_address.clone();
    let user_addr = setup.user_address.clone();
    
    // Owner pausa o contrato
    setup.blockchain_wrapper.execute_tx(&owner_addr, &setup.contract_wrapper, &rust_biguint!(0), |sc| {
        sc.pause();
    }).assert_ok();
    
    // Verifica se o contrato está pausado
    setup.blockchain_wrapper.execute_query(&setup.contract_wrapper, |sc| {
        assert_eq!(true, sc.is_paused());
    });
    
    // Tenta fazer uma transferência enquanto pausado - deve falhar
    let result = setup.blockchain_wrapper.execute_tx(&owner_addr, &setup.contract_wrapper, &rust_biguint!(0), |sc| {
        let user_managed_addr = ManagedAddress::from_address(&user_addr);
        let amount = BigUint::from(100u64);
        
        sc.transfer(&user_managed_addr, &amount);
    });
    
    assert!(result.result.is_err());
    
    // Owner despausa o contrato
    setup.blockchain_wrapper.execute_tx(&owner_addr, &setup.contract_wrapper, &rust_biguint!(0), |sc| {
        sc.unpause();
    }).assert_ok();
    
    // Tenta transferir novamente - agora deve funcionar
    setup.blockchain_wrapper.execute_tx(&owner_addr, &setup.contract_wrapper, &rust_biguint!(0), |sc| {
        let user_managed_addr = ManagedAddress::from_address(&user_addr);
        let amount = BigUint::from(100u64);
        
        sc.transfer(&user_managed_addr, &amount);
    }).assert_ok();
}

#[test]
fn test_burn_own() {
    let mut setup = setup_contract(erc20_token::contract_obj);
    let owner_addr = setup.owner_address.clone();
    let user_addr = setup.user_address.clone();
    
    // Primeiro transfere alguns tokens para o usuário
    setup.blockchain_wrapper.execute_tx(&owner_addr, &setup.contract_wrapper, &rust_biguint!(0), |sc| {
        let user_managed_addr = ManagedAddress::from_address(&user_addr);
        let amount = BigUint::from(500u64);
        
        sc.transfer(&user_managed_addr, &amount);
    }).assert_ok();
    
    // Usuário queima 200 dos seus próprios tokens
    setup.blockchain_wrapper.execute_tx(&user_addr, &setup.contract_wrapper, &rust_biguint!(0), |sc| {
        let amount = BigUint::from(200u64);
        
        sc.burn_own(&amount);
    }).assert_ok();
    
    // Verifica se o saldo foi atualizado e o supply reduziu
    setup.blockchain_wrapper.execute_query(&setup.contract_wrapper, |sc| {
        let user_managed_addr = ManagedAddress::from_address(&user_addr);
        
        assert_eq!(
            BigUint::from(300u64),  // 500 - 200
            sc.balance_of(&user_managed_addr)
        );
        
        // Supply total deve ser 999.800 (1.000.000 - 200)
        assert_eq!(
            BigUint::from(999_800u64),
            sc.total_supply()
        );
    });
}

#[test]
fn test_invalid_operations() {
    let mut setup = setup_contract(erc20_token::contract_obj);
    let owner_addr = setup.owner_address.clone();
    let user_addr = setup.user_address.clone();
    
    // 1. Teste: Usuário tenta transferir mais do que possui
    let result = setup.blockchain_wrapper.execute_tx(&user_addr, &setup.contract_wrapper, &rust_biguint!(0), |sc| {
        let owner_managed_addr = ManagedAddress::from_address(&owner_addr);
        let amount = BigUint::from(1000u64);  // Usuário não tem tokens inicialmente
        
        sc.transfer(&owner_managed_addr, &amount);
    });
    
    assert!(result.result.is_err());
    
    // 2. Teste: Usuário tenta definir a taxa (apenas owner pode)
    let result = setup.blockchain_wrapper.execute_tx(&user_addr, &setup.contract_wrapper, &rust_biguint!(0), |sc| {
        sc.set_fee_percentage(200u64);
    });
    
    assert!(result.result.is_err());
    
    // 3. Teste: Owner tenta definir taxa acima do limite (10000 basis points)
    let result = setup.blockchain_wrapper.execute_tx(&owner_addr, &setup.contract_wrapper, &rust_biguint!(0), |sc| {
        sc.set_fee_percentage(15000u64);  // 150% - acima do limite
    });
    
    assert!(result.result.is_err());
}

// Função approve precisa ser implementada no contrato principal antes de executar este teste
// Caso contrário, adicione a implementação aqui ou comente este teste
#[test]
fn test_approve() {
    let mut setup = setup_contract(erc20_token::contract_obj);
    let owner_addr = setup.owner_address.clone();
    let user_addr = setup.user_address.clone();
    
    // Implementation of approve if not in main contract
    // Adicione ao trait Erc20Token:
    /*
    #[endpoint]
    fn approve(&self, spender: &ManagedAddress, amount: &BigUint) {
        self.require_not_paused();
        
        let caller = self.blockchain().get_caller();
        self.allowances(&caller, spender).set(amount);
        
        self.approve_event(&caller, spender, amount);
    }
    */
    
    // Owner aprova o usuário a gastar 1000 tokens
    setup.blockchain_wrapper.execute_tx(&owner_addr, &setup.contract_wrapper, &rust_biguint!(0), |sc| {
        let user_managed_addr = ManagedAddress::from_address(&user_addr);
        let amount = BigUint::from(1000u64);
        
        sc.approve(&user_managed_addr, &amount);
    }).assert_ok();
    
    // Verifica se a allowance foi configurada corretamente
    setup.blockchain_wrapper.execute_query(&setup.contract_wrapper, |sc| {
        let owner_managed_addr = ManagedAddress::from_address(&owner_addr);
        let user_managed_addr = ManagedAddress::from_address(&user_addr);
        
        assert_eq!(
            BigUint::from(1000u64),
            sc.allowance(&owner_managed_addr, &user_managed_addr)
        );
    });
}

// Mock da função approve para o teste acima
impl<Api: multiversx_sc::api::VMApi> Erc20Token for ContractObj<Api> {
    #[endpoint]
    fn approve(&self, spender: &ManagedAddress, amount: &BigUint) {
        self.require_not_paused();
        
        let caller = self.blockchain().get_caller();
        self.allowances(&caller, spender).set(amount);
        
        self.approve_event(&caller, spender, amount);
    }
}