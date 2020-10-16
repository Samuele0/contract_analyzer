pragma solidity >=0.4.22 <0.7.0;

contract ArrayTest {

     uint[] values;
    
    
    function fn1(uint i, uint value) public {
       values[i]=value;
    }

    function retrieve(uint i) public view returns (uint256){
        return values[i];
    }
}