use cumulus_primitives_core::ParaId;
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup, Properties};
use sc_service::ChainType;
use hex_literal::hex;
use serde::{Deserialize, Serialize};
use sp_core::{Pair, Public, sr25519, H160, U256 };
use parachain_runtime::{DOLLARS,AccountId, Signature, SchedulerConfig, 
                        DemocracyConfig, EVMConfig, EthereumConfig,TokensConfig, 
                        ContractsConfig, ElectionsConfig, NFTConfig};
use sp_runtime::traits::{IdentifyAccount, Verify};
use std::collections::BTreeMap;
use std::str::FromStr;
use fp_currency::currency::{ELP,DOT,BTC};

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<parachain_runtime::GenesisConfig, Extensions>;

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
	/// The relay chain of the Parachain.
	pub relay_chain: String,
	/// The id of the Parachain.
	pub para_id: u32,
}

impl Extensions {
	/// Try to get the extension from the given `ChainSpec`.
	pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
		sc_chain_spec::get_extension(chain_spec.extensions())
	}
}

type AccountPublic = <Signature as Verify>::Signer;

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

pub fn development_config(id: ParaId) -> ChainSpec {
	ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Local,
		move || {
			testnet_genesis(
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				],
				id,
			)
		},
		vec![],
		None,
                Some("pnx"),  // Protocol ID
		None,
		Extensions {
			relay_chain: "rococo-dev".into(),
			para_id: id.into(),
		},
	)
}

pub fn local_testnet_config(id: ParaId) -> ChainSpec {
        let mut properties = Properties::new();
        properties.insert("ss58Format".into(), "42".into());
        properties.insert("tokenSymbol".into(), "PNX".into());
        properties.insert("tokenDecimals".into(), 18.into());

	ChainSpec::from_genesis(
		// Name
		"phoenix",
		// ID
		"phoenix",
		ChainType::Local,
		//ChainType::Live,
		move || {
			testnet_genesis(
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
                                        hex!["c2cdcf01af7163d2d99b2ec87954e4c1b735e9e9ea80f8775bf29dd9457eaca1"].into(),
                                        hex!["0d6d2fcaed2f2ccd5c1d5c86468490f2aafeec8b7cb14af512cdf8c7980183a3"].into(),
				],
				id,
			)
		},
                vec![], // Bootnodes
                None,   // Telemetry
                Some("pnx"),  // Protocol ID
                Some(properties),
                Extensions {
                        relay_chain: "rococo-local-raw.json".into(),
                        para_id: id.into(),
                },
	)
}

fn testnet_genesis(
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	id: ParaId,
) -> parachain_runtime::GenesisConfig {
        // add by WangYi
        const STASH: u128 = 20_000 * DOLLARS;
        const ENDOWMENT: u128 = 30_000 * DOLLARS;
        const ETH_BALANCE: u128 = 500_000 * DOLLARS;

        let num_endowed_accounts = endowed_accounts.len();

        let gerald_evm_account_id = H160::from_str("6be02d1d3665660d22ff9624b7be0551ee1ac91b").unwrap();
        let mut evm_accounts = BTreeMap::new();
        evm_accounts.insert(
                gerald_evm_account_id,
                pallet_evm::GenesisAccount {
                        nonce: 0.into(),
                        balance: U256::from(ETH_BALANCE),
                        storage: BTreeMap::new(),
                        code: vec![],
                },
        );

	parachain_runtime::GenesisConfig {
		frame_system: parachain_runtime::SystemConfig {
			code: parachain_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
			changes_trie_config: Default::default(),
		},
		pallet_balances: parachain_runtime::BalancesConfig {
			balances: endowed_accounts
				.iter()
				.cloned()
				.map(|k| (k, ENDOWMENT))
				.collect(),
		},
		pallet_sudo: parachain_runtime::SudoConfig { key: root_key },
		parachain_info: parachain_runtime::ParachainInfoConfig { parachain_id: id },

                pallet_scheduler: SchedulerConfig {},
                pallet_democracy: DemocracyConfig {},

                pallet_ethereum: EthereumConfig {},
                pallet_evm: EVMConfig {
                        accounts: evm_accounts,
                },

                pallet_contracts: ContractsConfig {
                    current_schedule: pallet_contracts::Schedule::default(),
                },

                pallet_elections_phragmen: ElectionsConfig {
                        members: endowed_accounts.iter()
                                                .take((num_endowed_accounts + 1) / 2)
                                                .cloned()
                                                .map(|member| (member, STASH))
                                                .collect(),
                },

                orml_tokens: TokensConfig {
                        endowed_accounts: endowed_accounts.iter()
                                .flat_map(|x| {
                                        vec![
                                                (x.clone(), ELP, 1_000_000 * DOLLARS),
                                                (x.clone(), DOT, 1_000_000 * DOLLARS),
                                                (x.clone(), BTC, 1_000_000 * DOLLARS),
                                        ]
                                })
                                .collect(),
                },

                orml_nft: NFTConfig { tokens: vec![] },
	}
}
