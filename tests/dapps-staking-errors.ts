import { setupContract } from './helper'
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

    it('[Error test] bond and stake with no value', async () => {
            const { contract, one, bob, defaultSigner, alice } = await setup()

            // register & Bob calls staking without funds
            // @ts-ignore
            await buildTx(api.registry, api.tx.sudo.sudo(api.tx.dappsStaking.register(defaultSigner.address, { Wasm: contract.address })), alice);
            await expect(contract.connect(bob).query.bondAndStake({ value: 0 })).to.output({ Err: 'InsufficientValue' })
        })


})
