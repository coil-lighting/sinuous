
// pattern 1 is five sequential integers.
// has_pattern1 returns a tuple of a boolean that says if the pattern was found, and a slice of data containing the pattern
// in hindsight, a more Rust-idiomatic way to write this function is to have it return Option<&[int]> and match on that.  Unimportant.
fn has_pattern1<'life>(data: &'life[int]) -> (bool, &'life[int]) {

    let mut has_seq = false;

    let mut seq: &[int] = &[];


    // data.windows(n) returns an iterator over data, iterating over a slice of data of length n
    // so each for loop iteration, set_of_five is a &[int] of length five starting at data[0] and incrementing by one each loop.
    for set_of_five in data.windows(5) {
        // if this is the pattern we're looking for:
        if is_seq_ascend(set_of_five) {
            has_seq = true;

            seq = set_of_five;
            break;
        }
    }

    (has_seq, seq)
}

// pattern 2 is five descending-sequential integers.
// has_pattern2 returns a tuple of a boolean that says if the pattern was found, and a slice of data containing the pattern
// in hindsight, a more Rust-idiomatic way to write this function is to have it return Option<&[int]> and match on that.  Unimportant.
fn has_pattern2<'life>(data: &'life[int]) -> (bool, &'life[int]) {

    let mut has_seq = false;

    let mut seq: &[int] = &[];


    // data.windows(n) returns an iterator over data, iterating over a slice of data of length n
    // so each for loop iteration, set_of_five is a &[int] of length five starting at data[0] and incrementing by one each loop.
    for set_of_five in data.windows(5) {
        // if this is the pattern we're looking for:
        if is_seq_descend(set_of_five) {
            has_seq = true;

            seq = set_of_five;
            break;
        }
    }

    (has_seq, seq)
}



// now suppose we have some huge array of ints that we want to find an instance of a pattern.
// finding a pattern might be an expensive operation
// suppose we have a hierarchy of patterns; if we find pattern1, we want to stop looking for others
// if we don't find pattern1, we search for pattern2
// and, etc.
// for now leave the example simple with two patterns

fn main() {
    // data 1 contains pattern 1 - five sequential ints
    let data1 = &[0,1,2,3,4,5,6,7,8,9,10];

    println!("looking for patterns in:");
    println!("{:?}", data1);

    let mut found_pattern: &[int] = &[];

    if match has_pattern1(data1) {
        // if we found pattern 1, set found_pattern and exit from out if/else if/else block
        (has_pat, pat) if has_pat => {found_pattern = pat; true},

        // if we didn't find pattern 2, this if block was not executed and we move to the next else if clause
        _ => { false }
    } { println!("found pattern 1"); } // this block executes and found_pattern is pattern 1
    else if match has_pattern2(data1) { // this match expression does not execute
        // if we found pattern 2, set found_pattern and exit from if/else if/else block
        (has_pat, pat) if has_pat => {found_pattern = pat; true},

        // if we didn't find pattern 2, this if block was not executed and we move to the next else if clause
        _ => { false }
     } { println!("found pattern 2"); } // this block does not execture
    else {
        found_pattern = &[]; // this block does not execute
        println!("found no patterns");
    }

    println!("found this pattern:");
    println!("{:?}",found_pattern);



    let data2 = &[0, 0, 0, 10, 9, 8, 7, 6, 5, 4, 3];

    println!("looking for patterns in:");
    println!("{:?}", data2);

    let mut found_pattern2: &[int] = &[];

    if match has_pattern1(data2) {
        // if we found pattern 1, set found_pattern and exit from out if/else if/else block
        (has_pat, pat) if has_pat => {found_pattern2 = pat; true},

        // if we didn't find pattern 2, this if block was not executed and we move to the next else if clause
        _ => { false }
    } { println!("found pattern 1"); } // this block executes and found_pattern is pattern 1
    else if match has_pattern2(data2) { // this match expression does not execute
        // if we found pattern 2, set found_pattern and exit from if/else if/else block
        (has_pat, pat) if has_pat => {found_pattern2 = pat; true},

        // if we didn't find pattern 2, this if block was not executed and we move to the next else if clause
        _ => { false }
     } { println!("found pattern 2"); } // this block does not execture
    else {
        found_pattern2 = &[]; // this block does not execute
        println!("found no patterns");
    }

    println!("found this pattern:");
    println!("{:?}",found_pattern2);

}





// returns true if the elements of seq are ascending by one
// might be a more elegant way to do this
fn is_seq_ascend(seq: &[int]) -> bool {
    let mut isseq = true;
    for pair in seq.windows(2) {
        match pair {
            [a, b] if b != a + 1 => { isseq = false; break; },
            _ => {}
        }
    }

    isseq
}



// returns true if the elements of seq are descending by one
// might be a more elegant way to do this
fn is_seq_descend(seq: &[int]) -> bool {
    let mut isseq = true;
    for pair in seq.windows(2) {
        match pair {
            [a, b] if b != a - 1 => { isseq = false; break; },
            _ => {}
        }
    }

    isseq
}