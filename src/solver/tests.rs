use std::str::FromStr;

use test_case::test_case;

use crate::{solver::strategies::all_strategies, solver::Solver, Grid};

#[test_case(
    "000120000900005060075040093092050007164030528300010640680090750020500001000082000",
    "436129875918375264275846193892654317164937528357218649683491752729563481541782936";
    "naked-single"
)]
#[test_case(
    "000031026000000000000507180049000200003070400008000590025906000000000000690850000",
    "857431926412698375936527184149385267563279418278164593725946831384712659691853742";
    "hidden-single"
)]
#[test_case(
    "400000038002004100005300240070609004020000070600703090057008300003900400240000009",
    "461572938732894156895316247378629514529481673614753892957248361183967425246135789";
    "naked-pair"
)]
#[test_case(
    "000000000904607000076804100309701080008000300050308702007502610000403208000000000",
    "583219467914637825276854139349721586728965341651348792497582613165493278832176954";
    "hidden-pair"
)]
#[test_case(
    "070008029002000004854020000008374200000000000003261700000090612200000400130600070",
    "671438529392715864854926137518374296726859341943261785487593612269187453135642978";
    "naked-triple"
)]
#[test_case(
    "000000000231090000065003100008924000100050006000136700009300570000010843000000000",
    "894571632231698457765243198678924315143857926952136784489362571526719843317485269";
    "hidden-and-naked-triples"
)]
#[test_case(
    "000030086000020000000008500371000094900000005400007600200700800030005000700004030",
    "142539786587621943693478521371856294968142375425397618214763859839215467756984132";
    "naked-quad"
)]
fn solver_test(sudoku: &str, solution: &str) {
    let solver = Solver::new(all_strategies());
    let mut sudoku = Grid::from_str(sudoku).unwrap();
    solver.solve(&mut sudoku);
    assert_eq!(solution.to_string(), sudoku.to_string());
}
