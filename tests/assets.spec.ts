import { expect } from 'chai';
import chaiAsPromised from 'chai-as-promised';
import BN from 'bn.js';
import { ApiPromise, WsProvider, Keyring } from '@polkadot/api';
import {KeyringPair} from "@polkadot/keyring/types";
import {assetsPalletTx, contractCall, deployContract} from "./helper";
import {WeightV2} from "@polkadot/types/interfaces";
import {stringToHex} from "@polkadot/util/string/toHex";
import {ContractPromise} from "@polkadot/api-contract/promise";
import {readFile} from "node:fs/promises";

const ONE = new BN(10).pow(new BN(18));

const ASSET_ID = 1;

describe('ASSETS EXAMPLE', () => {
    let api: ApiPromise;
    let alice: KeyringPair;
    let bob: KeyringPair;
    let charlie: KeyringPair;
    let assetsContract: ContractPromise;
    let gasLimit: WeightV2;

    beforeEach(async function() {
        // Create a new instance of contract
        const wsProvider = new WsProvider('ws://127.0.0.1:9944');
        // Create a keyring instance
        const keyring = new Keyring({ type: 'sr25519' });

        api = await ApiPromise.create({ provider: wsProvider, noInitWarn: true });
        // @ts-ignore
        alice = keyring.addFromUri('//Alice');
        // @ts-ignore
        bob = keyring.addFromUri('//Bob');
        // @ts-ignore
        charlie = keyring.addFromUri('//Charlie');
        gasLimit = api.registry.createType(
            'WeightV2',
            {
                refTime: 100_000_000_000,
                proofSize: 600_000,
            });

        const compiledContractFile = await readFile("./target/ink/assets_example/assets_example.contract");
        const compiledContract = JSON.parse(compiledContractFile.toString("utf8"));
        assetsContract = await deployContract(api, alice, compiledContract);
        console.log("Assets Contract deployed to:", assetsContract.address.toString());
    })

    afterEach(async function() {
        await assetsPalletTx(api, alice, 'startDestroy', [ASSET_ID]);
        await assetsPalletTx(api, alice, 'destroyAccounts', [ASSET_ID]);
        await assetsPalletTx(api, alice, 'destroyApprovals', [ASSET_ID]);
        await assetsPalletTx(api, alice, 'destroyAccounts', [ASSET_ID]);
        await assetsPalletTx(api, alice, 'destroyApprovals', [ASSET_ID]);
        await assetsPalletTx(api, alice, 'finishDestroy', [ASSET_ID]);
    })

    it('mint transfer burn works', async () => {
        // Arrange Create an asset and give to contract mint permission (Issuer role) and burn permission (Admin role)
        await assetsPalletTx(api, alice, 'create', [ASSET_ID, {id: alice.address}, 1]);
        await assetsPalletTx(api, alice, 'setTeam', [ASSET_ID, {id: assetsContract.address}, {id: assetsContract.address}, {id: assetsContract.address}]);

        // Act - mint to mint 1000 to contract
        await contractCall(api, assetsContract, 'mint', [ASSET_ID, assetsContract.address, 1000], alice);

        // Assert - check balance of contract
        expect((await assetsContract.query['balanceOf'](alice.address, {
            gasLimit,
            storageDepositLimit: null
            // @ts-ignore
        }, ASSET_ID, assetsContract.address)).output?.toHuman()?.Ok.replace(/,/g, '')).to.equal(new BN('1000').toString());

        // Act - transfer 100 to alice
        await contractCall(api, assetsContract, 'transfer', [ASSET_ID, alice.address, 100], alice);

        // Assert - check balance of alice
        expect((await assetsContract.query['balanceOf'](alice.address, {
            gasLimit,
            storageDepositLimit: null
            // @ts-ignore
        }, ASSET_ID, alice.address)).output?.toHuman()?.Ok.replace(/,/g, '')).to.equal(new BN('100').toString());

        // Act - burn 100 from contract
        await contractCall(api, assetsContract, 'burn', [ASSET_ID, assetsContract.address, 50], alice);

        // Assert - check balance of contract his 850 (-100 to alice and -50 burn)
        expect((await assetsContract.query['balanceOf'](alice.address, {
            gasLimit,
            storageDepositLimit: null
            // @ts-ignore
        }, ASSET_ID, assetsContract.address)).output?.toHuman()?.Ok.replace(/,/g, '')).to.equal(new BN('850').toString());
    })
})
