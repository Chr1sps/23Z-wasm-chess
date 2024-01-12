<script lang="ts">
	// export let piece: PieceType | null;
	import type { PieceData } from '$lib/index';
	import { piece_map } from '$lib/index';
	import { createEventDispatcher } from 'svelte';
	export let is_black: boolean;
	export let piece_data: PieceData | null = null;
	export let is_selected = false;
	const dispatcher = createEventDispatcher();
</script>

<td class={is_black ? 'black_square' : 'white_square'}>
	{#if piece_data !== null}
		{@const [player, piece_type] = piece_data}
		<svg
			class="svg-overlay"
			xmlns="http://www.w3.org/2000/svg"
			viewBox="0 0 100 100"
		>
			<image href={piece_map[player][piece_type]} width="100%" height="100%">
			</image></svg
		>
	{/if}
	{#if is_selected}
		<div class="circle"></div>
	{/if}
</td>

<style>
	td {
		position: relative;
		place-items: center;
		vertical-align: middle;
	}
	.black_square {
		background-color: rgb(9, 43, 0);
		height: 90px;
		width: 90px;
		position: relative;
		place-items: center;
	}
	.white_square {
		background-color: rgb(168, 211, 157);
		height: 90px;
		width: 90px;
		position: relative;
		place-items: center;
		vertical-align: middle;
	}
	.circle {
		background-color: grey;
		height: 30px;
		width: 30px;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		margin: auto;
		border-radius: 50%;
		position: absolute;
	}
</style>
