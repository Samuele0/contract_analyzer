pragma solidity >=0.4.22 <0.7.0;

contract HashMapTest {
    uint padding;
    mapping(uint=>uint) map;
    
    
    function fn1(uint i, uint value) public {
        padding=padding+5;
       map[i]=value;
    }

    function retrieve(uint i) public view returns (uint256){
        return map[i];
    }
}