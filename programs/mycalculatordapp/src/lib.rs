use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod mycalculatordapp {
    use super::*;

    // 計算機能に必要な関数: ctxは常にaccountの配列とプログラムIDから成る
    pub fn create(ctx:Context<Create>, init_message: String) -> ProgramResult{
        let calculator = &mut ctx.accounts.calculator;
        calculator.greeting = init_message; // calculatorに表示する初期メッセージ
        Ok(())  // ProgramgResult型のエラー処理，エラーがなければプログラムを続行
    }

    pub fn add(ctx: Context<Addition>, num1: i64, num2: i64) -> ProgramResult{
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 + num2;
        Ok(())
    }

    pub fn multiply(ctx: Context<Multiplication>, num1: i64, num2: i64) -> ProgramResult{
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 * num2;
        Ok(())
    }

    pub fn subtract(ctx: Context<Subtraction>, num1: i64, num2: i64) -> ProgramResult{
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 - num2;
        Ok(())
    }
    
    pub fn divide(ctx: Context<Division>, num1: i64, num2: i64) -> ProgramResult{
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 / num2;    // 商
        calculator.remainder = num1 % num2; // 剰余
        Ok(())
    }
}

// Accountsを宣言し，各構造体を定義
#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = user, space = 8 + 64 + 64 + 64)]    // init: mycalculatordappが所有するアカウントを作成するために使用
    pub calculator: Account<'info, Calculator>, // calculatorのaccount
    #[account(mut)]
    pub user: Signer<'info>,    // 計算をするユーザーのaccount
    pub system_program: Program<'info, System>, // Solana system_programのaccount
}

#[derive(Accounts)]
pub struct Addition<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

#[derive(Accounts)]
pub struct Multiplication<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

#[derive(Accounts)]
pub struct Subtraction<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

#[derive(Accounts)]
pub struct Division<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}



#[account]
// calculator accountで使用する構造
pub struct Calculator {
    pub greeting: String,
    pub result: i64,
    pub remainder: i64,
}

