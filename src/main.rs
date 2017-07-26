extern crate rand;

use rand::Rng;

enum RollResult
{
    Success(i32),
    Failure,
    Botch,
}

struct SkillRoll
{
    num_dice: i32,
    num_sides: i32,
    difficulty: i32,
    result : RollResult,
}

trait DiceRoller
{
    fn roll(&mut self);
    fn get_result(&self) -> &RollResult;
}

impl DiceRoller for SkillRoll
{
    fn get_result(&self) -> &RollResult
    {
        &self.result
    }


    fn roll(&mut self)
    {
        let mut ones = 0;
        let mut successes = 0;

        for _ in 1 .. self.num_dice
        {
            let r = rand::thread_rng().gen_range(1,self.num_sides);
            match r
            {
                1 =>
                {
                    ones += 1;
                },
                x =>
                {
                    if x >= self.difficulty
                    {
                        successes += 1;
                    }
                }
            }
        }
        let net_successes = successes - ones;

        if net_successes < 0
        {
            self.result = RollResult::Botch;
        }
        else
        if net_successes > 0
        {
            self.result = RollResult::Success(net_successes);
        }
        else
        {
            self.result = RollResult::Failure;
        }
    }
}

fn main() {
    let mut sword_strike = SkillRoll {num_dice: 6, num_sides: 10, difficulty: 6, result : RollResult::Failure};
    sword_strike.roll();
    match sword_strike.get_result()
    {
        &RollResult::Botch => println!("Botched!"),
        &RollResult::Failure => println!("Failed!"),
        &RollResult::Success(x) => println!("Succeeded with {}", x),
    }



}
