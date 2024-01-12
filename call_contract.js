const { ethers } = require('ethers');

// Set up the provider (e.g., Infura)
const provider = new ethers.JsonRpcProvider('http://localhost:8547');

// Your Ethereum wallet's private key
const privateKey = 'TODO';

// Set up a wallet
const wallet = new ethers.Wallet(privateKey, provider);

// Contract ABI and address
const counterAbi = [
    "function getPrompt() external view returns (string memory)",
    "function setPrompt(string calldata new_prompt) external",
    "function getAnswer() external view returns (string memory)",
    "function setAnswer(string calldata new_answer) external"
];
const counterAddress = '0xa8572fa8e3eec9643a5c4a81af638d0d5999077c';

// Create a contract instance
const counter = new ethers.Contract(counterAddress, counterAbi, wallet);

async function getPrompt() {
    const prompt = await counter.getPrompt();
    console.log("Current prompt:", prompt)
}

async function setPrompt(prompt) {
    const tx = await counter.setPrompt(prompt);
    await tx.wait();
    console.log("Prompt set to:", prompt)
}

async function getAnswer() {
    const answer = await counter.getAnswer();
    console.log("Current answer:", answer)
}

async function setAnswer(answer) {
    const tx = await counter.setAnswer(answer);
    await tx.wait();
    console.log("Answer set to:", answer)
}

async function main() {
    console.log("main")
    await getPrompt()
    await setPrompt("hello")
    await getPrompt()
    console.log("")
    await getAnswer()
    await setAnswer("world")
    await getAnswer()
}

main().catch(error => {
    console.error(error);
    process.exit(1);
});
