//! The source code of this crate demonstrates
//! most of the use cases for `covered` macro.
//! Worth to read it.

#![allow(warnings)]

use covers::{mock, mocked};

const ORIGINAL: &str = r#"

    I threw a wish in the well
    Don't ask me I'll never tell
    I looked at you as it fell
    And now you're in my way

    I'd trade my soul for a wish
    Pennies and dimes for a kiss
    I wasn't looking for this
    But now you're in my way

    Your stare was holding
    Ripped jeans, skin was showin'
    Hot night, wind was blowin'
    Where you think you're going baby?

    Hey, I just met you and this is crazy
    But here's my number, so call me maybe
    It's hard to look right at you baby
    But here's my number, so call me maybe

    Encore!

    "#;

const COVER: &str = r#"

    I searched for lib in the Web
    To test code Dry, avoid Wet
    There's no success, all crates fail
    To mock 'fn' in place

    'Mockall', 'moctopus' - too bold
    Struct, macros, trait and that's all
    So then GitHub please behold
    There is my brand new crate

    Your stare was holding
    Your stars, I am hoping,
    Let my project growing
    Test your code, do it safely

    Good, I create this and it is lightweight
    Your test environment is independent
    It's hard to test crate creating traits
    So here is 'covers', mock functions daily

    It rocks!

    "#;

fn main() {
    let args = &["the well", "never", "tell", "it fell"];
    assert_trimmed(call_me_maybe(args), ORIGINAL);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let args = &["the Web", "Dry", "Wet", "crates fail"];
        assert_trimmed(call_me_maybe(args), COVER);
    }
}

#[rustfmt::skip]
pub fn call_me_maybe(params: &[&str]) -> String {
    let (well, never, tell, mut it_fell) = (params[0].to_string(), params[1], params[2], params[3].to_string());

    format!(
        "{}\n{}\n{}\n{} {}\n\n{}\n{}\n{}\n{}\n\n{}\n{}\n{}\n{}\n\n{}\n{}\n{}\n{}\n\n{}\n",
        
                    i_threw_a_wish_in(well),
                    dont_ask_me_i_ll(never, tell),
                    i_looked_at_you_as(&mut it_fell),
                    and_now_youre_in_my_way().0, and_now_youre_in_my_way().1,
        
                    id_trade_my_soul_for_a_wish(),
                    pennies_and_dimes_for_a_kiss(),
                    i_wasnt_looking_for_this(),
                    but_now_youre_in_my_way(),
        
        pre_chorus::your_stare_was_holding(),
        pre_chorus::ripped_jeans_skin_was_showin(),
        pre_chorus::hot_night_wind_was_blowin(),
        pre_chorus::where_you_think_youre_going_baby(),
        
            Chorus::hey_i_just_met_you_and_this_is_crazy(),
          Chorus {}.but_heres_my_number_so_call_me_maybe(),
          Chorus {}.its_hard_to_look_right_at_you_baby(),
          Chorus {}.but_here_is_my_number_so_call_me_maybe(),
        
                    applause(),
    )
}

fn assert_trimmed(lyrics: String, etalon: &str) {
    let etalon = beautify(etalon);
    assert_eq!(beautify(&lyrics), etalon);
    println!("{}", lyrics);
}

fn beautify(lyrics: &str) -> String {
    lyrics
        .lines()
        .map(|line| String::from(line.trim()))
        .filter(|line| !line.is_empty())
        .collect::<Vec<String>>()
        .join("\n")
}

#[mocked(i_searched_for_lib_in_the_web)]
fn i_threw_a_wish_in(the_well: String) -> String {
    format!("I threw a wish in {}", the_well)
}

#[mocked(to_test_code_dry_avoid_wet)]
fn dont_ask_me_i_ll(never: &str, tell: &str) -> String {
    format!("Don't ask me I'll {} {}", never, tell)
}

#[mocked(theres_no_success_all)]
fn i_looked_at_you_as(it_fell: &mut String) -> String {
    format!("I looked at you as {}", it_fell)
}

#[mocked(to_mock_fn_in_place)]
fn and_now_youre_in_my_way() -> (String, String) {
    ("And now you're".to_string(), "in my way".to_string())
}

#[mocked(lyrics_mocks::mockall_moctopus_too_bold)]
fn id_trade_my_soul_for_a_wish() -> String {
    "I'd trade my soul for a wish".to_string()
}

#[mocked(Chorus::struct_macros_trait_and_thats_all)] // No chorus in real
fn pennies_and_dimes_for_a_kiss() -> String {
    "Pennies and dimes for a kiss".to_string()
}

