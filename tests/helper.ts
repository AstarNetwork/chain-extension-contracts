import {KeyringPair} from "@polkadot/keyring/types";
import {ApiPromise} from "@polkadot/api";
import {Abi, CodePromise, ContractPromise} from "@polkadot/api-contract";
import {WeightV2} from "@polkadot/types/interfaces";

export async function assetsPalletTx(api: ApiPromise, alice: KeyringPair, tx: any, params: any[]) {
    let finish = false;
    const unsub = await api.tx.assets[tx](...params)
        .signAndSend(alice, {nonce: -1}, ({status}) => {
            if (status.isFinalized) {
                finish = true;
                unsub();
            }
        });
    while (!finish) {
        await waitFor(1);
    }
}

export async function deployContract(api: ApiPromise, deployer: KeyringPair, contractRaw: string) {
    const contractAbi = new Abi(contractRaw);
    const estimatedGas = api.registry.createType(
        'WeightV2',
        {
            refTime: 70_000_000_000,
            proofSize: 100_000,
        });

    const code = new CodePromise(api, contractAbi, contractAbi.info.source.wasm);
    // @ts-ignore
    const tx = code.tx.new({gasLimit: estimatedGas})
    let finish = false;
    let promise: ContractPromise;
    // @ts-ignore
    const unsub = await tx.signAndSend(deployer, {nonce: -1}, ({contract, status,}) => {
        if (status.isFinalized) {
            let address = contract.address.toString();
            promise = new ContractPromise(api, contractAbi, address);
            finish = true;
            unsub();
        }
    });

    while (!finish) {
        await waitFor(1);
    }
    // @ts-ignore
    return promise;
}

export async function contractCall(api: ApiPromise, contract: ContractPromise, tx: any, params: any[], signer: any) {
    const gasLimit: WeightV2 = api.registry.createType(
        'WeightV2',
        {
            refTime: 100_000_000_000,
            proofSize: 600_000,
        });

    // Dry run to get logs
    const {result} = await contract.query[tx](
        signer.address,
        {
            gasLimit,
            storageDepositLimit: null
        },
        ...params
    )
    console.log("CONTRACT TX RESULT", result.toHuman())

    let finish = false;
    const unsub = await contract.tx[tx](
        {
            gasLimit: gasLimit,
            storageDepositLimit: null,
        },
        ...params
    )
        .signAndSend(signer, (res: any) => {
            if (res.status.isFinalized) {
                finish = true;
                unsub()
            }
        })

    while (!finish) {
        await waitFor(1);
    }
}

export async function waitFor(ms: any) {
    return new Promise((resolve) => setTimeout(resolve, ms));
}