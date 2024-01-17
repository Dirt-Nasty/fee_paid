mod abi;
mod pb;
use hex_literal::hex;
use pb::contract::v1 as contract;
use substreams::Hex;
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_database_change::tables::Tables as DatabaseChangeTables;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables as EntityChangesTables;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;
use std::str::FromStr;
use substreams::scalar::BigDecimal;

const TRACKED_CONTRACT: [u8; 20] = hex!("5cfd3aed08a444be32839bd911ebecd688861164");

substreams_ethereum::init!();

#[substreams::handlers::map]
fn map_events(blk: eth::Block) -> Result<contract::Events, substreams::errors::Error> {
    Ok(contract::Events {
        accepted_bids: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::AcceptedBid::match_and_decode(log) {
                            return Some(contract::AcceptedBid {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                bid_id: event.bid_id.to_string(),
                                lender: event.lender,
                            });
                        }

                        None
                })
            })
            .collect(),
        admin_changeds: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::AdminChanged::match_and_decode(log) {
                            return Some(contract::AdminChanged {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                new_admin: event.new_admin,
                                previous_admin: event.previous_admin,
                            });
                        }

                        None
                })
            })
            .collect(),
        beacon_upgradeds: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::BeaconUpgraded::match_and_decode(log) {
                            return Some(contract::BeaconUpgraded {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                beacon: event.beacon,
                            });
                        }

                        None
                })
            })
            .collect(),
        cancelled_bids: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::CancelledBid::match_and_decode(log) {
                            return Some(contract::CancelledBid {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                bid_id: event.bid_id.to_string(),
                            });
                        }

                        None
                })
            })
            .collect(),
        fee_paids: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::FeePaid::match_and_decode(log) {
                            return Some(contract::FeePaid {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                amount: event.amount.to_string(),
                                bid_id: event.bid_id.to_string(),
                                fee_type: event.fee_type,
                            });
                        }

                        None
                })
            })
            .collect(),
        initializeds: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::Initialized::match_and_decode(log) {
                            return Some(contract::Initialized {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                version: event.version.to_u64(),
                            });
                        }

                        None
                })
            })
            .collect(),
        loan_liquidateds: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::LoanLiquidated::match_and_decode(log) {
                            return Some(contract::LoanLiquidated {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                bid_id: event.bid_id.to_string(),
                                liquidator: event.liquidator,
                            });
                        }

                        None
                })
            })
            .collect(),
        loan_repaids: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::LoanRepaid::match_and_decode(log) {
                            return Some(contract::LoanRepaid {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                bid_id: event.bid_id.to_string(),
                            });
                        }

                        None
                })
            })
            .collect(),
        loan_repayments: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::LoanRepayment::match_and_decode(log) {
                            return Some(contract::LoanRepayment {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                bid_id: event.bid_id.to_string(),
                            });
                        }

                        None
                })
            })
            .collect(),
        market_forwarder_approveds: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::MarketForwarderApproved::match_and_decode(log) {
                            return Some(contract::MarketForwarderApproved {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                forwarder: event.forwarder,
                                market_id: event.market_id.to_string(),
                                sender: event.sender,
                            });
                        }

                        None
                })
            })
            .collect(),
        market_forwarder_renounceds: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::MarketForwarderRenounced::match_and_decode(log) {
                            return Some(contract::MarketForwarderRenounced {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                forwarder: event.forwarder,
                                market_id: event.market_id.to_string(),
                                sender: event.sender,
                            });
                        }

                        None
                })
            })
            .collect(),
        market_owner_cancelled_bids: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::MarketOwnerCancelledBid::match_and_decode(log) {
                            return Some(contract::MarketOwnerCancelledBid {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                bid_id: event.bid_id.to_string(),
                            });
                        }

                        None
                })
            })
            .collect(),
        ownership_transferreds: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::OwnershipTransferred::match_and_decode(log) {
                            return Some(contract::OwnershipTransferred {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                new_owner: event.new_owner,
                                previous_owner: event.previous_owner,
                            });
                        }

                        None
                })
            })
            .collect(),
        pauseds: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::Paused::match_and_decode(log) {
                            return Some(contract::Paused {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                account: event.account,
                            });
                        }

                        None
                })
            })
            .collect(),
        protocol_fee_sets: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::ProtocolFeeSet::match_and_decode(log) {
                            return Some(contract::ProtocolFeeSet {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                new_fee: event.new_fee.to_u64(),
                                old_fee: event.old_fee.to_u64(),
                            });
                        }

                        None
                })
            })
            .collect(),
        submitted_bids: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::SubmittedBid::match_and_decode(log) {
                            return Some(contract::SubmittedBid {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                bid_id: event.bid_id.to_string(),
                                borrower: event.borrower,
                                metadata_uri: Vec::from(event.metadata_uri),
                                receiver: event.receiver,
                            });
                        }

                        None
                })
            })
            .collect(),
        trusted_market_forwarder_sets: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::TrustedMarketForwarderSet::match_and_decode(log) {
                            return Some(contract::TrustedMarketForwarderSet {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                forwarder: event.forwarder,
                                market_id: event.market_id.to_string(),
                                sender: event.sender,
                            });
                        }

                        None
                })
            })
            .collect(),
        unpauseds: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::Unpaused::match_and_decode(log) {
                            return Some(contract::Unpaused {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                account: event.account,
                            });
                        }

                        None
                })
            })
            .collect(),
        upgradeds: blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| log.address == TRACKED_CONTRACT)
                    .filter_map(|log| {
                        if let Some(event) = abi::contract::events::Upgraded::match_and_decode(log) {
                            return Some(contract::Upgraded {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                implementation: event.implementation,
                            });
                        }

                        None
                })
            })
            .collect(),
    })
}

