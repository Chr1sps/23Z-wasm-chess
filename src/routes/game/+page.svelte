<script lang="ts">
	import type { PieceData, Position } from '$lib/index';
	import { PieceType, Player } from '$lib/index';
	import Field from './Field.svelte';
	import PromotionSelector from './PromotionSelector.svelte';

	let rows = [8, 7, 6, 5, 4, 3, 2, 1];
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
	let current_player = Player.White;
	let selected: Position | null = null;
	let is_promotion = true;
	const handleFieldClick = (event: CustomEvent<Position | null>) => {
		let new_selected = event.detail as Position;
		console.log(new_selected);
		if (selected === null) {
			selected = new_selected;
		} else {
			if (selected[0] === new_selected[0] && selected[1] === new_selected[1]) {
				selected = null;
			} else {
				// this is where a move would be created and done
				current_player =
					current_player == Player.White ? Player.Black : Player.White;
				selected = null;
			}
		}
	};
</script>

<div class="container">
	<table border="0">
		<tbody>
			<tr>
				<td width="20" height="20" class="upper_left"></td>
				{#each columns as column}
					<th>{column}</th>
				{/each}
				<td width="20" height="20" class="upper_right"></td>
			</tr>
			{#each starting_position.toReversed().entries() as [row_index, row]}
				<tr>
					<th scope="row">{rows[row_index]}</th>
					{#each row.entries() as [col_index, piece]}
						<Field
							on:click={handleFieldClick}
							is_black={(row_index + col_index) % 2 !== 0}
							piece_data={piece}
							is_selected={selected
								? selected[0] === row_index && selected[1] === col_index
								: false}
							position={[row_index, col_index]}
						></Field>
					{/each}
					<th scope="row">{rows[row_index]}</th>
				</tr>
			{/each}
			<tr>
				<td class="lower_left"></td>
				{#each columns as column}
					<th>{column}</th>
				{/each}
				<td class="lower_right"></td>
			</tr>
		</tbody>
	</table>
	<div class="gap"></div>
	{#if is_promotion}
		<PromotionSelector player={current_player} />
	{:else}
		<div class="filler"></div>
	{/if}
</div>

<style>
	.container {
		display: flex;
		justify-content: center;
		align-items: center;
	}
	.gap {
		width: 90px;
	}
	.filler {
		width: 90px;
	}
	table {
		border-collapse: collapse;
	}
	td,
	th {
		background-color: #5e724e;
		color: white;
	}
	.upper_left {
		border-radius: 50% 0 0 0;
	}
	.upper_right {
		border-radius: 0 50% 0 0;
	}
	.lower_left {
		border-radius: 0 0 0 50%;
	}
	.lower_right {
		border-radius: 0 0 50% 0;
	}
</style>
