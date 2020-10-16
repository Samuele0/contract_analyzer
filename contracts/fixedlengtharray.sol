pragma solidity >=0.4.22 <0.7.0;

contract StorageArray {

    uint[5] balance;

    function store(uint num) public {
        balance[3] = num;
    }


}