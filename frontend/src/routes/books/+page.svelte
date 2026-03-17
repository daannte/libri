<script lang="ts">
	import type { PageProps } from './$types';
	let { data }: PageProps = $props();
</script>

<div class="p-6">
	{#if data.books && data.books.length > 0}
		<div class="grid grid-cols-1 gap-4 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4">
			{#each data.books as book (book.id)}{/each}
		</div>
	{:else}
		<div class="flex flex-col items-center justify-center py-20 text-gray-500">
			<p>No books found.</p>
		</div>
	{/if}
</div>
