<script lang="ts">
	import { PieceType, Player } from '$lib/index';
	import Field from './Field.svelte';
	import PromotionChoice from './PromotionChoice.svelte';

	let rows = [1, 2, 3, 4, 5, 6, 7, 8];
	let columns = 'ABCDEFGH';
	type FieldData = {};
	let starting_position: Array<Array<FieldData | null>> = [[]];
	let promotion_types = [
		PieceType.Knight,
		PieceType.Bishop,
		PieceType.Rook,
		PieceType.Queen
	];
	let current_player = Player.White;
</script>

<!-- <button on:click={greet}>Greet</button> -->
<table border="0">
	<tbody>
		{#each rows.reverse() as row}
			<tr>
				<th scope="row">{row}</th>
				{#each columns as column}
					<Field
						is_black={(row + (column.charCodeAt(0) - 'A'.charCodeAt(0))) % 2 ==
							0}
						player={Player.White}
						piece_type={PieceType.Knight}
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
