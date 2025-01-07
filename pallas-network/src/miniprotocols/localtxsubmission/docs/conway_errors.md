# Conway Tx submission errors

## [ConwayLedgerPredFailure](https://github.com/IntersectMBO/cardano-ledger/blob/d30a7ae828e802e98277c82e278e570955afc273/eras/conway/impl/src/Cardano/Ledger/Conway/Rules/Ledger.hs#L138)

### 1 [ConwayUtxowFailure](https://github.com/IntersectMBO/cardano-ledger/blob/d30a7ae828e802e98277c82e278e570955afc273/eras/conway/impl/src/Cardano/Ledger/Conway/Rules/Utxow.hs#L94)

- #### 0 [UtxoFailure](https://github.com/IntersectMBO/cardano-ledger/blob/d30a7ae828e802e98277c82e278e570955afc273/eras/conway/impl/src/Cardano/Ledger/Conway/Rules/Utxo.hs#L78C6-L78C28)

  - ##### 0	[UtxosFailure](https://github.com/IntersectMBO/cardano-ledger/blob/d30a7ae828e802e98277c82e278e570955afc273/eras/conway/impl/src/Cardano/Ledger/Conway/Rules/Utxos.hs#L74C6-L74C28)

    - ###### 0 ValidationTagMismatch

    - ###### 1 [CollectErrors](https://github.com/IntersectMBO/cardano-ledger/blob/d30a7ae828e802e98277c82e278e570955afc273/eras/alonzo/impl/src/Cardano/Ledger/Alonzo/Plutus/Evaluate.hs#L79)

      - ####### 0	NoRedeemer

      - ####### 1	NoWitness

      - ####### 2	NoCostModel

      - ####### 3	[BadTranslation](https://github.com/IntersectMBO/cardano-ledger/blob/d30a7ae828e802e98277c82e278e570955afc273/eras/conway/impl/src/Cardano/Ledger/Conway/TxInfo.hs#L139)

        - ######## 0	[BabbageContextError](https://github.com/IntersectMBO/cardano-ledger/blob/d30a7ae828e802e98277c82e278e570955afc273/eras/babbage/impl/src/Cardano/Ledger/Babbage/TxInfo.hs#L230)

            - ######## 0	[AlonzoContextError](https://github.com/IntersectMBO/cardano-ledger/blob/d30a7ae828e802e98277c82e278e570955afc273/eras/alonzo/impl/src/Cardano/Ledger/Alonzo/Plutus/TxInfo.hs#L169)

              - ######### 0	TranslationLogicMissingInput

              - ######### 1	TimeTranslationPastHorizon

            - ######## 1	ByronTxOutInContext

            - ######## 2	RedeemerPointerPointsToNothing

            - ######## 3	InlineDatumsNotSupported

            - ######## 4	ReferenceScriptsNotSupported

            - ######## 5	ReferenceInputsNotSupported

        - ######## 1	CertificateNotSupported

        - ######## 2	PlutusPurposeNotSupported

        - ######## 3	CurrentTreasuryFieldNotSupported

        - ######## 4	VotingProceduresFieldNotSupported

        - ######## 5	ProposalProceduresFieldNotSupported

        - ######## 6	TreasuryDonationFieldNotSupported

  - ##### 1	BadInputsUTxO

  - ##### 2	OutsideValidityIntervalUTxO

  - ##### 3	MaxTxSizeUTxO

  - ##### 4	InputSetEmptyUTxO

  - ##### 5	FeeTooSmallUTxO

  - ##### 6	ValueNotConservedUTxO

  - ##### 7	WrongNetwork

  - ##### 8	WrongNetworkWithdrawal

  - ##### 9	OutputTooSmallUTxO

  - ##### 10	OutputBootAddrAttrsTooBig

  - ##### 11	OutputTooBigUTxO

  - ##### 12	InsufficientCollateral

  - ##### 13	ScriptsNotPaidUTxO

  - ##### 14	ExUnitsTooBigUTxO

  - ##### 15	CollateralContainsNonADA

  - ##### 16	WrongNetworkInTxBody

  - ##### 17	OutsideForecast

  - ##### 18	TooManyCollateralInputs

  - ##### 19	NoCollateralInputs

  - ##### 20	IncorrectTotalCollateralField

  - ##### 21	BabbageOutputTooSmallUTxO

  - ##### 22	BabbageNonDisjointRefInputs

- #### 1 InvalidWitnessesUTXOW

