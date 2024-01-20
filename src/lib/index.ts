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
import * as wasm from 'wasm-chess';

export const piece_map = {
	[wasm.Player.White]: {
		[wasm.PieceType.Pawn]: white_pawn,
		[wasm.PieceType.Knight]: white_knight,
		[wasm.PieceType.Bishop]: white_bishop,
		[wasm.PieceType.Rook]: white_rook,
		[wasm.PieceType.Queen]: white_queen,
		[wasm.PieceType.King]: white_king
	},
	[wasm.Player.Black]: {
		[wasm.PieceType.Pawn]: black_pawn,
		[wasm.PieceType.Knight]: black_knight,
		[wasm.PieceType.Bishop]: black_bishop,
		[wasm.PieceType.Rook]: black_rook,
		[wasm.PieceType.Queen]: black_queen,
		[wasm.PieceType.King]: black_king
	}
};

export const promotion_map = {
	[wasm.PromotionType.Knight]: wasm.PieceType.Knight,
	[wasm.PromotionType.Bishop]: wasm.PieceType.Bishop,
	[wasm.PromotionType.Rook]: wasm.PieceType.Rook,
	[wasm.PromotionType.Queen]: wasm.PieceType.Queen
};

export type Position = [number, number];
