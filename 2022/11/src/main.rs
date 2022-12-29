enum Action {
    Multiply(u64),
    Add(u64),
    Power(u32)
}

type Items = Vec<u64>;

struct Monkey {
    items: Items,
    operation: Action,
    test_divisible_by: u64,
    ok_test_index: usize,
    fail_test_index: usize,
    processed_items: usize
}

fn process_monkey(items: &Items, operation: &Action, test_divisible_by: u64, limit_base: u64) -> (Items, Items) { 
        let mut ok_list = vec![];
        let mut fail_list = vec![];
        items.iter().map(|e| -> u64 {
            match operation {
                Action::Add(to_add) => {
                    e.checked_add(*to_add).unwrap()
                },
                Action::Power(power_by) => {
                    e.checked_pow(*power_by).unwrap()
                },
                Action::Multiply(multiply_by) => {
                    e.checked_mul(*multiply_by).unwrap()
                }
            }
        }).map(|e| {
            e % limit_base
        }).for_each(|e| {
            if e % test_divisible_by == 0 {
                ok_list.push(e);
            } else {
                fail_list.push(e);
            }
        });
        (ok_list, fail_list)
}

fn process_round(monkeys: &mut Vec<Monkey>, limit_base: u64) {
    let len = monkeys.len();
    for i in 0..len {
        let (ok_list, ok_index, fail_list, fail_index) = {
            let monkey = &mut monkeys[i];
            monkey.processed_items += monkey.items.len();
            let (ok_list, fail_list) = process_monkey(&monkey.items, &monkey.operation, monkey.test_divisible_by, limit_base);
            monkey.items.clear();
            (ok_list, monkey.ok_test_index, fail_list, monkey.fail_test_index)
        };
        monkeys[ok_index].items.extend(ok_list.iter());
        monkeys[fail_index].items.extend(fail_list.iter());
    }
}

fn main() {
    let mut monkeys = vec![
        Monkey {
            items: vec![93, 54, 69, 66, 71],
            operation: Action::Multiply(3),
            test_divisible_by: 7,
            ok_test_index: 7,
            fail_test_index: 1,
            processed_items: 0,
        },
        Monkey {
            items: vec![89, 51, 80, 66],
            operation: Action::Multiply(17),
            test_divisible_by: 19,
            ok_test_index: 5,
            fail_test_index: 7,
            processed_items: 0,
        },
        Monkey {
            items: vec![90, 92, 63, 91, 96, 63, 64],
            operation: Action::Add(1),
            test_divisible_by: 13,
            ok_test_index: 4,
            fail_test_index: 3,
            processed_items: 0,
        },
        Monkey {
            items: vec![65, 77],
            operation: Action::Add(2),
            test_divisible_by: 3,
            ok_test_index: 4,
            fail_test_index: 6,
            processed_items: 0
        },
        Monkey {
            items: vec![76, 68, 94],
            operation: Action::Power(2),
            test_divisible_by: 2,
            ok_test_index: 0,
            fail_test_index: 6,
            processed_items: 0,
        },
        Monkey {
            items: vec![86, 65, 66, 97, 73, 83],
            operation: Action::Add(8),
            test_divisible_by: 11,
            ok_test_index: 2,
            fail_test_index: 3,
            processed_items: 0,
        },
        Monkey {
            items: vec![78],
            operation: Action::Add(6),
            test_divisible_by: 17,
            ok_test_index: 0,
            fail_test_index: 1,
            processed_items: 0,
        },
        Monkey {
            items: vec![89, 57, 59, 61, 87, 55, 55, 88],
            operation: Action::Add(7),
            test_divisible_by: 5,
            ok_test_index: 2,
            fail_test_index: 5,
            processed_items: 0
        },
    ];
    let data: std::collections::HashSet<u64> = monkeys.iter().map(|m| {
        m.test_divisible_by
    }).collect();
    let lcm = data.into_iter().fold(1, |l, el| {
        l * el
    });
    for _i in 0..10000 {
        process_round(&mut monkeys, lcm);
    }
    for i in 0..monkeys.len() {
        println!("Monkey #{}: {}", i, monkeys[i].processed_items);
    }
}
