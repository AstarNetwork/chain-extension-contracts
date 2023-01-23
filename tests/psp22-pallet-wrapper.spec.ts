import { expect, use } from 'chai';
import chaiAsPromised from 'chai-as-promised';
import BN from 'bn.js';
import psp22_constructor from '../types/constructors/psp22_pallet_wrapper';
import psp22_contract from '../types/contracts/psp22_pallet_wrapper';
import { ApiPromise, WsProvider, Keyring } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import {buildTx} from "./helper";
import {afterEach} from "mocha";

use(chaiAsPromised);

const ONE = new BN(10).pow(new BN(18));

// Create a new instance of contract
const wsProvider = new WsProvider('ws://127.0.0.1:9944');
// Create a keyring instance
const keyring = new Keyring({ type: 'sr25519' });

describe('PSP22 PALLET WRAPPER', () => {
    let api: ApiPromise;
    let alice: KeyringPair;
    let bob: KeyringPair;
    let psp22Constructor: psp22_constructor
    let psp22: psp22_contract;

     beforeEach(async function() {
        api = await ApiPromise.create({ provider: wsProvider });
        alice = keyring.addFromUri('//Alice');
        bob = keyring.addFromUri('//Bob');
        psp22Constructor = new psp22_constructor(api, alice);
        await buildTx(api.registry, api.tx.assets.create(1, {id: alice.address}, 1), alice)
        psp22 = new psp22_contract((await psp22Constructor.new(1)).address, alice, api);
    })

    afterEach( async function() {
        await buildTx(api.registry, api.tx.assets.destroy(1, {accounts: 10, sufficients: 10, approvals: 10}), alice)
    })

    it('get the proper asset id', async () => {
        await expect((await psp22.query.assetId()).value.toNumber()).to.equal(1)
    })

    it('deposit works', async () => {
        await buildTx(api.registry, api.tx.assets.mint(1, {id: alice.address}, 1000), alice)

        let { gasRequired } = await psp22.query.deposit(100);
        await psp22.tx.deposit(100, {gasLimit: gasRequired * 2n });

        await expect((await psp22.query.balanceOf(alice.address)).value.toNumber()).to.equal(100)
    })

    it('deposit & transfer to bob works', async () => {
        await buildTx(api.registry, api.tx.assets.mint(1, {id: alice.address}, 1000), alice)

        let { gasRequired } = await psp22.query.deposit(100);
        await psp22.tx.deposit(100, {gasLimit: gasRequired * 2n });

        let { gasRequired: gas }  = await psp22.query.transfer(bob.address ,100, ['']);
        await psp22.tx.transfer(bob.address ,100, [''], {gasLimit: gas * 2n });

        // @ts-ignore
        await expect((await api.query.assets.account(1, psp22.address)).unwrapOrDefault().balance.toNumber()).to.equal(100)
        // @ts-ignore
        await expect((await api.query.assets.account(1, alice.address)).unwrapOrDefault().balance.toNumber()).to.equal(1000 - 100)
        // @ts-ignore
        await expect((await api.query.assets.account(1, bob.address)).unwrapOrDefault().balance.toNumber()).to.equal(0)
        await expect((await psp22.query.balanceOf(alice.address)).value.toNumber()).to.equal(0)
        await expect((await psp22.query.balanceOf(bob.address)).value.toNumber()).to.equal(100)
    })

    it('deposit & transfer to bob works & bob withdraw wroks', async () => {
        await buildTx(api.registry, api.tx.assets.mint(1, {id: alice.address}, 1000), alice)

        let { gasRequired } = await psp22.query.deposit(100);
        await psp22.tx.deposit(100, {gasLimit: gasRequired * 2n });

        let { gasRequired: gas }  = await psp22.query.transfer(bob.address ,100, ['']);
        await psp22.tx.transfer(bob.address ,100, [''], {gasLimit: gas * 2n });

        let { gasRequired: gas2 }  = await psp22.withSigner(bob).query.withdraw(100);
        await psp22.withSigner(bob).tx.withdraw(100, {gasLimit: gas2 * 2n });

        // @ts-ignore
        await expect((await api.query.assets.account(1, psp22.address)).unwrapOrDefault().balance.toNumber()).to.equal(0)
        // @ts-ignore
        await expect((await api.query.assets.account(1, alice.address)).unwrapOrDefault().balance.toNumber()).to.equal(1000 - 100)
        // @ts-ignore
        await expect((await api.query.assets.account(1, bob.address)).unwrapOrDefault().balance.toNumber()).to.equal(100)
        await expect((await psp22.query.balanceOf(alice.address)).value.toNumber()).to.equal(0)
        await expect((await psp22.query.balanceOf(bob.address)).value.toNumber()).to.equal(0)
    })
})
