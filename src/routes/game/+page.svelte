<script lang="ts">
	import Field from './field.svelte';

	let rows = [1, 2, 3, 4, 5, 6, 7, 8];
	let columns = 'ABCDEFGH';
</script>

<!-- <button on:click={greet}>Greet</button> -->
<table border="0">
	<tbody>
		{#each rows.reverse() as row}
			<tr>
				<th scope="row">{row}</th>
				{#each columns as column}
					<Field is_black={(row + (column.charCodeAt(0) - 'A'.charCodeAt(0))) % 2 == 0}></Field>
				{/each}
			</tr>
		{/each}
		<tr>
			<td></td>
			{#each columns as column}
				<th>{column}</th>
			{/each}
		</tr>
	</tbody>
</table>
