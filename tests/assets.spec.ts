import { expect, use } from 'chai';
import chaiAsPromised from 'chai-as-promised';
import assets_constructor from '../types/constructors/asset_wrapper';
import BN from 'bn.js';
import assets_contract from '../types/contracts/asset_wrapper';
import { ApiPromise, WsProvider, Keyring } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import {buildTx} from "./helper";
import {afterEach} from "mocha";
import {WeightV2} from "@polkadot/types/interfaces";
import {stringToHex} from "@polkadot/util/string/toHex";

use(chaiAsPromised);

const ONE = new BN(10).pow(new BN(18));

// Create a new instance of contract
const wsProvider = new WsProvider('ws://127.0.0.1:9944');
// Create a keyring instance
const keyring = new Keyring({ type: 'sr25519' });

const ASSET_ID = 1

describe('ASSETS', () => {
    let api: ApiPromise;
    let alice: KeyringPair;
    let bob: KeyringPair;
    let charlie: KeyringPair;
    let assetsConstructor: assets_constructor
    let assets: assets_contract;

    beforeEach(async function() {
        api = await ApiPromise.create({ provider: wsProvider, noInitWarn: true });
        alice = keyring.addFromUri('//Alice');
        bob = keyring.addFromUri('//Bob');
        charlie = keyring.addFromUri('//Charlie');
        assetsConstructor = new assets_constructor(api, alice);
        assets = new assets_contract((await assetsConstructor.new()).address, alice, api);
    })

    afterEach(async function() {
        await assets.tx.transferOwnership(ASSET_ID, alice.address);
        await buildTx(api.registry, api.tx.assets.startDestroy(ASSET_ID), alice)
        await buildTx(api.registry, api.tx.assets.destroyAccounts(ASSET_ID), alice)
        await buildTx(api.registry, api.tx.assets.destroyApprovals(ASSET_ID), alice)
        await buildTx(api.registry, api.tx.assets.destroyAccounts(ASSET_ID), alice)
        await buildTx(api.registry, api.tx.assets.destroyApprovals(ASSET_ID), alice)
        await buildTx(api.registry, api.tx.assets.finishDestroy(ASSET_ID), alice)
    })

    it('create works', async () => {
        await assets.tx.create(ASSET_ID, 1,{ value: ONE });

        const prefix = api.consts.system.ss58Prefix
        // @ts-ignore
        const assetAddress = keyring.encodeAddress(assets.address, prefix);
        // @ts-ignore
        await expect((await api.query.assets.asset(1)).unwrapOrDefault().owner.toString()).to.equal(assetAddress)
    })

    it('mint works', async () => {
        await assets.tx.create(ASSET_ID, 1,{ value: ONE });

        await assets.tx.mint(ASSET_ID,  alice.address,1000);

        // @ts-ignore
        await expect((await api.query.assets.account(ASSET_ID, alice.address)).unwrapOrDefault().balance.toNumber()).to.equal(1000)
    })

    it('burn works', async () => {
        await assets.tx.create(ASSET_ID, 1,{ value: ONE });

        await assets.tx.mint(ASSET_ID, alice.address, 1000);

        await assets.tx.burn(ASSET_ID,  alice.address,100,);

        // @ts-ignore
        await expect((await api.query.assets.account(ASSET_ID, alice.address)).unwrapOrDefault().balance.toNumber()).to.equal(1000 - 100)
    })

    it('balance_of and total_supply are correct', async () => {
        await assets.tx.create(ASSET_ID, 1,{ value: ONE });

        await assets.tx.mint(ASSET_ID, alice.address,1000);

        // @ts-ignore
        await expect((await assets.query.balanceOf(ASSET_ID, alice.address)).value.unwrap().toNumber()).to.equal(1000)

        // @ts-ignore
        await expect((await assets.query.totalSupply(ASSET_ID)).value.unwrap().toNumber()).to.equal(1000)
    })

    it('approve transfer and check allowance', async () => {
        await assets.tx.create(ASSET_ID, 1,{ value: ONE.muln(10) });

        await assets.tx.mint(ASSET_ID, assets.address, ONE.muln(1000),);

        await assets.tx.approveTransfer(ASSET_ID, bob.address, ONE.muln(100));

        // @ts-ignore
         await expect((await assets.query.allowance(ASSET_ID, assets.address, bob.address)).value.unwrap().toString()).to.equal(ONE.muln(100).toString())
    })

    it('approve transfer, transfer and check balances', async () => {
        await buildTx(api.registry, api.tx.assets.create(ASSET_ID, {id: alice.address}, 1), alice)
        await buildTx(api.registry, api.tx.assets.mint(ASSET_ID, alice.address, ONE.muln(1000)), alice)
        await buildTx(api.registry, api.tx.assets.approveTransfer(ASSET_ID, assets.address, ONE.muln(100)), alice)

        await assets.tx.transferApproved(ASSET_ID, alice.address, bob.address, ONE.muln(50));

        // @ts-ignore
        await expect((await assets.query.allowance(ASSET_ID, alice.address, assets.address)).value.unwrap().toString()).to.equal(ONE.muln(50).toString())

        // @ts-ignore
        await expect((await assets.query.balanceOf(ASSET_ID, alice.address)).value.unwrap().toString()).to.equal(ONE.muln(950).toString())

        // @ts-ignore
        await expect((await assets.query.balanceOf(ASSET_ID, bob.address)).value.unwrap().toString()).to.equal(ONE.muln(50).toString())

        // @ts-ignore
        await expect((await assets.query.balanceOf(ASSET_ID, assets.address)).value.unwrap().toNumber()).to.equal(0)

        await buildTx(api.registry, api.tx.assets.transferOwnership(ASSET_ID, assets.address), alice)
    })

    it('cancel approval', async () => {
        await assets.tx.create(ASSET_ID, 1,{ value: ONE.muln(10) });

        await assets.tx.mint(ASSET_ID, assets.address, ONE.muln(1000));

        await assets.tx.approveTransfer(ASSET_ID, bob.address, ONE.muln(100));

        await assets.tx.cancelApproval(ASSET_ID, bob.address);

        // @ts-ignore
        await expect((await assets.withSigner(bob).query.transferApproved(ASSET_ID, alice.address, charlie.address, 100)).value.unwrap().err).to.equal('Unapproved')

        // @ts-ignore
        await expect((await assets.query.allowance(ASSET_ID, alice.address, bob.address)).value.unwrap().toNumber()).to.equal(0)

        // @ts-ignore
        await expect((await assets.query.balanceOf(ASSET_ID, alice.address)).value.unwrap().toNumber()).to.equal(0)

        // @ts-ignore
        await expect((await assets.query.balanceOf(ASSET_ID, bob.address)).value.unwrap().toNumber()).to.equal(0)

        // @ts-ignore
        await expect((await assets.query.balanceOf(ASSET_ID, charlie.address)).value.unwrap().toNumber()).to.equal(0)
    })

    it('set metadata and checks', async () => {
        await assets.tx.create(ASSET_ID, 1,{ value: ONE.muln(10) });

        await assets.tx.setMetadata(ASSET_ID, 'Shiden Token' as unknown as string[], 'TTT' as unknown as string[], 18);

        // @ts-ignore
        await expect((await assets.query.metadataName(1)).value.unwrap()).to.equal(stringToHex('Shiden Token'))

        // @ts-ignore
        await expect((await assets.query.metadataSymbol(1)).value.unwrap()).to.equal(stringToHex('TTT'))

        // @ts-ignore
        await expect((await assets.query.metadataDecimals(ASSET_ID, alice.address, bob.address)).value.unwrap()).to.equal(18)
    })

    it('transfer ownership', async () => {
        await assets.tx.create(ASSET_ID, 1,{ value: ONE.muln(10) });

        await assets.tx.transferOwnership(ASSET_ID, alice.address);


        const prefix = api.consts.system.ss58Prefix
        // @ts-ignore
        const aliceAddress = keyring.encodeAddress(alice.address, prefix);

        // @ts-ignore
        await expect((await api.query.assets.asset(ASSET_ID)).unwrapOrDefault().owner.toString()).to.equal(aliceAddress)

        // give back ownership to the contract
        await buildTx(api.registry, api.tx.assets.transferOwnership(ASSET_ID, {id: assets.address}), alice)
    })

    it('can not make tx on behalf of caller', async () => {
        await assets.tx.create(ASSET_ID, 1,{ value: ONE.muln(10) });

        // @ts-ignore
        await expect((await assets.query.createCaller(2,1, {value: ONE.muln(10)})).value.unwrap().err).to.equal('OriginCannotBeCaller')
    })
})