// Remaining...

#[mocked(so_then_github_please_behold)]
fn i_wasnt_looking_for_this() -> String {
    "I wasn't looking for this".to_string()
}

#[mocked(there_is_my_brand_new_crate)]
fn but_now_youre_in_my_way() -> String {
    "But now you're in my way".to_string()
}

mod pre_chorus {
    use super::*;

    #[mocked(your_stare_was_holding_)]
    pub fn your_stare_was_holding() -> String {
        "Your stare was holding".to_string()
    }

    #[mocked(your_stars_i_am_hoping)]
    pub fn ripped_jeans_skin_was_showin() -> String {
        "Ripped jeans, skin was showin'".to_string()
    }

    #[mocked(lyrics_mocks::let_my_project_growing)]
    pub fn hot_night_wind_was_blowin() -> String {
        "Hot night, wind was blowin'".to_string()
    }

    #[mocked(Chorus::test_your_code_do_it_safely)] // Still no chorus, that's the only struct in lib
    pub fn where_you_think_youre_going_baby() -> String {
        "Where you think you're going baby?".to_string()
    }

    // Mock close to tested code if it's your case

    fn your_stars_i_am_hoping() -> String {
        "Your stars, I am hoping,".to_string()
    }
}

pub struct Chorus {}

impl Chorus {
    #[mocked(good_i_create_this_and_it_is_lightweight, scope = impl)]
    pub fn hey_i_just_met_you_and_this_is_crazy() -> String {
        "Hey, I just met you and this is crazy".to_string()
    }

    #[mocked(Chorus::your_test_environment_is_independent)]
    fn but_heres_my_number_so_call_me_maybe(self) -> String {
        "But here's my number, so call me maybe".to_string()
    }

    #[mocked(lyrics_mocks::its_hard_to_test_crate_creating_traits)]
    fn its_hard_to_look_right_at_you_baby(&self) -> String {
        "It's hard to look right at you baby".to_string()
    }

    #[mocked(LyricsMocks::so_here_is_covers_mock_functions_daily)]
    fn but_here_is_my_number_so_call_me_maybe(&mut self) -> String {
        "But here's my number, so call me maybe".to_string()
    }

    // You even can implement mocks right here

    fn struct_macros_trait_and_thats_all() -> String {
        "Struct, macros, trait and that's all".to_string()
    }

    fn your_test_environment_is_independent(self) -> String {
        "Your test environment is independent".to_string()
    }

    fn test_your_code_do_it_safely() -> String {
        "Test your code, do it safely".to_string()
    }
}

#[mocked(it_rocks)]
fn applause() -> String {
    "Encore!".to_string()
}

// Mock lyrics inline

fn i_searched_for_lib_in_the_web(web: String) -> String {
    format!("I searched for lib in {}", web)
}

fn to_test_code_dry_avoid_wet(dry: &str, wet: &str) -> String {
    format!("To test code {}, avoid {}", dry, wet)
}

fn theres_no_success_all(crates_fail: &mut String) -> String {
    format!("There's no success, all {}", crates_fail)
}

fn to_mock_fn_in_place() -> (String, String) {
    ("To mock".to_string(), "'fn' in place".to_string())
}

// Mock lyrics inside modules

mod lyrics_mocks {
    use super::*;

    pub fn mockall_moctopus_too_bold() -> String {
        "'Mockall', 'moctopus' - too bold".to_string()
    }

    pub fn its_hard_to_test_crate_creating_traits(_self: &Chorus) -> String {
        "It's hard to test crate creating traits".to_string()
    }

    pub fn let_my_project_growing() -> String {
        "Let my project growing".to_string()
    }
}

fn so_then_github_please_behold() -> String {
    "So then GitHub please behold".to_string()
}

fn there_is_my_brand_new_crate() -> String {
    "There is my brand new crate".to_string()
}

// Mock lyrics inside struct's impl

struct LyricsMocks {}

impl LyricsMocks {
    pub fn so_here_is_covers_mock_functions_daily(_this: &mut Chorus) -> String {
        "So here is 'covers', mock functions daily".to_string()
    }
}

#[mock]
fn your_stare_was_holding_() -> String {
    pre_chorus::_your_stare_was_holding()
}

fn good_i_create_this_and_it_is_lightweight() -> String {
    "Good, I create this and it is lightweight".to_string()
}

fn it_rocks() -> String {
    "It rocks!".to_string()
}
