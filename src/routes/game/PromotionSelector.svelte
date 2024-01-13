<script lang="ts">
	// import type { PieceData } from '$lib/index.js';
	import { PieceType, Player } from '$lib/index.js';

	import Field from './Field.svelte';
	export let player: Player = Player.White;
	const promotion_types = [
		PieceType.Knight,
		PieceType.Bishop,
		PieceType.Rook,
		PieceType.Queen
	];
</script>

<table>
	<tbody>
		{#each promotion_types.entries() as [index, promotion_type]}
			<tr>
				<Field is_black={!!(index % 2)} piece_data={[player, promotion_type]}
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
