// SPDX-License-Identifier: GPL-3.0-or-later
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

contract;

abi WeightedPoolFactory {
    fn create(
        // string memory name,
        // string memory symbol,
        tokens: Vec<ContractId>,
        weights: Vec<u256>,
        assetManagers: Vec<Address>,
        swapFeePercentage: u256,
        owner: Address
    ) -> Address ; 
}

// import "@balancer-labs/v2-interfaces/contracts/vault/IVault.sol";

// import "@balancer-labs/v2-pool-utils/contracts/factories/BasePoolSplitCodeFactory.sol";
// import "@balancer-labs/v2-pool-utils/contracts/factories/FactoryWidePauseWindow.sol";

// import "./WeightedPool.sol"; 
// use WeightedPool::*;

// contract WeightedPoolFactory is BasePoolSplitCodeFactory, FactoryWidePauseWindow {
impl WeightedPoolFactory for Contract {

    // constructor(IVault vault) BasePoolSplitCodeFactory(vault, type(WeightedPool).creationCode) {
    //     // solhint-disable-previous-line no-empty-blocks
    // }

    /**
     * @dev Deploys a new `WeightedPool`.
     */
    fn create(
        // string memory name,
        // string memory symbol,
        tokens: Vec<ContractId>,
        weights: Vec<u256>,
        assetManagers: Vec<Address>,
        swapFeePercentage: u256,
        owner: Address
    ) -> Address {

    let pausedWindowData: (u256, u256) = getPauseConfiguration();

        return
            _create(
                abi.encode(
                    getVault(),
                    // name,
                    // symbol,
                    tokens,
                    weights,
                    assetManagers,
                    swapFeePercentage,
                    pausedWindowData.0,
                    pausedWindowData.1,
                    owner
                )
            );
    }
}
