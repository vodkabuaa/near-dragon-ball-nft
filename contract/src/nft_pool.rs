use std::collections::HashMap;
use near_sdk::env;

use crate::TokenMetadata;

#[derive(Debug)]
pub struct DragonBallNFT {
    pub title: String,
    pub description: String,
    pub media: String,
}

pub(crate) fn rand_range(from: u16, to: u16) -> u16 {
    let seed_num = env::random_seed();
    let sum:u16 = seed_num.iter().map(|&x| x as u16).sum();
    from + sum % (to-from)
}



lazy_static! {
    static ref NFT: HashMap<String, DragonBallNFT> = {
        let mut map = HashMap::new();
        map.insert("Bills".to_string(), 
            DragonBallNFT{
                title: "Bills".to_string(), 
                description:"Bills (ビルス), also called God of Destruction Bills (破壊神ビルス)".to_string(), 
                media:"https://bafkreigjspj4a7uuamzaemzcj5uscfmckstmma3ch7unsswml7gtsonfru.ipfs.nftstorage.link/".to_string()
            }
       );
        
        map.insert("Broly".to_string(),         
            DragonBallNFT{
                title:"Broly".to_string(), 
                description:"Broly (ブロリー Burorī) is a powerful Saiyan mutant and the son of Paragus.".to_string(), 
                media:"https://bafkreiclmupvtcugq3itio6spnsk2honyekbhb4e6ghpqafoceowwjss7a.ipfs.nftstorage.link/".to_string()});        

        map.insert("Goguetta".to_string(),         
            DragonBallNFT{
                title:"Goguetta".to_string(), 
                description:"Gogeta (ゴジータ, Gojīta) is the Metamoran fusion of Goku and Vegeta when they perform the Fusion Dance properly.".to_string(), 
                media:"https://bafkreidlqhuey3xmvggrguammh4hny3rzitvbdlfgilowisoxrkm5nptqa.ipfs.nftstorage.link/".to_string()});
            
        map.insert("Vegito".to_string(),         
            DragonBallNFT{
                title:"Vegito".to_string(), 
                description:"Vegito (ベジット Bejitto), also known as Vegerot in the Viz English manga, is the resulting fusion between Goku and Vegeta by the use of the Potara Earrings.".to_string(), 
                media:"https://bafkreigru3yidatowzryujtvxablwpma4xi5a4tjgkpx2dzwita32vmsym.ipfs.nftstorage.link/".to_string()});
        map                    
        };

    static ref NFT_LIST:Vec<String> = vec!["Bills".to_string(), "Broly".to_string(), "Goguetta".to_string(), "Vegito".to_string()];
}




pub fn random_draw_nft() -> TokenMetadata {
    let index = rand_range(0, NFT_LIST.len() as u16);
    println!("{:?}", index);
    let name = NFT_LIST.get(index as usize).unwrap();
    let dragon_ball_NFT = NFT.get(name).unwrap();
    TokenMetadata{
        title: Some(dragon_ball_NFT.title.clone()),
        description: Some(dragon_ball_NFT.description.clone()),
        media: Some(dragon_ball_NFT.media.clone()),
        media_hash: None,
        copies: Some(1),
        issued_at: None,  
        expires_at: None, 
        starts_at: None, 
        updated_at: None, 
        extra: None, 
        reference: None, 
        reference_hash: None,       
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_random_range(){
        let seed = env::random_seed();
        println!("{:?}",seed);
        let random_int = rand_range(0,4);
        println!("{:?}", random_int);
    }

    #[test]
    fn test_random_draw_nft(){
        let random_nft = random_draw_nft();
        println!("{:?}", random_nft);
    }
}