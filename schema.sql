CREATE TABLE IF NOT EXISTS accepted_bid (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "bid_id" DECIMAL,
    "lender" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS admin_changed (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "new_admin" VARCHAR(40),
    "previous_admin" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS beacon_upgraded (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "beacon" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS cancelled_bid (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "bid_id" DECIMAL,
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS fee_paid (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "amount" DECIMAL,
    "bid_id" DECIMAL,
    "fee_type" TEXT,
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS initialized (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "version" INT,
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS loan_liquidated (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "bid_id" DECIMAL,
    "liquidator" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS loan_repaid (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "bid_id" DECIMAL,
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS loan_repayment (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "bid_id" DECIMAL,
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS market_forwarder_approved (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "forwarder" VARCHAR(40),
    "market_id" DECIMAL,
    "sender" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS market_forwarder_renounced (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "forwarder" VARCHAR(40),
    "market_id" DECIMAL,
    "sender" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS market_owner_cancelled_bid (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "bid_id" DECIMAL,
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS ownership_transferred (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "new_owner" VARCHAR(40),
    "previous_owner" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS paused (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "account" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS protocol_fee_set (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "new_fee" INT,
    "old_fee" INT,
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS submitted_bid (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "bid_id" DECIMAL,
    "borrower" VARCHAR(40),
    "metadata_uri" TEXT,
    "receiver" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS trusted_market_forwarder_set (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "forwarder" VARCHAR(40),
    "market_id" DECIMAL,
    "sender" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS unpaused (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "account" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);
CREATE TABLE IF NOT EXISTS upgraded (
    "evt_tx_hash" VARCHAR(64),
    "evt_index" INT,
    "evt_block_time" TIMESTAMP,
    "evt_block_number" DECIMAL,
    "implementation" VARCHAR(40),
    PRIMARY KEY(evt_tx_hash,evt_index)
);

