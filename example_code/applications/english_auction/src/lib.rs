// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use std::borrow::BorrowMut;

/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{alloy_primitives::{Address, U256}, block, call::{transfer_eth, Call}, contract, evm, msg, prelude::*};
use alloy_sol_types::sol;

// Import the IERC721 interface.
sol_interface! {
    interface IERC721 {
        // Required methods.
        function safeTransferFrom(address from, address to, uint256 token_id) external;
        function transferFrom(address, address, uint256) external;
    }
}

// Define the events and errors for the contract.
sol!{
    // Define the events for the contract.
    event Start(); // Start the auction.
    event Bid(address indexed sender, uint256 amount); // Bid on the auction.
    event Withdraw(address indexed bidder, uint256 amount); // Withdraw a bid.
    event End(address winner, uint256 amount); // End the auction.

    // Define the errors for the contract.
    error AlreadyInitialized(); // The contract has already been initialized.
    error AlreadyStarted(); // The auction has already started.
    error NotSeller(); // The sender is not the seller.
    error AuctionEnded(); // The auction has ended.
    error BidTooLow(); // The bid is too low.
    error NotStarted(); // The auction has not started.
    error NotEnded(); // The auction has not ended.
}


#[derive(SolidityError)]
pub enum EnglishAuctionError {
    // Define the errors for the contract.
    AlreadyInitialized(AlreadyInitialized),
    AlreadyStarted(AlreadyStarted),
    NotSeller(NotSeller),
    AuctionEnded(AuctionEnded),
    BidTooLow(BidTooLow),
    NotStarted(NotStarted),
    NotEnded(NotEnded),
}

// Define some persistent storage using the Solidity ABI.
// `Counter` will be the entrypoint.
sol_storage! {
    #[entrypoint]
    pub struct EnglishAuction {
        address nft_address; // The address of the NFT contract.
        uint256 nft_id; // The ID of the NFT.

        address seller; // The address of the seller.
        uint256 end_at; // The end time of the auction.
        bool started; // The auction has started or not.
        bool ended; // The auction has ended or not.

        address highest_bidder; // The address of the highest bidder.
        uint256 highest_bid; // The highest bid.
        mapping(address => uint256) bids; // The bids of the bidders.
    }
}

/// Declare that `Counter` is a contract with the following external methods.
#[public]
impl EnglishAuction {
    pub const ONE_DAY: u64 = 86400; // 1 day = 24 hours * 60 minutes * 60 seconds = 86400 seconds.
    
    // Get nft address
    pub fn nft(&self) -> Result<Address, EnglishAuctionError> {
        Ok(self.nft_address.get())
    }

    // Get nft id
    pub fn nft_id(&self) -> Result<U256, EnglishAuctionError> {
        Ok(self.nft_id.get())
    }
    // Get seller address
    pub fn seller(&self) -> Result<Address, EnglishAuctionError> {
        Ok(self.seller.get())
    }

    // Get end time
    pub fn end_at(&self) -> Result<U256, EnglishAuctionError> {
        Ok(self.end_at.get())
    }

    // Get started status
    pub fn started(&self) -> Result<bool, EnglishAuctionError> {
        Ok(self.started.get())
    }

    // Get ended status
    pub fn ended(&self) -> Result<bool, EnglishAuctionError> {
        Ok(self.ended.get())
    }

    // Get highest bidder address
    pub fn highest_bidder(&self) -> Result<Address, EnglishAuctionError> {
        Ok(self.highest_bidder.get())
    }

    // Get highest bid amount
    pub fn highest_bid(&self) -> Result<U256, EnglishAuctionError> {
        Ok(self.highest_bid.get())
    }

    // Get bid amount of a bidder
    pub fn bids(&self, bidder: Address) -> Result<U256, EnglishAuctionError> {
        Ok(self.bids.getter(bidder).get())
    }

    // Initialize program
    pub fn initialize(&mut self, nft: Address, nft_id: U256, starting_bid: U256) -> Result<(), EnglishAuctionError> {
        // Check if the contract has already been initialized.
        if self.seller.get() != Address::default() {
            // Return an error if the contract has already been initialized.
            return Err(EnglishAuctionError::AlreadyInitialized(AlreadyInitialized{}));
        }
        
        // Initialize the contract with the NFT address, the NFT ID, the seller, and the starting bid.
        self.nft_address.set(nft);
        self.nft_id.set(nft_id);
        self.seller.set(msg::sender());
        self.highest_bid.set(starting_bid);
        Ok(())
    }

