import { expect, use } from 'chai';
import chaiAsPromised from 'chai-as-promised';
import psp22_constructor from '../types/constructors/psp22_pallet_wrapper';
import psp22_contract from '../types/contracts/psp22_pallet_wrapper';
import { ApiPromise, WsProvider, Keyring } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import {buildTx} from "./helper";
import {afterEach} from "mocha";
import {WeightV2} from "@polkadot/types/interfaces";
import BN from "bn.js";

use(chaiAsPromised);

const ONE = new BN(10).pow(new BN(18));

const ASSET_ID = 1

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
         api = await ApiPromise.create({ provider: wsProvider, noInitWarn: true });
        alice = keyring.addFromUri('//Alice');
        bob = keyring.addFromUri('//Bob');
        psp22Constructor = new psp22_constructor(api, alice);
        await buildTx(api.registry, api.tx.assets.create(1, {id: alice.address}, 1), alice)
        psp22 = new psp22_contract((await psp22Constructor.new(1)).address, alice, api);
    })

    afterEach( async function() {
        await buildTx(api.registry, api.tx.assets.startDestroy(ASSET_ID), alice)
        // Should be done 2 times because sometimes fails with InUse Error
        await buildTx(api.registry, api.tx.assets.destroyApprovals(ASSET_ID), alice)
        await buildTx(api.registry, api.tx.assets.destroyAccounts(ASSET_ID), alice)
        await buildTx(api.registry, api.tx.assets.destroyApprovals(ASSET_ID), alice)
        await buildTx(api.registry, api.tx.assets.destroyAccounts(ASSET_ID), alice)
        await buildTx(api.registry, api.tx.assets.finishDestroy(ASSET_ID), alice)
    })

    it('get the proper asset id', async () => {
        await expect((await psp22.query.assetId()).value.unwrap().toNumber()).to.equal(ASSET_ID)
    })

    it('deposit works', async () => {
        await buildTx(api.registry, api.tx.assets.mint(ASSET_ID, {id: alice.address}, ONE.muln(1000)), alice)
        await buildTx(api.registry, api.tx.assets.approveTransfer(ASSET_ID, psp22.address, ONE.muln(100)), alice)

        await psp22.tx.deposit(ONE.muln(100));

        await expect((await psp22.query.balanceOf(alice.address)).value.unwrap().toString()).to.equal(ONE.muln(100).toString())
    })

    it('deposit & transfer to bob works', async () => {
        await buildTx(api.registry, api.tx.assets.mint(ASSET_ID, {id: alice.address}, ONE.muln(1000)), alice)
        await buildTx(api.registry, api.tx.assets.approveTransfer(ASSET_ID, psp22.address, ONE.muln(100)), alice)

        await psp22.tx.deposit(ONE.muln(100));

        await psp22.tx.transfer(bob.address, ONE.muln(100), ['']);

        // @ts-ignore
        await expect((await api.query.assets.account(ASSET_ID, psp22.address)).unwrapOrDefault().balance.toString()).to.equal(ONE.muln(100).toString())
        // @ts-ignore
        await expect((await api.query.assets.account(ASSET_ID, alice.address)).unwrapOrDefault().balance.toString()).to.equal(ONE.muln(900).toString())
        // @ts-ignore
        await expect((await api.query.assets.account(ASSET_ID, bob.address)).unwrapOrDefault().balance.toNumber()).to.equal(0)
        await expect((await psp22.query.balanceOf(alice.address)).value.unwrap().toNumber()).to.equal(0)
        await expect((await psp22.query.balanceOf(bob.address)).value.unwrap().toString()).to.equal(ONE.muln(100).toString())
    })

    it('deposit & transfer to bob works & bob withdraw wroks', async () => {
        await buildTx(api.registry, api.tx.assets.mint(ASSET_ID, {id: alice.address}, ONE.muln(1000)), alice)
        await buildTx(api.registry, api.tx.assets.approveTransfer(ASSET_ID, psp22.address, ONE.muln(100)), alice)

        await psp22.tx.deposit(ONE.muln(100));

        await psp22.tx.transfer(bob.address, ONE.muln(100), ['']);

        await psp22.withSigner(bob).tx.withdraw(ONE.muln(100));

        // @ts-ignore
        await expect((await api.query.assets.account(ASSET_ID, psp22.address)).unwrapOrDefault().balance.toNumber()).to.equal(0)
        // @ts-ignore
        await expect((await api.query.assets.account(ASSET_ID, alice.address)).unwrapOrDefault().balance.toString()).to.equal(ONE.muln(900).toString())
        // @ts-ignore
        await expect((await api.query.assets.account(ASSET_ID, bob.address)).unwrapOrDefault().balance.toString()).to.equal(ONE.muln(100).toString())
        await expect((await psp22.query.balanceOf(alice.address)).value.unwrap().toNumber()).to.equal(0)
        await expect((await psp22.query.balanceOf(bob.address)).value.unwrap().toNumber()).to.equal(0)
    })
})
