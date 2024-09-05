use anchor_lang::prelude::*;

declare_id!("Gv6oGiZrAXqbY9fRv5iweVZQi8t7mGDAB5S8DAaSupi1");

#[program]
pub mod tryrust {
    use super::*;
    use std::collections::HashMap;


    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn age_checker(ctx: Context<Initialize>, age: u64) -> Result<()> {
        let result = if age >= 18 {
            "You are 18 years old or above"
        } else {
            "You are below 18 years old"
        };
        msg!("{:?}", result);
        Ok(())
    }

    pub fn age_checker1(ctx: Context<Initialize>, age: u64) -> Result<()> {
        match age {
            1 => {
                msg!("The age is 1");
            }
            2 | 3 => {
                msg!("The age is either 2 or 3");
            }
            4..=6 => {
                msg!("The age is between 4 and 6");
            }
            _ => {
                msg!("The age is something else");
            }
        }
        for i in (0..20).step_by(3) {
            msg!("{}", i);
        }
        Ok(())
    }
    pub fn array_vector(ctx: Context<Initialize>) -> Result<()> {
        let my_array: [u32; 5] = [10, 20, 30, 40, 50];

        let first_element = my_array[0];
        let third_element = my_array[2];

        let mut mutable_array: [u32; 3] = [100, 200, 300];
        mutable_array[1] = first_element;
        msg!("new second element is {}", mutable_array[1]);

        let mut dynamic_array: Vec<u32> = Vec::new();

        dynamic_array.push(10);
        dynamic_array.push(20);
        dynamic_array.push(my_array[2]);

        msg!("dynamic array 3rd element:{}", dynamic_array[2]);
    
        Ok(())
    }

    pub fn key_value_mapping(ctx: Context<Initialize>, key: String, value: String) -> Result<()> {
        let mut my_map = HashMap::new();
        my_map.insert(key.to_string(), value.to_string());
        msg!("My name is {}", my_map[&key]);
        Ok(())
    }
    pub fn usize_example(ctx: Context<Initialize>) -> Result<()> {
        let mut dynamic_array: Vec<u32> = Vec::from([1,2,3,4,5,6]);
        let len = dynamic_array.len();

        let another_var: u64 = 5;



        let len_plus_another_var = len as u64 + another_var;
        msg!("The result is {}", len_plus_another_var);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
