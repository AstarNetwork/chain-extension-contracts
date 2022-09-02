import { forceEras, setupContract } from './helper'
import { network } from 'redspot'
import { expect } from "./setup/chai";
import { Option } from '@polkadot/types';
import { buildTx } from '@redspot/patract/buildTx'
import { AccountLedger, EraInfo, EraStakingPointsIndividualClaim, GeneralStakerInfo } from "./types";
const { api } = network

describe('DAPPS STAKING', () => {
    async function setup () {
        return await setupContract('staking_example', 'new')
    }

    it('should read current era', async () => {
        const { contract } = await setup()

        const currentEra = await api.query.dappsStaking.currentEra()
        await expect(contract.query.readCurrentEra()).to.output(currentEra)
    })

    it('should read unbonding period', async () => {
        const { contract } = await setup()

        const bondingDuration = api.consts.dappsStaking.unbondingPeriod
        await expect(contract.query.readUnbondingPeriod()).to.output(bondingDuration)
    })

    it('should read era reward', async () => {
        const { contract, defaultSigner, one, alice } = await setup()

        // @ts-ignore
        await buildTx(api.registry, api.tx.sudo.sudo(api.tx.dappsStaking.register(defaultSigner.address, { Wasm: contract.address })), alice);
        await buildTx(api.registry, api.tx.dappsStaking.bondAndStake({ Wasm: contract.address }, one.muln(10000)), defaultSigner.address)

        const currentEra = await api.query.dappsStaking.currentEra()
        const generalEraInfo = (await api.query.dappsStaking.generalEraInfo<Option<EraInfo>>(currentEra))?.unwrapOrDefault()
        // @ts-ignore
        await expect(contract.query.readEraReward(currentEra)).to.output(generalEraInfo.rewards.dapps + generalEraInfo.rewards.stakers)
    })

    it('should read era staked', async () => {
        const { contract, defaultSigner, one, alice } = await setup()

        // @ts-ignore
        await buildTx(api.registry, api.tx.sudo.sudo(api.tx.dappsStaking.register(defaultSigner.address, { Wasm: contract.address })), alice); await buildTx(api.registry, api.tx.dappsStaking.bondAndStake({ Wasm: contract.address }, one.muln(10000)), defaultSigner.address)

        const currentEra = await api.query.dappsStaking.currentEra()
        const generalEraInfo = (await api.query.dappsStaking.generalEraInfo<Option<EraInfo>>(currentEra))?.unwrapOrDefault()
        await expect(contract.query.readEraStaked(currentEra)).to.output(generalEraInfo.staked)
    })

    it('should read contract stake', async () => {
        const { contract, defaultSigner, one, alice } = await setup()

        // @ts-ignore
        await buildTx(api.registry, api.tx.sudo.sudo(api.tx.dappsStaking.register(defaultSigner.address, { Wasm: contract.address })), alice); await buildTx(api.registry, api.tx.dappsStaking.bondAndStake({ Wasm: contract.address }, one.muln(10000)), defaultSigner.address)

        const ledger = await api.query.dappsStaking.ledger(defaultSigner.address)
        // @ts-ignore
        await expect(contract.query.readStakedAmount(defaultSigner.address)).to.output(ledger.locked)
    })

    it('should read staked amount on contract', async () => {
        const { contract, defaultSigner, one, alice } = await setup()

        // @ts-ignore
        await buildTx(api.registry, api.tx.sudo.sudo(api.tx.dappsStaking.register(defaultSigner.address, { Wasm: contract.address })), alice); await buildTx(api.registry, api.tx.dappsStaking.bondAndStake({ Wasm: contract.address }, one.muln(10000)), defaultSigner.address)

        const generalStakerInfo = await api.query.dappsStaking.generalStakerInfo<GeneralStakerInfo>(
            defaultSigner.address,
            {
                Wasm: contract.address,
            }
        );
        await expect(contract.query.readStakedAmountOnContract(defaultSigner.address, contract.address))
            .to.output(generalStakerInfo.stakes[generalStakerInfo.stakes.length - 1].staked)
    })

    it('should read contract stake', async () => {
        const { contract, one, defaultSigner, alice } = await setup()

        // @ts-ignore
        await buildTx(api.registry, api.tx.sudo.sudo(api.tx.dappsStaking.register(defaultSigner.address, { Wasm: contract.address })), alice); await buildTx(api.registry, api.tx.dappsStaking.bondAndStake({ Wasm: contract.address }, one.muln(10000)), defaultSigner.address)

        const currentEra = await api.query.dappsStaking.currentEra()
        const contractStake = (await api.query.dappsStaking.contractEraStake<Option<EraStakingPointsIndividualClaim>>({ Wasm: contract.address }, currentEra))?.unwrapOrDefault();
        await expect(contract.query.readContractStake(contract.address)).to.output(contractStake.total)
    })

    it('should bond and stake on contract', async () => {
        const { contract, one, bob, defaultSigner, alice } = await setup()

        // register & Bob sends funds on contract to be staked
        // @ts-ignore
        await buildTx(api.registry, api.tx.sudo.sudo(api.tx.dappsStaking.register(defaultSigner.address, { Wasm: contract.address })), alice);
        const stakingAmount = one.muln(50000);
        await contract.connect(bob).tx.bondAndStake({ value: stakingAmount })

        // verify contract's stake
        const generalStakerInfo = await api.query.dappsStaking.generalStakerInfo<GeneralStakerInfo>(
            contract.address,
            {
                Wasm: contract.address,
            }
        );

        const minimumRemainingAmount = api.consts.dappsStaking.minimumRemainingAmount;
        // @ts-ignore
        expect(generalStakerInfo.stakes[generalStakerInfo.stakes.length - 1].staked.toBn()).to.equal(stakingAmount.sub(minimumRemainingAmount))

        // verify that Bob did not directly stake. It is contract that staked in the Bob's name
        const bobStakerInfo = await api.query.dappsStaking.generalStakerInfo<GeneralStakerInfo>(
            bob.address,
            {
                Wasm: contract.address,
            }
        );
        expect(bobStakerInfo.stakes.length).to.equal(0)
    })

    it('should unbond and unstake on contract', async () => {
        const { contract, one, bob, defaultSigner, alice } = await setup();

        // register & Bob sends funds on contract to be staked
        // @ts-ignore
        await buildTx(api.registry, api.tx.sudo.sudo(api.tx.dappsStaking.register(defaultSigner.address, { Wasm: contract.address })), alice);
        const stakingAmount = one.muln(1000);
        await contract.connect(bob).tx.bondAndStake({ value: stakingAmount });

        // unbond & unstake 600
        const unbondAmount = one.muln(600);
        await contract.connect(bob).tx.unbondAndUnstake(unbondAmount);

        // verify contract stake is decreased by 600, and now is 400
        const generalStakerInfo = await api.query.dappsStaking.generalStakerInfo<GeneralStakerInfo>(
            contract.address,
            {
                Wasm: contract.address,
            }
        );
        expect(generalStakerInfo.stakes).not.equal([]);
        expect(generalStakerInfo.stakes[generalStakerInfo.stakes.length - 1].staked.toBn()).to.equal(stakingAmount.sub(unbondAmount).sub(one));
    })

    it('should withdraw unbonded', async () => {
        const { contract, one, alice, bob, defaultSigner } = await setup()

        const bondingDuration = api.consts.dappsStaking.unbondingPeriod

        // register & Bob sends funds on contract to be staked
        // @ts-ignore
        await buildTx(api.registry, api.tx.sudo.sudo(api.tx.dappsStaking.register(defaultSigner.address, { Wasm: contract.address })), alice);
        const stakingAmount = one.muln(50000);
        await contract.connect(bob).tx.bondAndStake({ value: stakingAmount });

        // @ts-ignore
        await forceEras(1, alice)

        await contract.connect(bob).tx.unbondAndUnstake(stakingAmount);

        // @ts-ignore
        await forceEras(bondingDuration + 1, alice)

        const balanceBefore = await api.query.system.account(bob.address)
        // @ts-ignore
        expect(balanceBefore.data.reserved).to.not.equal(balanceBefore.data.free)

        await contract.connect(bob).tx.withdrawUnbonded()

        const balanceAfter = await api.query.system.account(bob.address)
        // @ts-ignore
        expect(balanceAfter.data.reserved).to.not.equal(balanceAfter.data.free)
        // @ts-ignore
        expect(balanceAfter.data.reserved).to.equal(0)
    })

    it('should claim staker', async () => {
        const { contract, one, bob, alice, defaultSigner } = await setup()

        // register & Bob sends funds on contract to be staked
        // @ts-ignore
        await buildTx(api.registry, api.tx.sudo.sudo(api.tx.dappsStaking.register(defaultSigner.address, { Wasm: contract.address })), alice);
        const stakingAmount = one.muln(50000);
        await contract.connect(bob).tx.bondAndStake({ value: stakingAmount });

        // advance 4 eras
        await forceEras(4, alice)

        // claim staker's reward
        const balanceBefore = await api.query.system.account(contract.address)
        // @ts-ignore
        expect(balanceBefore.data.free).to.equal(stakingAmount)
        await contract.tx.claimStaker()
        const balanceAfter = await api.query.system.account(contract.address)
        // @ts-ignore
        expect(balanceBefore.data.free).is.below(balanceAfter.data.free)
    })

    // it('should claim dapp', async () => {
    //     const {contract, one, alice, defaultSigner} = await setup()

    //     // register & bond 50000 on contract
    //     await buildTx(api.registry, api.tx.dappsStaking.register({ Wasm: contract.address }), defaultSigner.address)
    //     await buildTx(api.registry, api.tx.dappsStaking.bondAndStake({ Wasm: contract.address }, one.muln(50000)), alice.address)

    //     const bondingDuration = api.consts.dappsStaking.unbondingPeriod
    //     // @ts-ignore
    //     await forceEras(bondingDuration + 1, alice)

    //     const balanceBefore = await api.query.system.account(defaultSigner.address)
    //     const currentEra = await api.query.dappsStaking.currentEra()
    //     // @ts-ignore
    //     const generalEraInfo = (await api.query.dappsStaking.generalEraInfo<Option<EraInfo>>(currentEra - 1))?.unwrapOrDefault()
    //     // @ts-ignore
    //     const contractStake = (await api.query.dappsStaking.contractEraStake<Option<EraStakingPointsIndividualClaim>>({Wasm: contract.address}, currentEra - 1))?.unwrapOrDefault();
    //     // @ts-ignore
    //     const theoricalrewards = contractStake.total / generalEraInfo.staked * generalEraInfo.rewards.dapps

    //     // @ts-ignore
    //     await contract.tx.claimDapp(contract.address, currentEra - 1)

    //     const balanceAfter = await api.query.system.account(defaultSigner.address)
    //     // @ts-ignore
    //     const actualRewards = balanceAfter.data.free - balanceBefore.data.free

    //     // should be theoretically just below (because of gas paid for tx)
    //     expect(actualRewards).is.below(theoricalrewards)
    //     // check that actualRewards is within a +3% range
    //     expect(actualRewards * 1.03).is.above(theoricalrewards)
    // })

    it('should update reward destination', async () => {
        const { contract, bob, defaultSigner, one, alice } = await setup()

        // register & Bob sends funds on contract to be staked
        // @ts-ignore
        await buildTx(api.registry, api.tx.sudo.sudo(api.tx.dappsStaking.register(defaultSigner.address, { Wasm: contract.address })), alice);
        const stakingAmount = one.muln(50000);
        await contract.connect(bob).tx.bondAndStake({ value: stakingAmount });

        // by default is set to StakeBalance
        const ledger = await api.query.dappsStaking.ledger<AccountLedger>(contract.address)

        // @ts-ignore
        expect(ledger.rewardDestination).to.equal('StakeBalance')

        // set to FreeBalance
        await contract.tx.setRewardDestination(0)

        const ledger2 = await api.query.dappsStaking.ledger<AccountLedger>(contract.address)
        // @ts-ignore
        await expect(ledger2.rewardDestination).to.equal('FreeBalance')
    })

    it('should nomination transfer', async () => {
        const { contract, defaultSigner, one, alice, bob } = await setup()
        const { contract: contract2 } = await setup()

        // register both contracts & stake in the first one
        // @ts-ignore
        await buildTx(api.registry, api.tx.sudo.sudo(api.tx.dappsStaking.register(defaultSigner.address, { Wasm: contract.address })), alice);
        // @ts-ignore
        await buildTx(api.registry, api.tx.sudo.sudo(api.tx.dappsStaking.register(bob.address, { Wasm: contract2.address })), alice);
        const stakingAmount = one.muln(50000);
        const transferAmount = one.muln(1000)
        // @ts-ignore
        await contract.connect(bob).tx.bondAndStake({ value: stakingAmount });

        const stakerInfoContract1Before = await api.query.dappsStaking.generalStakerInfo<GeneralStakerInfo>(
            contract.address,
            {
                Wasm: contract.address,
            }
        );
        const stakerInfoContract2Before = await api.query.dappsStaking.generalStakerInfo<GeneralStakerInfo>(
            contract2.address,
            {
                Wasm: contract2.address,
            }
        );

        await contract.connect(alice).tx.nominationTransfer(contract.address, contract2.address, transferAmount)

        const stakerInfoContract1After = await api.query.dappsStaking.generalStakerInfo<GeneralStakerInfo>(
            contract.address,
            {
                Wasm: contract.address,
            }
        );
        const stakerInfoContract2After = await api.query.dappsStaking.generalStakerInfo<GeneralStakerInfo>(
            contract.address,
            {
                Wasm: contract2.address,
            }
        );

        const minimumRemainingAmount = api.consts.dappsStaking.minimumRemainingAmount;
        // @ts-ignore
        expect(stakerInfoContract1Before.stakes[stakerInfoContract1Before.stakes.length - 1].staked.toBn()).to.equal(stakingAmount.sub(minimumRemainingAmount))
        expect(stakerInfoContract2Before.stakes).to.equal([])
        // @ts-ignore
        expect(stakerInfoContract1After.stakes[stakerInfoContract1After.stakes.length - 1].staked.toBn()).to.equal(stakingAmount.sub(transferAmount).sub(minimumRemainingAmount))
        expect(stakerInfoContract2After.stakes[stakerInfoContract2After.stakes.length - 1].staked.toBn()).to.equal(transferAmount)
    })
})
