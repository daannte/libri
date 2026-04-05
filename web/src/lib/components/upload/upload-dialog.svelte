<script lang="ts">
	import { createClient } from '@shiori/api-client';

	import * as Dialog from '../ui/dialog';
	import { Button, buttonVariants } from '../ui/button';

	import FilesList from './files-list.svelte';
	import Dropzone from './dropzone.svelte';
	import { invalidate } from '$app/navigation';

	interface Props {
		id: number;
		isOpen: boolean;
	}

	let client = createClient({ fetch });

	let { id, isOpen = $bindable() }: Props = $props();

	let files = $state<File[]>([]);

	async function uploadFiles() {
		if (!files.length) return;

		const formData = new FormData();
		files.forEach((f) => {
			formData.append('files', f);
		});

		try {
			let res = await client.POST('/api/v1/libraries/{id}/media', {
				params: { path: { id } },
				// @ts-ignore
				body: formData
			});
			if (!res.data || res.error) throw new Error('Failed to upload files: ', res.error);
			isOpen = false;
			invalidate('libraries:media');
		} catch (e) {
			console.error(e);
		}
	}
</script>

<Dialog.Root bind:open={isOpen}>
	<Dialog.Content showCloseButton={false}>
		<Dialog.Header>
			<Dialog.Title>Upload Files</Dialog.Title>
		</Dialog.Header>
		<Dropzone bind:files />
		<FilesList bind:files />
		<Dialog.Footer>
			<Dialog.Close onclick={() => (files = [])} class={buttonVariants({ variant: 'secondary' })}
				>Cancel</Dialog.Close
			>
			<Button onclick={uploadFiles}>Upload</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
