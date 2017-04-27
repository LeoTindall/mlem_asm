#[derive(Debug, PartialEq)]
enum LexState {
    Seperator,
    Token,
    Done,
}

// Lex a line of characters into tokens
pub fn lex_line(line: &str) -> Vec<&str> {
    let mut v = Vec::new();
    // keep track of the start of the current token
    let mut start_index = 0;
    // keep track of the end of the current token
    let mut stop_index = 0;
    // the state of the lexer
    let mut state = LexState::Seperator;

    for character in line.chars() {
        match character {
            ';' | '\n' | '\r' => {
                // A comment is starting or the line is ending. End the current token and stop lexing the line.
                if state == LexState::Token { v.push(&line[start_index..stop_index]); }
                state == LexState::Done;
                break;
            }
            ' ' | '\t' => { 
                // The current token is ending, or we'er in a long seperator.
                // End the token and set the state, and proceed.
                if state == LexState::Token { v.push(&line[start_index..stop_index]); }
                state = LexState::Seperator;
            }
            _ => {
                // Something else; a token.
                // Reset the window and set the state.
                if state == LexState::Seperator { start_index = stop_index; }
                state = LexState::Token;
            }
        }
        // Advance the window's end index
        stop_index += 1;
    }
    // Input is over; potentially end a token
    // This is kind of a hack, to prevent double ending tokens.
    if state == LexState::Token {
        let new_token = &line[start_index..stop_index];
        // If there's nothing, there's no possibility for a duplicate.
        if v.len() == 0 { v.push(new_token); }
        // Check for duplicates
        else {
            let last_token = v.pop().unwrap();
            // If they're not the same, put it back
            if new_token != last_token { v.push(last_token); }
            // No matter what, put the new one in. That way, if they're the same,
            // there will only be one.
            v.push(new_token); 
        }
    }
    return v;
}

pub fn lex(source: &str) -> Vec<Vec<&str>> {
    let mut v = Vec::new();
    for line in source.lines() {
        v.push(lex_line(line));
    };
    return v;
}

#[cfg(test)]
mod test_lex {
    use super::*;
    #[test]
    fn test_lex_line_with_comment() {
        let result = lex_line("ident1 ident2:ident2more\tident3; comment");
        assert_eq!(&result[..], ["ident1", "ident2:ident2more", "ident3"]);
    }

    #[test]
    fn test_lex_line_lexes_only_one_line() {
        let result = lex_line("ident1 ident2 \n ident3");
        assert_eq!(&result[..], ["ident1", "ident2"]);
    }
    fn test_lex_multiple_lines() {
        let result = lex("l1i1 l1i2 ; line 1 comment\nl2i1 l2i2 l2i3 ; line 2 comment");
        assert_eq!(&result[..], &[vec!["l1i1", "l1i2"], vec!["l2i1", "l2i1", "l2i3"]]);
    }
}