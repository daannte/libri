<script lang="ts">
	import { type components } from '@shiori/api-client';
	import * as Dialog from '../ui/dialog';
	import FolderRow from './folder-row.svelte';
	import { Button } from '../ui/button';

	type Directory = components['schemas']['EncodableDirectory'];

	interface Props {
		dirs: Directory[];
		parent?: string | null;
		isOpen: boolean;
		directory: Directory | undefined;
	}

	let { dirs, parent, isOpen = $bindable(), directory = $bindable() }: Props = $props();
</script>

<Dialog.Root bind:open={isOpen}>
	<Dialog.Content>
		<Dialog.Header>
			<Dialog.Title>Select Folder</Dialog.Title>
			<Dialog.Description>Paths are relative to the application's base directory</Dialog.Description
			>
		</Dialog.Header>
		{#if directory && parent != undefined && parent != null}
			<Button
				onclick={() => {
					if (directory) {
						directory.path = parent;
					}
				}}>Back</Button
			>
		{/if}
		<div class="mt-4 flex flex-col gap-2">
			{#each dirs as dir (dir.path)}
				<FolderRow {dir} onclick={(d: Directory) => (directory = d)} />
			{/each}
		</div>
	</Dialog.Content>
</Dialog.Root>
