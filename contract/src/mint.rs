use crate::*;

#[near_bindgen]
impl Contract {
    fn create_token_id(&self, receiver_id: &AccountId)->String {
        if self.tokens_per_owner.contains_key(&receiver_id) {
            let len = self.tokens_per_owner.get(&receiver_id).unwrap().len();
            return format!("{}-{}",receiver_id, len+1);
        }
        return format!("{}-{}",receiver_id, 0);
    }


    #[payable]
    pub fn nft_mint(
        &mut self,
        receiver_id: AccountId,
    ) {
        //measure the initial storage being used on the contract
        let initial_storage_usage = env::storage_usage();
        let token_id = self.create_token_id(&receiver_id);
        //specify the token struct that contains the owner ID 
        let token = Token {
            //set the owner ID equal to the receiver ID passed into the function
            owner_id: receiver_id,
        };

        

        //insert the token ID and token struct and make sure that the token doesn't exist
        assert!(
            self.tokens_by_id.insert(&token_id, &token).is_none(),
            "Token already exists"
        );

        let metadata = random_draw_nft();
        //insert the token ID and metadata
        self.token_metadata_by_id.insert(&token_id, &metadata);

        //call the internal method for adding the token to the owner
        self.internal_add_token_to_owner(&token.owner_id, &token_id);


        // Construct the mint log as per the events standard.
        let nft_mint_log: EventLog = EventLog {
            // Standard name ("nep171").
            standard: NFT_STANDARD_NAME.to_string(),
            // Version of the standard ("nft-1.0.0").
            version: NFT_METADATA_SPEC.to_string(),
            // The data related with the event stored in a vector.
            event: EventLogVariant::NftMint(vec![NftMintLog {
                // Owner of the token.
                owner_id: token.owner_id.to_string(),
                // Vector of token IDs that were minted.
                token_ids: vec![token_id.to_string()],
                // An optional memo to include.
                memo: None,
            }]),
        };

        // Log the serialized json.
        env::log_str(&nft_mint_log.to_string());


        //calculate the required storage which was the used - initial
        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;
        println!("{:?}", required_storage_in_bytes);
        //refund any excess storage if the user attached too much. Panic if they didn't attach enough to cover the required.
        refund_deposit(required_storage_in_bytes);
    }
}