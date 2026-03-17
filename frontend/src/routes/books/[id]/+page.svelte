<script lang="ts">
	import Button from '$lib/components/ui/button.svelte';
	import { Database, Download } from '@lucide/svelte';

	let data = {
		title: 'Schoolgirl',
		authors: [
			{ name: 'Osamu Dazai', bio: null },
			{ name: 'Allison Markin Powell', bio: null }
		],
		description: `Osamu Dazai's 1939 novella depicts a day in the life of a Tokyo schoolgirl. 
		Propelling Dazai into the literary elite of post-war Japan, <i>Schoolgirl</i> gained notoriety 
		for its ironic and inventive use of language. Now it illuminates the prevalent social structures 
		of a lost time, as well as the struggle of the individual against "them" -- a theme that occupied 
		Dazai's life both personally and professionally. This new translation preserves the playful language 
		of the original and offers the reader a new window into the mind of one of the greatest Japanese authors of the 20th century.`,
		cover_image:
			'https://m.media-amazon.com/images/S/compressed.photo.goodreads.com/books/1647450251i/12483882.jpg',
		asin: '1935548085',
		isbn: '1935548085',
		isbn13: '9781935548089',
		pages: 103,
		publication_time: '2011-10-15T00:00:00Z',
		publisher: 'One Peace Books',
		language: 'English'
	};
</script>

<div class="mx-auto h-screen max-w-full bg-cream-200 p-8">
	<div class="mx-auto flex max-w-5xl">
		<img src={data.cover_image} alt={`${data.title} cover image`} class="h-96" />

		<div class="mt-8 ml-16 flex flex-col gap-4">
			<h1 class="text-4xl font-semibold">{data.title}</h1>
			<p class="flex flex-wrap items-center gap-2 text-lg text-gray-600">
				{#each data.authors as author, i}
					<span>{author.name}{i < data.authors.length - 1 ? ',' : ''}</span>
				{/each}
			</p>
			<div class="mt-auto flex justify-end gap-4 bg-cream-100 p-4">
				<Button aria-label="Metadata" variant="outline" size="icon">
					<Database />
				</Button>
				<Button aria-label="Download" variant="outline" size="icon">
					<Download />
				</Button>
			</div>
			<div class="border border-cream-800/50"></div>
		</div>
	</div>
	<div class="mx-auto mt-16 flex max-w-7xl gap-8 bg-cream-100 p-8 px-16">
		<div class="flex-1">
			<h3 class="mb-4 text-2xl font-semibold">Description</h3>
			<p class="leading-relaxed text-gray-700">{@html data.description}</p>
		</div>

		<div class="ml-16 flex-1 flex-col gap-6">
			<div>
				<h3 class="mb-2 text-2xl font-semibold">Language</h3>
				<p class="leading-relaxed text-gray-700">{data.language}</p>
			</div>

			<div>
				<h3 class="mb-2 text-2xl font-semibold">Pages</h3>
				<p class="leading-relaxed text-gray-700">{data.pages}</p>
			</div>

			<div>
				<h3 class="mb-2 text-2xl font-semibold">Published</h3>
				<p class="leading-relaxed text-gray-700">
					{new Date(data.publication_time).toLocaleDateString('en-US', {
						year: 'numeric',
						month: 'long',
						day: 'numeric'
					})}
				</p>
			</div>
		</div>
	</div>
</div>
