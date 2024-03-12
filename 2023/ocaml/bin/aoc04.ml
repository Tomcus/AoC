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

let stoi str =
    let str = String.trim str in
    int_of_string_opt str

let calculate_score_for_line line =
    let first_split = String.split_on_char ':' line in
    let data = List.nth first_split 1 in
    let second_split = String.split_on_char '|' (String.trim data) in
    let winning = List.filter_map stoi (String.split_on_char ' ' (List.nth second_split 0)) in
    let drawn = List.filter_map stoi (String.split_on_char ' ' (List.nth second_split 1)) in
    let count = calculate_score winning drawn in
    powish_2 count

let () =
    let res = Utils.for_each_line_from_file_input calculate_score_for_line ( + ) 0 in
    Printf.printf "Result: %d\n" res
