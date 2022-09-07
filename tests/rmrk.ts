import { forceEras, setupContract } from './helper'
import { network } from 'redspot'
import { expect } from "./setup/chai";
import { Option } from '@polkadot/types';
import { buildTx } from '@redspot/patract/buildTx'
import { AccountLedger } from "./types";
const { api } = network

describe('RMRK', () => {
    async function setup () {
        return await setupContract('rmrk_example', 'new')
    }

    it('should read collection index', async () => {
        const { contract, one} = await setup();

        const expectedIndex = await api.query.rmrkCore.collectionIndex();
        await expect(contract.query.collectionIndex()).to.output(expectedIndex);
    })
})
