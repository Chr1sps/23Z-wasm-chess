<script lang="ts">
	// import type { PieceData } from '$lib/index.js';
	import { PieceType, Player } from '$lib/index.js';

	import Field from './Field.svelte';
	export let promotion_type: PieceType;
	export let player: Player = Player.White;
	// export let piece_data: PieceData;
	export let is_black: boolean = false;
	let is_valid = ![PieceType.King, PieceType.Pawn].includes(promotion_type);
</script>

{#if is_valid}
	<Field {is_black} piece_data={[player, promotion_type]}></Field>
{:else}
	<div>Invalid promotion type.</div>
{/if}
