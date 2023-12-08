let rec powish_2 numb =
    match numb with
    | 0 -> 0
    | 1 -> 1
    | _ -> 2 * powish_2 (numb - 1)

let rec is_winning winning num =
    match winning with
    | [] -> false
    | first :: rest ->
        if num = first then
            true
        else
            is_winning rest num

let rec calculate_score winning drawn =
    match drawn with
    | [] -> 0
    | first :: rest ->
        if is_winning winning first then
            1 + calculate_score winning rest
        else
            calculate_score winning rest

let calculate_score_for_line line =
    let (winning, drawn) = Scanf.sscanf line "Card %s: %s | %s" (fun _ winning_raw drawn_raw -> (Utils.parse_list winning_raw "%d" " "), (Utils.parse_list drawn_raw "%d" " ")) in
    let count = calculate_score winning, drawn in
    powish_2 count

let () =
    let res = Utils.for_each_line_from_file_input calculate_score_for_line ( + ) 0 in
    Printf.printf "Result: %d\n" res
