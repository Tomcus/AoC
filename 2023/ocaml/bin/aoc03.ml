let file_name = Array.get Sys.argv 1

let rec parse_line_impl line read numbers chars =
    let line_len = String.length line in
    if line_len = 0 then
        (numbers, chars)
    else
        match line.[0] with
        | '0'
        | '1'
        | '2'
        | '3'
        | '4'
        | '5'
        | '6'
        | '7'
        | '8'
        | '9' -> Scanf.sscanf line "%d%s" (fun num rest ->
            let num_len = String.length (string_of_int num) in
            let new_nums = (num, read, read + num_len - 1) :: numbers in
            parse_line_impl rest (read + num_len) new_nums chars
        )
        | '*' ->
            let new_chars = (read, -1, -1) :: chars in
            parse_line_impl (String.sub line 1 (line_len - 1)) (read + 1) numbers new_chars
        | _ -> parse_line_impl (String.sub line 1 (line_len - 1)) (read + 1) numbers chars
            

let parse_line line =
    parse_line_impl line 0 [] []

let rec sum_gear_ratio chars num num_start num_end =
    match chars with
    | [] -> (0, chars)
    | (pos, first, second) :: rest ->
        if pos >= (num_start - 1) && pos <= (num_end + 1) then
            if first = -1 then (
                (0, (pos, num, -1) :: rest)
            ) else (
                (first * num, rest)
            )
        else
            match sum_gear_ratio rest num num_start num_end with
            | (res, new_chars) -> 
                (res, (pos, first, second) :: new_chars)


let rec process_collisions current_sum nums chars =
    match nums with
    | [] -> (current_sum, nums, chars)
    | (num, num_start, num_end) :: rest ->
        (match sum_gear_ratio chars num num_start num_end with
        | (gear_ration, new_chars) ->
            if gear_ration > 0 then
                process_collisions (gear_ration + current_sum) rest new_chars
            else
                match process_collisions current_sum rest new_chars with
                | (sum, new_nums, new_chars) ->
                    (sum, (num, num_start, num_end) :: new_nums, new_chars)
        )

let process_data nums prev_nums chars prev_chars =
    match process_collisions 0 prev_nums chars with
    | (sum1, _rem, chars) ->
        match process_collisions sum1 nums prev_chars with
        | (sum2, nums1, _prev_chars) ->
            match process_collisions sum2 nums1 chars with
            | (sum3, rest, chars) -> (sum3, rest, chars)

let rec process_file_impl file prev_numbers prev_chars =
    let line = try input_line file with End_of_file -> "" in
    if String.length line = 0 then
        0
    else
        match parse_line line with
        | (numbers, characters) ->
            match process_data numbers prev_numbers characters prev_chars with
            | (sum, limited_nums, modified_chars) -> sum + process_file_impl file limited_nums modified_chars

let process_file file =
    process_file_impl file [] []

let () =
    let file = open_in file_name in
    let res = process_file file in
    Printf.printf "Result: %d\n" res;
    ()

