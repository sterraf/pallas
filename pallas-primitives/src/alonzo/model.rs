//! Ledger primitives and cbor codec for the Alonzo era
//!
//! Handcrafted, idiomatic rust artifacts based on based on the [Alonzo CDDL](https://github.com/input-output-hk/cardano-ledger/blob/master/eras/alonzo/test-suite/cddl-files/alonzo.cddl) file in IOHK repo.

use serde::{Deserialize, Serialize};

use pallas_codec::minicbor::{self, Decode, Encode};

pub use pallas_codec::codec_by_datatype;

pub use crate::{
    plutus_data::*, AddrKeyhash, AssetName, Bytes, Coin, CostModel, DatumHash, DnsName, Epoch,
    ExUnitPrices, ExUnits, GenesisDelegateHash, Genesishash, Hash, IPv4, IPv6, Int, KeepRaw,
    KeyValuePairs, MaybeIndefArray, Metadata, Metadatum, MetadatumLabel, NetworkId, Nonce,
    NonceVariant, Nullable, PlutusScript, PolicyId, PoolKeyhash, PoolMetadata, PoolMetadataHash,
    Port, PositiveInterval, ProtocolVersion, RationalNumber, Relay, RewardAccount, ScriptHash,
    StakeCredential, TransactionIndex, TransactionInput, UnitInterval, VrfCert, VrfKeyhash,
};

#[derive(Serialize, Deserialize, Encode, Decode, Debug, PartialEq, Eq, Clone)]
pub struct HeaderBody {
    #[n(0)]
    pub block_number: u64,

    #[n(1)]
    pub slot: u64,

    #[n(2)]
    pub prev_hash: Option<Hash<32>>,

    #[n(3)]
    pub issuer_vkey: Bytes,

    #[n(4)]
    pub vrf_vkey: Bytes,

    #[n(5)]
    pub nonce_vrf: VrfCert,

    #[n(6)]
    pub leader_vrf: VrfCert,

    #[n(7)]
    pub block_body_size: u64,

    #[n(8)]
    pub block_body_hash: Hash<32>,

    #[n(9)]
    pub operational_cert_hot_vkey: Bytes,

    #[n(10)]
    pub operational_cert_sequence_number: u64,

    #[n(11)]
    pub operational_cert_kes_period: u64,

    #[n(12)]
    pub operational_cert_sigma: Bytes,

    #[n(13)]
    pub protocol_major: u64,

    #[n(14)]
    pub protocol_minor: u64,
}

pub type MintedHeaderBody<'a> = KeepRaw<'a, HeaderBody>;

#[derive(Serialize, Deserialize, Encode, Decode, Debug, PartialEq, Eq, Clone)]
pub struct PseudoHeader<T1> {
    #[n(0)]
    pub header_body: T1,

    #[n(1)]
    pub body_signature: Bytes,
}

pub type Header = PseudoHeader<HeaderBody>;

pub type MintedHeader<'a> = KeepRaw<'a, PseudoHeader<MintedHeaderBody<'a>>>;

impl<'a> From<MintedHeader<'a>> for Header {
    fn from(x: MintedHeader<'a>) -> Self {
        let x = x.unwrap();
        Self {
            header_body: x.header_body.into(),
            body_signature: x.body_signature,
        }
    }
}

impl<'a> From<MintedHeaderBody<'a>> for HeaderBody {
    fn from(x: MintedHeaderBody<'a>) -> Self {
        x.unwrap()
    }
}

pub type Multiasset<A> = KeyValuePairs<PolicyId, KeyValuePairs<AssetName, A>>;

pub type Mint = Multiasset<i64>;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum Value {
    Coin(Coin),
    Multiasset(Coin, Multiasset<Coin>),
}

codec_by_datatype! {
    Value,
    U8 | U16 | U32 | U64 => Coin,
    (coin | U8 | U16 | U32 | U64, multi | Map | MapIndef => Multiasset)
}

#[derive(Serialize, Deserialize, Encode, Decode, Debug, PartialEq, Eq, Clone)]
pub struct TransactionOutput {
    #[n(0)]
    pub address: Bytes,

    #[n(1)]
    pub amount: Value,

    #[n(2)]
    pub datum_hash: Option<DatumHash>,
}

