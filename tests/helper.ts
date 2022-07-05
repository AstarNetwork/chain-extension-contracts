import {artifacts, network, patract} from 'redspot'
import BN from "bn.js";

const {getContractFactory, getRandomSigner} = patract
const {getSigners} = network

export const setupContract = async (name, constructor, ...args) => {
    const one = new BN(10000000000000)
    const signers = await getSigners()
    const defaultSigner = await getRandomSigner(signers[0], one.muln(10))
    const alice = await getRandomSigner(signers[1], one.muln(10))

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