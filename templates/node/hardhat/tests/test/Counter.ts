import { expect } from "chai";
import { ethers } from "hardhat";

describe("Counter", function () {
  it("increments", async function () {
    const counter = await ethers.deployContract("Counter");
    await counter.increment();
    expect(await counter.value()).to.equal(1n);
  });
});
