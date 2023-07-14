# sovereign-labs-rollup
<!-- DESCRIBE your rollup. -->
sovereign-labs-rollup is a ZK succinct-proof Rollup built on the sovereign-labs SDK. If in develop we find RISC0 portability to be intractable or testing we find RISC0 proof generation to be too costly, we may convert to Rollkit for an optimistic rollup.

## Technology
<!-- INCLUDE a UML diagram for key systems in your chosen technology and systems you will implement. Use whatever diagramming tool you like. Recommendation: https://www.lucidchart.com/pages/landing?utm_source=google&utm_medium=cpc&utm_campaign=_chart_en_us_mixed_search_brand_exact_&km_CPC_CampaignId=1457964857&km_CPC_AdGroupID=57044764032&km_CPC_Keyword=lucidchart&km_CPC_MatchType=e&km_CPC_ExtensionID=&km_CPC_Network=g&km_CPC_AdPosition=&km_CPC_Creative=442433231228&km_CPC_TargetID=aud-552508845082:kwd-33511936169&km_CPC_Country=9031914&km_CPC_Device=c&km_CPC_placement=&km_CPC_target=&gclid=CjwKCAjwwb6lBhBJEiwAbuVUSu8uD6Szuco3LffO7NHSA1hLah1873is1ZSpIaw3VPZnU--xD7NtqBoCiwIQAvD_BwE -->
![Sovereign Labs Rollup](./Sovereign%20Labs%20Rollup.png)

<!-- DESCRIBE at a high-level your rollup technology. -->

### How does it work?
<!-- DESCRIBE how your chosen rollup technology works in detail. -->
The sovereign labs SDK is a tool for building rollups. It supports both ZK succinct rollups based on the RISC0 machine and optimistic rollups, based on the 

### How will the VM be integrated?
<!-- DESCRIBE how a Move VM (MoveVM or AptosVM) or other will be integrated into your chosen technology. -->
The VM will be a program compiled for the RISC0 VM. We will first attempt to build this using Aptos VM. The state transition of the VM will rely on the Aptos executor. 

For the proof of concept, whether the state of the Aptos DB will be represented in the Celestia DA or settlement will occur via Aptos remains outstanding. The complexity of syscalls from RISC0 versus the decomposability of the Aptos VM is the primary tradeoff.

### How will the rollup be composed?
<!-- DESCRIBE which technologies are responsible for which rollup functionality. -->
RISC0 is responsible for execution. The sovereign SDK is responsible for submittig proofs to Celestia. We should be able to tightly integrate Aptos VM with our Celestia data availability layer, in which case the RISC0's proofs server the function of settlement. If some other kind of settlement is needed, our first instinct will be to use Aptos VM.

#### Settlement and Fraud Proofs
<!-- DESCRIBE how settlement and fraud proving will be handled. -->
Settlement is dependent upon how tightly coupled the data availability layer and the Aptos VM are. In the case in which we are able to effectively port Aptos DB to Celestia, we will be able entirely rely on RISC0 proofs via the Sovereign Labs SDK for settlement. If not, while it will still only be the RISC0 proofs that are submitted to Celestia, we may need to synchronize Aptos DB state in some other manner, such as running the Aptos VM consensus.

#### Data Availability
<!-- DESCRIBE the technologies responsible for data availability.  -->
Celestia is our intended data availability layer. It is the most well-supported DA adapter for the Sovereign Labs SDK.

#### Consensus
<!-- DESCRIBE how consensus is achieved. -->
[Celestia](https://docs.celestia.org/concepts/how-celestia-works/data-availability-layer/) uses Namespaced Merkle Trees in combination with a PoS blockchain to order transactions.

#### Contract Storage
<!-- DESCRIBE how smart contracts are stored. -->
Initially, smart contracts would be stored on Celestia.

#### Execution
<!-- DESCRIBE how smart contracts are executed. -->
Smart contracts are executed by the Aptos VM executor.

### How will smart contrcats be deployed
<!-- DESCRIBE how smart contracts will be deployed. -->
A smart contract will be deployed from `movement` by calling a command similar to `movement move publish --sovereign-zk`. This will then send a transaction for contract installation through the sovereign labs rollup nodes to the Celestia data availability layer.

#### Layer 4
<!-- ...DESCRIBE with respect to settlement/rollup layer. -->
The settlement/rollup layer merely sends the compiled contract through to Celestia.

#### Layer 3
<!-- ...DESCRIBE with respect to the data availability layer.-->
Celestia stores the contract in the appropriate MNT.

#### Layer 2
<!-- ...DESCRIBE with respect to the consensus layer. -->
Celestia manages contract consensus.

## Plan
<!-- DESCRIBE your plan to implement this rollup at a high-level. -->
The implementation of the rollup will begin with a proof of concept which sees some form of Aptos execution. The MVP should ideally improve open the proof of concept to reduce the cost of settlement communication. The MLP should feature an ideal strategy for Aptos DB availability.

### Proof of Concept
<!-- DESCRIBE your plan to implement a proof of concept. -->
Proof of concept implements a Sovereign Labs rollup with some form of Aptos execution. This execution may be settled via Aptos consensus. Aptos itself does not necessarily have to run within RISC0, if such provides an easier path to implementation.

At this stage, we only require sending the proofs and a transaction ids to Celestia. We do not require any synchronization of the Aptos DB with the Celestia DA.

#### Features
<!-- DESCRIBE the features your proof of concept will have. -->
- Aptos Execution
- Proofs of Aptos Execution
- Celestia proofs and counters.

#### Milestones
<!-- DESCRIBE a list of milestones for your proof of concept. Assign timelines to these milestones. Proof of concept development can include milestones that are simply related to getting your chosen technology to work in the appropriate manner. -->
1. Compile Hello World rollup with Aptos VM ping. (Demonstrates successful mereger of source.) [Day 2]
2. Run Aptos executor upon rollup call. [Day 3]
3. Submit proofs and counter to Celestia. [Day 4]

### MVP
<!-- DESCRIBE your plan to implement an MVP. -->
MVP implements a Sovereign Labs rollup with proven Aptos execution. This execution may still be settled via Aptos consensus, but would preferably be able to rely directly upon Celestia DA and settle with ZK proofs accordingly.

At this stage, we require sending the proofs and transactions to Celestia.

#### Features
<!-- DESCRIBE the features your MVP will have. -->
- Aptos Execution
- Proofs of Aptos Execution
- Celestia proofs and Aptos DB state.
- OR: some other solution for synchronizing Aptos DB state.

#### Milestones
<!-- DESCRIBE a list of milestones for your MVP. Assign timelines to these milestones. -->
1. Proven Aptos VM execution. [Day 2]
2. DA integration. [Day 4] --OR-- Aptos settlement [Day 3]

### MLP
<!-- DESCRIBE your plan to implement an MLP. (Minimum Lovable Product) -->

#### Features
<!-- DESCRIBE the features your MLP will have. -->

#### Milestones
<!-- DESCRIBE a list of milestones for your MLP. Assign timelines to these milestones. -->
