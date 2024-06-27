/// https://www.codewars.com/kata/525caa5c1bf619d28c000335/train/rust
pub fn is_solved(board: &[&[u8; 3]; 3]) -> i8 {
    let my_board = Board::create_from(board);
    let winner = my_board.winner();

    if winner != DRAW {
        return winner;
    }

    if my_board.is_finished() {
        return DRAW;
    }

    NOT_FINISHED
}

const NOT_FINISHED: i8 = -1;
const DRAW: i8 = 0;
const EMPTY: u8 = 0;
const X: u8 = 1;
const O: u8 = 2;

struct Board {
    rows: [RowColDiag; 3],
    cols: [RowColDiag; 3],
    diag: [RowColDiag; 2],
}

impl Board {
    fn winner(&self) -> i8 {
        self.row_winner()
            .max(self.col_winner())
            .max(self.diag_winner())
    }

    fn row_winner(&self) -> i8 {
        self.rows.iter()
            .map(|row| row.winner())
            .find(|&x| x != DRAW)
            .unwrap_or(DRAW)
    }

    fn col_winner(&self) -> i8 {
        self.cols.iter()
            .map(|col| col.winner())
            .find(|&x| x != DRAW)
            .unwrap_or(DRAW)
    }

    fn diag_winner(&self) -> i8 {
        self.diag.iter()
            .map(|diag| diag.winner())
            .find(|&x| x != DRAW)
            .unwrap_or(DRAW)
    }

    fn is_finished(&self) -> bool {
        for row in self.rows.iter() {
            if !row.is_finished() {
                return false;
            }
        }

        true
    }

    fn create_from(board: &[&[u8; 3]; 3]) -> Board {
        let mut rows: [RowColDiag; 3] = [RowColDiag::empty(), RowColDiag::empty(), RowColDiag::empty()];
        let mut cols: [RowColDiag; 3] = [RowColDiag::empty(), RowColDiag::empty(), RowColDiag::empty()];
        let mut diag: [RowColDiag; 2] = [RowColDiag::empty(), RowColDiag::empty()];

        for i in 0..3 {
            rows[i] = RowColDiag::create_from([board[i][0], board[i][1], board[i][2]]);
            cols[i] = RowColDiag::create_from([board[0][i], board[1][i], board[2][i]]);
        }

        diag[0] = RowColDiag::create_from([board[0][0], board[1][1], board[2][2]]);
        diag[1] = RowColDiag::create_from([board[0][2], board[1][1], board[2][0]]);

        Board { rows, cols, diag }
    }
}

struct RowColDiag {
    fields: [u8; 3],
}

impl RowColDiag {
    fn winner(&self) -> i8 {
        if self.fields.iter().all(|&x| x == X) {
            return X as i8;
        }

        if self.fields.iter().all(|&x| x == O) {
            return O as i8;
        }

        DRAW
    }

    fn is_finished(&self) -> bool {
        self.fields.iter().all(|&x| x != EMPTY)
    }

    fn create_from(fields: [u8; 3]) -> RowColDiag {
        RowColDiag { fields }
    }

    fn empty() -> RowColDiag {
        RowColDiag { fields: [0; 3] }
    }
}
