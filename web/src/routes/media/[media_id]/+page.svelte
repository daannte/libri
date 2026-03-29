<script lang="ts">
	import { get_cover_url } from '@shiori/api-client';

	let { data } = $props();

	let metadataArr = $derived.by(() =>
		data.metadata
			? Object.entries(data.metadata).filter(
					([key]) => !['genres', 'description', 'authors'].includes(key)
				)
			: []
	);
</script>

<div class="flex h-screen">
	<div class="relative flex flex-1 items-center justify-center">
		<img
			class="z-1 aspect-2/3 max-h-[60vh] w-auto rounded-xl object-contain"
			src={get_cover_url(data.cover_path) ?? ''}
			alt={`${data.name} cover image`}
		/>
	</div>
	<div class="flex flex-1 flex-col justify-center pr-48">
		<div class="flex flex-col gap-2">
			<h1 class="font-serif text-2xl md:text-3xl lg:text-4xl">
				{data.name}
			</h1>
			<h2 class="md:text-xl">
				by <span class="font-medium">{data.metadata?.authors}</span>
			</h2>
			{#if data.metadata?.genres?.length}
				<div class="text-sm text-muted-foreground">
					{data.metadata.genres.join(', ')}
				</div>
			{/if}
		</div>

		<p class="mt-4 md:mt-8">{@html data.metadata?.description}</p>

		<div class="my-8 border-t-2"></div>

		<div class="grid grid-cols-[auto_1fr_auto_1fr] gap-x-8 gap-y-2">
			{#each metadataArr as [label, value]}
				{@render metadata(label, value ?? 'Unknown')}
			{/each}
		</div>
	</div>
</div>

{#snippet metadata(label: string, value: string | string[])}
	<span class="font-medium">
		{label.toLowerCase() === 'isbn'
			? 'ISBN'
			: label.replace(/_/g, ' ').replace(/\b\w/g, (c) => c.toUpperCase())}
	</span>
	<span class="text-muted-foreground">
		{#if label.toLowerCase() === 'published_at' && typeof value === 'string'}
			{new Date(value).toLocaleDateString('en-US', {
				year: 'numeric',
				month: 'long',
				day: 'numeric'
			})}
		{:else}
			{value}
		{/if}
	</span>
{/snippet}
