use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod mycalculatordapp {
    use super::*;

    pub fn create(ctx: Context<Create>, init_message: String) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.greeting = init_message;
        Ok(())
    }

    pub fn add(ctx: Context<Addition>, first_number: i64, second_number: i64) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = first_number + second_number;
        Ok(())
    }

    pub fn substract(ctx: Context<Substract>, first_number: i64, second_number: i64) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = first_number - second_number;
        Ok(())
    }

    pub fn multiply(ctx: Context<Multiply>, first_number: i64, second_number: i64) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = first_number * second_number;
        Ok(())
    }

    pub fn divide(ctx: Context<Divide>, first_number: i64, second_number: i64) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = first_number / second_number;
        calculator.remainder = first_number % second_number;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer=user, space=264)]
    pub calculator: Account<'info, Calculator>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Addition<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

#[derive(Accounts)]
pub struct Substract<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

#[derive(Accounts)]
pub struct Multiply<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

#[derive(Accounts)]
pub struct Divide<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

#[account]
pub struct Calculator {
    pub greeting: String,
    pub result: i64,
    pub remainder: i64,
}