#[substreams::handlers::map]
fn db_out(events: contract::Events) -> Result<DatabaseChanges, substreams::errors::Error> {
    // Initialize changes container
    let mut tables = DatabaseChangeTables::new();

    // Loop over all the abis events to create changes
    events.accepted_bids.into_iter().for_each(|evt| {
        tables
            .create_row("accepted_bid", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("bid_id", BigDecimal::from_str(&evt.bid_id).unwrap())
            .set("lender", Hex(&evt.lender).to_string());
    });
    events.admin_changeds.into_iter().for_each(|evt| {
        tables
            .create_row("admin_changed", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("new_admin", Hex(&evt.new_admin).to_string())
            .set("previous_admin", Hex(&evt.previous_admin).to_string());
    });
    events.beacon_upgradeds.into_iter().for_each(|evt| {
        tables
            .create_row("beacon_upgraded", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("beacon", Hex(&evt.beacon).to_string());
    });
    events.cancelled_bids.into_iter().for_each(|evt| {
        tables
            .create_row("cancelled_bid", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("bid_id", BigDecimal::from_str(&evt.bid_id).unwrap());
    });
    events.fee_paids.into_iter().for_each(|evt| {
        tables
            .create_row("fee_paid", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("amount", BigDecimal::from_str(&evt.amount).unwrap())
            .set("bid_id", BigDecimal::from_str(&evt.bid_id).unwrap())
            .set("fee_type", evt.fee_type);
    });
    events.initializeds.into_iter().for_each(|evt| {
        tables
            .create_row("initialized", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("version", evt.version);
    });
    events.loan_liquidateds.into_iter().for_each(|evt| {
        tables
            .create_row("loan_liquidated", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("bid_id", BigDecimal::from_str(&evt.bid_id).unwrap())
            .set("liquidator", Hex(&evt.liquidator).to_string());
    });
    events.loan_repaids.into_iter().for_each(|evt| {
        tables
            .create_row("loan_repaid", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("bid_id", BigDecimal::from_str(&evt.bid_id).unwrap());
    });
    events.loan_repayments.into_iter().for_each(|evt| {
        tables
            .create_row("loan_repayment", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("bid_id", BigDecimal::from_str(&evt.bid_id).unwrap());
    });
    events.market_forwarder_approveds.into_iter().for_each(|evt| {
        tables
            .create_row("market_forwarder_approved", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("forwarder", Hex(&evt.forwarder).to_string())
            .set("market_id", BigDecimal::from_str(&evt.market_id).unwrap())
            .set("sender", Hex(&evt.sender).to_string());
    });
    events.market_forwarder_renounceds.into_iter().for_each(|evt| {
        tables
            .create_row("market_forwarder_renounced", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("forwarder", Hex(&evt.forwarder).to_string())
            .set("market_id", BigDecimal::from_str(&evt.market_id).unwrap())
            .set("sender", Hex(&evt.sender).to_string());
    });
    events.market_owner_cancelled_bids.into_iter().for_each(|evt| {
        tables
            .create_row("market_owner_cancelled_bid", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("bid_id", BigDecimal::from_str(&evt.bid_id).unwrap());
    });
    events.ownership_transferreds.into_iter().for_each(|evt| {
        tables
            .create_row("ownership_transferred", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("new_owner", Hex(&evt.new_owner).to_string())
            .set("previous_owner", Hex(&evt.previous_owner).to_string());
    });
    events.pauseds.into_iter().for_each(|evt| {
        tables
            .create_row("paused", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("account", Hex(&evt.account).to_string());
    });
    events.protocol_fee_sets.into_iter().for_each(|evt| {
        tables
            .create_row("protocol_fee_set", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("new_fee", evt.new_fee)
            .set("old_fee", evt.old_fee);
    });
    events.submitted_bids.into_iter().for_each(|evt| {
        tables
            .create_row("submitted_bid", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("bid_id", BigDecimal::from_str(&evt.bid_id).unwrap())
            .set("borrower", Hex(&evt.borrower).to_string())
            .set("metadata_uri", Hex(&evt.metadata_uri).to_string())
            .set("receiver", Hex(&evt.receiver).to_string());
    });
    events.trusted_market_forwarder_sets.into_iter().for_each(|evt| {
        tables
            .create_row("trusted_market_forwarder_set", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("forwarder", Hex(&evt.forwarder).to_string())
            .set("market_id", BigDecimal::from_str(&evt.market_id).unwrap())
            .set("sender", Hex(&evt.sender).to_string());
    });
    events.unpauseds.into_iter().for_each(|evt| {
        tables
            .create_row("unpaused", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("account", Hex(&evt.account).to_string());
    });
    events.upgradeds.into_iter().for_each(|evt| {
        tables
            .create_row("upgraded", [("evt_tx_hash", evt.evt_tx_hash),("evt_index", evt.evt_index.to_string())])
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("implementation", Hex(&evt.implementation).to_string());
    });

    Ok(tables.to_database_changes())
}

#[substreams::handlers::map]
fn graph_out(events: contract::Events) -> Result<EntityChanges, substreams::errors::Error> {
    // Initialize changes container
    let mut tables = EntityChangesTables::new();

    // Loop over all the abis events to create changes
    events.accepted_bids.into_iter().for_each(|evt| {
        tables
            .create_row("accepted_bid", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("bid_id", BigDecimal::from_str(&evt.bid_id).unwrap())
            .set("lender", Hex(&evt.lender).to_string());
    });
    events.admin_changeds.into_iter().for_each(|evt| {
        tables
            .create_row("admin_changed", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("new_admin", Hex(&evt.new_admin).to_string())
            .set("previous_admin", Hex(&evt.previous_admin).to_string());
    });
    events.beacon_upgradeds.into_iter().for_each(|evt| {
        tables
            .create_row("beacon_upgraded", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("beacon", Hex(&evt.beacon).to_string());
    });
    events.cancelled_bids.into_iter().for_each(|evt| {
        tables
            .create_row("cancelled_bid", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("bid_id", BigDecimal::from_str(&evt.bid_id).unwrap());
    });
    events.fee_paids.into_iter().for_each(|evt| {
        tables
            .create_row("fee_paid", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("amount", BigDecimal::from_str(&evt.amount).unwrap())
            .set("bid_id", BigDecimal::from_str(&evt.bid_id).unwrap())
            .set("fee_type", evt.fee_type);
    });
    events.initializeds.into_iter().for_each(|evt| {
        tables
            .create_row("initialized", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("version", evt.version);
    });
    events.loan_liquidateds.into_iter().for_each(|evt| {
        tables
            .create_row("loan_liquidated", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("bid_id", BigDecimal::from_str(&evt.bid_id).unwrap())
            .set("liquidator", Hex(&evt.liquidator).to_string());
    });
    events.loan_repaids.into_iter().for_each(|evt| {
        tables
            .create_row("loan_repaid", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("bid_id", BigDecimal::from_str(&evt.bid_id).unwrap());
    });
    events.loan_repayments.into_iter().for_each(|evt| {
        tables
            .create_row("loan_repayment", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("bid_id", BigDecimal::from_str(&evt.bid_id).unwrap());
    });
    events.market_forwarder_approveds.into_iter().for_each(|evt| {
        tables
            .create_row("market_forwarder_approved", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("forwarder", Hex(&evt.forwarder).to_string())
            .set("market_id", BigDecimal::from_str(&evt.market_id).unwrap())
            .set("sender", Hex(&evt.sender).to_string());
    });
    events.market_forwarder_renounceds.into_iter().for_each(|evt| {
        tables
            .create_row("market_forwarder_renounced", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("forwarder", Hex(&evt.forwarder).to_string())
            .set("market_id", BigDecimal::from_str(&evt.market_id).unwrap())
            .set("sender", Hex(&evt.sender).to_string());
    });
    events.market_owner_cancelled_bids.into_iter().for_each(|evt| {
        tables
            .create_row("market_owner_cancelled_bid", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("bid_id", BigDecimal::from_str(&evt.bid_id).unwrap());
    });
    events.ownership_transferreds.into_iter().for_each(|evt| {
        tables
            .create_row("ownership_transferred", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("new_owner", Hex(&evt.new_owner).to_string())
            .set("previous_owner", Hex(&evt.previous_owner).to_string());
    });
    events.pauseds.into_iter().for_each(|evt| {
        tables
            .create_row("paused", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("account", Hex(&evt.account).to_string());
    });
    events.protocol_fee_sets.into_iter().for_each(|evt| {
        tables
            .create_row("protocol_fee_set", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("new_fee", evt.new_fee)
            .set("old_fee", evt.old_fee);
    });
    events.submitted_bids.into_iter().for_each(|evt| {
        tables
            .create_row("submitted_bid", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("bid_id", BigDecimal::from_str(&evt.bid_id).unwrap())
            .set("borrower", Hex(&evt.borrower).to_string())
            .set("metadata_uri", Hex(&evt.metadata_uri).to_string())
            .set("receiver", Hex(&evt.receiver).to_string());
    });
    events.trusted_market_forwarder_sets.into_iter().for_each(|evt| {
        tables
            .create_row("trusted_market_forwarder_set", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("forwarder", Hex(&evt.forwarder).to_string())
            .set("market_id", BigDecimal::from_str(&evt.market_id).unwrap())
            .set("sender", Hex(&evt.sender).to_string());
    });
    events.unpauseds.into_iter().for_each(|evt| {
        tables
            .create_row("unpaused", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("account", Hex(&evt.account).to_string());
    });
    events.upgradeds.into_iter().for_each(|evt| {
        tables
            .create_row("upgraded", format!("{}-{}", evt.evt_tx_hash, evt.evt_index))
            .set("evt_tx_hash", evt.evt_tx_hash)
            .set("evt_index", evt.evt_index)
            .set("evt_block_time", evt.evt_block_time.unwrap())
            .set("evt_block_number", evt.evt_block_number)
            .set("implementation", Hex(&evt.implementation).to_string());
    });

    Ok(tables.to_entity_changes())
}