    pub fn start(&mut self) -> Result<(), EnglishAuctionError> {
        // Check if the auction has already started.
        if self.started.get() {
            return Err(EnglishAuctionError::AlreadyStarted(AlreadyStarted{}));
        }
        
        // Check if the sender is the seller.
        if self.seller.get() != msg::sender() {
            // Return an error if the sender is the seller.
            return Err(EnglishAuctionError::NotSeller(NotSeller{}));
        }
        
        // Create a new instance of the IERC721 interface.
        let nft = IERC721::new(*self.nft_address);
        // Get the NFT ID.
        let nft_id = self.nft_id.get();

        // Transfer the NFT to the contract.
        let config = Call::new_in(self);
        let result = nft.transfer_from(config, msg::sender(), contract::address(), nft_id);
        
        match result {
            // If the transfer is successful, start the auction.
            Ok(_) => {
                self.started.set(true);
                // Set the end time of the auction to 7 days from now.
                self.end_at.set(U256::from(block::timestamp() + 7 * Self::ONE_DAY));
                // Log the start event.
                evm::log(Start {});
                Ok(())
            },
            // If the transfer fails, return an error.
            Err(_) => {
                return Err(EnglishAuctionError::NotSeller(NotSeller{}));
            }
            
        }
    }

    // The bid method allows bidders to place a bid on the auction.
    #[payable]
    pub fn bid(&mut self) -> Result<(), EnglishAuctionError> {
        // Check if the auction has started.
        if !self.started.get() {
            // Return an error if the auction has not started.
            return Err(EnglishAuctionError::NotSeller(NotSeller{}));
        }
        
        // Check if the auction has ended.
        if U256::from(block::timestamp()) >= self.end_at.get() {
            // Return an error if the auction has ended.
            return Err(EnglishAuctionError::AuctionEnded(AuctionEnded{}));
        }
        
        // Check if the bid amount is higher than the current highest bid.
        if msg::value() <= self.highest_bid.get() {
            // Return an error if the bid amount is too low.
            return Err(EnglishAuctionError::BidTooLow(BidTooLow{}));
        }
        
        // Refund the previous highest bidder. (But will not transfer back at this call, needs bidders to call withdraw() to get back the fund.
        if self.highest_bidder.get() != Address::default() {
            let mut bid = self.bids.setter(self.highest_bidder.get());
            let current_bid = bid.get();
            bid.set(current_bid + self.highest_bid.get());
        }
        
        // Update the highest bidder and the highest bid.
        self.highest_bidder.set(msg::sender());
        self.highest_bid.set(msg::value());

        // Update the bid of the current bidder.
        evm::log(Bid {
            sender: msg::sender(),
            amount: msg::value(),
        });
        Ok(())
    }

    // The withdraw method allows bidders to withdraw their bid.
    pub fn withdraw(&mut self) -> Result<(), EnglishAuctionError> {
        // Get the current bid of the bidder.
        let mut current_bid = self.bids.setter(msg::sender());
        let bal = current_bid.get();
        // Set the record of this bidder to 0 and transfer back tokens.
        current_bid.set(U256::from(0));
        let _ = transfer_eth(msg::sender(), bal);

        // Log the withdraw event.
        evm::log(Withdraw {
            bidder: msg::sender(),
            amount: bal,
        });
        Ok(())
    }

    // The end method allows the seller to end the auction.
    pub fn end<S: TopLevelStorage + BorrowMut<Self>>(storage: &mut S) -> Result<(), EnglishAuctionError> {
        // Check if the auction has started.
        if !storage.borrow_mut().started.get() {
            // Return an error if the auction has not started.
            return Err(EnglishAuctionError::NotStarted(NotStarted{}));
        }
        
        // Check if the auction has ended.
        if U256::from(block::timestamp()) < storage.borrow_mut().end_at.get() {
            // Return an error if the auction has not ended.
            return Err(EnglishAuctionError::NotEnded(NotEnded{}));
        }
        
        // Check if the auction has already ended.
        if storage.borrow_mut().ended.get() {
            // Return an error if the auction has already ended.
            return Err(EnglishAuctionError::AuctionEnded(AuctionEnded{}));
        }
        
        // End the auction and transfer the NFT and the highest bid to the winner.
        storage.borrow_mut().ended.set(true);
        let nft_contract_address = *storage.borrow_mut().nft_address;
        let seller_address = storage.borrow_mut().seller.get();
        let highest_bid = storage.borrow_mut().highest_bid.get();
        let highest_bidder = storage.borrow_mut().highest_bidder.get();
        let nft_id = storage.borrow_mut().nft_id.get();
        let config = Call::new_in(storage.borrow_mut());
        
        let nft = IERC721::new(nft_contract_address);
        
        // Check if there is highest bidder.
        if highest_bidder != Address::default() {
            // If there is a highest bidder, transfer the NFT to the highest bidder.
            let _ = nft.safe_transfer_from(config, contract::address(), highest_bidder, nft_id);
            // Transfer the highest bid to the seller.
            let _ = transfer_eth(seller_address, highest_bid);
        } else {
            // If there is no highest bidder, transfer the NFT back to the seller.
            let _ = nft.safe_transfer_from(config, contract::address(), seller_address, nft_id);
        }

        // Log the end event.
        evm::log(End {
            winner: highest_bidder,
            amount: highest_bid,
        });
        Ok(())
    }
}