<script lang="ts">
	import type { Position } from '$lib';
	import { onMount } from 'svelte';
	import init, * as wasm from 'wasm-chess';
	import Field from './Field.svelte';
	import PromotionSelector from './PromotionSelector.svelte';

	let rows = [8, 7, 6, 5, 4, 3, 2, 1];
	let columns = 'ABCDEFGH';
	let current_player = wasm.Player.White;
	let choosing_promotion = false;
	let promotion_type: wasm.PromotionType | undefined = undefined;
	let game: wasm.Game;
	let possible_moves: Array<Position> = [];
	let selected_pos: Position | null = null;
	let new_selected: Position;

	$: console.log(selected_pos);
	$: possible_moves = selected_pos
		? game.get_moves(selected_pos[0], selected_pos[1]).map((js_pos) => {
				return [js_pos.row, js_pos.col];
			})
		: [];

	const getPromotionType = async () => {
		while (promotion_type === undefined) {}
		return promotion_type;
	};

	const handleFieldClick = (event: CustomEvent<Position | null>) => {
		new_selected = event.detail!;
		let [row, col] = new_selected;

		console.log('New selected: %d,%d', row, col);
		if (selected_pos === null) {
			let piece_data = game.get_piece_data(row, col);
			if (piece_data !== undefined) {
				let player = piece_data.get_player();
				if (player === current_player) {
					selected_pos = new_selected;
				}
			}
		} else {
			if (!(selected_pos[0] === row && selected_pos[1] === col)) {
				console.log('creating a move');
				let [from_row, from_col] = selected_pos;
				let [to_row, to_col] = new_selected;
				if (!game.is_promotion_move(from_row, from_col, to_row, to_col)) {
					try {
						console.log('trying a move');
						game.make_move(from_row, from_col, to_row, to_col, promotion_type);
						current_player = wasm.get_opponent(current_player);
						console.log('move complete');
					} catch (error) {
						console.log(error);
					}
				} else {
					choosing_promotion = true;
				}
			}
			selected_pos = null;
			rows = rows;
			columns = columns;
		}
	};
	const check_selected = (
		pos: [number, number],
		current: [number, number] | null
	): boolean => {
		let [row, col] = pos;
		let is_selected = current
			? current[0] === row && current[1] === col
			: false;
		return is_selected || check_get_moves(pos);
	};
	const check_get_moves = (expected: [number, number]) => {
		let [expected_row, expected_col] = expected;
		return (
			possible_moves.find((pos) => {
				let [row, col] = pos;
				return expected_row === row && expected_col === col;
			}) !== undefined
		);
	};
	onMount(async () => {
		await init();
		game = wasm.Game.new();
	});
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
			{#if game !== undefined}
				{#each rows as row_number}
					{@const row_index = row_number - 1}
					<tr>
						<th scope="row">{row_number}</th>
						{#each [0, 1, 2, 3, 4, 5, 6, 7] as col_index}
							<Field
								on:click={handleFieldClick}
								is_black={(row_index + col_index) % 2 === 0}
								piece_data={game.get_piece_data(row_index, col_index) ?? null}
								is_selected={check_selected(
									[row_index, col_index],
									selected_pos
								)}
								position={[row_index, col_index]}
							></Field>
						{/each}
						<th scope="row">{row_number}</th>
					</tr>
				{/each}
			{/if}
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
	{#if choosing_promotion}
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
