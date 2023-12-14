// place files you want to import through the `$lib` alias in this folder.
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

export const piece_img_map = {
	white_knight,
	white_bishop,
	white_pawn,
	white_king,
	white_rook,
	white_queen,
	black_knight,
	black_bishop,
	black_pawn,
	black_king,
	black_rook,
	black_queen
};
