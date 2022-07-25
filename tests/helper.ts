import {artifacts, network, patract} from 'redspot'
import BN from "bn.js";
import {createSigner} from "redspot/provider";
import { Keyring } from '@polkadot/keyring'
import {buildTx} from "@redspot/patract/buildTx";
const {api} = network

const {getContractFactory, getRandomSigner} = patract
const {getSigners} = network

export const setupContract = async (name, constructor, ...args) => {
    await api.isReady
    const one = new BN(10).pow(new BN(api.registry.chainDecimals[0]))
    const signers = await getSigners()
    const signer = await getRandomSigner(signers[0], one.muln(100000))
    // @ts-ignore
    const alice = createSigner(signer, new Keyring({ type: 'sr25519'}).addFromUri('//Alice'));
    const defaultSigner = await getRandomSigner(signers[0], one.muln(100000))

    const contractFactory = await getContractFactory(name, defaultSigner)
    const contract = await contractFactory.deploy(constructor, ...args)
    const abi = artifacts.readArtifact(name)

    return {
        defaultSigner,
        alice,
        accounts: [alice, await getRandomSigner(), await getRandomSigner()],
        contractFactory,
        contract,
        abi,
        one,
        query: contract.query,
        tx: contract.tx
    }
}

export const forceEras = async (eras, caller) => {
    const forceNewEra = api.tx.sudo.sudo(api.tx.dappsStaking.forceNewEra())
    for (let i = 0; i < eras; i++) {
        await buildTx(api.registry, forceNewEra, caller.address)
    }
}