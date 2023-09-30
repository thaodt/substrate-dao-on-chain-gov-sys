# on-chain DAO pallet
This pallet is used to manage an on-chain government system, with a configurable maximum number of weights.
Participants can join the DAO by paying a certain amount of membership fee, designed to be upgradable. The membership fee is used to buy weights, which are used to vote on proposals.
The DAO is designed to be used as a random number generator, by using the membership weights as a source of randomness.


## Random Generation Process
The economic model rewards participants for their contributions to the random number generation process and charges users or dApps for using the generated random numbers. It is designed to be upgradable.
The random generation process is triggered by submitting masks of their real values. It includes 3 rounds:
- Submit round (10 blocks): members can submit their masks;
- Reveal round (10 blocks): members reveal them after a suitable time delay;
- Random-created round (80 blocks): a random number is generated.

Besides that, I will use dynamic time delays to prevent the DAO from being manipulated by a single member. The time delay is designed to be upgradable. Im going to decrease the time delays as the number of participants increases to ensure faster random number generation.
In case there are no DAO participants, the decentralized bots (Active Bots) that participate in the random number generation process. 

## Fall back mechanism (was not handled here)
This is an alternative source of randomness, we may create a separate feature that retrieves random numbers from an off-chain oracle or a verifiable random function (VRF). In case the DAO fails to generate a random number, the fallback mechanism will be triggered.
