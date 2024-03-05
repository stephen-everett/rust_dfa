/*
    CECS 329 Spring 2024 Section 03 
    Assignment 1 Group 2
    Author: Stephen Everett
    Contributors: Crystal Diaz, Arnav Mishra, Philip Rah
 */

fn main() {
    /*
        First Implementation:
         Parent DFA is an enum and can be one of two types: FirstDFA and SecondDFA
         
         FirstDFA is an enum of type States. The number of states that the enum States can be is directly
         related to the number of characters in the first name of the given student. (Philip)

         SecondDFA is an enum of type SecondStates. The number of second states the enum SecondStates can be
         is directly related to the number of characters in the last name of the given student (Rah)

         The logic for both is mostly the same. We loop through each character, and depending on the character
         and the state, we change the ParentDFA state to the corresponding state.

         For instance, When ParentDFA is of type FirstDFA, and FirstDFA is of type Start, then reading the character
         'p' will move the FirstDFA to type P.

         Once we reach the last character in the FirstDFA states, the make the next state the start state of SecondStates.
         SecondStates continues with the same logic, only this time instead of linking to another DFA, we move to the accept state.

         After it is over, we check the type of ParentDFA. If it's of type SecondStates::Accept, then the string has been accepted.
         If it's any other state, then the string is rejected.
    */
    enum ParentDFA {
        FirstDFA(States),
        SecondDFA(SecondStates)
    }
    enum States{
        Start,
        P,
        H,
        I,
        L,
        I2,
    }

    #[derive(Clone)]
    enum SecondStates {
        Start,
        R,
        A,
        Accept
    }
 
    let test_string = "pphilphiliprarrrarah".to_string();
    let mut dfa = ParentDFA::FirstDFA(States::Start);

    for char in test_string.chars() {
        match dfa {
            ParentDFA::FirstDFA(first_dfa) => {
                match first_dfa {
                    States::Start => {
                        match char {
                            'p' => dfa = ParentDFA::FirstDFA(States::P),
                            _ => dfa = ParentDFA::FirstDFA(States::Start)
                        };
                    }
                    States::P => {
                        match char {
                            'h' => dfa = ParentDFA::FirstDFA(States::H),
                            'p' => dfa = ParentDFA::FirstDFA(States::P),
                            _ => dfa = ParentDFA::FirstDFA(States::Start)
                        };
                    }
                    States::H => {
                        match char {
                            'i' => dfa = ParentDFA::FirstDFA(States::I),
                            'p' => dfa = ParentDFA::FirstDFA(States::P),
                            _ => dfa = ParentDFA::FirstDFA(States::Start)
                        };
                    }
                    States::I => {
                        match char {
                            'l' => dfa = ParentDFA::FirstDFA(States::L),
                            'p' => dfa = ParentDFA::FirstDFA(States::P),
                            _ => dfa = ParentDFA::FirstDFA(States::Start)
                        };
                    }
                    States::L => {
                        match char {
                            'i' => dfa = ParentDFA::FirstDFA(States::I2),
                            'p' => dfa = ParentDFA::FirstDFA(States::P),
                            _ => dfa = ParentDFA::FirstDFA(States::Start)
                        };
                    }
                    States::I2 => {
                        match char {
                            'p' => dfa = ParentDFA::SecondDFA(SecondStates::Start),
                            _ => dfa = ParentDFA::FirstDFA(States::Start)
                        };
                    }
                }
            }
            ParentDFA::SecondDFA(ref second_dfa) => {
                match second_dfa {
                    SecondStates::Start => {
                        match char {
                            'r' => dfa = ParentDFA::SecondDFA(SecondStates::R),
                            _ => dfa = ParentDFA::SecondDFA(SecondStates::Start)
                        }
                    }
                    SecondStates::R => {
                        match char {
                            'a' => dfa = ParentDFA::SecondDFA(SecondStates::A),
                            'r' => dfa = ParentDFA::SecondDFA(SecondStates::R),
                            _ => dfa = ParentDFA::SecondDFA(SecondStates::Start)
                        }
                    }
                    SecondStates::A => {
                        match char {
                            'h' => dfa = ParentDFA::SecondDFA(SecondStates::Accept),
                            'r' => dfa = ParentDFA::SecondDFA(SecondStates::R),
                            _ => dfa = ParentDFA::SecondDFA(SecondStates::Start)
                        }
                    }
                    SecondStates::Accept => (),
                }
            }
        }
            
    }

    match dfa {
        ParentDFA::SecondDFA(state) => {
            match state {
                SecondStates::Accept => println!("String accepted!"),
                _ => println!("String rejected!")
            }
        },
        _ => println!("String rejected!")
    };

    /*
        Second Implementation:

        This implementation allows for an arbitrary number of strings of arbitrary words. It is not hard-coded for a specific word
        like the first implementation

        Define the strings to look for in string_arr. The strings have to occur in the order they're defined for the test_string to be accepted.

        This implementation uses an enum named DFANode. DFANode can be of two types: RunningState and Accept.
        RunningState is a structure with a delta_char. delta_char is the character required to move onto the next node. It's an 
        Option, so it can be Some(char) or None. This is used to catch the case where there is an empty string. This DFA rejects empty strings.

        1.) We start at the first element in string_arr and create an iterator named target_iterator from that string. 
        2.) We then iterate through each character in the test_string, and if that character matches the DFANode::RunningState delta_char, 
        call next() on the target_iterator. 
        3.) If target_iterator.next() returns None, then we have reached the end of that particular string and we create a new iterator with the next 
        element in string_arr. If target_iterator.next() returns Some(char) then we set the DFANode to RunningState with the delta_char equal to that
        char
        4.) If target_iterator.next() returns None, and we are at the end of the string_arr, then move DFANode to the Accept state
     */

    // define State and DFANode
    pub struct State{
        delta_char: Option<char>
    }

    pub enum DFANode {
        RunningState(State),
        Accept
    }
 
    // initialize the strings to look for 
    let string_arr = [String::from("khirby"), String::from("calma")]; // add extra strings to look for more than only 2 strings

    // string to search against
    let test_string = String::from("khkhikhirbyrbyccalma");

    // intialize starting values
    let mut string_arr_index = 0;
    let mut target_iterator = string_arr[string_arr_index].char_indices();

    // get first character in first element of string_arr. If empty then the state is set
    // If empty then eventually reject
    let mut test_dfa = match target_iterator.next() {
        Some((_, char)) => DFANode::RunningState(State{delta_char:Some(char)}),
        None => DFANode::RunningState(State{delta_char:None})
    };

    // iterate through test_string characters
    for char in test_string.chars(){

        // test_dfa will be RunningState, or Accept. If accept, then do nothing
        match test_dfa {
           DFANode::RunningState(ref state) => {
                match state.delta_char {
                    Some(c) => {
                        // compare current char to delta_char
                        if c == char {
                            
                            // grab next character target_iterator. 
                            match target_iterator.next() {
                                Some((_,char_2)) => {
                                    test_dfa = DFANode::RunningState(State{delta_char:Some(char_2)})
                                }
                                // if None then check if there is another string in str_arr, or else move to accept
                                None => {
                                    test_dfa = match string_arr_index >= string_arr.len()-1 {
                                        true => DFANode::Accept,
                                        false => {
                                            string_arr_index += 1;
                                            target_iterator = string_arr[string_arr_index].char_indices();
                                            match target_iterator.next() {
                                                Some((_, start_char)) => {
                                                    DFANode::RunningState(State{delta_char:Some(start_char)})
                                                }
                                                None => DFANode::RunningState(State{delta_char:None})
                                            }
                                        },
                                    };
                                    
                                }
                                    
                            }
                        }
                        // if c != char, then reset iterator and state to the start of the current string
                        else {
                            if char == string_arr[string_arr_index].chars().next().unwrap() {
                                target_iterator = string_arr[string_arr_index].char_indices();
                                target_iterator.next();
                            }
                            else {
                                target_iterator = string_arr[string_arr_index].char_indices();
                            }
                            
                            match target_iterator.next() {
                                Some((_,start_char)) => {
                                    if char == start_char {
                                        match target_iterator.next() {
                                            Some((_, char_2)) => DFANode::RunningState(State{delta_char:Some(char_2)}),
                                            None => DFANode::RunningState(State{delta_char:None})
                                        };
                                    }
                                    else {
                                        test_dfa = DFANode::RunningState(State{delta_char:Some(start_char)})
                                    }
                                },
                                None => ()
                            }
                        }
                    }
                    None => (),
                }
            },
            DFANode::Accept => (),
        }
    }

    match test_dfa {
        DFANode::Accept => println!("String accepted!"),
        _ => println!("Rejected")
    }

}
