<script lang="ts">
	import { promotion_map } from '$lib/index.js';
	import * as wasm from 'wasm-chess';

	import Field from './Field.svelte';
	export let player: wasm.Player = wasm.Player.White;
	export let hidden: boolean = false;

	const promotion_types = [
		wasm.PromotionType.Knight,
		wasm.PromotionType.Bishop,
		wasm.PromotionType.Rook,
		wasm.PromotionType.Queen
	];
</script>

<table {hidden}>
	<tbody>
		{#each promotion_types.entries() as [index, promotion_type]}
			<tr>
				<Field
					is_black={!!(index % 2)}
					piece_data={wasm.PieceData.new(
						player,
						promotion_map[promotion_type]
					)}
				></Field></tr
			>
		{/each}</tbody
	>
</table>

<style>
	table {
		border-collapse: collapse;
	}
</style>
