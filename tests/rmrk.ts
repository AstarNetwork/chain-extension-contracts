import { setupContract } from './helper'
import { network } from 'redspot'
import { expect } from "./setup/chai";
import { Option } from '@polkadot/types';
import { buildTx } from '@redspot/patract/buildTx'
import { AccountLedger } from "./types";
// import BN from "bn.js";
import { BigNumber } from '@redspot/patract/types';
import Contract from '@redspot/patract/contract';
import { Signer } from 'redspot/provider';
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

        const collectionId = await createCollection(contract, alice, "mintingCollectionMetadata", null, 'MCM', one.muln(100))
        console.log("created collectionId:", collectionId);
    });

    it("mint NFT", async () => {
        const { contract, one, alice, bob } = await setup();

        const royalty = 70000;
        const nftMetadata = "recipient-royalty-NFT-test-metadata";

        const collectionId = await createCollection(contract, alice, "mintingCollectionMetadata", null, 'MCM', one.muln(100))
        console.log("created collectionId:", collectionId);

        try {
            await mintNft(
                contract,
                alice.address,
                23,
                collectionId,
                null,
                null,
                nftMetadata,
                true,
                null
            );
        } catch (e) {
            console.error(e);
        }
    });

    it("[Error test] mint already existing NFT", async () => {
        const { contract, one, alice, bob } = await setup();

        const nftId = 42;
        const nftMetadata = "recipient-royalty-NFT-test-metadata";

        const collectionId = await createCollection(contract, alice, "mintingCollectionMetadata", null, 'MCM', one.muln(100))
        console.log("created collectionId:", collectionId);

        await mintNft(
            contract,
            alice.address,
            nftId,
            collectionId,
            null,
            null,
            nftMetadata,
            true,
            null
        );

        // now try to mint the same nft but expect error NftAlreadyExists
        await expect(contract.connect(alice).query.mintNft(
            alice.address,
            nftId,
            collectionId,
            null,
            null,
            nftMetadata,
            true,
            null
        )
        ).to.output({ Err: 'NftAlreadyExists' })

    });

    const createCollection = async (contract, owner, collectionMetadata, collectionMax, collectionSymbol, reserveValue) => {

        const currentIndex = await api.query.rmrkCore.collectionIndex();
        await contract.connect(owner).tx.createCollection(collectionMetadata, collectionMax, collectionSymbol, { value: reserveValue });
        const afterIndex = await api.query.rmrkCore.collectionIndex();
        expect(Number(afterIndex.toString())).is.equal(Number(currentIndex.toString()) + 1)
        return Number(currentIndex.toString());
    }

    const mintNft = async (
        contract: Contract,
        owner,
        nftId: number,
        collectionId: number,
        royaltyRecipient,
        royalty: number | null = null,
        nftMetadata: string,
        transferable: boolean = true,
        resources:
            | { resource: { basic?: any; composable?: any; slot?: any }; id: number }[]
            | null = null
    ) => {

        const royaltyOptional = royalty ? royalty.toString() : null;
        const royaltyRecepientOptional = royalty ? royaltyRecipient.toString() : null;
        try {
            await contract.tx.mintNft(
                owner,
                nftId,
                collectionId,
                royaltyRecepientOptional,
                royaltyOptional,
                nftMetadata,
                transferable,
                resources
            );
        } catch (e) {
            console.error(e);
        }
        const mintedNft = await api.query.rmrkCore.nfts(collectionId, nftId);
        // @ts-ignore
        expect(mintedNft.isSome).to.be.true;

        console.log("Minted (collectionId:", collectionId, ", nftId:", nftId)
    }

})
