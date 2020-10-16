pragma solidity >=0.4.22 <0.7.0;

contract VecIterationTest {

    uint[] number;


    function store(uint start_val) public {
        for (uint i=0; i<number.length; i++){
            number[i]=start_val+i;
        }
    }

}