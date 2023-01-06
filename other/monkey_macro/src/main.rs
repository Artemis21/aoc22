macro_rules! monkeys {
    {$(
        Monkey $monkey_num:literal:
          Starting items: $($item:literal),*
          Operation: new = $left:tt $op:tt $right:tt
          Test: divisible by $test_divisor:literal
            If true: throw to monkey $if_true:literal
            If false: throw to monkey $if_false:literal
    )*} => {
        #[allow(unused_assignments)]
        const MONKEY_COUNT: usize = {
            let mut num = 0;
            $( num = $monkey_num; )*
            num + 1
        };

        const WORRY_MOD: usize = 1 $( * $test_divisor )*;

        fn monkey_business<const WORRY_DIV: usize>(rounds: usize) -> usize {
            let mut activity = [0; MONKEY_COUNT];
            $(
                $(
                    pass_item::<WORRY_DIV>(&mut activity, $monkey_num, $item, rounds);
                )*
            )*
            let (mut max, mut second) = (0, 0);
            for &act in activity.iter() {
                if act > max {
                    (second, max) = (max, act);
                } else if act > second {
                    second = act;
                }
            }
            max * second
        }

        fn pass_item<const WORRY_DIV: usize>(
            activity: &mut [usize; MONKEY_COUNT],
            mut monkey: usize,
            mut item: usize,
            mut rounds: usize,
        ) {
            while rounds > 0 {
                activity[monkey] += 1;
                let new_monkey = match monkey {$(
                    $monkey_num => {
                        item = operation!(item => $left $op $right);
                        item /= WORRY_DIV;
                        if item % $test_divisor == 0 {
                            $if_true
                        } else {
                            $if_false
                        }
                    }
                    )*
                    _ => unreachable!()
                };
                if monkey >= new_monkey {
                    rounds -= 1;
                }
                item %= WORRY_MOD;
                monkey = new_monkey;
            }
        }
    };
}

macro_rules! operation {
    ($item:ident => $left:tt + $right:tt) => {
        operand!($item => $left) + operand!($item => $right)
    };
    ($item:ident => $left:tt * $right:tt) => {
        operand!($item => $left) * operand!($item => $right)
    };
}

macro_rules! operand {
    ($item:ident => old) => {
        $item
    };
    ($item:ident => $lit:literal) => {
        $lit
    };
}

monkeys! {
Monkey 0:
  Starting items: 65, 58, 93, 57, 66
  Operation: new = old * 7
  Test: divisible by 19
    If true: throw to monkey 6
    If false: throw to monkey 4

Monkey 1:
  Starting items: 76, 97, 58, 72, 57, 92, 82
  Operation: new = old + 4
  Test: divisible by 3
    If true: throw to monkey 7
    If false: throw to monkey 5

Monkey 2:
  Starting items: 90, 89, 96
  Operation: new = old * 5
  Test: divisible by 13
    If true: throw to monkey 5
    If false: throw to monkey 1

Monkey 3:
  Starting items: 72, 63, 72, 99
  Operation: new = old * old
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 4

Monkey 4:
  Starting items: 65
  Operation: new = old + 1
  Test: divisible by 2
    If true: throw to monkey 6
    If false: throw to monkey 2

Monkey 5:
  Starting items: 97, 71
  Operation: new = old + 8
  Test: divisible by 11
    If true: throw to monkey 7
    If false: throw to monkey 3

Monkey 6:
  Starting items: 83, 68, 88, 55, 87, 67
  Operation: new = old + 2
  Test: divisible by 5
    If true: throw to monkey 2
    If false: throw to monkey 1

Monkey 7:
  Starting items: 64, 81, 50, 96, 82, 53, 62, 92
  Operation: new = old + 5
  Test: divisible by 7
    If true: throw to monkey 3
    If false: throw to monkey 0
}

fn main() {
    let part_a = monkey_business::<3>(20);
    let part_b = monkey_business::<1>(10_000);
    println!("{} {}", part_a, part_b);
}
