<script lang="ts">
	import { resolve } from '$app/paths';
	import { get_cover_url, type operations } from '@shiori/api-client';
	import { Progress } from './ui/progress';

	type Media = operations['get_media']['responses']['200']['content']['application/json'];

	interface Props {
		media: Media;
	}

	let { media }: Props = $props();
</script>

<a
	href={resolve('/(app)/media/[media_id]', { media_id: media.id.toString() })}
	class="group w-full"
>
	<div class="relative aspect-2/3 overflow-hidden rounded-xl sm:rounded-2xl">
		<img
			class="h-full w-full object-cover transition duration-300 ease-out group-hover:scale-103"
			src={get_cover_url(media.cover_path) ?? ''}
			alt={`${media.name} cover image`}
		/>

		{#if media.progress}
			<div class="absolute right-2 bottom-2 left-2">
				<Progress value={media.progress.percentage_completed} max={1} />
			</div>
		{/if}
	</div>

	<div class="mt-1 px-0.5 sm:mt-2 sm:px-1">
		<h3 class="text-sm font-medium sm:text-base">
			{media.name}
		</h3>
	</div>
</a>
