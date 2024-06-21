use crate::*;
use near_contract_standards::fungible_token::metadata::{
    FungibleTokenMetadata, FungibleTokenMetadataProvider, FT_METADATA_SPEC,
};

const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/svg+xml,%3Csvg width='40' height='40' fill='none' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath fill-rule='evenodd' clip-rule='evenodd' d='M30.74 28.78c.21-.36.32-.78.32-1.21v-6.33c0-.24-.06-.46-.17-.66-.12-.21-.28-.38-.48-.49l-16.86-9.77c-.35-.2-.73-.31-1.13-.32-.64 0-1.24.26-1.69.72-.45.45-.7 1.07-.71 1.71v6.39c0 .24.06.47.18.67.11.21.28.37.48.48l16.82 9.71c.36.22.77.32 1.19.32.41 0 .82-.12 1.19-.33.36-.21.66-.52.86-.89zm.32-16.27c0-.42-.1-.84-.29-1.21-.2-.38-.49-.7-.85-.93-.36-.22-.78-.34-1.21-.34-.42 0-.84.12-1.21.34l-4.97 2.81c-.04.02-.08.07-.1.12-.03.04-.04.1-.04.14 0 .06.01.11.04.16.02.04.06.08.1.1l8.05 4.62c.05.02.1.04.15.04.05 0 .1-.02.15-.05.04-.02.08-.06.11-.11.02-.04.04-.1.03-.15l.04-5.54zM10.05 27.5c-.01.43.08.86.28 1.23.2.38.49.7.85.93.36.22.78.34 1.2.34.42 0 .83-.12 1.2-.34l4.92-2.85c.04-.02.08-.06.1-.11.03-.04.05-.1.05-.15 0-.05-.02-.11-.05-.15-.02-.05-.06-.09-.1-.12l-8.05-4.6c-.05-.03-.09-.04-.15-.04-.05 0-.1.01-.15.04-.05.02-.08.07-.11.12-.02.04-.04.09-.04.14l.05 5.56z' fill='url(%23paint0_linear_186_370)'/%3E%3Cdefs%3E%3ClinearGradient id='paint0_linear_186_370' x1='11.135' y1='11.152' x2='30.145' y2='30.457' gradientUnits='userSpaceOnUse'%3E%3Cstop stop-color='%231BB3CC'/%3E%3Cstop offset='1' stop-color='%23824ACC'/%3E%3C/linearGradient%3E%3C/defs%3E%3C/svg%3E";

#[near_bindgen]
impl FungibleTokenMetadataProvider for LiquidStakingContract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        FungibleTokenMetadata {
            spec: FT_METADATA_SPEC.to_string(),
            name: String::from("LiNEAR"),
            symbol: String::from("LINEAR"),
            icon: Some(String::from(DATA_IMAGE_SVG_NEAR_ICON)),
            reference: None,
            reference_hash: None,
            decimals: 24,
        }
    }
}
