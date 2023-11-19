class Move {
	board_id: number;
	field_id: number;
	player: Player;

	constructor(board_id: number, field_id: number, player: Player) {
		this.field_id = field_id;
		this.board_id = board_id;
		this.player = player;
	}
}

enum Player {
	O,
	X,
	None
}

class Board {
	board: Array<Player>;
    winner: Player;

	constructor() {
		this.board = new Array();
        this.winner = Player.None;
		for (let index = 0; index < 9; index++) {
			this.board.push(Player.None);
		}
	}
    public make_move(move:Move) {
        this.board[move.field_id] = move.player;
    }

    public check_win() {
        return Player.None;
    }
}

class MetaBoard {
	boards: Array<Board>;
    winner: Player;
	constructor() {
		this.boards = new Array();
        this.winner = Player.None;
		for (let index = 0; index < 9; index++) {
			this.boards.push(new Board());
		}
	}

    public make_move(move:Move) {
        this.boards[move.board_id].make_move(move);
    }

    public check_win() {
        
    }
}
