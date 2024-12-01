#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;
use alloy_sol_types::sol;

use stylus_sdk::{
    alloy_primitives::{Address, U256},
    block,
    call::Call,
    contract, evm, msg,
    prelude::*,
};

sol_interface! {
    interface IERC20 {
    function transfer(address, uint256) external returns (bool);
    function transferFrom(address, address, uint256) external returns (bool);
    }
}

sol! {
    error StartTimePassed();
    error NotStarted();
    error NotEnded();
    error AlreadyEnded();
    error StartGreaterThanEnd();
    error EndGreaterThanMaxDuration();
    error SenderIsNotCreator();
    error AlreadyClaimed();
    error GoalNotReached();
    error GoalReached();
    
    event Launch(
        uint256 id,
        address indexed creator,
        uint256 goal,
        uint256 start_at,
        uint256 end_at
    );
    event Cancel(uint256 id);
    event Pledge(uint256 indexed id, address indexed caller, uint256 amount);
    event Unpledge(uint256 indexed id, address indexed caller, uint256 amount);
    event Claim(uint256 id);
    event Refund(uint256 id, address indexed caller, uint256 amount);
}

#[derive(SolidityError)]
pub enum CrowdFundErrors {
    StartTimePassed(StartTimePassed),
    NotStarted(NotStarted),
    NotEnded(NotEnded),
    AlreadyEnded(AlreadyEnded),
    StartGreaterThanEnd(StartGreaterThanEnd),
    EndGreaterThanMaxDuration(EndGreaterThanMaxDuration),
    SenderIsNotCreator(SenderIsNotCreator),
    GoalNotReached(GoalNotReached),
    GoalReached(GoalReached),
    AlreadyClaimed(AlreadyClaimed)

}

sol_storage! {
    #[entrypoint]
    pub struct CrowdFund{
    // Total count of campaigns created.
    // It is also used to generate id for new campaigns.
    uint256 count;
    // The address of the NFT contract.
    address token_address;
    // Mapping from id to Campaign
    CampaignStruct[] campaigns; // The transactions array
    // Mapping from campaign id => pledger => amount pledged
    mapping(uint256 => mapping(address => uint256)) pledged_amount;
    }
    pub struct CampaignStruct {
        // Creator of campaign
        address creator;
        // Amount of tokens to raise
        uint256 goal;
        // Total amount pledged
        uint256 pledged;
        // Timestamp of start of campaign
        uint256 start_at;
        // Timestamp of end of campaign
        uint256 end_at;
        // True if goal was reached and creator has claimed the tokens.
        bool claimed;
    }

}

/// Declare that `CrowdFund` is a contract with the following external methods.
#[public]
impl CrowdFund {
    pub const ONE_DAY: u64 = 86400; // 1 day = 24 hours * 60 minutes * 60 seconds = 86400 seconds.

