use std::io;

macro_rules! input {
    () => {{
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("erreur");
        input.trim().to_string()
    }};
}


#[derive(Clone)]
struct Cell{
   status : (bool,char) // bool = is_empty , String = la figure dans la case : _ si empty , X ou O sinon
}

#[derive(Clone)]
struct Grid{
    size : usize,
    cells : Vec<Cell>
}


impl Grid{
    fn create_grid() -> Grid {
        let size = 3;
        let mut filled_cells: Vec<Cell> = vec![];
        for _ in 1..=(size*size){
            let this_cell = Cell{status : (false,'_')};
            filled_cells.push(this_cell);

        }
        Grid{size: size,cells:filled_cells}
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
            let mut selected_cell:&Cell = &cloned_grid[index];
            if selected_cell.status.0 == false{
                let updated_cell = Cell{status : (true,symbol)};
                cloned_grid[index] = updated_cell.clone();
                self.cells = cloned_grid.clone();
            }
            else{
                println!("choose an empty cell ")
            }
        }
        else {
            println!("use only  X or O ")
        }
    }

    fn is_finished(&self)->bool{
        for i in 0..=self.size{
            for j in 0..=self.size{
                let current_index = self.index(i,j);
                let current_cell = &self.cells[current_index];
                let current_symbol = current_cell.status.1;
                if current_cell.status.0{
                    if (i>=0 || j>=0) && i!=j && (i<=self.size || j<=self.size){
                        if self.check_line(i,j,current_symbol){
                            return true;
                        }
                        else { return false; }

                    }
                    else if i==1 && j == 1 {
                        if self.check_diag(i,j,current_symbol){
                            return true;
                        }
                        else { return false; }
                    }
                    else { return false; }
                }
                else { return false; }
            }
            }
        false

    }

    fn check_diag(&self,i:usize, j:usize, symbol:char)->bool {

        let up_right = self.index(i-1,j-1);
        let up_left = self.index(i-1,j+1);
        let dn_right = self.index(i+1,j-1);
        let dn_left = self.index(i+1,j+1);

        if self.cells[up_right].status.1 == symbol && self.cells[dn_left].status.1 == symbol {
            return true;
        }
        else if self.cells[up_left].status.1 == symbol && self.cells[dn_right].status.1 == symbol {
            return true;
        }
        else {
           return  false;
        }


    }

    fn check_line(&self,i:usize, j:usize, symbol:char)->bool {

        let right = self.index(i,j+1);
        let left = self.index(i,j-1);
        let up = self.index(i-1,j);
        let down = self.index(i+1,j);

        if self.cells[right].status.1 == symbol && self.cells[left].status.1 == symbol {
            true
        }
        else if self.cells[up].status.1 == symbol && self.cells[down].status.1 == symbol {
            true
        }
        else {
            false
        }


    }

    fn handler(&mut self){
        let mut play_j1 =true;
        self.print_grid();
        loop {
            if play_j1 {
                println!("J1 turn you play X");
                let choosen_cell = self.handle_coordinate();
                self.next_round(choosen_cell,'X');
                play_j1 = false;
                if self.is_finished(){break}
            }
            else {
                println!("J2 turn you play O");
                let choosen_cell = self.handle_coordinate();
                self.next_round(choosen_cell,'O');
                play_j1 = true;
                if self.is_finished(){break}
            }
            self.print_grid()
        }
    }

    fn handle_coordinate(&self)->usize{
        println!("choose your row");
        let row = input!().parse::<u32>().unwrap();
        println!("choose your column");
        let column = input!().parse::<u32>().unwrap();
        println!("---------------");
        self.index(row as usize , column as usize)
    }












}














fn main() {
    let mut grid = Grid::create_grid();
    grid.handler()

}
