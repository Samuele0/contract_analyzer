pragma solidity >=0.4.22 <0.7.0;

import "1_Storage.sol";

contract Calltest {
    Storage st;
    
    constructor(Storage s) public {
        st=s;
    }
    function call() public{
        st.store(500);
    }
    function set_storage(Storage s) public {
        st=s;
    }
}