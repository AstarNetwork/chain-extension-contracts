import {setupContract} from './helper'
import {network} from 'redspot'
import {expect} from "./setup/chai";
import type { AccountId } from '@polkadot/types/interfaces';
import { Struct, u32, bool, Option } from '@polkadot/types';
import { Balance } from '@polkadot/types/interfaces';
import { buildTx } from '@redspot/patract/buildTx'
const {api} = network

interface EraStake extends Struct {
    staked: Balance;
    era: number;
}
interface GeneralStakerInfo extends Struct {
    readonly stakes: EraStake[];
}

interface EraStakingPointsIndividualClaim extends Struct {
    total: Balance;
    numberOfStakers: u32;
    contractRewardClaimed: bool;
}

interface RegisteredDapps extends Struct {
    readonly developer: AccountId;
    readonly state: State;
}

interface State {
    isUnregistered: boolean;
    asUnregistered: {
        // Memo: era of unregistered
        words: number[];
    };
}

interface EraInfo extends Struct {
    rewards: {
        stakers: Balance;
        dapps: Balance;
    };
    staked: Balance;
    locked: Balance;
}

describe('DAPPS STAKING', () => {
    async function setup() {
        return await setupContract('staking_example', 'new')
    }

    it('should read current era', async () => {
        const {contract} = await setup()

        let currentEra = await api.query.dappsStaking.currentEra()
        await expect(contract.query.readCurrentEra()).to.output(currentEra)
    })

    it('should read unbonding period', async () => {
        const {contract} = await setup()

        const bondingDuration = api.consts.dappsStaking.unbondingPeriod
        await expect(contract.query.readUnbondingPeriod()).to.output(bondingDuration)
    })

    it('should read era reward', async () => {
        const {contract, defaultSigner, one} = await setup()

        await buildTx(api.registry, api.tx.dappsStaking.register({ Wasm: contract.address }), defaultSigner.address)
        await buildTx(api.registry, api.tx.dappsStaking.bondAndStake({ Wasm: contract.address }, one.muln(10000)), defaultSigner.address)

        let currentEra = await api.query.dappsStaking.currentEra()
        const generalEraInfo = (await api.query.dappsStaking.generalEraInfo<Option<EraInfo>>(currentEra))?.unwrapOrDefault()
        // @ts-ignore
        await expect(contract.query.readEraReward(currentEra)).to.output(generalEraInfo.rewards.dapps + generalEraInfo.rewards.stakers)
    })

    it('should read era staked', async () => {
        const {contract, defaultSigner, one} = await setup()

        await buildTx(api.registry, api.tx.dappsStaking.register({ Wasm: contract.address }), defaultSigner.address)
        await buildTx(api.registry, api.tx.dappsStaking.bondAndStake({ Wasm: contract.address }, one.muln(10000)), defaultSigner.address)

        let currentEra = await api.query.dappsStaking.currentEra()
        const generalEraInfo = (await api.query.dappsStaking.generalEraInfo<Option<EraInfo>>(currentEra))?.unwrapOrDefault()
        await expect(contract.query.readEraStaked(currentEra)).to.output(generalEraInfo.staked)
    })

    it('should read contract stake', async () => {
        const {contract, defaultSigner, one} = await setup()

        await buildTx(api.registry, api.tx.dappsStaking.register({ Wasm: contract.address }), defaultSigner.address)
        await buildTx(api.registry, api.tx.dappsStaking.bondAndStake({ Wasm: contract.address }, one.muln(10000)), defaultSigner.address)

        let ledger = await api.query.dappsStaking.ledger(defaultSigner.address)
        // @ts-ignore
        await expect(contract.query.readStakedAmount(defaultSigner.address)).to.output(ledger.locked)
    })

    it('should read staked amount on contract', async () => {
        const {contract, defaultSigner, one} = await setup()

        await buildTx(api.registry, api.tx.dappsStaking.register({ Wasm: contract.address }), defaultSigner.address)
        await buildTx(api.registry, api.tx.dappsStaking.bondAndStake({ Wasm: contract.address }, one.muln(10000)), defaultSigner.address)

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
        const {contract, one, defaultSigner} = await setup()

        await buildTx(api.registry, api.tx.dappsStaking.register({ Wasm: contract.address }), defaultSigner.address)
        await buildTx(api.registry, api.tx.dappsStaking.bondAndStake({ Wasm: contract.address }, one.muln(10000)), defaultSigner.address)

        let currentEra = await api.query.dappsStaking.currentEra()
        const contractStake = (await api.query.dappsStaking.contractEraStake<Option<EraStakingPointsIndividualClaim>>({Wasm: contract.address}, currentEra))?.unwrapOrDefault();
        await expect(contract.query.readContractStake(contract.address)).to.output(contractStake.total)
    })

    it('should register contract', async () => {
        const {contract, one, defaultSigner} = await setup()
        // register contract
        await contract.tx.register(contract.address, {gasLimit: 30000000000})

        // bond and stake & verify
        let currentEra = await api.query.dappsStaking.currentEra()
        await buildTx(api.registry, api.tx.dappsStaking.bondAndStake({ Wasm: contract.address }, one.muln(10000)), defaultSigner.address)

        const contractStake = (await api.query.dappsStaking.contractEraStake<Option<EraStakingPointsIndividualClaim>>({Wasm: contract.address}, currentEra))?.unwrapOrDefault()
        await expect(contract.query.readContractStake(contract.address)).to.output(contractStake.total)
    })

    it('should bond and stake on contract', async () => {
        const {contract, one, defaultSigner} = await setup()
        // register contract
        await buildTx(api.registry, api.tx.dappsStaking.register({ Wasm: contract.address }), defaultSigner.address)

        // bond on stake from contract
        await contract.tx.bondAndStake(contract.address, one.muln(100))

        // verify stake
        const generalStakerInfo = await api.query.dappsStaking.generalStakerInfo<GeneralStakerInfo>(
            defaultSigner.address,
            {
                Wasm: contract.address,
            }
        );
        expect(generalStakerInfo.stakes[generalStakerInfo.stakes.length - 1].staked.toBn()).to.equal(one.muln(100))
    })

    it('should unbond and unstake on contract', async () => {
        const {contract, one, defaultSigner} = await setup()

        // register & bond 1000 on contract
        await buildTx(api.registry, api.tx.dappsStaking.register({ Wasm: contract.address }), defaultSigner.address)
        await buildTx(api.registry, api.tx.dappsStaking.bondAndStake({ Wasm: contract.address }, one.muln(1000)), defaultSigner.address)

        // unbond & unstake 600
        await contract.tx.unbondAndUnstake(contract.address, one.muln(600))

        // verify stake
        const generalStakerInfo = await api.query.dappsStaking.generalStakerInfo<GeneralStakerInfo>(
            defaultSigner.address,
            {
                Wasm: contract.address,
            }
        );
        expect(generalStakerInfo.stakes[generalStakerInfo.stakes.length - 1].staked.toBn()).to.equal(one.muln(400))
    })

    it('should withdraw unbonded', async () => {
        const {contract, one, defaultSigner, alice} = await setup()

        // @ts-ignore
        const { data: balanceBefore } = await api.query.system.account(defaultSigner.address)
        console.log(balanceBefore.free.toHuman())

        // register & bond 1000 on contract
        await buildTx(api.registry, api.tx.dappsStaking.register({ Wasm: contract.address }), defaultSigner.address)
        await buildTx(api.registry, api.tx.dappsStaking.bondAndStake({ Wasm: contract.address }, one.muln(1000)), defaultSigner.address)

        // unbond & unstake 600
        await buildTx(api.registry, api.tx.dappsStaking.unbondAndUnstake({ Wasm: contract.address }, one.muln(600)), defaultSigner.address)

        let currentEra = await api.query.dappsStaking.currentEra()
        console.log(currentEra)
        // advance 3 eras
        await buildTx(api.registry, api.tx.dappsStaking.forceNewEra(), alice.address)
        await buildTx(api.registry, api.tx.dappsStaking.forceNewEra(), alice.address)
        await buildTx(api.registry, api.tx.dappsStaking.forceNewEra(), alice.address)

        let currentEra2 = await api.query.dappsStaking.currentEra()
        console.log(currentEra2)

        // @ts-ignore
        const { data: balanceAfter } = await api.query.system.account(defaultSigner.address)
        console.log(balanceAfter.free.toHuman())

        // unbond & unstake 600
        await contract.query.withdrawUnbonded()

        // @ts-ignore
        const { data: balanceAfter3 } = await api.query.system.account(defaultSigner.address)
        console.log(balanceAfter3.free.toHuman())
    })
})