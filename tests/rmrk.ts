import { forceEras, setupContract } from './helper'
import { network } from 'redspot'
import { expect } from "./setup/chai";
import { Option } from '@polkadot/types';
import { buildTx } from '@redspot/patract/buildTx'
import { AccountLedger } from "./types";
import BN from "bn.js";
const { api } = network

describe('RMRK', () => {
    async function setup () {
        return await setupContract('rmrk_example', 'new')
    }

    it('should read collection index', async () => {
        const { contract } = await setup();

        const expectedIndex = await api.query.rmrkCore.collectionIndex();
        await expect(contract.query.collectionIndex()).to.output(expectedIndex);
    })



    it('create NFT collection', async () => {
        const { contract, one, unit, alice } = await setup();

        const currentIndex = await api.query.rmrkCore.collectionIndex();
        console.log("currentIndex:", currentIndex)
        const reserveValue = one.muln(100);
        const tx = await contract.connect(alice).tx.createCollection('test-metadata', 42, 'test-symbol', { value: reserveValue });
        console.log("create connection tx:", tx)
        const afterIndex = await api.query.rmrkCore.collectionIndex();
        console.log("afterIndex:", afterIndex)
        // expect(await api.query.rmrkCore.collectionIndex()).not.equal(currentIndex);
    });

    it('[error] create NFT collection for 0 NFTs', async () => {
        const { contract, one, alice } = await setup();

        const currentIndex = await api.query.rmrkCore.collectionIndex();
        console.log("currentIndex:", currentIndex)
        const reserveValue = one.muln(1);
        const tx = await contract.connect(alice).query.createCollection('', 0, '', { value: reserveValue });
        console.log("create connection tx:", tx)
        const afterIndex = await api.query.rmrkCore.collectionIndex();
        console.log("afterIndex:", afterIndex)
    });


})
