<script lang="ts">
	// export let piece: PieceType | null;
	import { piece_map, type Position } from '$lib/index';
	import { createEventDispatcher } from 'svelte';
	import type * as wasm from 'wasm-chess';
	export let is_black: boolean;
	export let piece_data: wasm.PieceData | null = null;
	export let is_selected = false;
	export let position: Position | null = null;
	const dispatch = createEventDispatcher<{ click: null | Position }>();
	const handleClick = () => {
		dispatch('click', position);
	};
</script>

<td class={is_black ? 'black_square' : 'white_square'} on:click={handleClick}>
	{#if piece_data !== null}
		{@const player = piece_data.get_player()}
		{@const piece_type = piece_data.get_type()}
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
		background-color: #91b493;
		height: 90px;
		width: 90px;
		position: relative;
		place-items: center;
	}
	.white_square {
		background-color: #d3e3c9;
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
