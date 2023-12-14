<script lang="ts">
	// export let piece: PieceType | null;
	export let is_black: boolean;
	import { piece_img_map } from '$lib/index';
	import { createEventDispatcher } from 'svelte';
	// export let draw_move: boolean;
	const dispatcher = createEventDispatcher();
</script>

{#if is_black}
	<td class="black_square">
		<div class="circle"></div>
	</td>
{:else}
	<td class="white_square">
		<svg class="svg-overlay" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">
			<image href={piece_img_map.black_pawn} width="100%" height="100%"> </image></svg
		>
		<div class="circle"></div>
	</td>
{/if}

<style>
	td {
		position: relative;
		place-items: center;
		vertical-align: middle;
	}
	.black_square {
		background-color: black;
		height: 90px;
		width: 90px;
		position: relative;
		place-items: center;
	}
	.white_square {
		background-color: white;
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
