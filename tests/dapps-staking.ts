import {setupContract} from './helper'
import {network} from 'redspot'
import {expect} from "./setup/chai";
import type { AccountId } from '@polkadot/types/interfaces';
import { createType } from '@polkadot/types';
import { Struct } from '@polkadot/types';
import { Balance } from '@polkadot/types/interfaces';
const {api} = network

export interface EraStake extends Struct {
    staked: Balance;
    era: number;
}
export interface GeneralStakerInfo extends Struct {
    readonly stakes: EraStake[];
}

describe('DAPPS STAKING', () => {
    async function setup() {
        return setupContract('staking_example', 'new')
    }

    it('quey trhought ink chain-extension query should equal dapps-staking pallet', async () => {
        const {contract, defaultSigner} = await setup()

        let currentEra = await api.query.dappsStaking.currentEra()
        console.log(currentEra.toHuman())
        // await expect(contract.query.readCurrentEra()).to.output(currentEra) //TODO

        // read_unbonding_period() // TODO
        const bondingDuration = api.consts.staking
        console.log(bondingDuration)

        let generalEraInfo = await api.query.dappsStaking.generalEraInfo(currentEra)
        console.log(generalEraInfo.toHuman())
        // await expect(contract.query.read_era_reward(generalEraInfo)).to.output(currentEra) //TODO

        let staked = await api.query.dappsStaking.ledger(defaultSigner.address)
        console.log(staked.locked.toHuman())

        const generalStakerInfo = await api.query.dappsStaking.generalStakerInfo<GeneralStakerInfo>(
            defaultSigner.address,
            {
                Wasm: contract.address,
            }
        );
        console.log(generalStakerInfo.toHuman())
    })
})