<script lang="ts">
	import * as Empty from '$lib/components/ui/empty';
	import MediaCard from '$lib/components/media-card.svelte';

	import BookText from '@lucide/svelte/icons/book-text';

	let { data } = $props();
</script>

<div class="flex p-6">
	{#if data.media.length > 0}
		<div
			class="grid w-full grid-cols-2 gap-6 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6"
		>
			{#each data.media as media (media.id)}
				<MediaCard {media} />
			{/each}
		</div>
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
