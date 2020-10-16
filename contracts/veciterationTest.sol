pragma solidity >=0.4.22 <0.7.0;

contract VecIterationTest {
    struct Data{
        uint v1;
        uint v2;
    }
    Data[] vec;


    function store(uint ind) public {
        vec[ind].v2=vec[ind].v2+25;
    }

}