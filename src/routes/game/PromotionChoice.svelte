<script lang="ts">
	import { PieceType, Player } from '$lib/index.js';
	import Field from './Field.svelte';
	export let promotion_type: PieceType;
	export let player: Player = Player.White;
	export let is_black: boolean = false;
	let is_valid = ![PieceType.King, PieceType.Pawn].includes(promotion_type);
</script>

{#if is_valid}
	<Field {is_black} {player} piece_type={promotion_type}></Field>
{:else}
	<div>Invalid promotion type.</div>
{/if}
