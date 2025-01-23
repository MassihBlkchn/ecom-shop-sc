use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

declare_id!("YourProgramIDHere");

#[program]
pub mod ecommerce_shop {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, shop_owner: Pubkey) -> Result<()> {
        let shop_account = &mut ctx.accounts.shop_account;
        shop_account.owner = shop_owner;
        shop_account.product_count = 0;
        Ok(())
    }

    pub fn add_product(ctx: Context<AddProduct>, name: String, price: u64) -> Result<()> {
        let shop_account = &mut ctx.accounts.shop_account;
        let product = Product {
            id: shop_account.product_count,
            name,
            price,
            owner: shop_account.owner,
        };

        shop_account.products.push(product);
        shop_account.product_count += 1;
        Ok(())
    }

    pub fn buy_product(ctx: Context<BuyProduct>, product_id: u64) -> Result<()> {
        let shop_account = &mut ctx.accounts.shop_account;
        let buyer = &ctx.accounts.buyer;
        let system_program = &ctx.accounts.system_program;

        // Find the product
        let product = shop_account.products.iter().find(|p| p.id == product_id).ok_or(ErrorCode::ProductNotFound)?;

        // Transfer funds
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &buyer.key(),
            &product.owner,
            product.price,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[buyer.to_account_info(), system_program.to_account_info()],
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + ShopAccount::LEN)]
    pub shop_account: Account<'info, ShopAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddProduct<'info> {
    #[account(mut, has_one = owner)]
    pub shop_account: Account<'info, ShopAccount>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct BuyProduct<'info> {
    #[account(mut)]
    pub shop_account: Account<'info, ShopAccount>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ShopAccount {
    pub owner: Pubkey,
    pub product_count: u64,
    pub products: Vec<Product>,
}

impl ShopAccount {
    const LEN: usize = 32 + 8 + (8 + 32 + 4 + 50) * 100; // Adjust for your max number of products
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Product {
    pub id: u64,
    pub name: String,
    pub price: u64,
    pub owner: Pubkey,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Product not found.")]
    ProductNotFound,
}
