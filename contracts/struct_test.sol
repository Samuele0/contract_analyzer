pragma solidity >=0.4.22 <0.7.0;

contract StructTest {

    struct Data{
        uint field1;
        uint field2;
    }

    Data data;
    
    function fn1(uint v1,uint v2) public {
       data.field1=v1;
       data.field2=v2;
    }

    function retrieve() public view returns (uint256){
        return data.field2;
    }
}