- #### 2 MissingVKeyWitnessesUTXOW

- #### 3 MissingScriptWitnessesUTXOW

- #### 4 ScriptWitnessNotValidatingUTXOW

- #### 5 MissingTxBodyMetadataHash

- #### 6 MissingTxMetadata

- #### 7 ConflictingMetadataHash

- #### 8 InvalidMetadata

- #### 8 ExtraneousScriptWitnessesUTXOW

- #### 10 MissingRedeemers

- #### 11 MissingRequiredDatums

- #### 12 NotAllowedSupplementalDatums

- #### 13 PPViewHashesDontMatch

- #### 14 UnspendableUTxONoDatumHash

- #### 15 ExtraRedeemers

- #### 16 MalformedScriptWitnesses

- #### 17 MalformedReferenceScripts

### 2 [ConwayCertsFailure](https://github.com/IntersectMBO/cardano-ledger/blob/d30a7ae828e802e98277c82e278e570955afc273/eras/conway/impl/src/Cardano/Ledger/Conway/Rules/Certs.hs#L115)

- #### 0	WithdrawalsNotInRewardsCERTS

- #### 1	[CertFailure](https://github.com/IntersectMBO/cardano-ledger/blob/d30a7ae828e802e98277c82e278e570955afc273/eras/conway/impl/src/Cardano/Ledger/Conway/Rules/Cert.hs#L103)

  - ##### 1	[DelegFailure](https://github.com/IntersectMBO/cardano-ledger/blob/d30a7ae828e802e98277c82e278e570955afc273/eras/conway/impl/src/Cardano/Ledger/Conway/Rules/Deleg.hs#L104)

    - ###### 1	IncorrectDepositDELEG

    - ###### 2	StakeKeyRegisteredDELEG

    - ###### 3	StakeKeyNotRegisteredDELEG

    - ###### 4	StakeKeyHasNonZeroRewardAccountBalanceDELEG

    - ###### 5	DelegateeDRepNotRegisteredDELEG

    - ###### 6	DelegateeStakePoolNotRegisteredDELEG

  - ##### 2	[PoolFailure](https://github.com/IntersectMBO/cardano-ledger/blob/d30a7ae828e802e98277c82e278e570955afc273/eras/shelley/impl/src/Cardano/Ledger/Shelley/Rules/Pool.hs#L94)

    - ###### 0	StakePoolNotRegisteredOnKeyPOOL

    - ###### 1	StakePoolRetirementWrongEpochPOOL

    - ###### 3	StakePoolCostTooLowPOOL

    - ###### 4	WrongNetworkPOOL

    - ###### 5	PoolMedataHashTooBig

  - ##### 3	[GovCertFailure](https://github.com/IntersectMBO/cardano-ledger/blob/d30a7ae828e802e98277c82e278e570955afc273/eras/conway/impl/src/Cardano/Ledger/Conway/Rules/GovCert.hs#L113C6-L113C30)

    - ###### 0	ConwayDRepAlreadyRegistered

    - ###### 1	ConwayDRepNotRegistered

    - ###### 2	ConwayDRepIncorrectDeposit

    - ###### 3	ConwayCommitteeHasPreviouslyResigned

    - ###### 4	ConwayDRepIncorrectRefund 

    - ###### 5	ConwayCommitteeIsUnknown

### 3 ConwayGovFailure

- #### 0	GovActionsDoNotExist

- #### 1	MalformedProposal

- #### 2	ProposalProcedureNetworkIdMismatch

- #### 3	TreasuryWithdrawalsNetworkIdMismatch

- #### 4	ProposalDepositIncorrect

- #### 5	DisallowedVoters

- #### 6	ConflictingCommitteeUpdate

- #### 7	ExpirationEpochTooSmall

- #### 8	InvalidPrevGovActionId

- #### 9	VotingOnExpiredGovAction

- #### 10	ProposalCantFollow

- #### 11	InvalidPolicyHash

- #### 12	DisallowedProposalDuringBootstrap

- #### 13	DisallowedVotesDuringBootstrap

- #### 14	VotersDoNotExist

- #### 15	ZeroTreasuryWithdrawals

- #### 16	ProposalReturnAccountDoesNotExist

- #### 17	TreasuryWithdrawalReturnAccountsDoNotExist

### 4 ConwayWdrlNotDelegatedToDRep

### 5 ConwayTreasuryValueMismatch

### 6 ConwayTxRefScriptsSizeTooBig

### 7 ConwayMempoolFailure
