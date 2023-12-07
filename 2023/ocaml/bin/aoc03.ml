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
        | '.' -> parse_line_impl (String.sub line 1 (line_len - 1)) (read + 1) numbers chars
        | _ ->
            let new_chars = read :: chars in
            parse_line_impl (String.sub line 1 (line_len - 1)) (read + 1) numbers new_chars
            

let parse_line line =
    parse_line_impl line 0 [] []

let rec find_collision chars num_start num_end =
    match chars with
    | [] -> false
    | pos :: rest ->
        if pos >= (num_start - 1) && pos <= (num_end + 1) then
            true
        else
            find_collision rest num_start num_end

let rec process_collisions current_sum nums chars =
    match nums with
    | (num, num_start, num_end) :: rest ->
        if find_collision chars num_start num_end then
            process_collisions (current_sum + num) rest chars
        else
            (match process_collisions current_sum rest chars with
            | (sum, new_nums) ->
                (sum, (num, num_start, num_end) :: new_nums))
    | [] -> (current_sum, nums)

let process_data nums prev_nums chars prev_chars =
    match process_collisions 0 prev_nums chars with
    | (sum1, _rem) ->
        match process_collisions sum1 nums prev_chars with
        | (sum2, nums1) ->
            match process_collisions sum2 nums1 chars with
            | (sum3, rest) -> (sum3, rest)

let rec process_file_impl file prev_numbers prev_chars =
    let line = try input_line file with End_of_file -> "" in
    if String.length line = 0 then
        0
    else
        match parse_line line with
        | (numbers, characters) ->
            match process_data numbers prev_numbers characters prev_chars with
            | (sum, limited_nums) -> sum + process_file_impl file limited_nums characters

let process_file file =
    process_file_impl file [] []

let () =
    let file = open_in file_name in
    let res = process_file file in
    Printf.printf "Result: %d\n" res;
    ()
