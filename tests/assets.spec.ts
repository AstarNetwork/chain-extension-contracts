import { expect, use } from 'chai';
import chaiAsPromised from 'chai-as-promised';
import assets_constructor from '../types/constructors/asset_wrapper_mock';
import assets_contract from '../types/contracts/asset_wrapper_mock';
import { ApiPromise, WsProvider, Keyring } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import {buildTx} from "./helper";
import {afterEach} from "mocha";
import {WeightV2} from "@polkadot/types/interfaces";
import {stringToHex} from "@polkadot/util/string/toHex";

use(chaiAsPromised);

// Create a new instance of contract
const wsProvider = new WsProvider('ws://127.0.0.1:9944');
// Create a keyring instance
const keyring = new Keyring({ type: 'sr25519' });

describe('ASSETS', () => {
    let api: ApiPromise;
    let alice: KeyringPair;
    let bob: KeyringPair;
    let charlie: KeyringPair;
    let assetsConstructor: assets_constructor
    let assets: assets_contract;

    let gasRequired: WeightV2;

    beforeEach(async function() {
        api = await ApiPromise.create({ provider: wsProvider });
        alice = keyring.addFromUri('//Alice');
        bob = keyring.addFromUri('//Bob');
        charlie = keyring.addFromUri('//Charlie');
        assetsConstructor = new assets_constructor(api, alice);
        assets = new assets_contract((await assetsConstructor.new()).address, alice, api);
    })

    afterEach( async function() {
        await buildTx(api.registry, api.tx.assets.destroy(1, {accounts: 10, sufficients: 10, approvals: 10}), alice)
    })

    it('create works', async () => {
        let { gasRequired: gas }  = await assets.query.create(1,1, {
            value: 1000000,
        });
        await assets.tx.create(1, 1,{gasLimit: gas, value: 1000000 });

        // @ts-ignore
        await expect((await api.query.assets.asset(1)).unwrapOrDefault().owner.toU8a().toString()).to.equal(alice.addressRaw.toString())
    })

    it('mint works', async () => {
        let { gasRequired: gas }  = await assets.query.create(1,1, {
            value: 1000000,
        });
        await assets.tx.create(1, 1,{gasLimit: gas, value: 1000000 });

        let { gasRequired: gas2 }  = await assets.query.mint(1,1000);
        await assets.tx.mint(1, 1000,{gasLimit: gas2 });

        // @ts-ignore
        await expect((await api.query.assets.account(1, alice.address)).unwrapOrDefault().balance.toNumber()).to.equal(1000)
    })

    it('burn works', async () => {
        let { gasRequired: gas }  = await assets.query.create(1,1, {
            value: 1000000,
        });
        await assets.tx.create(1, 1,{gasLimit: gas, value: 1000000 });

        let { gasRequired: gas2 }  = await assets.query.mint(1,1000);
        await assets.tx.mint(1, 1000,{gasLimit: gas2 });

        let { gasRequired: gas3 }  = await assets.query.burn(1,100);
        await assets.tx.burn(1, 100,{gasLimit: gas3 });

        // @ts-ignore
        await expect((await api.query.assets.account(1, alice.address)).unwrapOrDefault().balance.toNumber()).to.equal(1000 - 100)
    })

    it('balance_of and total_supply are correct', async () => {
        let { gasRequired: gas }  = await assets.query.create(1,1, {
            value: 1000000,
        });
        await assets.tx.create(1, 1,{gasLimit: gas, value: 1000000 });

        let { gasRequired: gas2 }  = await assets.query.mint(1,1000);
        await assets.tx.mint(1, 1000,{gasLimit: gas2 });

        // @ts-ignore
        await expect((await assets.query.balanceOf(1, alice.address)).value.toNumber()).to.equal(1000)

        // @ts-ignore
        await expect((await assets.query.totalSupply(1)).value.toNumber()).to.equal(1000)
    })

    it('approve transfer and check allowance', async () => {
        let { gasRequired: gas }  = await assets.query.create(1,1, {
            value: 1000000,
        });
        await assets.tx.create(1, 1,{gasLimit: gas, value: 1000000 });

        let { gasRequired: gas2 }  = await assets.query.mint(1,1000);
        await assets.tx.mint(1, 1000,{gasLimit: gas2 });

        let { gasRequired: gas3 }  = await assets.query.approveTransfer(1,bob.address, 100);
        await assets.tx.approveTransfer(1, bob.address, 100, {gasLimit: gas3 });

        // @ts-ignore
        await expect((await assets.query.allowance(1, alice.address, bob.address)).value.toNumber()).to.equal(100)
    })

    it('approve transfer and check allowance', async () => {
        let { gasRequired: gas }  = await assets.query.create(1,1, {
            value: 1000000,
        });
        await assets.tx.create(1, 1,{gasLimit: gas, value: 1000000 });

        let { gasRequired: gas2 }  = await assets.query.mint(1,1000);
        await assets.tx.mint(1, 1000,{gasLimit: gas2 });

        let { gasRequired: gas3 }  = await assets.query.approveTransfer(1,bob.address, 100);
        await assets.tx.approveTransfer(1, bob.address, 100, {gasLimit: gas3 });

        let { gasRequired: gas4}  = await assets.withSigner(bob).query.transferApproved(1, alice.address, charlie.address, 50);
        await assets.withSigner(bob).tx.transferApproved(1, alice.address, charlie.address, 50, {gasLimit: gas4});

        // @ts-ignore
        await expect((await assets.query.allowance(1, alice.address, bob.address)).value.toNumber()).to.equal(50)

        // @ts-ignore
        await expect((await assets.query.balanceOf(1, alice.address)).value.toNumber()).to.equal(950)

        // @ts-ignore
        await expect((await assets.query.balanceOf(1, bob.address)).value.toNumber()).to.equal(0)

        // @ts-ignore
        await expect((await assets.query.balanceOf(1, charlie.address)).value.toNumber()).to.equal(50)
    })

    it('cancel approval', async () => {
        let { gasRequired: gas }  = await assets.query.create(1,1, {
            value: 1000000,
        });
        await assets.tx.create(1, 1,{gasLimit: gas, value: 1000000 });

        let { gasRequired: gas2 }  = await assets.query.mint(1,1000);
        await assets.tx.mint(1, 1000,{gasLimit: gas2 });

        let { gasRequired: gas3 }  = await assets.query.approveTransfer(1,bob.address, 100);
        await assets.tx.approveTransfer(1, bob.address, 100, {gasLimit: gas3 });

        let { gasRequired: gas4 }  = await assets.query.cancelApproval(1,bob.address);
        await assets.tx.cancelApproval(1, bob.address,{gasLimit: gas4 });

        // @ts-ignore
        await expect((await assets.withSigner(bob).query.transferApproved(1, alice.address, charlie.address, 100)).value.err).to.equal('Unapproved')

        // @ts-ignore
        await expect((await assets.query.allowance(1, alice.address, bob.address)).value.toNumber()).to.equal(0)

        // @ts-ignore
        await expect((await assets.query.balanceOf(1, alice.address)).value.toNumber()).to.equal(1000)

        // @ts-ignore
        await expect((await assets.query.balanceOf(1, bob.address)).value.toNumber()).to.equal(0)

        // @ts-ignore
        await expect((await assets.query.balanceOf(1, charlie.address)).value.toNumber()).to.equal(0)
    })

    it('set metadata and checks', async () => {
        let { gasRequired: gas }  = await assets.query.create(1,1, {
            value: 1000000,
        });
        await assets.tx.create(1, 1,{gasLimit: gas, value: 1000000 });

        let { gasRequired: gas3 }  = await assets.query.setMetadata(1, 'Shiden Token' as unknown as string[], 'TTT' as unknown as string[], 18);
        await assets.tx.setMetadata(1, 'Shiden Token' as unknown as string[], 'TTT' as unknown as string[], 18, {gasLimit: gas3 });

        // @ts-ignore
        await expect((await assets.query.metadataName(1)).value).to.equal(stringToHex('Shiden Token'))

        // @ts-ignore
        await expect((await assets.query.metadataSymbol(1)).value).to.equal(stringToHex('TTT'))

        // @ts-ignore
        await expect((await assets.query.metadataDecimals(1, alice.address, bob.address)).value).to.equal(18)
    })
})
