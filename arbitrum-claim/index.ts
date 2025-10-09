import Web3 from "web3";
import EventEmitter from "events";
import config from "./config.json";

const {
    rpcs,
    privateKeys,
    cexAddr
} = config;

const contractAddr = "0x67a24CE4321aB3aF51c2D0a4801c3E111D88C9d9";
const abi = [{
    "inputs": [],
    "name": "claim",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
}] as any;

const multicallAbi = [{
    "inputs": [],
    "name": "getL1BlockNumber",
    "outputs": [
        {
            "internalType": "uint256",
            "name": "l1BlockNumber",
            "type": "uint256"
        }
    ],
    "stateMutability": "view",
    "type": "function"
}] as any;

const erc20abi = [{
    "constant": false,
    "inputs": [
        {
            "name": "_to",
            "type": "address"
        },
        {
            "name": "_value",
            "type": "uint256"
        }
    ],
    "name": "transfer",
    "outputs": [
        {
            "name": "",
            "type": "bool"
        }
    ],
    "payable": false,
    "stateMutability": "nonpayable",
    "type": "function"
},
{
    "constant": true,
    "inputs": [
        {
            "name": "_owner",
            "type": "address"
        }
    ],
    "name": "balanceOf",
    "outputs": [
        {
            "name": "balance",
            "type": "uint256"
        }
    ],
    "payable": false,
    "stateMutability": "view",
    "type": "function"
}] as any;
const arbAddr = "0x912CE59144191C1204E64559FE8253a0e49E6548";
const multicallAddr = "0x842eC2c7D803033Edf55E478F461FC547Bc54EB2";

const target = 16890400;

const claim = async (pk: string, rpc: string) => {
    try {
        let web3 = new Web3(rpc);
        let claimContract = new web3.eth.Contract(abi, contractAddr);
        let addr = web3.eth.accounts.privateKeyToAccount(pk).address;
        let signedTx = await web3.eth.accounts.signTransaction({
            to: contractAddr,
            data: claimContract.methods.claim().encodeABI(),
            from: addr,
            gas: 1000000
        }, pk);

        let result = (await web3.eth.sendSignedTransaction(signedTx.rawTransaction!)).status;
        if (!result)
            console.log(`Failed to claim: Addr ${addr}`);
        else
            console.log(`Claimed: Addr ${addr}`);

        return result;
    } catch (e: any) {
        console.log(e?.message);
        return false;
    }
}

const sendToCex = async (pk: string, rpc: string) => {
    try {
        let web3 = new Web3(rpc);
        let arbContract = new web3.eth.Contract(erc20abi, arbAddr);
        let addr = web3.eth.accounts.privateKeyToAccount(pk).address;
        let claimedAmount = await arbContract.methods.balanceOf(addr).call();
        if (claimedAmount === 0) {
            console.log("ClaimedAmount 0");
            return false;
        }

        let signedSendToCexTx = await web3.eth.accounts.signTransaction({
            to: arbAddr,
            data: arbContract.methods.transfer(cexAddr, claimedAmount).encodeABI(),
            from: addr,
            gas: 1000000
        }, pk);
        let result = (await web3.eth.sendSignedTransaction(signedSendToCexTx.rawTransaction!)).status;
        console.log(`Sent to CEX: Addr ${addr} Amount ${claimedAmount}`);
        return result;
    }
    catch (e: any) {
        console.log(e);
        return false;
    }
}

(async () => {
    await Promise.any(
        rpcs.map(async rpc => {
            let web3 = new Web3(rpc);
            let contract = new web3.eth.Contract(multicallAbi, multicallAddr);

            let l1Height = 0;
            while (l1Height < target) {
                await new Promise(res => setTimeout(res, 200));
                l1Height = await contract.methods.getL1BlockNumber().call();
                console.log(`RPC ${rpc} height ${l1Height}`);
            }

            return Promise.resolve();
        })
    );

    await Promise.allSettled(
        privateKeys.map(async pk => {
            let result = false;
            while (!result) {
                for (const rpc of rpcs) {
                    console.log(`Processing pk ${pk.slice(0, 10)} on rpc ${rpc}`)
                    if (await claim(pk, rpc))
                        result = await sendToCex(pk, rpc)
                };

                await new Promise(res => setTimeout(res, 300))
            }
        })
    )
})();