/* move_instantaneous_reward = [ 0 / 1, { * stake_credential => delta_coin } / coin ]
; The first field determines where the funds are drawn from.
; 0 denotes the reserves, 1 denotes the treasury.
; If the second field is a map, funds are moved to stake credentials,
; otherwise the funds are given to the other accounting pot.
 */

#[derive(Encode, Decode, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[cbor(index_only)]
pub enum InstantaneousRewardSource {
    #[n(0)]
    Reserves,
    #[n(1)]
    Treasury,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum InstantaneousRewardTarget {
    StakeCredentials(KeyValuePairs<StakeCredential, i64>),
    OtherAccountingPot(Coin),
}

codec_by_datatype! {
    InstantaneousRewardTarget,
    Map | MapIndef => StakeCredentials,
    U8 | U16 | U32 | U64 | I8 | I16 | I32 | I64 | Int => OtherAccountingPot,
    ()
}

#[derive(Serialize, Deserialize, Encode, Decode, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[cbor()]
pub struct MoveInstantaneousReward {
    #[n(0)]
    pub source: InstantaneousRewardSource,

    #[n(1)]
    pub target: InstantaneousRewardTarget,
}

pub type Withdrawals = KeyValuePairs<RewardAccount, Coin>;

pub type RequiredSigners = Vec<AddrKeyhash>;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Encode, Decode)]
#[cbor(flat)]
pub enum Certificate {
    #[n(0)]
    StakeRegistration(#[n(0)] StakeCredential),
    #[n(1)]
    StakeDeregistration(#[n(0)] StakeCredential),
    #[n(2)]
    StakeDelegation(#[n(0)] StakeCredential, #[n(1)] PoolKeyhash),
    #[n(3)]
    PoolRegistration {
        #[n(0)]
        operator: PoolKeyhash,
        #[n(1)]
        vrf_keyhash: VrfKeyhash,
        #[n(2)]
        pledge: Coin,
        #[n(3)]
        cost: Coin,
        #[n(4)]
        margin: UnitInterval,
        #[n(5)]
        reward_account: RewardAccount,
        #[n(6)]
        pool_owners: Vec<AddrKeyhash>,
        #[n(7)]
        relays: Vec<Relay>,
        #[n(8)]
        pool_metadata: Nullable<PoolMetadata>,
    },
    #[n(4)]
    PoolRetirement(#[n(0)] PoolKeyhash, #[n(1)] Epoch),
    #[n(5)]
    GenesisKeyDelegation(
        #[n(0)] Genesishash,
        #[n(1)] GenesisDelegateHash,
        #[n(2)] VrfKeyhash,
    ),
    #[n(6)]
    MoveInstantaneousRewardsCert(#[n(0)] MoveInstantaneousReward),
}

#[derive(Serialize, Deserialize, Encode, Decode, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[cbor(index_only)]
pub enum Language {
    #[n(0)]
    PlutusV1,
}

#[deprecated(since = "0.31.0", note = "use `CostModels` instead")]
pub type CostMdls = CostModels;

pub type CostModels = KeyValuePairs<Language, CostModel>;

#[derive(Serialize, Deserialize, Encode, Decode, Debug, PartialEq, Eq, Clone)]
#[cbor(map)]
pub struct ProtocolParamUpdate {
    #[n(0)]
    pub minfee_a: Option<u32>,
    #[n(1)]
    pub minfee_b: Option<u32>,
    #[n(2)]
    pub max_block_body_size: Option<u32>,
    #[n(3)]
    pub max_transaction_size: Option<u32>,
    #[n(4)]
    pub max_block_header_size: Option<u32>,
    #[n(5)]
    pub key_deposit: Option<Coin>,
    #[n(6)]
    pub pool_deposit: Option<Coin>,
    #[n(7)]
    pub maximum_epoch: Option<Epoch>,
    #[n(8)]
    pub desired_number_of_stake_pools: Option<u32>,
    #[n(9)]
    pub pool_pledge_influence: Option<RationalNumber>,
    #[n(10)]
    pub expansion_rate: Option<UnitInterval>,
    #[n(11)]
    pub treasury_growth_rate: Option<UnitInterval>,
    #[n(12)]
    pub decentralization_constant: Option<UnitInterval>,
    #[n(13)]
    pub extra_entropy: Option<Nonce>,
    #[n(14)]
    pub protocol_version: Option<ProtocolVersion>,
    #[n(16)]
    pub min_pool_cost: Option<Coin>,
    #[n(17)]
    pub ada_per_utxo_byte: Option<Coin>,
    #[n(18)]
    pub cost_models_for_script_languages: Option<CostModels>,
    #[n(19)]
    pub execution_costs: Option<ExUnitPrices>,
    #[n(20)]
    pub max_tx_ex_units: Option<ExUnits>,
    #[n(21)]
    pub max_block_ex_units: Option<ExUnits>,
    #[n(22)]
    pub max_value_size: Option<u32>,
    #[n(23)]
    pub collateral_percentage: Option<u32>,
    #[n(24)]
    pub max_collateral_inputs: Option<u32>,
}

#[derive(Serialize, Deserialize, Encode, Decode, Debug, PartialEq, Eq, Clone)]
pub struct Update {
    #[n(0)]
    pub proposed_protocol_parameter_updates: KeyValuePairs<Genesishash, ProtocolParamUpdate>,

    #[n(1)]
    pub epoch: Epoch,
}

// Can't derive encode for TransactionBody because it seems to require a very
// particular order for each key in the map
#[derive(Serialize, Deserialize, Encode, Decode, Debug, PartialEq, Eq, Clone)]
#[cbor(map)]
pub struct TransactionBody {
    #[n(0)]
    pub inputs: Vec<TransactionInput>,

    #[n(1)]
    pub outputs: Vec<TransactionOutput>,

    #[n(2)]
    pub fee: u64,

    #[n(3)]
    pub ttl: Option<u64>,

    #[n(4)]
    pub certificates: Option<Vec<Certificate>>,

    #[n(5)]
    pub withdrawals: Option<Withdrawals>,

    #[n(6)]
    pub update: Option<Update>,

    #[n(7)]
    pub auxiliary_data_hash: Option<Bytes>,

    #[n(8)]
    pub validity_interval_start: Option<u64>,

    #[n(9)]
    pub mint: Option<Multiasset<i64>>,

    #[n(11)]
    pub script_data_hash: Option<Hash<32>>,

    #[n(13)]
    pub collateral: Option<Vec<TransactionInput>>,

    #[n(14)]
    pub required_signers: Option<RequiredSigners>,

    #[n(15)]
    pub network_id: Option<NetworkId>,
}

#[derive(Serialize, Deserialize, Encode, Decode, Debug, PartialEq, Eq, Clone)]
pub struct VKeyWitness {
    #[n(0)]
    pub vkey: Bytes,

    #[n(1)]
    pub signature: Bytes,
}

#[derive(Encode, Decode, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[cbor(flat)]
pub enum NativeScript {
    #[n(0)]
    ScriptPubkey(#[n(0)] AddrKeyhash),
    #[n(1)]
    ScriptAll(#[n(0)] Vec<NativeScript>),
    #[n(2)]
    ScriptAny(#[n(0)] Vec<NativeScript>),
    #[n(3)]
    ScriptNOfK(#[n(0)] u32, #[n(1)] Vec<NativeScript>),
    #[n(4)]
    InvalidBefore(#[n(0)] u64),
    #[n(5)]
    InvalidHereafter(#[n(0)] u64),
}

#[derive(Serialize, Deserialize, Encode, Decode, Debug, PartialEq, Eq, Clone, Copy)]
#[cbor(index_only)]
pub enum RedeemerTag {
    #[n(0)]
    Spend,
    #[n(1)]
    Mint,
    #[n(2)]
    Cert,
    #[n(3)]
    Reward,
}

#[derive(Serialize, Deserialize, Encode, Decode, Debug, PartialEq, Eq, Clone)]
pub struct Redeemer {
    #[n(0)]
    pub tag: RedeemerTag,

    #[n(1)]
    pub index: u32,

    #[n(2)]
    pub data: PlutusData,

    #[n(3)]
    pub ex_units: ExUnits,
}

#[derive(Serialize, Deserialize, Encode, Decode, Debug, PartialEq, Eq, Clone, Copy)]
pub struct RedeemerPointer {
    #[n(0)]
    pub tag: RedeemerTag,

    #[n(1)]
    pub index: u32,
}

/* bootstrap_witness =
[ public_key : $vkey
, signature  : $signature
, chain_code : bytes .size 32
, attributes : bytes
] */

#[derive(Serialize, Deserialize, Encode, Decode, Debug, PartialEq, Eq, Clone)]
pub struct BootstrapWitness {
    #[n(0)]
    pub public_key: Bytes,

    #[n(1)]
    pub signature: Bytes,

    #[n(2)]
    pub chain_code: Bytes,

    #[n(3)]
    pub attributes: Bytes,
}

#[derive(Serialize, Deserialize, Encode, Decode, Debug, PartialEq, Clone)]
#[cbor(map)]
pub struct WitnessSet {
    #[n(0)]
    pub vkeywitness: Option<Vec<VKeyWitness>>,

    #[n(1)]
    pub native_script: Option<Vec<NativeScript>>,

    #[n(2)]
    pub bootstrap_witness: Option<Vec<BootstrapWitness>>,

    #[n(3)]
    pub plutus_script: Option<Vec<PlutusScript<1>>>,

    #[n(4)]
    pub plutus_data: Option<Vec<PlutusData>>,

    #[n(5)]
    pub redeemer: Option<Vec<Redeemer>>,
}

#[derive(Encode, Decode, Debug, PartialEq, Clone)]
#[cbor(map)]
pub struct MintedWitnessSet<'b> {
    #[n(0)]
    pub vkeywitness: Option<Vec<VKeyWitness>>,

    #[n(1)]
    pub native_script: Option<Vec<KeepRaw<'b, NativeScript>>>,

    #[n(2)]
    pub bootstrap_witness: Option<Vec<BootstrapWitness>>,

    #[n(3)]
    pub plutus_script: Option<Vec<PlutusScript<1>>>,

    #[b(4)]
    pub plutus_data: Option<Vec<KeepRaw<'b, PlutusData>>>,

    #[n(5)]
    pub redeemer: Option<Vec<Redeemer>>,
}

impl<'b> From<MintedWitnessSet<'b>> for WitnessSet {
    #[allow(deprecated)]
    fn from(x: MintedWitnessSet<'b>) -> Self {
        WitnessSet {
            vkeywitness: x.vkeywitness,
            native_script: x
                .native_script
                .map(|x| x.into_iter().map(|x| x.unwrap()).collect()),
            bootstrap_witness: x.bootstrap_witness,
            plutus_script: x.plutus_script,
            plutus_data: x
                .plutus_data
                .map(|x| x.into_iter().map(|x| x.unwrap()).collect()),
            redeemer: x.redeemer,
        }
    }
}

#[derive(Serialize, Deserialize, Encode, Decode, Debug, PartialEq, Clone)]
#[cbor(map, tag(259))]
pub struct PostAlonzoAuxiliaryData {
    #[n(0)]
    pub metadata: Option<Metadata>,

    #[n(1)]
    pub native_scripts: Option<Vec<NativeScript>>,

    #[n(2)]
    pub plutus_scripts: Option<Vec<PlutusScript<1>>>,
}

#[derive(Serialize, Deserialize, Encode, Decode, Debug, PartialEq, Clone)]
pub struct ShelleyMaAuxiliaryData {
    #[n(0)]
    pub transaction_metadata: Metadata,

    #[n(1)]
    pub auxiliary_scripts: Option<Vec<NativeScript>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum AuxiliaryData {
    Shelley(Metadata),
    ShelleyMa(ShelleyMaAuxiliaryData),
    PostAlonzo(PostAlonzoAuxiliaryData),
}

codec_by_datatype! {
    AuxiliaryData,
    Map | MapIndef => Shelley,
    Array | ArrayIndef => ShelleyMa,
    Tag => PostAlonzo,
    ()
}

#[derive(Serialize, Deserialize, Encode, Decode, Debug, PartialEq, Clone)]
pub struct Block {
    #[n(0)]
    pub header: Header,

    #[b(1)]
    pub transaction_bodies: Vec<TransactionBody>,

    #[n(2)]
    pub transaction_witness_sets: Vec<WitnessSet>,

    #[n(3)]
    pub auxiliary_data_set: KeyValuePairs<TransactionIndex, AuxiliaryData>,

    #[n(4)]
    pub invalid_transactions: Option<Vec<TransactionIndex>>,
}

/// A memory representation of an already minted block
///
/// This structure is analogous to [Block], but it allows to retrieve the
/// original CBOR bytes for each structure that might require hashing. In this
/// way, we make sure that the resulting hash matches what exists on-chain.
#[derive(Encode, Decode, Debug, PartialEq, Clone)]
pub struct MintedBlock<'b> {
    #[n(0)]
    pub header: KeepRaw<'b, MintedHeader<'b>>,

    #[b(1)]
    pub transaction_bodies: MaybeIndefArray<KeepRaw<'b, TransactionBody>>,

    #[n(2)]
    pub transaction_witness_sets: MaybeIndefArray<KeepRaw<'b, MintedWitnessSet<'b>>>,

    #[n(3)]
    pub auxiliary_data_set: KeyValuePairs<TransactionIndex, KeepRaw<'b, AuxiliaryData>>,

    #[n(4)]
    pub invalid_transactions: Option<MaybeIndefArray<TransactionIndex>>,
}

impl<'b> From<MintedBlock<'b>> for Block {
    fn from(x: MintedBlock<'b>) -> Self {
        Block {
            header: x.header.unwrap().into(),
            transaction_bodies: x
                .transaction_bodies
                .to_vec()
                .into_iter()
                .map(|x| x.unwrap())
                .collect(),
            transaction_witness_sets: x
                .transaction_witness_sets
                .to_vec()
                .into_iter()
                .map(|x| x.unwrap())
                .map(WitnessSet::from)
                .collect(),
            auxiliary_data_set: x
                .auxiliary_data_set
                .to_vec()
                .into_iter()
                .map(|(k, v)| (k, v.unwrap()))
                .collect::<Vec<_>>()
                .into(),
            invalid_transactions: x.invalid_transactions.map(|x| x.into()),
        }
    }
}

#[derive(Serialize, Deserialize, Encode, Decode, Debug)]
pub struct Tx {
    #[n(0)]
    pub transaction_body: TransactionBody,

    #[n(1)]
    pub transaction_witness_set: WitnessSet,

    #[n(2)]
    pub success: bool,

    #[n(3)]
    pub auxiliary_data: Nullable<AuxiliaryData>,
}

#[derive(Encode, Decode, Debug, Clone)]
pub struct MintedTx<'b> {
    #[b(0)]
    pub transaction_body: KeepRaw<'b, TransactionBody>,

    #[n(1)]
    pub transaction_witness_set: KeepRaw<'b, MintedWitnessSet<'b>>,

    #[n(2)]
    pub success: bool,

    #[n(3)]
    pub auxiliary_data: Nullable<KeepRaw<'b, AuxiliaryData>>,
}

#[cfg(test)]
mod tests {
    use pallas_codec::minicbor::{self, to_vec};

    use crate::{alonzo::PlutusData, Fragment};

    use super::{Header, MintedBlock};

    type BlockWrapper<'b> = (u16, MintedBlock<'b>);

    #[test]
    fn block_isomorphic_decoding_encoding() {
        let test_blocks = vec![
            include_str!("../../../test_data/alonzo1.block"),
            include_str!("../../../test_data/alonzo2.block"),
            include_str!("../../../test_data/alonzo3.block"),
            include_str!("../../../test_data/alonzo4.block"),
            include_str!("../../../test_data/alonzo5.block"),
            include_str!("../../../test_data/alonzo6.block"),
            include_str!("../../../test_data/alonzo7.block"),
            include_str!("../../../test_data/alonzo8.block"),
            include_str!("../../../test_data/alonzo9.block"),
            // old block without invalid_transactions fields
            include_str!("../../../test_data/alonzo10.block"),
            // peculiar block with protocol update params
            include_str!("../../../test_data/alonzo11.block"),
            // peculiar block with decoding issue
            // https://github.com/txpipe/oura/issues/37
            include_str!("../../../test_data/alonzo12.block"),
            // peculiar block with protocol update params, including nonce
            include_str!("../../../test_data/alonzo13.block"),
            // peculiar block with overflow crash
            // https://github.com/txpipe/oura/issues/113
            include_str!("../../../test_data/alonzo14.block"),
            // peculiar block with many move-instantaneous-rewards certs
            include_str!("../../../test_data/alonzo15.block"),
            // peculiar block with protocol update values
            include_str!("../../../test_data/alonzo16.block"),
            // peculiar block with missing nonce hash
            include_str!("../../../test_data/alonzo17.block"),
            // peculiar block with strange AuxiliaryData variant
            include_str!("../../../test_data/alonzo18.block"),
            // peculiar block with strange AuxiliaryData variant
            include_str!("../../../test_data/alonzo18.block"),
            // peculiar block with nevative i64 overflow
            include_str!("../../../test_data/alonzo19.block"),
            // peculiar block with very BigInt in plutus code
            include_str!("../../../test_data/alonzo20.block"),
            // peculiar block with bad tx hash
            include_str!("../../../test_data/alonzo21.block"),
            // peculiar block with bad tx hash
            include_str!("../../../test_data/alonzo22.block"),
            // peculiar block with indef byte array in plutus data
            include_str!("../../../test_data/alonzo23.block"),
            // peculiar block with invalid address (pointer overflow)
            include_str!("../../../test_data/alonzo27.block"),
        ];

        for (idx, block_str) in test_blocks.iter().enumerate() {
            println!("decoding test block {}", idx + 1);
            let bytes = hex::decode(block_str).unwrap_or_else(|_| panic!("bad block file {idx}"));

            let block: BlockWrapper = minicbor::decode(&bytes[..])
                .unwrap_or_else(|_| panic!("error decoding cbor for file {idx}"));

            let bytes2 = to_vec(block)
                .unwrap_or_else(|_| panic!("error encoding block cbor for file {idx}"));

            assert!(bytes.eq(&bytes2), "re-encoded bytes didn't match original");
        }
    }

    #[test]
    fn header_isomorphic_decoding_encoding() {
        let test_headers = [
            // peculiar alonzo header used as origin for a vasil devnet
            include_str!("../../../test_data/alonzo26.header"),
        ];

        for (idx, header_str) in test_headers.iter().enumerate() {
            println!("decoding test header {}", idx + 1);
            let bytes = hex::decode(header_str).unwrap_or_else(|_| panic!("bad header file {idx}"));

            let header: Header = minicbor::decode(&bytes[..])
                .unwrap_or_else(|_| panic!("error decoding cbor for file {idx}"));

            let bytes2 = to_vec(header)
                .unwrap_or_else(|_| panic!("error encoding header cbor for file {idx}"));

            assert!(bytes.eq(&bytes2), "re-encoded bytes didn't match original");
        }
    }

    #[test]
    fn plutus_data_isomorphic_decoding_encoding() {
        let datas = [
            // unit = Constr 0 []
            "d87980",
            // pltmap = Map [(I 1, unit), (I 2, pltlist)]
            "a201d87980029f000102ff",
            // pltlist = List [I 0, I 1, I 2]
            "9f000102ff",
            // Constr 5 [pltmap, Constr 5 [Map [(pltmap, toData True), (pltlist, pltmap), (List [], List [I 1])], unit, toData (0, 1)]]
            "d87e9fa201d87980029f000102ffd87e9fa3a201d87980029f000102ffd87a809f000102ffa201d87980029f000102ff809f01ffd87980d8799f0001ffffff",
            // Constr 5 [List [], List [I 1], Map [], Map [(I 1, unit), (I 2, Constr 2 [I 2])]]
            "d87e9f809f01ffa0a201d8798002d87b9f02ffff",
            // B (B.replicate 32 105)
            "58206969696969696969696969696969696969696969696969696969696969696969",
            // B (B.replicate 67 105)
            "5f58406969696969696969696969696969696969696969696969696969696969696969696969696969696969696969696969696969696969696969696969696969696943696969ff",
            // B B.empty
            "40"
        ];
        for data_hex in datas {
            let data_bytes = hex::decode(data_hex).unwrap();
            let data = PlutusData::decode_fragment(&data_bytes).unwrap();
            assert_eq!(data.encode_fragment().unwrap(), data_bytes);
        }
    }
}
