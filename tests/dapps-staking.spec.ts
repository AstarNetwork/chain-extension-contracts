// import { expect, use } from 'chai';
// import chaiAsPromised from 'chai-as-promised';
// import BN from 'bn.js';
// import staking_constructor from '../types/constructors/staking_example';
// import staking_contract from '../types/contracts/staking_example';
// import { ApiPromise, WsProvider, Keyring } from '@polkadot/api';
// import { KeyringPair } from '@polkadot/keyring/types';
// import { AccountLedger, EraInfo, EraStakingPointsIndividualClaim, GeneralStakerInfo, RewardDestination } from "./types";
// import { Option } from '@polkadot/types';
// import {buildTx} from "./helper";
// import {WeightV2} from "@polkadot/types/interfaces";
//
// use(chaiAsPromised);
//
// const ONE = new BN(10).pow(new BN(18));
//
// // Create a new instance of contract
// const wsProvider = new WsProvider('ws://127.0.0.1:9944');
// // Create a keyring instance
// const keyring = new Keyring({ type: 'sr25519' });
//
// describe('DAPPS STAKING', () => {
//     let api: ApiPromise;
//     let alice: KeyringPair;
//     let bob: KeyringPair;
//     let stakingConstructor: staking_constructor
//     let stakingContract: staking_contract;
//     let gasRequired: WeightV2;
//
//     beforeEach(async function() {
//         api = await ApiPromise.create({ provider: wsProvider });
//         alice = keyring.addFromUri('//Alice');
//         bob = keyring.addFromUri('//Bob');
//         stakingConstructor = new staking_constructor(api, alice);
//         stakingContract = new staking_contract((await stakingConstructor.new()).address, alice, api);
//         await buildTx(api.registry, api.tx.balances.transfer(stakingContract.address, ONE.muln(10000)), alice)
//     });
//
//     it('should read current era', async () => {
//         const currentEra = await api.query.dappsStaking.currentEra()
//         // @ts-ignore
//         await expect((await stakingContract.query.readCurrentEra()).value.ok).to.equal(currentEra.toNumber())
//     })
//
//     it('should read unbonding period', async () => {
//         const bondingDuration = api.consts.dappsStaking.unbondingPeriod
//         await expect((await stakingContract.query.readUnbondingPeriod()).value.ok.toLocaleString()).to.equal(bondingDuration.toString())
//     })
//
//     it('should read era reward', async () => {
//         // @ts-ignore
//         await buildTx(api.registry, api.tx.sudo.sudo(api.tx.dappsStaking.register(stakingContract.address, { Wasm: stakingContract.address })), alice);
//         await buildTx(api.registry, api.tx.dappsStaking.bondAndStake({ Wasm: stakingContract.address }, ONE.muln(10000)), bob)
//
//         await forceEras(4, alice)
//
//         const currentEra = await api.query.dappsStaking.currentEra()
//         const generalEraInfo = (await api.query.dappsStaking.generalEraInfo<Option<EraInfo>>(currentEra))?.unwrapOrDefault()
//         // @ts-ignore
//         await expect((await stakingContract.query.readEraReward(currentEra)).value.ok.toNumber()).to.equal(generalEraInfo.rewards.dapps.toNumber() + generalEraInfo.rewards.stakers.toNumber())
//     })
//
//     it('should read era staked', async () => {
//         // @ts-ignore
//         await buildTx(api.registry, api.tx.sudo.sudo(api.tx.dappsStaking.register(stakingContract.address, { Wasm: stakingContract.address })), alice);
//         await buildTx(api.registry, api.tx.dappsStaking.bondAndStake({ Wasm: stakingContract.address }, ONE.muln(10000)), bob)
//
//         const currentEra = await api.query.dappsStaking.currentEra()
//         const generalEraInfo = (await api.query.dappsStaking.generalEraInfo<Option<EraInfo>>(currentEra))?.unwrapOrDefault()
//         // @ts-ignore
//         await expect((await stakingContract.query.readEraStaked(currentEra)).value.ok.toString()).to.equal(generalEraInfo.staked.toString())
//     })
//
//     it('should read contract stake', async () => {
//         // @ts-ignore
//         await buildTx(api.registry, api.tx.sudo.sudo(api.tx.dappsStaking.register(stakingContract.address, { Wasm: stakingContract.address })), alice);
//         await buildTx(api.registry, api.tx.dappsStaking.bondAndStake({ Wasm: stakingContract.address }, ONE.muln(10000)), bob)
//
//         const ledger = await api.query.dappsStaking.ledger(bob.address)
//         // @ts-ignore
//         await expect((await stakingContract.query.readStakedAmount(bob.address)).value.ok.toString()).to.equal(ledger.locked.toString())
//     })
//
//     it('should read staked amount on contract', async () => {
//         // @ts-ignore
//         await buildTx(api.registry, api.tx.sudo.sudo(api.tx.dappsStaking.register(stakingContract.address, { Wasm: stakingContract.address })), alice);
//         await buildTx(api.registry, api.tx.dappsStaking.bondAndStake({ Wasm: stakingContract.address }, ONE.muln(10000)), bob)
//
//         const generalStakerInfo = await api.query.dappsStaking.generalStakerInfo<GeneralStakerInfo>(
//             bob.address,
//             {
//                 Wasm: stakingContract.address,
//             }
//         );
//         await expect((await stakingContract.query.readStakedAmountOnContract(bob.address, stakingContract.address)).value.ok.toString())
//             .to.equal(generalStakerInfo.stakes[generalStakerInfo.stakes.length - 1].staked.toString())
//     })
//
//     it('should read contract stake', async () => {
//         // @ts-ignore
//         await buildTx(api.registry, api.tx.sudo.sudo(api.tx.dappsStaking.register(stakingContract.address, { Wasm: stakingContract.address })), alice);
//         await buildTx(api.registry, api.tx.dappsStaking.bondAndStake({ Wasm: stakingContract.address }, ONE.muln(10000)), bob)
//
//         const currentEra = await api.query.dappsStaking.currentEra()
//         const contractStake = (await api.query.dappsStaking.contractEraStake<Option<EraStakingPointsIndividualClaim>>({ Wasm: stakingContract.address }, currentEra))?.unwrapOrDefault();
//
//         await expect((await stakingContract.query.readContractStake(stakingContract.address)).value.ok.toString()).to.equal(contractStake.total.toString())
//     })
//
//     it('should bond and stake on contract', async () => {
//         // register & Bob sends funds on contract to be staked
//         // @ts-ignore
//         await buildTx(api.registry, api.tx.sudo.sudo(api.tx.dappsStaking.register(stakingContract.address, { Wasm: stakingContract.address })), alice);
//         const stakingAmount = ONE.muln(50000);
//
//         let { gasRequired } = await stakingContract.withSigner(bob).query.bondAndStake({ value: stakingAmount });
//         await stakingContract.withSigner(bob).tx.bondAndStake({ value: stakingAmount, gasLimit: gasRequired });
//
//         // verify contract's stake
//         const generalStakerInfo = await api.query.dappsStaking.generalStakerInfo<GeneralStakerInfo>(
//             stakingContract.address,
//             {
//                 Wasm: stakingContract.address,
//             }
//         );
//
//         // @ts-ignore
//         expect(generalStakerInfo.stakes[generalStakerInfo.stakes.length - 1].staked.toString()).to.equal(stakingAmount.toString())
//
//         // verify that Bob did not directly stake. It is contract that staked in the Bob's name
//         const bobStakerInfo = await api.query.dappsStaking.generalStakerInfo<GeneralStakerInfo>(
//             bob.address,
//             {
//                 Wasm: stakingContract.address,
//             }
//         );
//         expect(bobStakerInfo.stakes.length).to.equal(0)
//     })
//
//     it('should unbond and unstake on contract', async () => {
//         // register & Bob sends funds on contract to be staked
//         // @ts-ignore
//         await buildTx(api.registry, api.tx.sudo.sudo(api.tx.dappsStaking.register(stakingContract.address, { Wasm: stakingContract.address })), alice);
//         const stakingAmount = ONE.muln(1000);
//
//         let { gasRequired } = await stakingContract.withSigner(bob).query.bondAndStake({ value: stakingAmount });
//         await stakingContract.withSigner(bob).tx.bondAndStake({ value: stakingAmount, gasLimit: gasRequired });
//
//         // unbond & unstake 600
//         const unbondAmount = ONE.muln(600);
//         let { gasRequired: gas } = await stakingContract.withSigner(bob).query.unbondAndUnstake(unbondAmount );
//         await stakingContract.withSigner(bob).tx.unbondAndUnstake(unbondAmount, { gasLimit: gas });
//
//         // verify contract stake is decreased by 600, and now is 400
//         const generalStakerInfo = await api.query.dappsStaking.generalStakerInfo<GeneralStakerInfo>(
//             stakingContract.address,
//             {
//                 Wasm: stakingContract.address,
//             }
//         );
//         expect(generalStakerInfo.stakes).not.equal([]);
//         expect(generalStakerInfo.stakes[generalStakerInfo.stakes.length - 1].staked.toString()).to.eqls(stakingAmount.sub(unbondAmount).toString());
//     })
//
//     it('should withdraw unbonded', async () => {
//         const bondingDuration = api.consts.dappsStaking.unbondingPeriod
//
//         // register & Bob sends funds on contract to be staked
//         // @ts-ignore
//         await buildTx(api.registry, api.tx.sudo.sudo(api.tx.dappsStaking.register(stakingContract.address, { Wasm: stakingContract.address })), alice);
//         const stakingAmount = ONE.muln(50000);
//         let { gasRequired } = await stakingContract.withSigner(bob).query.bondAndStake({ value: stakingAmount });
//         await stakingContract.withSigner(bob).tx.bondAndStake({ value: stakingAmount, gasLimit: gasRequired });
//
//         // @ts-ignore
//         await forceEras(1, alice)
//
//         let { gasRequired: gas } = await stakingContract.withSigner(bob).query.unbondAndUnstake(stakingAmount);
//         await stakingContract.withSigner(bob).tx.unbondAndUnstake(stakingAmount, { gasLimit: gas });
//
//         // @ts-ignore
//         await forceEras(bondingDuration + 1, alice)
//
//         const balanceBefore = await api.query.system.account(bob.address)
//         // @ts-ignore
//         expect(balanceBefore.data.reserved).to.not.equal(balanceBefore.data.free)
//
//         let { gasRequired: gas2 } = await stakingContract.withSigner(bob).query.withdrawUnbonded();
//         await stakingContract.withSigner(bob).tx.withdrawUnbonded({ gasLimit: gas2 });
//
//         const balanceAfter = await api.query.system.account(bob.address)
//         // @ts-ignore
//         expect(balanceAfter.data.reserved).to.not.equal(balanceAfter.data.free)
//         // @ts-ignore
//         expect(balanceAfter.data.reserved.toNumber()).to.equal(0)
//     })
//
//     it('should claim staker', async () => {
//         const balanceStakingContract = await api.query.system.account(stakingContract.address)
//
//         const bondingDuration = api.consts.dappsStaking.unbondingPeriod
//         // register & Bob sends funds on contract to be staked
//         // @ts-ignore
//         await buildTx(api.registry, api.tx.sudo.sudo(api.tx.dappsStaking.register(stakingContract.address, { Wasm: stakingContract.address })), alice);
//         const stakingAmount = ONE.muln(50000);
//         let { gasRequired } = await stakingContract.withSigner(bob).query.bondAndStake({ value: stakingAmount });
//         await stakingContract.withSigner(bob).tx.bondAndStake({ value: stakingAmount, gasLimit: gasRequired });
//
//         // @ts-ignore
//         await forceEras(bondingDuration + 1, alice)
//
//         // claim staker's reward
//         const balanceBefore = await api.query.system.account(stakingContract.address)
//         // @ts-ignore
//         expect(balanceBefore.data.free.toString()).to.equal(new BN(stakingAmount).add(new BN(balanceStakingContract.data.free).sub(ONE.muln(100))).toString())
//
//         let { gasRequired: gas } = await stakingContract.withSigner(bob).query.claimStaker();
//         await stakingContract.withSigner(bob).tx.claimStaker({ gasLimit: gas });
//
//         const balanceAfter = await api.query.system.account(stakingContract.address)
//         // @ts-ignore
//         expect(balanceAfter.data.free - balanceBefore.data.free).to.be.above(0)
//     })
//
//     it('should claim dapp', async () => {
//         // register & Bob sends funds on contract to be staked
//         // @ts-ignore
//         await buildTx(api.registry, api.tx.sudo.sudo(api.tx.dappsStaking.register(stakingContract.address, { Wasm: stakingContract.address })), alice);
//         const stakingAmount = ONE.muln(50000);
//         let { gasRequired } = await stakingContract.withSigner(bob).query.bondAndStake({ value: stakingAmount });
//         await stakingContract.withSigner(bob).tx.bondAndStake({ value: stakingAmount, gasLimit: gasRequired });
//
//         // advance an era to be eligible for era reward
//         await forceEras(2, alice)
//
//         const balanceBefore = await api.query.system.account(stakingContract.address)
//         const currentEra = await api.query.dappsStaking.currentEra()
//         // @ts-ignore
//         const generalEraInfo = (await api.query.dappsStaking.generalEraInfo<Option<EraInfo>>(currentEra - 1))?.unwrapOrDefault()
//         // @ts-ignore
//         const contractStake = (await api.query.dappsStaking.contractEraStake<Option<EraStakingPointsIndividualClaim>>({ Wasm: stakingContract.address }, currentEra - 1))?.unwrapOrDefault();
//         // @ts-ignore
//         const theoricalrewards = contractStake.total / generalEraInfo.staked * generalEraInfo.rewards.dapps
//
//         let { gasRequired: gas } = await stakingContract.withSigner(alice).query.claimDapp(stakingContract.address, Number(currentEra) - 1);
//         await stakingContract.withSigner(alice).tx.claimDapp(stakingContract.address, Number(currentEra) - 1 ,{ gasLimit: gas });
//
//         const balanceAfter = await api.query.system.account(stakingContract.address)
//         // @ts-ignore
//         const actualRewards = balanceAfter.data.free - balanceBefore.data.free
//
//         // should be theoretically just below (because of gas paid for tx)
//         expect(actualRewards).is.below(theoricalrewards)
//         // check that actualRewards is within a +0.01% range
//         expect(actualRewards * 1.0001).is.above(theoricalrewards)
//     })
//
//     it('should update reward destination', async () => {
//          // register & Bob sends funds on contract to be staked
//         // @ts-ignore
//         await buildTx(api.registry, api.tx.sudo.sudo(api.tx.dappsStaking.register(stakingContract.address, { Wasm: stakingContract.address })), alice);
//         const stakingAmount = ONE.muln(50000);
//         let { gasRequired } = await stakingContract.withSigner(bob).query.bondAndStake({ value: stakingAmount });
//         await stakingContract.withSigner(bob).tx.bondAndStake({ value: stakingAmount, gasLimit: gasRequired });
//
//         // by default is set to StakeBalance
//         const ledger = await api.query.dappsStaking.ledger<AccountLedger>(stakingContract.address)
//
//         // @ts-ignore
//         expect(ledger.rewardDestination.toString()).to.equal('StakeBalance')
//
//         // set to FreeBalance
//         let { gasRequired: gas } = await stakingContract.query.setRewardDestination(0);
//         await stakingContract.tx.setRewardDestination(0,{ gasLimit: gas });
//
//         const ledger2 = await api.query.dappsStaking.ledger<AccountLedger>(stakingContract.address)
//         // @ts-ignore
//         await expect(ledger2.rewardDestination.toString()).to.equal('FreeBalance')
//     })
//
//     it('should nomination transfer', async () => {
//         let constructor = new staking_constructor(api, alice);
//         let contract2 = new staking_contract((await constructor.new()).address, alice, api);
//         await buildTx(api.registry, api.tx.balances.transfer(contract2.address, ONE.muln(10000)), alice)
//
//         // register both contracts & stake in the first one
//         // @ts-ignore
//         await buildTx(api.registry, api.tx.sudo.sudo(api.tx.dappsStaking.register(stakingContract.address, { Wasm: stakingContract.address })), alice);
//         // @ts-ignore
//         await buildTx(api.registry, api.tx.sudo.sudo(api.tx.dappsStaking.register(contract2.address, { Wasm: contract2.address })), alice);
//         const stakingAmount = ONE.muln(50000);
//         const transferAmount = ONE.muln(1000)
//
//         let { gasRequired } = await stakingContract.withSigner(bob).query.bondAndStake({ value: stakingAmount });
//         await stakingContract.withSigner(bob).tx.bondAndStake({ value: stakingAmount, gasLimit: gasRequired });
//
//         const stakerInfoContract1Before = await api.query.dappsStaking.generalStakerInfo<GeneralStakerInfo>(
//             stakingContract.address,
//             {
//                 Wasm: stakingContract.address,
//             }
//         );
//         const stakerInfoContract2Before = await api.query.dappsStaking.generalStakerInfo<GeneralStakerInfo>(
//             contract2.address,
//             {
//                 Wasm: contract2.address,
//             }
//         );
//
//         let { gasRequired: gas } = await stakingContract.withSigner(bob).query.nominationTransfer(stakingContract.address,contract2.address, transferAmount);
//         await stakingContract.withSigner(bob).tx.nominationTransfer(stakingContract.address, contract2.address, transferAmount,{ gasLimit: gas });
//
//         const stakerInfoContract1After = await api.query.dappsStaking.generalStakerInfo<GeneralStakerInfo>(
//             stakingContract.address,
//             {
//                 Wasm: stakingContract.address,
//             }
//         );
//         const stakerInfoContract2After = await api.query.dappsStaking.generalStakerInfo<GeneralStakerInfo>(
//             stakingContract.address,
//             {
//                 Wasm: contract2.address,
//             }
//         );
//
//         // @ts-ignore
//         expect(stakerInfoContract1Before.stakes[stakerInfoContract1Before.stakes.length - 1].staked.toString()).to.equal(stakingAmount.toString())
//
//         expect(stakerInfoContract2Before.stakes.toString()).to.equal('[]')
//         // @ts-ignore
//         expect(stakerInfoContract1After.stakes[stakerInfoContract1After.stakes.length - 1].staked.toString()).to.equal(stakingAmount.sub(transferAmount).toString())
//         expect(stakerInfoContract2After.stakes[stakerInfoContract2After.stakes.length - 1].staked.toString()).to.equal(transferAmount.toString())
//     })
//
//     const forceEras = async (eras, sudo) => {
//         const forceNewEra = api.tx.sudo.sudo(api.tx.dappsStaking.forceNewEra())
//         let n = await api.rpc.system.accountNextIndex(sudo.address)
//         for (let i = 0; i < eras; i++) {
//             await buildTx(api.registry, forceNewEra, sudo, {nonce: n.toNumber() + i})
//         }
//     }
// })