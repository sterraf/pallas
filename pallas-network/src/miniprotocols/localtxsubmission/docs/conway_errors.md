# ConwayLedgerPredFailure

[At ledger repo]((https://github.com/IntersectMBO/cardano-ledger/blob/d30a7ae828e802e98277c82e278e570955afc273/eras/conway/impl/src/Cardano/Ledger/Conway/Rules/Ledger.hsL138)

## 1 ConwayUtxowFailure

[At ledger repo]((https://github.com/IntersectMBO/cardano-ledger/blob/d30a7ae828e802e98277c82e278e570955afc273/eras/conway/impl/src/Cardano/Ledger/Conway/Rules/Utxow.hsL94)

### 0 UtxoFailure

[At ledger repo]((https://github.com/IntersectMBO/cardano-ledger/blob/d30a7ae828e802e98277c82e278e570955afc273/eras/conway/impl/src/Cardano/Ledger/Conway/Rules/Utxo.hsL78C6-L78C28)

#### 0	UtxosFailure

[At ledger repo]((https://github.com/IntersectMBO/cardano-ledger/blob/d30a7ae828e802e98277c82e278e570955afc273/eras/conway/impl/src/Cardano/Ledger/Conway/Rules/Utxos.hsL74C6-L74C28)

##### 0 ValidationTagMismatch

##### 1 CollectErrors

[At ledger repo]((https://github.com/IntersectMBO/cardano-ledger/blob/d30a7ae828e802e98277c82e278e570955afc273/eras/alonzo/impl/src/Cardano/Ledger/Alonzo/Plutus/Evaluate.hsL79)

###### 0	NoRedeemer

###### 1	NoWitness

###### 2	NoCostModel

###### 3	BadTranslation

[At ledger repo]((https://github.com/IntersectMBO/cardano-ledger/blob/d30a7ae828e802e98277c82e278e570955afc273/eras/conway/impl/src/Cardano/Ledger/Conway/TxInfo.hsL139)

- 0 BabbageContextError

  [At ledger repo]((https://github.com/IntersectMBO/cardano-ledger/blob/d30a7ae828e802e98277c82e278e570955afc273/eras/babbage/impl/src/Cardano/Ledger/Babbage/TxInfo.hsL230)
  - 0 AlonzoContextError

	  [At ledger repo]((https://github.com/IntersectMBO/cardano-ledger/blob/d30a7ae828e802e98277c82e278e570955afc273/eras/alonzo/impl/src/Cardano/Ledger/Alonzo/Plutus/TxInfo.hsL169)

      - 0 TranslationLogicMissingInput

      - 1 TimeTranslationPastHorizon

  - 1	ByronTxOutInContext

  - 2	RedeemerPointerPointsToNothing

  - 3	InlineDatumsNotSupported

  - 4	ReferenceScriptsNotSupported

  - 5	ReferenceInputsNotSupported

- 1	CertificateNotSupported

- 2	PlutusPurposeNotSupported

- 3	CurrentTreasuryFieldNotSupported

- 4	VotingProceduresFieldNotSupported

- 5	ProposalProceduresFieldNotSupported

- 6	TreasuryDonationFieldNotSupported

#### 1	BadInputsUTxO

#### 2	OutsideValidityIntervalUTxO

#### 3	MaxTxSizeUTxO

#### 4	InputSetEmptyUTxO

#### 5	FeeTooSmallUTxO

#### 6	ValueNotConservedUTxO

#### 7	WrongNetwork

#### 8	WrongNetworkWithdrawal

#### 9	OutputTooSmallUTxO

#### 10	OutputBootAddrAttrsTooBig

#### 11	OutputTooBigUTxO

#### 12	InsufficientCollateral

#### 13	ScriptsNotPaidUTxO

#### 14	ExUnitsTooBigUTxO

#### 15	CollateralContainsNonADA

#### 16	WrongNetworkInTxBody

#### 17	OutsideForecast

#### 18	TooManyCollateralInputs

#### 19	NoCollateralInputs

#### 20	IncorrectTotalCollateralField

#### 21	BabbageOutputTooSmallUTxO

#### 22	BabbageNonDisjointRefInputs

### 1 InvalidWitnessesUTXOW

### 2 MissingVKeyWitnessesUTXOW

### 3 MissingScriptWitnessesUTXOW

### 4 ScriptWitnessNotValidatingUTXOW

### 5 MissingTxBodyMetadataHash

### 6 MissingTxMetadata

### 7 ConflictingMetadataHash

### 8 InvalidMetadata

### 8 ExtraneousScriptWitnessesUTXOW

### 10 MissingRedeemers

### 11 MissingRequiredDatums

### 12 NotAllowedSupplementalDatums

### 13 PPViewHashesDontMatch

### 14 UnspendableUTxONoDatumHash

### 15 ExtraRedeemers

### 16 MalformedScriptWitnesses

### 17 MalformedReferenceScripts

## 2 ConwayCertsFailure

[At ledger repo]((https://github.com/IntersectMBO/cardano-ledger/blob/d30a7ae828e802e98277c82e278e570955afc273/eras/conway/impl/src/Cardano/Ledger/Conway/Rules/Certs.hsL115)

### 0	WithdrawalsNotInRewardsCERTS

### 1	CertFailure

[At ledger repo]((https://github.com/IntersectMBO/cardano-ledger/blob/d30a7ae828e802e98277c82e278e570955afc273/eras/conway/impl/src/Cardano/Ledger/Conway/Rules/Cert.hsL103)

#### 1	DelegFailure

[At ledger repo]((https://github.com/IntersectMBO/cardano-ledger/blob/d30a7ae828e802e98277c82e278e570955afc273/eras/conway/impl/src/Cardano/Ledger/Conway/Rules/Deleg.hsL104)

##### 1	IncorrectDepositDELEG

##### 2	StakeKeyRegisteredDELEG

##### 3	StakeKeyNotRegisteredDELEG

##### 4	StakeKeyHasNonZeroRewardAccountBalanceDELEG

##### 5	DelegateeDRepNotRegisteredDELEG

##### 6	DelegateeStakePoolNotRegisteredDELEG

#### 2	PoolFailure

[At ledger repo]((https://github.com/IntersectMBO/cardano-ledger/blob/d30a7ae828e802e98277c82e278e570955afc273/eras/shelley/impl/src/Cardano/Ledger/Shelley/Rules/Pool.hsL94)

##### 0	StakePoolNotRegisteredOnKeyPOOL

##### 1	StakePoolRetirementWrongEpochPOOL

##### 3	StakePoolCostTooLowPOOL

##### 4	WrongNetworkPOOL

##### 5	PoolMedataHashTooBig

#### 3	GovCertFailure

[At ledger repo]((https://github.com/IntersectMBO/cardano-ledger/blob/d30a7ae828e802e98277c82e278e570955afc273/eras/conway/impl/src/Cardano/Ledger/Conway/Rules/GovCert.hsL113C6-L113C30)

##### 0	ConwayDRepAlreadyRegistered

##### 1	ConwayDRepNotRegistered

##### 2	ConwayDRepIncorrectDeposit

##### 3	ConwayCommitteeHasPreviouslyResigned

##### 4	ConwayDRepIncorrectRefund 

##### 5	ConwayCommitteeIsUnknown

## 3 ConwayGovFailure

### 0	GovActionsDoNotExist

### 1	MalformedProposal

### 2	ProposalProcedureNetworkIdMismatch

### 3	TreasuryWithdrawalsNetworkIdMismatch

### 4	ProposalDepositIncorrect

### 5	DisallowedVoters

### 6	ConflictingCommitteeUpdate

### 7	ExpirationEpochTooSmall

### 8	InvalidPrevGovActionId

### 9	VotingOnExpiredGovAction

### 10	ProposalCantFollow

### 11	InvalidPolicyHash

### 12	DisallowedProposalDuringBootstrap

### 13	DisallowedVotesDuringBootstrap

### 14	VotersDoNotExist

### 15	ZeroTreasuryWithdrawals

### 16	ProposalReturnAccountDoesNotExist

### 17	TreasuryWithdrawalReturnAccountsDoNotExist

## 4 ConwayWdrlNotDelegatedToDRep

## 5 ConwayTreasuryValueMismatch

## 6 ConwayTxRefScriptsSizeTooBig

## 7 ConwayMempoolFailure
