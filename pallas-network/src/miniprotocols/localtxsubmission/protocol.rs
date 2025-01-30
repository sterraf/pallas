use super::Value;
pub use crate::miniprotocols::localstate::queries_v16::{TransactionInput, TransactionOutput};
use pallas_codec::minicbor::{self, Decode, Encode};
use pallas_codec::utils::{AnyCbor, Bytes};
use std::collections::BTreeSet;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum State {
    Idle,
    Busy,
    Done,
}

#[derive(Debug)]
pub enum Message<Tx, Reject> {
    SubmitTx(Tx),
    AcceptTx,
    RejectTx(Reject),
    Done,
}

// The bytes of a transaction with an era number.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct EraTx(pub u16, pub Vec<u8>);

// TODO: Temporary aliases before we decode this
pub type PlutusPurpose = AnyCbor;
pub type ScriptHash = AnyCbor;
pub type Language = AnyCbor;
pub type ContextError = AnyCbor;
pub type FailureDescription = AnyCbor;

/// Tag mismatch description for UTXO validation. It corresponds to
/// [TagMismatchDescription](https://github.com/IntersectMBO/cardano-ledger/blob/d30a7ae828e802e98277c82e278e570955afc273/eras/alonzo/impl/src/Cardano/Ledger/Alonzo/Rules/Utxos.hs#L367)
/// in the Haskell sources.
///
/// Represents the reasons why a tag mismatch occurred during validation.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TagMismatchDescription {
    PassedUnexpectedly,
    // FIXME: Do we want to use `NonEmptySet`? Check other occurrences of `BTreeSet`.
    FailedUnexpectedly(BTreeSet<FailureDescription>),
}

/// Errors that can occur when collecting inputs for phase-2 scripts.
/// It corresponds to [CollectError](https://github.com/IntersectMBO/cardano-ledger/blob/d30a7ae828e802e98277c82e278e570955afc273/eras/alonzo/impl/src/Cardano/Ledger/Alonzo/Plutus/Evaluate.hs#L78-L83).
#[derive(Encode, Decode, Debug, Clone, Eq, PartialEq)]
pub enum CollectError {
    #[n(0)]
    NoRedeemer(#[n(0)] PlutusPurpose),
    #[n(1)]
    NoWitness(#[n(0)] ScriptHash),
    #[n(2)]
    NoCostModel(#[n(0)] Language),
    #[n(3)]
    BadTranslation(#[n(0)] ContextError),
}

#[derive(Encode, Decode, Debug, Clone, Eq, PartialEq)]
#[cbor(transparent)]
pub struct IsValid(#[n(0)] pub bool);

/// Conway Utxo subtransition errors. It corresponds to [ConwayUtxosPredFailure](https://github.com/IntersectMBO/cardano-ledger/blob/d30a7ae828e802e98277c82e278e570955afc273/eras/conway/impl/src/Cardano/Ledger/Conway/Rules/Utxos.hs#L74C6-L74C28)
/// in the Haskell sources. Not to be confused with [UtxoFailure].
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum UtxosFailure {
    ValidationTagMismatch(IsValid, TagMismatchDescription),
    CollectErrors(CollectError),
}

#[derive(Encode, Decode, Debug, Clone, Eq, PartialEq)]
#[cbor(index_only)]
pub enum Network {
    #[n(0)]
    Testnet,
    #[n(1)]
    Mainnet,
}

pub type Slot = u64;

#[derive(Encode, Decode, Debug, Clone, Eq, PartialEq)]
pub struct ValidityInterval {
    #[n(0)]
    invalid_before: SMaybe<u64>,
    #[n(1)]
    invalid_hereafter: SMaybe<u64>,
}

/// Conway Utxo transaction errors. It corresponds to [ConwayUtxoPredFailure](https://github.com/IntersectMBO/cardano-ledger/blob/d30a7ae828e802e98277c82e278e570955afc273/eras/conway/impl/src/Cardano/Ledger/Conway/Rules/Utxo.hs#L78C6-L78C28)
/// in the Haskell sources. Not to be confused with [UtxosFailure].
///
/// It is partially structured; the `Raw` variant collects errors that have not
/// been implemented yet keeping their raw form (to be deprecated).
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum UtxoFailure {
    UtxosFailure(UtxosFailure),
    BadInputsUTxO(BTreeSet<TransactionInput>),
    OutsideValidityIntervalUTxO(ValidityInterval, Slot),
    MaxTxSizeUTxO(u64, u64),
    InputSetEmptyUTxO,
    FeeTooSmallUTxO(u64, u64),
    ValueNotConservedUTxO(Value, Value),
    WrongNetwork(Network, BTreeSet<Bytes>),
    InsufficientCollateral(i64, u64),
    CollateralContainsNonADA(Value),
    TooManyCollateralInputs(u16, u16),
    NoCollateralInputs,
    IncorrectTotalCollateralField(i64, u64),
    BabbageOutputTooSmallUTxO(Vec<(TransactionOutput, u64)>),
    Raw(Vec<u8>),
}

/// An option type that de/encodes equivalently to [`StrictMaybe`](https://github.com/IntersectMBO/cardano-ledger/blob/d30a7ae828e802e98277c82e278e570955afc273/libs/cardano-ledger-binary/src/Cardano/Ledger/Binary/Encoding/Encoder.hs#L326-L329) in the Haskel sources.
///
/// `None` encodes as `[]`, Some(x) as `[encode(x)]`.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SMaybe<T> {
    Some(T),
    None,
}

/// Conway era transaction errors. It corresponds to [ConwayUtxowPredFailure](https://github.com/IntersectMBO/cardano-ledger/blob/d30a7ae828e802e98277c82e278e570955afc273/eras/conway/impl/src/Cardano/Ledger/Conway/Rules/Utxow.hs#L94)
/// in the Haskell sources.
///
/// It is partially structured; the `Raw` variant collects errors that have not
/// been implemented yet keeping their raw form (to be deprecated).
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum UtxowFailure {
    ExtraneousScriptWitnessesUTXOW(Vec<Bytes>),
    UtxoFailure(UtxoFailure),
    MissingTxBodyMetadataHash(Bytes),
    PPViewHashesDontMatch(SMaybe<Bytes>, SMaybe<Bytes>),
    NotAllowedSupplementalDatums(BTreeSet<Bytes>, BTreeSet<Bytes>),
    ExtraRedeemers(PlutusPurpose),
    Raw(Vec<u8>),
}

/// Conway era ledger transaction errors.
/// It is partially structured; the `Raw` variant collects errors that have not
/// been implemented yet keeping their raw form (to be deprecated).
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TxError {
    ConwayUtxowFailure(UtxowFailure),
    Raw(Vec<u8>),
}

// Raw reject reason.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RejectReason {
    EraErrors(u8, Vec<TxError>),
    Plutus(String),
}

impl From<String> for RejectReason {
    fn from(string: String) -> RejectReason {
        RejectReason::Plutus(string)
    }
}
