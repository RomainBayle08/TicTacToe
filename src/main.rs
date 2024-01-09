use std::io;

macro_rules! input {
    () => {{
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("erreur");
        input.trim().to_string()
    }};
}

#[derive(Clone)]
struct Case{
    index : u32,
    to_check: Vec<(usize, usize)>
}
#[derive(Clone)]
struct Table{
    cases: Vec<Case>
}


#[derive(Clone)]
struct Cell{
   status : (bool,char) // bool = is_empty , String = la figure dans la case : _ si empty , X ou O sinon
}

#[derive(Clone)]
struct Grid{
    size : usize,
    cells : Vec<Cell>,
    is_j1play: bool,
    check_table : Table
}



impl Grid{
    fn create_grid() -> Grid {
        let size = 3;
        let mut filled_cells: Vec<Cell> = vec![];
        let check_table = Table::create();
        for _ in 1..=(size*size){
            let this_cell = Cell{status : (false,'_')};
            filled_cells.push(this_cell);

        }
        Grid{size: size,cells:filled_cells, is_j1play: true,check_table}
    }

    fn print_grid(&self){
        for row in self.cells.chunks(self.size){
            for cell in row{
                print!("{}" , if cell.status.0 == false{"□"}else{if cell.status.1 == 'X'{"X"}else{"O"}});
            }
            println!();
        }
        println!("-------------")
    }
    fn index(&self,i:usize,j:usize)->usize{
        i*self.size+j
    }

    fn next_round(&mut self, index:usize, symbol : char){
        if symbol == 'X' || symbol == 'O'{
            let mut cloned_grid = &mut self.cells.clone();
            let mut cloned_isj1play = &mut self.is_j1play.clone();
            let mut selected_cell:&Cell = &cloned_grid[index];
            if selected_cell.status.0 == false{
                let updated_cell = Cell{status : (true,symbol)};
                cloned_grid[index] = updated_cell.clone();

                let updated_cloned_isj1play = &mut match cloned_isj1play {
                    true => false,
                    _ => true,
                };
                self.cells = cloned_grid.clone();
                self.is_j1play = updated_cloned_isj1play.clone();

            }
            else{
                println!("choose an empty cell ")
            }
        }
        else {
            println!("use only  X or O ")
        }
    }


    fn check_if_finished(&mut self,index : usize)->u32{
        let mut is_finished = 0;
        let current_symbol:char = self.cells[index].status.1.clone();
        let to_check = &self.check_table.return_value_to_check(index).clone();
        for i in 0..to_check.len(){
            let current_tuple = to_check[i];
            let symbol_cell_1:char = self.cells[current_tuple.0].status.1.clone();
            let symbol_cell_2:char = self.cells[current_tuple.1].status.1.clone();

            if current_symbol.eq(&symbol_cell_1) && current_symbol.eq(&symbol_cell_2){
                is_finished = 1;
                break;
            }
        }
        is_finished

    }


    fn handler(&mut self){
        let mut symbol : char ;
        self.print_grid();
        loop {
            if self.is_j1play {
                println!("J1 turn you play X");
                symbol = 'X';
            }
            else {
                println!("J2 turn you play O");
                symbol = 'O'
            }

            let chosen_cell = self.handle_coordinate();
            let index_chosen_cell = self.index(chosen_cell.0,chosen_cell.1);
            self.next_round(index_chosen_cell,symbol);
            if self.check_if_finished(index_chosen_cell) == 1{
                self.print_grid();
                if self.is_j1play {
                    println!("J2 win")
                }
                else {
                    println!("J1 win")
                }
                break;
            }
            self.print_grid()
        }
    }

    fn handle_coordinate(&self)->(usize,usize){
        println!("choose your row");
        let row = input!().parse::<u32>().unwrap() as usize;
        println!("choose your column");
        let column = input!().parse::<u32>().unwrap() as usize;
        println!("---------------");
        (row,column)
    }


}


impl Table {
    fn create()->Table{
        let mut cases: Vec<Case>=vec![];

        let mut zero_case:Vec<(usize,usize)> = vec![];
        zero_case.push((1,2));
        zero_case.push((3,6));
        zero_case.push((4,8));
        let zero = Case{index:0, to_check:zero_case};
        cases.push(zero);

        let mut one_case:Vec<(usize,usize)> = vec![];
        one_case.push((0,2));
        one_case.push((4,7));
        let one = Case{index:1, to_check:one_case};
        cases.push(one);

        let mut two_case:Vec<(usize,usize)> = vec![];
        two_case.push((0, 1));
        two_case.push((5, 8));
        two_case.push((0 ,4 ));
        let two = Case{index:2, to_check: two_case };
        cases.push(two);

        let mut three_case:Vec<(usize,usize)> = vec![];
        three_case.push((0, 6));
        three_case.push((4, 5));
        let three = Case{index:3, to_check: three_case };
        cases.push(three);

        let mut four_case:Vec<(usize,usize)> = vec![];
        four_case.push((0,8));
        four_case.push((2,6));
        four_case.push((3,5));
        four_case.push((1,7));
        let four = Case{index:4, to_check:four_case};
        cases.push(four);

        let mut five_case:Vec<(usize,usize)> = vec![];
        five_case.push((2, 8));
        five_case.push((3, 4));
        let five = Case{index:5, to_check: five_case };
        cases.push(five);

        let mut six_case:Vec<(usize,usize)> = vec![];
        six_case.push((0, 3));
        six_case.push((7, 8));
        six_case.push((4,2));
        let six = Case{index:6, to_check: six_case };
        cases.push(six);

        let mut seven_case:Vec<(usize,usize)> = vec![];
        seven_case.push((6, 8));
        seven_case.push((1, 4));
        let seven = Case{index:7, to_check: seven_case };
        cases.push(seven);

        let mut eight_case:Vec<(usize,usize)> = vec![];
        eight_case.push((7, 6));
        eight_case.push((2, 5));
        eight_case.push((0,4));
        let eight = Case{index:8, to_check: eight_case };
        cases.push(eight);

        Table{ cases: cases}
    }

    fn return_value_to_check(&mut self, value:usize)->Vec<(usize,usize)>{
        let mut to_return : Vec<(usize,usize)>= vec![];
        let cloned_cases  = &mut self.cases.clone();
        if value>=0 && value <=8{
             let current_cases : &Case= &cloned_cases[value];
             let possible_values = &current_cases.to_check;
             for i in 0..possible_values.len(){
                 to_return.push(possible_values[i]);
             }
        }
        else {
            panic!("selectionner une valeur adequat")
        }
        to_return
    }

}
















fn main() {
    let mut grid = Grid::create_grid();
    grid.handler()
   // println!("{}",0%2==1)

}
