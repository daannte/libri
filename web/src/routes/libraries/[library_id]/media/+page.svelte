<script lang="ts">
	import * as Empty from '$lib/components/ui/empty';

	import BookText from '@lucide/svelte/icons/book-text';

	let { data } = $props();
</script>

<div class="min-h-screen p-6">
	{#if data.media.length > 0}
		{#each data.media as media (media.id)}
			<div class="m-2 rounded-2xl bg-secondary p-4">
				<a href="#top">
					<h3>{media.name}</h3>
				</a>
			</div>
		{/each}
	{:else}
		<Empty.Root class="min-h-screen">
			<Empty.Header>
				<Empty.Media variant="icon">
					<BookText />
				</Empty.Media>
				<Empty.Title>No media found</Empty.Title>
				<Empty.Description>Click below to upload new media</Empty.Description>
			</Empty.Header>
			<Empty.Content>
				<p>TODO add button to upload media</p>
			</Empty.Content>
		</Empty.Root>
	{/if}
</div>
