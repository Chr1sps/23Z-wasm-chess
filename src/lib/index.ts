import black_bishop from '$lib/assets/black_bishop.svg';
import black_king from '$lib/assets/black_king.svg';
import black_knight from '$lib/assets/black_knight.svg';
import black_pawn from '$lib/assets/black_pawn.svg';
import black_queen from '$lib/assets/black_queen.svg';
import black_rook from '$lib/assets/black_rook.svg';
import white_bishop from '$lib/assets/white_bishop.svg';
import white_king from '$lib/assets/white_king.svg';
import white_knight from '$lib/assets/white_knight.svg';
import white_pawn from '$lib/assets/white_pawn.svg';
import white_queen from '$lib/assets/white_queen.svg';
import white_rook from '$lib/assets/white_rook.svg';
export const enum PieceType {
	Pawn,
	Knight,
	Bishop,
	Rook,
	Queen,
	King
}

export const enum Player {
	White,
	Black
}
export type PieceData = [Player, PieceType];

export const piece_map = {
	[Player.White]: {
		[PieceType.Pawn]: white_pawn,
		[PieceType.Knight]: white_knight,
		[PieceType.Bishop]: white_bishop,
		[PieceType.Rook]: white_rook,
		[PieceType.Queen]: white_queen,
		[PieceType.King]: white_king
	},
	[Player.Black]: {
		[PieceType.Pawn]: black_pawn,
		[PieceType.Knight]: black_knight,
		[PieceType.Bishop]: black_bishop,
		[PieceType.Rook]: black_rook,
		[PieceType.Queen]: black_queen,
		[PieceType.King]: black_king
	}
};

export type Position = [number, number];
