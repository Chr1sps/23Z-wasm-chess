<script lang="ts">
	// export let piece: PieceType | null;
	export let is_black: boolean;
	import { piece_map, PieceType, Player } from '$lib/index';
	import { createEventDispatcher } from 'svelte';
	export let piece_type: PieceType;
	export let player: Player;
	export let is_selected = false;
	const dispatcher = createEventDispatcher();
</script>

{#if is_black}
	<td class="black_square">
		<svg
			class="svg-overlay"
			xmlns="http://www.w3.org/2000/svg"
			viewBox="0 0 100 100"
		>
			<image href={piece_map[player][piece_type]} width="100%" height="100%">
			</image></svg
		>
		{#if is_selected}
			<div class="circle"></div>
		{/if}
	</td>
{:else}
	<td class="white_square">
		<svg
			class="svg-overlay"
			xmlns="http://www.w3.org/2000/svg"
			viewBox="0 0 100 100"
		>
			<image href={piece_map[player][piece_type]} width="100%" height="100%">
			</image></svg
		>
		{#if is_selected}
			<div class="circle"></div>
		{/if}
	</td>
{/if}

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
