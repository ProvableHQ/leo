/*
namespace: Execute
expectation: Pass
cases:
    - program: metadata.aleo
      function: is_block_height
      input: ["2u32"]
    - program: metadata.aleo
      function: is_block_height
      input: ["3u32"]
    - program: metadata.aleo
      function: is_block_height
      input: ["4u32"]
    - program: metadata.aleo
      function: is_block_height
      input: ["0u32"]
    - program: metadata.aleo
      function: is_network_id
      input: ["0u16"]
    - program: metadata.aleo
      function: is_network_id
      input: ["1u16"]
    - program: metadata.aleo
      function: is_network_id
      input: ["2u16"]
    - program: metadata.aleo
      function: is_network_id
      input: ["3u16"]
*/


program metadata.aleo {

    async transition is_block_height(block_height: u32) -> Future {
        return finalize_is_block_height(block_height);
    }

    async function finalize_is_block_height(block_height: u32) {
        assert_eq(block_height, block.height);
    }

    async transition is_network_id(network_id: u16) -> Future {
        return finalize_is_network_id(network_id);
    }

    async function finalize_is_network_id(network_id: u16) {
        assert_eq(network_id, network.id);
    }
}
