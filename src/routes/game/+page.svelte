<script lang="ts">
	import type { PieceData } from '$lib/index';
	import { PieceType, Player } from '$lib/index';
	import Field from './Field.svelte';
	import PromotionChoice from './PromotionChoice.svelte';

	let rows = [1, 2, 3, 4, 5, 6, 7, 8];
	let columns = 'ABCDEFGH';
	let starting_position: Array<Array<PieceData | null>> = [
		[
			[Player.White, PieceType.Rook],
			[Player.White, PieceType.Knight],
			[Player.White, PieceType.Bishop],
			[Player.White, PieceType.Queen],
			[Player.White, PieceType.King],
			[Player.White, PieceType.Bishop],
			[Player.White, PieceType.Knight],
			[Player.White, PieceType.Rook]
		],
		Array(8).fill([Player.White, PieceType.Pawn]),
		[null, null, null, null, null, null, null, null],
		[null, null, null, null, null, null, null, null],
		[null, null, null, null, null, null, null, null],
		[null, null, null, null, null, null, null, null],
		Array(8).fill([Player.Black, PieceType.Pawn]),
		[
			[Player.Black, PieceType.Rook],
			[Player.Black, PieceType.Knight],
			[Player.Black, PieceType.Bishop],
			[Player.Black, PieceType.Queen],
			[Player.Black, PieceType.King],
			[Player.Black, PieceType.Bishop],
			[Player.Black, PieceType.Knight],
			[Player.Black, PieceType.Rook]
		]
	];
	let promotion_types = [
		PieceType.Knight,
		PieceType.Bishop,
		PieceType.Rook,
		PieceType.Queen
	];
	let current_player = Player.White;
	let selected: [number, number] | null;
</script>

<table border="0">
	<tbody>
		{#each starting_position.toReversed().entries() as [row_index, row]}
			<tr>
				<th scope="row">{rows[row_index]}</th>
				{#each row.entries() as [col_index, piece]}
					<Field is_black={(row_index + col_index) % 2 == 0} piece_data={piece}
					></Field>
				{/each}
			</tr>
		{/each}
		<tr>
			<td></td>
			{#each columns as column}
				<th>{column}</th>
			{/each}
		</tr>
	</tbody>
</table>
{#each promotion_types.entries() as [index, promotion_type] (index)}
	<PromotionChoice
		{promotion_type}
		player={current_player}
		is_black={index % 2 != 0}
	/>
{/each}