    pub fn launch(&mut self, goal: U256, start_at: U256, end_at: U256) -> Result<(), CrowdFundErrors> {
        if start_at < U256::from(block::timestamp()) {
            return Err(CrowdFundErrors::StartTimePassed(StartTimePassed {}));
        }
        if end_at < start_at{
            return Err(CrowdFundErrors::StartGreaterThanEnd(StartGreaterThanEnd {}));
        }
        if end_at > U256::from(block::timestamp() + 7 * Self::ONE_DAY) {
            return Err(CrowdFundErrors::EndGreaterThanMaxDuration(EndGreaterThanMaxDuration {}));
        }
        let number = self.count.get();
        self.count.set(number + U256::from(1));

        let mut new_campaign = self.campaigns.grow();
        new_campaign.creator.set(msg::sender());
        new_campaign.goal.set(goal);
        new_campaign.pledged.set(U256::from(0));
        new_campaign.start_at.set(start_at);
        new_campaign.end_at.set(end_at);
        new_campaign.claimed.set(false);
        let number = U256::from(self.campaigns.len());
        evm::log(Launch {
            id: number - U256::from(1),
            creator: msg::sender(),
            goal: goal,
            start_at: start_at,
            end_at: end_at,
        });
        Ok(())
    }
    pub fn cancel(&mut self, id: U256) -> Result<(), CrowdFundErrors> {
        if let Some(mut entry) = self.campaigns.get_mut(id) {
            if entry.creator.get() != msg::sender() {
                return Err(CrowdFundErrors::SenderIsNotCreator(SenderIsNotCreator {}));
            }
            if U256::from(block::timestamp()) < entry.start_at.get() {
                return Err(CrowdFundErrors::NotStarted(NotStarted {}));
            }
            entry.creator.set(Address::ZERO);
            entry.goal.set(U256::from(0));
            entry.pledged.set(U256::from(0));
            entry.start_at.set(U256::from(0));
            entry.end_at.set(U256::from(0));
            entry.claimed.set(false);
            evm::log(Cancel { id: id });
        }
        Ok(())
    }
    pub fn pledge(&mut self, id: U256, amount: U256) -> Result<(), CrowdFundErrors> {
        if let Some(mut entry) = self.campaigns.get_mut(id) {
            if U256::from(block::timestamp()) < entry.start_at.get() {
                return Err(CrowdFundErrors::NotStarted(NotStarted {}));
            }
            if U256::from(block::timestamp()) > entry.end_at.get() {
                return Err(CrowdFundErrors::AlreadyEnded(AlreadyEnded {}));
            }
            let pledged = U256::from(entry.pledged.get());
            entry.pledged.set(pledged + amount);
            let mut pledged_amount_info = self.pledged_amount.setter(id);
            let mut pledged_amount_sender = pledged_amount_info.setter(msg::sender());
            let old_amount = pledged_amount_sender.get();
            pledged_amount_sender.set(old_amount + amount);

            let token = IERC20::new(*self.token_address);
            let config = Call::new_in(self);
            token.transfer(config, contract::address(), amount).unwrap();
        }
        Ok(())
    }
    pub fn unpledge(&mut self, id: U256, amount: U256) -> Result<(), CrowdFundErrors> {
        if let Some(mut entry) = self.campaigns.get_mut(id) {
            if U256::from(block::timestamp()) > entry.end_at.get() {
                return Err(CrowdFundErrors::AlreadyEnded(AlreadyEnded {}));
            }
            let pledged = U256::from(entry.pledged.get());
            entry.pledged.set(pledged - amount);
            let mut pledged_amount_info = self.pledged_amount.setter(id);
            let mut pledged_amount_sender = pledged_amount_info.setter(msg::sender());
            let old_amount = pledged_amount_sender.get();
            pledged_amount_sender.set(old_amount - amount);
            // Token transfer
            let token = IERC20::new(*self.token_address);
            let config = Call::new_in(self);
            token.transfer(config, contract::address(), amount).unwrap();
            // Emit the log
            evm::log(Unpledge {
                id: id,
                caller: msg::sender(),
                amount: amount,
            });
        }
        Ok(())
    }
    pub fn claim(&mut self, id: U256) -> Result<(), CrowdFundErrors> {
        // First mutable borrow to access campaigns and the entry
        if let Some(mut entry) = self.campaigns.get_mut(id) {
            let creator = entry.creator.get();
            let end_at = entry.end_at.get();
            let pledged = entry.pledged.get();
            let goal = entry.goal.get();
            let claimed = entry.claimed.get();

            // Check conditions on the entry
            if creator != msg::sender() {
                return Err(CrowdFundErrors::SenderIsNotCreator(SenderIsNotCreator {}));
            }
            if U256::from(block::timestamp()) < end_at {
                return Err(CrowdFundErrors::NotEnded(NotEnded {}));
            }
            if pledged < goal {
                return Err(CrowdFundErrors::GoalNotReached(GoalNotReached {}));

            }
            if claimed {
                return Err(CrowdFundErrors::AlreadyClaimed(AlreadyClaimed {}));
            }
            // Mark the entry as claimed
            entry.claimed.set(true);

            // Now, perform the token transfer
            let token_address = *self.token_address;
            let token = IERC20::new(token_address);

            let config = Call::new_in(self);
            token.transfer(config, creator, pledged).unwrap();
            evm::log(Claim { id: id });
        }
        Ok(())
    }

    pub fn refund(&mut self, id: U256) -> Result<(), CrowdFundErrors> {
        // First mutable borrow to access campaigns and the entry
        if let Some(entry) = self.campaigns.get_mut(id) {
            let end_at = entry.end_at.get();
            let goal = entry.goal.get();
            let pledged = entry.pledged.get();

            if U256::from(block::timestamp()) <= end_at {
                return Err(CrowdFundErrors::NotEnded(NotEnded {}));
            } 
            if pledged >= goal {
                return Err(CrowdFundErrors::GoalReached(GoalReached {}));
            }
            let mut pledged_amount_info = self.pledged_amount.setter(id);
            let mut pledged_amount_sender = pledged_amount_info.setter(msg::sender());
            let old_balance = pledged_amount_sender.get();
            pledged_amount_sender.set(U256::from(0));
            let token_address = *self.token_address;
            let token = IERC20::new(token_address);

            let config = Call::new_in(self);
            token.transfer(config, msg::sender(), old_balance).unwrap();
            evm::log(Refund {
                id: id,
                caller: msg::sender(),
                amount: old_balance,
            });
        }
        Ok(())
    }
}
